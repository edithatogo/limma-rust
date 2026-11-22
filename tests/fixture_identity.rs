//! Identity contracts for the existing CSV reference harness, not a production adapter.
mod common;

use common::matrix::{assert_model_identity, Matrix};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

struct FixtureDir(PathBuf);

impl FixtureDir {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "limma-fixture-identity-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).expect("isolated fixture directory");
        Self(path)
    }

    fn write(&self, file: &str, text: &str) {
        std::fs::write(self.0.join(file), text).expect("write synthetic fixture");
    }
}

impl Drop for FixtureDir {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).expect("remove isolated fixture directory");
    }
}

#[test]
fn ambiguous_matrix_axes_are_rejected() {
    let dir = FixtureDir::new();
    let mut accepted = Vec::new();
    for (name, text) in [
        ("duplicate_rows", "row,s1,s2\nf1,1,2\nf1,3,4\n"),
        ("duplicate_columns", "row,s1,s1\nf1,1,2\nf2,3,4\n"),
        ("empty_row", "row,s1,s2\n,1,2\nf2,3,4\n"),
        ("empty_column", "row,,s2\nf1,1,2\nf2,3,4\n"),
        ("blank_row", "row,s1,s2\n\"  \",1,2\nf2,3,4\n"),
        ("blank_column", "row,\"  \",s2\nf1,1,2\nf2,3,4\n"),
    ] {
        dir.write("matrix.csv", text);
        if std::panic::catch_unwind(|| Matrix::read(&dir.0, "matrix.csv")).is_ok() {
            accepted.push(name);
        }
    }
    assert!(
        accepted.is_empty(),
        "ambiguous fixture IDs accepted: {accepted:?}"
    );
}

#[test]
fn valid_quoted_axes_and_numeric_order_are_preserved() {
    let dir = FixtureDir::new();
    dir.write(
        "matrix.csv",
        "row,\"sample,1\",sample2\n\"feature,1\",1,NA\nfeature2,3,4\n",
    );
    let matrix = Matrix::read(&dir.0, "matrix.csv");
    assert_eq!(matrix.row_names, ["feature,1", "feature2"]);
    assert_eq!(matrix.col_names, ["sample,1", "sample2"]);
    assert_eq!(&matrix.data[..2], [1.0, 3.0]);
    assert!(matrix.data[2].is_nan());
    assert_eq!(matrix.data[3], 4.0);
}

#[test]
fn exact_model_identity_is_required() {
    let dir = FixtureDir::new();
    for (name, input, design, contrasts, expected_error) in [
        (
            "aligned",
            "row,s1,s2\nf1,1,2\nf2,3,4\n",
            "row,intercept,group\ns1,1,0\ns2,1,1\n",
            "row,B_minus_A\nintercept,0\ngroup,1\n",
            None,
        ),
        (
            "reordered_samples",
            "row,s2,s1\nf1,1,2\nf2,3,4\n",
            "row,intercept,group\ns1,1,0\ns2,1,1\n",
            "row,B_minus_A\nintercept,0\ngroup,1\n",
            Some("sample order"),
        ),
        (
            "reordered_design_rows",
            "row,s1,s2\nf1,1,2\nf2,3,4\n",
            "row,intercept,group\ns2,1,0\ns1,1,1\n",
            "row,B_minus_A\nintercept,0\ngroup,1\n",
            Some("sample order"),
        ),
        (
            "unmapped_positional_design",
            "row,s1,s2\nf1,1,2\nf2,3,4\n",
            "row,intercept,group\n1,1,0\n2,1,1\n",
            "row,B_minus_A\nintercept,0\ngroup,1\n",
            Some("sample order"),
        ),
        (
            "reordered_contrast_rows",
            "row,s1,s2\nf1,1,2\nf2,3,4\n",
            "row,intercept,group\ns1,1,0\ns2,1,1\n",
            "row,B_minus_A\ngroup,0\nintercept,1\n",
            Some("coefficient order"),
        ),
        (
            "foreign_sample",
            "row,s1,s2\nf1,1,2\nf2,3,4\n",
            "row,intercept,group\ns1,1,0\nunknown,1,1\n",
            "row,B_minus_A\nintercept,0\ngroup,1\n",
            Some("sample order"),
        ),
        (
            "foreign_coefficient",
            "row,s1,s2\nf1,1,2\nf2,3,4\n",
            "row,intercept,group\ns1,1,0\ns2,1,1\n",
            "row,B_minus_A\nintercept,0\nunknown,1\n",
            Some("coefficient order"),
        ),
    ] {
        dir.write("input.csv", input);
        dir.write("design.csv", design);
        dir.write("contrasts.csv", contrasts);
        let result = std::panic::catch_unwind(|| {
            let y = Matrix::read(&dir.0, "input.csv");
            let x = Matrix::read(&dir.0, "design.csv");
            let cm = Matrix::read(&dir.0, "contrasts.csv");
            assert_model_identity(&y, &x, Some(&cm));
        });
        match expected_error {
            None => assert!(result.is_ok(), "valid named case rejected: {name}"),
            Some(expected) => {
                let error = result.err().unwrap_or_else(|| panic!("{name} accepted"));
                let message = error
                    .downcast_ref::<String>()
                    .map(String::as_str)
                    .or_else(|| error.downcast_ref::<&str>().copied())
                    .expect("identity error message");
                assert!(message.contains(expected), "{name}: {message}");
            }
        }
    }
}
