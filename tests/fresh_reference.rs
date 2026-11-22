//! Fresh public synthetic R oracle (LM01a), using the existing Matrix/Report harness.
//! The four cases are independent of the historical corpus. See the recipe and
//! reference-environment.json. R is needed only to regenerate, never at test runtime.
mod common;

use common::matrix::Matrix;
use common::{Report, Tol, EXACT};
use limma_core::contrasts::contrasts_fit;
use limma_core::ebayes::{at, ebayes, EBayesOptions};
use limma_core::fit::{lm_fit_matrix, MArrayLm};
use limma_core::toptable::{top_table_f, top_table_t};
use std::path::{Path, PathBuf};

const ABS: Tol = Tol {
    rel: 0.0,
    abs_floor: 1e-10,
};
const REL: Tol = Tol {
    rel: 1e-8,
    abs_floor: 0.0,
};
const PRIOR: Tol = Tol {
    rel: 1e-6,
    abs_floor: 0.0,
};
const PROB: Tol = Tol {
    rel: 1e-8,
    abs_floor: 1e-12,
};
// Unit-scale design: analytically zero covariance entries incur ~1e-17 QR
// rounding. The initial pure-relative failure is retained in LM01a evidence.
const COVARIANCE: Tol = Tol {
    rel: 1e-8,
    abs_floor: 1e-14,
};
// Same cancellation-aware policy as LODS_BLOCK in tests/matrix_ebayes.rs.
const LODS: Tol = Tol {
    rel: 1e-8,
    abs_floor: 1e-10,
};

