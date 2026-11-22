//! Named consumer contract against the independently generated R 4.5.0 /
//! limma 3.68.5 complete and missing fixtures. No reference is regenerated here.
mod common;

use common::matrix::Matrix;
use common::{Report, Tol};
use limma_core::consumer::{fit_named_contrasts, InputScale, NamedMatrix};
use std::path::PathBuf;

const ABS: Tol = Tol {
    rel: 0.0,
    abs_floor: 1e-10,
};
const REL: Tol = Tol {
    rel: 1e-8,
    abs_floor: 0.0,
};
const DF: Tol = Tol {
    rel: 1e-6,
    abs_floor: 0.0,
};
const PROB: Tol = Tol {
    rel: 1e-8,
    abs_floor: 1e-12,
};

fn reference(case: &str) -> PathBuf {
    common::validation_root().join("fresh-reference").join(case)
}

fn inputs(case: &str) -> [Matrix; 3] {
    let dir = reference(case);
    ["input_log2.csv", "design.csv", "contrasts.csv"].map(|file| Matrix::read(&dir, file))
}

fn named(matrix: &Matrix) -> NamedMatrix<'_> {
    NamedMatrix {
        data: &matrix.data,
        row_ids: &matrix.row_names,
        column_ids: &matrix.col_names,
    }
}

fn rejected(matrices: &[Matrix; 3], expected: &str) {
    let error = fit_named_contrasts(
        named(&matrices[0]),
        named(&matrices[1]),
        named(&matrices[2]),
        InputScale::Log2Expression,
    )
    .expect_err("unsupported named input accepted");
    assert!(
        error.to_string().contains(expected),
        "expected {expected:?}, got {error}"
    );
}

#[test]
fn named_results_preserve_full_reference_universe_and_scaled_uncertainty() {
    for case in ["complete", "missing"] {
        let dir = reference(case);
        let [y, x, contrasts] = inputs(case);
        let got = fit_named_contrasts(
            named(&y),
            named(&x),
            named(&contrasts),
            InputScale::Log2Expression,
        )
        .expect("supported independent reference");
        assert_eq!(got.input_scale, InputScale::Log2Expression);
        assert_eq!(got.feature_ids, y.row_names);
        assert_eq!(got.sample_ids, y.col_names);
        assert_eq!(got.design_ids, x.col_names);
        assert_eq!(got.contrast_ids, contrasts.col_names);
        assert_eq!(got.rows.len(), y.nrow * contrasts.ncol);
        assert!(got.warnings.is_empty());
        let se = Matrix::read(&dir, "ebayes_robFALSE_trendFALSE_moderated_se.csv");
        let unscaled = Matrix::read(&dir, "contrasts_stdev_unscaled.csv");
        let eb = Matrix::read(&dir, "ebayes_robFALSE_trendFALSE_scalars.csv");
        let lm = Matrix::read(&dir, "lmfit_scalars.csv");
        let mut report = Report::new(&format!("named/{case}"));
        for c in 0..contrasts.ncol {
            let table = Matrix::read(
                &dir,
                &format!("ebayes_robFALSE_trendFALSE_toptable_{}.csv", c + 1),
            );
            assert_eq!(table.row_names, y.row_names);
            for g in 0..y.nrow {
                let row = &got.rows[c * y.nrow + g];
                assert_eq!(row.feature_id, y.row_names[g]);
                assert_eq!(row.contrast_id, contrasts.col_names[c]);
                assert_eq!(row.non_estimable, table.col("logFC")[g].is_nan());
                for (name, actual, expected, tolerance) in [
                    ("estimate", row.estimate, table.col("logFC")[g], ABS),
                    (
                        "stdev_unscaled",
                        row.stdev_unscaled,
                        unscaled.get(g, c),
                        REL,
                    ),
                    ("s2_post", row.s2_post, eb.col("s2_post")[g], REL),
                    (
                        "ordinary_se",
                        row.ordinary_se,
                        lm.col("sigma")[g] * unscaled.get(g, c),
                        REL,
                    ),
                    ("moderated_se", row.moderated_se, se.get(g, c), REL),
                    (
                        "df_residual",
                        row.df_residual,
                        lm.col("df_residual")[g],
                        REL,
                    ),
                    ("df_total", row.df_total, eb.col("df_total")[g], DF),
                    ("t", row.t, table.col("t")[g], REL),
                    ("p_value", row.p_value, table.col("P.Value")[g], PROB),
                    (
                        "adj_p_value",
                        row.adj_p_value,
                        table.col("adj.P.Val")[g],
                        PROB,
                    ),
                    ("ci_lower", row.ci_lower, table.col("CI.L")[g], ABS),
                    ("ci_upper", row.ci_upper, table.col("CI.R")[g], ABS),
                ] {
                    report.check(tolerance, format!("{c}/{g}/{name}"), actual, expected);
                }
                // Independent SE above is the oracle; this identity also prevents
                // exporting the unscaled design SD under the moderated-SE label.
                if !row.non_estimable {
                    report.check(
                        REL,
                        format!("{c}/{g}/estimate_over_se"),
                        row.estimate / row.moderated_se,
                        row.t,
                    );
                }
            }
        }
        report.finish();
    }
}

