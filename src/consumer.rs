//! A named, bounded consumer of the existing least-squares and eBayes core.
//!
//! This interface preserves the entire supplied feature universe and rejects
//! misaligned names. It performs no parsing, preprocessing, filtering or implicit
//! reordering. See `docs/named-consumer-v1.md` for the supported domain and evidence.

use std::collections::HashSet;

use crate::contrasts::contrasts_fit;
use crate::ebayes::{ebayes, EBayesOptions};
use crate::fit::lm_fit_matrix;
use crate::toptable::adj_p_and_ci_margin;
use crate::{LimmaError, Result};

/// Version of the named numerical contract, independent of crate release numbering.
pub const CONTRACT_ID: &str = "limma-named-contrasts-v1";

/// The caller declares the input scale; it is never inferred from numeric values.
/// An estimate is the requested linear contrast on this scale. Its interpretation
/// also depends on the design and contrast coefficients (for example, a slope).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputScale {
    Log2Expression,
    Log2MethylationOdds,
}

impl InputScale {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Log2Expression => "log2_expression",
            Self::Log2MethylationOdds => "log2_methylation_odds",
        }
    }
}

/// A borrowed column-major float64 matrix. Axis lengths define its dimensions.
#[derive(Debug, Clone, Copy)]
pub struct NamedMatrix<'a> {
    pub data: &'a [f64],
    pub row_ids: &'a [String],
    pub column_ids: &'a [String],
}

impl NamedMatrix<'_> {
    fn validate(&self, label: &str) -> Result<()> {
        if self.row_ids.is_empty() || self.column_ids.is_empty() {
            return Err(LimmaError::Invalid(format!(
                "{label}: axes must be nonempty"
            )));
        }
        if self.row_ids.len().checked_mul(self.column_ids.len()) != Some(self.data.len()) {
            return Err(LimmaError::Invalid(format!(
                "{label}: data shape does not match IDs"
            )));
        }
        for ids in [self.row_ids, self.column_ids] {
            let mut seen = HashSet::with_capacity(ids.len());
            if ids
                .iter()
                .any(|id| id.trim().is_empty() || !seen.insert(id))
            {
                return Err(LimmaError::Invalid(format!(
                    "{label}: IDs must be unique and nonblank"
                )));
            }
        }
        Ok(())
    }
}

/// One feature/contrast result. NaN values retain their numerical meaning; a
/// non-estimable result remains in its original position in the feature universe.
#[derive(Debug)]
pub struct NamedEstimate<'a> {
    pub feature_id: &'a str,
    pub contrast_id: &'a str,
    pub estimate: f64,
    /// Design uncertainty; this is not an effect standard error by itself.
    pub stdev_unscaled: f64,
    pub s2_post: f64,
    /// Residual sigma times unscaled design SD.
    pub ordinary_se: f64,
    /// Square root of posterior residual variance times unscaled design SD.
    pub moderated_se: f64,
    pub df_residual: f64,
    pub df_total: f64,
    pub t: f64,
    pub p_value: f64,
    /// Existing limma-compatible BH adjustment within this full contrast family.
    pub adj_p_value: f64,
    /// Bounds of the two-sided 95% moderated t confidence interval.
    pub ci_lower: f64,
    pub ci_upper: f64,
    /// Whether the coefficient or its unscaled SD is NaN, not a p-value filter.
    pub non_estimable: bool,
}

#[derive(Debug)]
pub struct NamedAnalysis<'a> {
    pub input_scale: InputScale,
    pub feature_ids: &'a [String],
    pub sample_ids: &'a [String],
    pub design_ids: &'a [String],
    pub contrast_ids: &'a [String],
    /// Contrast-major, then original feature order; never ranked or filtered.
    pub rows: Vec<NamedEstimate<'a>>,
    /// Original eBayes diagnostic warnings, in order.
    pub warnings: Vec<&'static str>,
}