fn reference_root() -> PathBuf {
    let dir = std::env::var_os("LIMMA_FRESH_REFERENCE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| common::validation_root().join("fresh-reference"));
    assert!(dir.is_dir(), "fresh reference absent: {}", dir.display());
    let mut versions =
        csv::Reader::from_path(dir.join("versions.csv")).expect("reference versions");
    let rows: Vec<Vec<String>> = versions
        .records()
        .map(|r| r.expect("versions row").iter().map(str::to_owned).collect())
        .collect();
    assert_eq!(
        rows,
        [["R", "4.5.0"], ["limma", "3.68.5"], ["statmod", "1.5.2"]],
        "reference version contract changed"
    );
    dir
}

fn compare(report: &mut Report, dir: &Path, file: &str, got: &[f64], tol: Tol) {
    let want = Matrix::read(dir, file);
    assert_eq!(got.len(), want.data.len(), "{file}: dimensions");
    if file.ends_with("cov_coefficients.csv") {
        let (max_abs, max_rel) =
            got.iter()
                .zip(&want.data)
                .fold((0.0_f64, 0.0_f64), |(a, r), (&actual, &expected)| {
                    let error = (actual - expected).abs();
                    (
                        a.max(error),
                        if expected != 0.0 {
                            r.max(error / expected.abs())
                        } else {
                            r
                        },
                    )
                });
        println!("{}/{file}: max absolute error {max_abs:e}; max relative error including near-zero references {max_rel:e}", dir.file_name().unwrap().to_string_lossy());
    }
    for (i, (&actual, &expected)) in got.iter().zip(&want.data).enumerate() {
        // Report checks NA/NaN and infinity in kind/sign; do not skip them.
        report.check(tol, format!("{file}[{i}]"), actual, expected);
    }
}

fn load(dir: &Path) -> (Matrix, MArrayLm, MArrayLm) {
    let y = Matrix::read(dir, "input_log2.csv");
    let x = Matrix::read(dir, "design.csv");
    let cm = Matrix::read(dir, "contrasts.csv");
    assert_eq!(y.col_names, x.row_names, "explicit sample order");
    assert_eq!(x.col_names, cm.row_names, "explicit coefficient order");
    let fit = lm_fit_matrix(&y.data, y.nrow, y.ncol, &x.data, x.ncol).expect("lmFit");
    let cf = contrasts_fit(&fit, &cm.data, cm.ncol).expect("estimable contrasts.fit");
    (y, fit, cf)
}

fn check_fit(case: &str) {
    let dir = reference_root().join(case);
    let (_, fit, cf) = load(&dir);
    let mut r = Report::new(&format!("fresh/{case}/fit"));
    for (name, values, tol) in [
        ("lmfit_coefficients.csv", &fit.coefficients, ABS),
        ("lmfit_stdev_unscaled.csv", &fit.stdev_unscaled, REL),
        (
            "lmfit_cov_coefficients.csv",
            &fit.cov_coefficients,
            COVARIANCE,
        ),
        ("contrasts_coefficients.csv", &cf.coefficients, ABS),
        ("contrasts_stdev_unscaled.csv", &cf.stdev_unscaled, REL),
        (
            "contrasts_cov_coefficients.csv",
            &cf.cov_coefficients,
            COVARIANCE,
        ),
    ] {
        compare(&mut r, &dir, name, values, tol);
    }
    let scalars: Vec<f64> = [&fit.sigma, &fit.df_residual, &fit.amean]
        .into_iter()
        .flat_map(|v| v.iter().copied())
        .collect();
    compare(&mut r, &dir, "lmfit_scalars.csv", &scalars, REL);
    compare(&mut r, &dir, "lmfit_rank.csv", &[fit.rank as f64], EXACT);
    let pivot: Vec<f64> = fit.pivot.iter().map(|&p| p as f64).collect();
    compare(&mut r, &dir, "lmfit_pivot.csv", &pivot, EXACT);
    r.finish();
}

#[test]
fn fresh_fit_complete() {
    check_fit("complete");
}

#[test]
fn fresh_fit_missing() {
    check_fit("missing");
}

#[test]
fn fresh_fit_missing_edges() {
    check_fit("missing_edges");
}

#[test]
fn fresh_fit_singular_design() {
    check_fit("singular_design");
}
#[test]
fn fresh_moderated_inference() {
    let root = reference_root();
    for case in ["complete", "missing"] {
        let dir = root.join(case);
        let (y, _, cf) = load(&dir);
        for robust in [false, true] {
            for trend in [false, true] {
                let tag = format!(
                    "ebayes_rob{}_trend{}",
                    if robust { "TRUE" } else { "FALSE" },
                    if trend { "TRUE" } else { "FALSE" }
                );
                let opts = EBayesOptions {
                    robust,
                    trend,
                    ..EBayesOptions::default()
                };
                let eb = ebayes(&cf, &opts).expect("eBayes");
                let mut r = Report::new(&format!("fresh/{case}/{tag}"));
                for (suffix, values, tol) in [
                    ("t", &eb.t, REL),
                    ("p", &eb.p_value, PROB),
                    ("lods", &eb.lods, LODS),
                ] {
                    compare(&mut r, &dir, &format!("{tag}_{suffix}.csv"), values, tol);
                }
                let se: Vec<f64> = cf
                    .stdev_unscaled
                    .iter()
                    .enumerate()
                    .map(|(i, s)| s * eb.s2_post[i % cf.ngenes].sqrt())
                    .collect();
                compare(&mut r, &dir, &format!("{tag}_moderated_se.csv"), &se, REL);
                let stats = Matrix::read(&dir, &format!("{tag}_scalars.csv"));
                for (name, values, tol) in [
                    ("s2_post", eb.s2_post.as_slice(), REL),
                    ("df_total", eb.df_total.as_slice(), PRIOR),
                    ("F", eb.f.as_ref().expect("moderated F").as_slice(), REL),
                    (
                        "F_p",
                        eb.f_p_value.as_ref().expect("F probability").as_slice(),
                        PROB,
                    ),
                    ("s2_prior", eb.s2_prior.as_slice(), PRIOR),
                    ("df_prior", eb.df_prior.as_slice(), PRIOR),
                ] {
                    let want = stats.col(name);
                    assert!(values.len() == 1 || values.len() == want.len());
                    for (i, &value) in want.iter().enumerate() {
                        r.check(tol, format!("{name}[{i}]"), at(values, i), value);
                    }
                }
                for k in 0..cf.ncoef {
                    let want = Matrix::read(&dir, &format!("{tag}_toptable_{}.csv", k + 1));
                    let rows = top_table_t(&cf, &eb, k, 0.95).expect("topTable t");
                    assert_eq!(rows.len(), want.nrow);
                    for row in rows {
                        let name = &y.row_names[row.gene];
                        let i = want
                            .row_names
                            .iter()
                            .position(|n| n == name)
                            .expect("feature name");
                        for (field, value, tol) in [
                            ("logFC", row.log_fc, ABS),
                            ("CI.L", row.ci_l, ABS),
                            ("CI.R", row.ci_r, ABS),
                            ("AveExpr", row.ave_expr, REL),
                            ("t", row.t, REL),
                            ("P.Value", row.p_value, PROB),
                            ("adj.P.Val", row.adj_p_value, PROB),
                            ("B", row.b, LODS),
                        ] {
                            r.check(
                                tol,
                                format!("tt{k}/{name}/{field}"),
                                value,
                                want.col(field)[i],
                            );
                        }
                    }
                }
                let want = Matrix::read(&dir, &format!("{tag}_toptable_F.csv"));
                let rows = top_table_f(&cf, &eb).expect("topTable F");
                assert_eq!(rows.len(), want.nrow);
                for row in rows {
                    let name = &y.row_names[row.gene];
                    let i = want
                        .row_names
                        .iter()
                        .position(|n| n == name)
                        .expect("feature name");
                    for (k, &coefficient) in row.coefs.iter().enumerate() {
                        r.check(
                            ABS,
                            format!("ttF/{name}/coef{k}"),
                            coefficient,
                            want.get(i, k),
                        );
                    }
                    for (field, value, tol) in [
                        ("AveExpr", row.ave_expr, REL),
                        ("F", row.f, REL),
                        ("P.Value", row.p_value, PROB),
                        ("adj.P.Val", row.adj_p_value, PROB),
                    ] {
                        r.check(
                            tol,
                            format!("ttF/{name}/{field}"),
                            value,
                            want.col(field)[i],
                        );
                    }
                }
                r.finish();
            }
        }
    }
}

#[test]
fn fresh_non_estimable_contrast_rejected() {
    let dir = reference_root().join("singular_design");
    let (_, fit, _) = load(&dir);
    let invalid = Matrix::read(&dir, "invalid_contrasts.csv");
    let expected = std::fs::read_to_string(dir.join("invalid_contrast_error.txt"))
        .expect("fresh R error reference");
    let error =
        contrasts_fit(&fit, &invalid.data, invalid.ncol).expect_err("non-estimable contrast");
    let limma_core::LimmaError::Invalid(message) = error;
    assert_eq!(message, expected.trim());
}
