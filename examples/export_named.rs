//! Synthetic qualification exporter, not a production CSV ingestion interface.
//! Reuses the committed reference-fixture reader and existing development deps.
//! Usage: cargo run --locked --example export_named -- FIXTURE_DIR NEW_OUTPUT_DIR
//!        log2_expression|log2_methylation_odds
#[path = "../tests/common/mod.rs"]
mod common;

use common::matrix::Matrix;
use limma_core::consumer::{fit_named_contrasts, InputScale, NamedMatrix, CONTRACT_ID};
use std::error::Error;
use std::ffi::OsString;
use std::path::Path;

const COLUMNS: [&str; 16] = [
    "feature_id",
    "contrast_id",
    "estimate",
    "stdev_unscaled",
    "s2_post",
    "ordinary_se",
    "moderated_se",
    "df_residual",
    "df_total",
    "t",
    "p_value",
    "adj_p_value",
    "ci_lower",
    "ci_upper",
    "non_estimable",
    "input_scale",
];

fn named(matrix: &Matrix) -> NamedMatrix<'_> {
    NamedMatrix {
        data: &matrix.data,
        row_ids: &matrix.row_names,
        column_ids: &matrix.col_names,
    }
}

fn run(args: &[OsString]) -> Result<(), Box<dyn Error>> {
    if args.len() != 3 {
        return Err(
            "usage: export_named FIXTURE_DIR NEW_OUTPUT_DIR log2_expression|log2_methylation_odds"
                .into(),
        );
    }
    let scale = match args[2].to_str() {
        Some("log2_expression") => InputScale::Log2Expression,
        Some("log2_methylation_odds") => InputScale::Log2MethylationOdds,
        _ => return Err("unsupported input scale".into()),
    };
    let input = Path::new(&args[0]);
    let output = Path::new(&args[1]);
    let [y, x, contrasts] =
        ["input_log2.csv", "design.csv", "contrasts.csv"].map(|file| Matrix::read(input, file));
    let result = fit_named_contrasts(named(&y), named(&x), named(&contrasts), scale)?;
    // Refuse to overwrite prior evidence; a failed command does not qualify a directory.
    std::fs::create_dir(output)?;
    let mut writer = csv::Writer::from_path(output.join("estimates.csv"))?;
    writer.write_record(COLUMNS)?;
    for row in &result.rows {
        writer.write_record([
            row.feature_id.to_owned(),
            row.contrast_id.to_owned(),
            row.estimate.to_string(),
            row.stdev_unscaled.to_string(),
            row.s2_post.to_string(),
            row.ordinary_se.to_string(),
            row.moderated_se.to_string(),
            row.df_residual.to_string(),
            row.df_total.to_string(),
            row.t.to_string(),
            row.p_value.to_string(),
            row.adj_p_value.to_string(),
            row.ci_lower.to_string(),
            row.ci_upper.to_string(),
            row.non_estimable.to_string(),
            scale.as_str().to_owned(),
        ])?;
    }
    writer.flush()?;
    let metadata = serde_json::json!({
        "schema_version": 1,
        "contract_id": CONTRACT_ID,
        "crate_version": limma_core::CRATE_VERSION,
        "purpose": "synthetic_qualification",
        "input_scale": scale.as_str(),
        "feature_ids": result.feature_ids,
        "sample_ids": result.sample_ids,
        "design_ids": result.design_ids,
        "contrast_ids": result.contrast_ids,
        "layout": "contrast_major_then_feature",
        "columns": COLUMNS,
        "method": {
            "fit": "unweighted_least_squares",
            "moderation": "ebayes_default",
            "robust": false,
            "trend": false,
            "adjustment": "BH_per_contrast",
            "confidence_level": 0.95,
            "numeric_type": "float64",
            "missing_value": "NaN"
        },
        "warnings": result.warnings
    });
    std::fs::write(
        output.join("metadata.json"),
        serde_json::to_vec_pretty(&metadata)?,
    )?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run(&std::env::args_os().skip(1).collect::<Vec<_>>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Temporary(PathBuf);

    impl Temporary {
        fn new() -> Self {
            static NEXT: AtomicUsize = AtomicUsize::new(0);
            let path = std::env::temp_dir().join(format!(
                "limma-named-export-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for Temporary {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).unwrap();
        }
    }

    #[test]
    fn fixture_exchange_roundtrips_every_float_and_preserves_missingness() {
        let temporary = Temporary::new();
        for (case, scale) in [
            ("complete", InputScale::Log2Expression),
            ("missing", InputScale::Log2MethylationOdds),
        ] {
            // The second label tests metadata transport, not methylation admission.
            let input = common::validation_root().join("fresh-reference").join(case);
            let output = temporary.0.join(case);
            run(&[
                input.clone().into(),
                output.clone().into(),
                scale.as_str().into(),
            ])
            .unwrap();
            let [y, x, contrasts] = ["input_log2.csv", "design.csv", "contrasts.csv"]
                .map(|file| Matrix::read(&input, file));
            let expected =
                fit_named_contrasts(named(&y), named(&x), named(&contrasts), scale).unwrap();
            let metadata: serde_json::Value =
                serde_json::from_slice(&std::fs::read(output.join("metadata.json")).unwrap())
                    .unwrap();
            assert_eq!(metadata["contract_id"], CONTRACT_ID);
            assert_eq!(metadata["input_scale"], scale.as_str());
            assert_eq!(metadata["feature_ids"], serde_json::json!(y.row_names));
            assert_eq!(metadata["sample_ids"], serde_json::json!(y.col_names));
            assert_eq!(metadata["design_ids"], serde_json::json!(x.col_names));
            assert_eq!(
                metadata["contrast_ids"],
                serde_json::json!(contrasts.col_names)
            );
            assert_eq!(metadata["warnings"], serde_json::json!(expected.warnings));
            let mut csv = csv::Reader::from_path(output.join("estimates.csv")).unwrap();
            assert_eq!(csv.headers().unwrap().iter().collect::<Vec<_>>(), COLUMNS);
            let records: Vec<_> = csv.records().map(Result::unwrap).collect();
            assert_eq!(records.len(), expected.rows.len());
            for (record, row) in records.iter().zip(&expected.rows) {
                assert_eq!(&record[0], row.feature_id);
                assert_eq!(&record[1], row.contrast_id);
                let values = [
                    row.estimate,
                    row.stdev_unscaled,
                    row.s2_post,
                    row.ordinary_se,
                    row.moderated_se,
                    row.df_residual,
                    row.df_total,
                    row.t,
                    row.p_value,
                    row.adj_p_value,
                    row.ci_lower,
                    row.ci_upper,
                ];
                for (text, value) in record.iter().skip(2).zip(values) {
                    let actual: f64 = text.parse().unwrap();
                    if value.is_nan() {
                        assert!(actual.is_nan());
                    } else {
                        assert_eq!(
                            actual.to_bits(),
                            value.to_bits(),
                            "float64 text lost precision"
                        );
                    }
                }
                assert_eq!(&record[14], row.non_estimable.to_string());
                assert_eq!(&record[15], scale.as_str());
            }
        }
    }

    #[test]
    fn invalid_invocation_and_existing_output_are_rejected() {
        assert!(run(&[]).unwrap_err().to_string().contains("usage"));
        assert!(run(&["unused".into(), "unused".into(), "beta".into()])
            .unwrap_err()
            .to_string()
            .contains("unsupported input scale"));
        let temporary = Temporary::new();
        let marker = temporary.0.join("preserve.txt");
        std::fs::write(&marker, "prior evidence").unwrap();
        let input = common::validation_root().join("fresh-reference/complete");
        assert!(run(&[
            input.into(),
            temporary.0.clone().into(),
            "log2_expression".into()
        ])
        .is_err());
        assert_eq!(std::fs::read_to_string(marker).unwrap(), "prior evidence");
        assert!(!temporary.0.join("estimates.csv").exists());
    }
}