#[test]
fn ambiguous_axes_and_silent_reordering_are_rejected() {
    // Every matrix axis is validated, including contrasts and the feature universe.
    for matrix in 0..3 {
        for rows in [false, true] {
            for duplicate in [false, true] {
                let mut data = inputs("complete");
                let ids = if rows {
                    &mut data[matrix].row_names
                } else {
                    &mut data[matrix].col_names
                };
                ids[0] = if duplicate {
                    ids[1].clone()
                } else {
                    "  ".into()
                };
                rejected(&data, "IDs must be unique and nonblank");
            }
        }
    }
    for (matrix, rows, expected) in [
        (0, false, "sample order"),
        (1, true, "sample order"),
        (1, false, "coefficient order"),
        (2, true, "coefficient order"),
    ] {
        for unknown in [false, true] {
            let mut data = inputs("complete");
            let ids = if rows {
                &mut data[matrix].row_names
            } else {
                &mut data[matrix].col_names
            };
            if unknown {
                ids[0] = "unmapped".into();
            } else {
                ids.swap(0, 1);
            }
            rejected(&data, expected);
        }
    }
}

#[test]
fn malformed_shapes_and_nonfinite_values_return_errors() {
    for matrix in 0..3 {
        let mut data = inputs("complete");
        data[matrix].data.pop();
        rejected(&data, "shape");
        for rows in [false, true] {
            let mut data = inputs("complete");
            data[matrix].data.clear();
            if rows {
                data[matrix].row_names.clear();
            } else {
                data[matrix].col_names.clear();
            }
            rejected(&data, "nonempty");
        }
        for value in [f64::INFINITY, f64::NEG_INFINITY, f64::NAN] {
            if matrix == 0 && value.is_nan() {
                continue; // Supported missingness is checked against R above.
            }
            let mut data = inputs("complete");
            data[matrix].data[0] = value;
            rejected(&data, "finite");
        }
    }
}

#[test]
fn unqualified_designs_zero_df_and_zero_contrasts_are_rejected() {
    rejected(&inputs("singular_design"), "full-rank design");
    rejected(
        &inputs("missing_edges"),
        "positive residual degrees of freedom",
    );
    let mut data = inputs("complete");
    // A valid-sized saturated model must fail before the low-level fitting call.
    data[1].col_names = data[1].row_names.clone();
    data[1].data = (0..data[1].nrow * data[1].nrow)
        .map(|k| {
            if k / data[1].nrow == k % data[1].nrow {
                1.0
            } else {
                0.0
            }
        })
        .collect();
    data[2].row_names = data[1].col_names.clone();
    data[2].data = vec![1.0; data[2].row_names.len() * data[2].col_names.len()];
    rejected(&data, "more samples than design coefficients");
    for c in 0..2 {
        let mut data = inputs("complete");
        let ncoef = data[2].nrow;
        data[2].data[c * ncoef..(c + 1) * ncoef].fill(0.0);
        rejected(&data, "each contrast must be nonzero");
    }
}

#[test]
fn explicit_scale_and_exact_unusual_ids_are_preserved() {
    let [mut y, x, mut contrasts] = inputs("complete");
    y.row_names[0] = "feature,\"quoted\"".into();
    contrasts.col_names[0] = "slope,not a fold change".into();
    let result = fit_named_contrasts(
        named(&y),
        named(&x),
        named(&contrasts),
        InputScale::Log2MethylationOdds,
    )
    .expect("explicit scale and opaque IDs");
    assert_eq!(result.rows[0].feature_id, y.row_names[0]);
    assert_eq!(result.rows[0].contrast_id, contrasts.col_names[0]);
    assert_eq!(result.input_scale.as_str(), "log2_methylation_odds");
    assert_eq!(InputScale::Log2Expression.as_str(), "log2_expression");
}

#[test]
fn diagnostic_warnings_and_unestimable_rows_are_not_discarded() {
    let [mut y, x, contrasts] = inputs("complete");
    // Remove all group-B observations while retaining positive residual df.
    // This checks diagnostic transport, not an additional R numerical claim.
    for sample in 4..8 {
        y.data[sample * y.nrow..(sample + 1) * y.nrow].fill(f64::NAN);
    }
    let result = fit_named_contrasts(
        named(&y),
        named(&x),
        named(&contrasts),
        InputScale::Log2Expression,
    )
    .expect("positive-df per-feature rank loss");
    assert_eq!(result.warnings, [limma_core::ebayes::VAR_PRIOR_WARNING]);
    assert_eq!(result.rows.len(), y.nrow * contrasts.ncol);
    assert!(result.rows[..y.nrow].iter().all(|row| row.non_estimable));
    assert!(result.rows[y.nrow..].iter().all(|row| !row.non_estimable));
}
