//! Reader for the `matrix/<case>/*.csv` goldens: a `row` column of quoted names, then
//! numeric columns; `NA` becomes `NaN`. Values are returned column-major so they drop
//! straight into the crate's matrix slices.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use super::{case_file, corpus_root};

pub struct Matrix {
    pub file: String,
    pub row_names: Vec<String>,
    pub col_names: Vec<String>,
    pub nrow: usize,
    pub ncol: usize,
    /// Column-major `nrow x ncol`.
    pub data: Vec<f64>,
}

impl Matrix {
    pub fn read(dir: &Path, file: &str) -> Matrix {
        let path = case_file(dir, file);
        let mut reader = csv::Reader::from_path(&path)
            .unwrap_or_else(|e| panic!("cannot open {}: {e}", path.display()));
        let headers: Vec<String> = reader
            .headers()
            .unwrap_or_else(|e| panic!("{file}: cannot read the header row: {e}"))
            .iter()
            .map(str::to_owned)
            .collect();
        assert_eq!(
            headers.first().map(String::as_str),
            Some("row"),
            "{file}: first column must be `row`"
        );
        let col_names = headers[1..].to_vec();
        let ncol = col_names.len();
        let mut row_names = Vec::new();
        let mut rows: Vec<Vec<f64>> = Vec::new();
        for (i, record) in reader.records().enumerate() {
            let record = record.unwrap_or_else(|e| panic!("{file}: line {}: {e}", i + 2));
            row_names.push(record[0].to_owned());
            let vals: Vec<f64> = record
                .iter()
                .skip(1)
                .map(|cell| {
                    parse_cell(cell)
                        .unwrap_or_else(|| panic!("{file}: line {}: cannot parse {cell:?}", i + 2))
                })
                .collect();
            assert_eq!(vals.len(), ncol, "{file}: line {}: ragged row", i + 2);
            rows.push(vals);
        }
        let nrow = rows.len();
        let mut data = vec![0.0; nrow * ncol];
        for (i, row) in rows.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                data[j * nrow + i] = v;
            }
        }
        let matrix = Matrix {
            file: file.to_owned(),
            row_names,
            col_names,
            nrow,
            ncol,
            data,
        };
        matrix.assert_unique_axes();
        matrix
    }

    fn assert_unique_axes(&self) {
        for (axis, names) in [("row", &self.row_names), ("column", &self.col_names)] {
            let mut seen = HashSet::with_capacity(names.len());
            for (index, name) in names.iter().enumerate() {
                assert!(
                    !name.trim().is_empty(),
                    "{}: empty {axis} ID at position {}",
                    self.file,
                    index + 1
                );
                assert!(
                    seen.insert(name),
                    "{}: duplicate {axis} ID {name:?}",
                    self.file
                );
            }
        }
    }

    pub fn col(&self, name: &str) -> &[f64] {
        let j = self
            .col_names
            .iter()
            .position(|c| c == name)
            .unwrap_or_else(|| panic!("{}: no column {name:?}", self.file));
        &self.data[j * self.nrow..(j + 1) * self.nrow]
    }

    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.data[j * self.nrow + i]
    }
}

/// Validate named inputs before their numeric slices reach the core. No implicit
/// reordering or positional fallback: a fixture must state the intended identity.
pub fn assert_model_identity(exprs: &Matrix, design: &Matrix, contrasts: Option<&Matrix>) {
    exprs.assert_unique_axes();
    design.assert_unique_axes();
    assert_eq!(exprs.ncol, design.nrow, "input/design sample dimensions");
    assert_eq!(
        exprs.col_names, design.row_names,
        "input columns must exactly match design rows in sample order"
    );
    if let Some(contrasts) = contrasts {
        contrasts.assert_unique_axes();
        assert_eq!(
            design.ncol, contrasts.nrow,
            "design/contrast coefficient dimensions"
        );
        assert_eq!(
            design.col_names, contrasts.row_names,
            "design columns must exactly match contrast rows in coefficient order"
        );
    }
}

fn parse_cell(cell: &str) -> Option<f64> {
    match cell {
        "NA" | "NaN" => Some(f64::NAN),
        "Inf" => Some(f64::INFINITY),
        "-Inf" => Some(f64::NEG_INFINITY),
        "TRUE" => Some(1.0),
        "FALSE" => Some(0.0),
        _ => cell.parse().ok(),
    }
}

/// `matrix/<case>/` under the corpus root. Always `Some`; the `Option` keeps the callers' early
/// return.
pub fn matrix_dir(_test: &str, case: &str) -> Option<PathBuf> {
    let root = corpus_root();
    let dir = root.join("matrix").join(case);
    assert!(dir.is_dir(), "{} has no matrix/{case}/", root.display());
    Some(dir)
}

/// The three matrix cases in the corpus.
pub const MATRIX_CASES: [&str; 3] = [
    "conditions_only",
    "conditions_numeric_covariate",
    "conditions_categorical_numeric_covariates",
];