/// Fit unweighted least squares, named contrasts and default eBayes to the entire
/// supplied feature universe. No weights, blocks, robust/trend options or joint-F
/// export are accepted by this v1 interface.
///
/// All IDs must be unique and nonblank, expression columns must exactly equal
/// design rows, and design columns must exactly equal contrast rows. Expressions
/// may contain NaN (missing), but no infinities. Design and contrasts must be finite
/// and each contrast must be nonzero. Global rank loss, saturated designs and any
/// feature with zero residual degrees of freedom are outside this bounded contract.
/// Per-feature rank loss with positive residual df preserves non-estimable NaNs.
pub fn fit_named_contrasts<'a>(
    expression: NamedMatrix<'a>,
    design: NamedMatrix<'a>,
    contrasts: NamedMatrix<'a>,
    input_scale: InputScale,
) -> Result<NamedAnalysis<'a>> {
    expression.validate("expression")?;
    design.validate("design")?;
    contrasts.validate("contrasts")?;
    if expression.column_ids != design.row_ids {
        return Err(LimmaError::Invalid(
            "expression/design sample order must match exactly".into(),
        ));
    }
    if design.column_ids != contrasts.row_ids {
        return Err(LimmaError::Invalid(
            "design/contrast coefficient order must match exactly".into(),
        ));
    }
    if expression.data.iter().any(|v| v.is_infinite()) {
        return Err(LimmaError::Invalid(
            "expression must contain finite values or NaN missing values".into(),
        ));
    }
    if design
        .data
        .iter()
        .chain(contrasts.data)
        .any(|v| !v.is_finite())
    {
        return Err(LimmaError::Invalid(
            "design and contrasts must contain only finite values".into(),
        ));
    }
    let ngenes = expression.row_ids.len();
    let nsamples = expression.column_ids.len();
    let ncoef = design.column_ids.len();
    if nsamples <= ncoef {
        return Err(LimmaError::Invalid(
            "named v1 requires more samples than design coefficients".into(),
        ));
    }
    if contrasts
        .data
        .chunks(ncoef)
        .any(|column| column.iter().all(|&v| v == 0.0))
    {
        return Err(LimmaError::Invalid("each contrast must be nonzero".into()));
    }
    let nrows = ngenes
        .checked_mul(contrasts.column_ids.len())
        .ok_or_else(|| LimmaError::Invalid("output shape is too large".into()))?;
    let fit = lm_fit_matrix(expression.data, ngenes, nsamples, design.data, ncoef)?;
    if fit.rank != ncoef {
        return Err(LimmaError::Invalid(
            "named v1 requires a full-rank design".into(),
        ));
    }
    if fit.df_residual.iter().any(|&df| df <= 0.0) {
        return Err(LimmaError::Invalid(
            "named v1 requires positive residual degrees of freedom for every feature".into(),
        ));
    }
    let fit = contrasts_fit(&fit, contrasts.data, contrasts.column_ids.len())?;
    let eb = ebayes(&fit, &EBayesOptions::default())?;
    let mut rows = Vec::with_capacity(nrows);
    for (c, contrast_id) in contrasts.column_ids.iter().enumerate() {
        let (adj, margin) = adj_p_and_ci_margin(&fit, &eb, c, 0.95);
        for (g, feature_id) in expression.row_ids.iter().enumerate() {
            let k = c * ngenes + g;
            rows.push(NamedEstimate {
                feature_id,
                contrast_id,
                estimate: fit.coefficients[k],
                stdev_unscaled: fit.stdev_unscaled[k],
                s2_post: eb.s2_post[g],
                ordinary_se: fit.sigma[g] * fit.stdev_unscaled[k],
                moderated_se: eb.s2_post[g].sqrt() * fit.stdev_unscaled[k],
                df_residual: fit.df_residual[g],
                df_total: eb.df_total[g],
                t: eb.t[k],
                p_value: eb.p_value[k],
                adj_p_value: adj[g],
                ci_lower: fit.coefficients[k] - margin[g],
                ci_upper: fit.coefficients[k] + margin[g],
                non_estimable: fit.coefficients[k].is_nan() || fit.stdev_unscaled[k].is_nan(),
            });
        }
    }
    Ok(NamedAnalysis {
        input_scale,
        feature_ids: expression.row_ids,
        sample_ids: expression.column_ids,
        design_ids: design.column_ids,
        contrast_ids: contrasts.column_ids,
        rows,
        warnings: eb.warnings,
    })
}
