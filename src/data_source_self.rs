use polars::frame::DataFrame;
use polars::prelude::{Column, DataType::Float64, PolarsDataType, PolarsError};
use polars::series::Series;

use crate::data_sources::{DataError, OwnedColumnSource};

// gets data from self (lifetime pinned to self)
pub trait DataSourceSelf {
    fn get_numeric_column(&self, name: &str) -> Result<&[f64], DataError>;

    fn n_rows(&self) -> usize;

    fn has_columns(&self, name: &str) -> bool;
}

impl DataSourceSelf for OwnedColumnSource {
    fn get_numeric_column(&self, name: &str) -> Result<&[f64], DataError> {
        if name == self.name {
            return Ok(&self.data);
        } else {
            return Err(DataError::ColumnNotFound(format!(
                "Column '{}' not found",
                name
            )));
        }
    }

    fn n_rows(&self) -> usize {
        self.data.len()
    }

    fn has_columns(&self, name: &str) -> bool {
        self.name == name
    }
}

impl DataSourceSelf for DataFrame {
    fn has_columns(&self, name: &str) -> bool {
        let column_names = self.get_column_names();
        column_names.iter().any(|&col_name| col_name == name)
    }

    fn n_rows(&self) -> usize {
        self.height()
    }

    fn get_numeric_column(&self, name: &str) -> Result<&[f64], DataError> {
        let column = self.column(name)?;
        if column.dtype() != &Float64 {
            return Err(DataError::TypeMismatch(format!(
                "Column '{}' is not f64",
                name
            )));
        } else {
            column.f64()?.cont_slice().map_err(Into::into)
        }
    }
}

impl DataSourceSelf for Series {
    fn has_columns(&self, name: &str) -> bool {
        let column_name = self.name();
        column_name == name
    }

    fn n_rows(&self) -> usize {
        self.len()
    }

    fn get_numeric_column(&self, name: &str) -> Result<&[f64], DataError> {
        if self.dtype() == &Float64 {
            return Err(DataError::TypeMismatch(format!(
                "Column '{}' is not f64",
                name
            )));
        } else {
            self.f64()?.cont_slice().map_err(Into::into)
        }
    }
}
mod tests {
    use super::*; // Imports everything from the outer file

    #[test]
    // test data source owned for owned_column_source
    fn test_data_source_borrowed_for_borrowed_column_source() {
        let test_data: Vec<f64> = vec![1.0, 2.0, 3.0];
        // create clone to use for evaluation
        let test_data_clone = test_data.clone();

        let borrowed_column = OwnedColumnSource {
            name: "positive_ints".to_string(),
            data: test_data,
        };
        let answer = borrowed_column.get_numeric_column("positive_ints").unwrap();

        assert_eq!(answer, &test_data_clone);

        let number_rows = borrowed_column.n_rows();
        assert_eq!(number_rows, test_data_clone.len());

        assert!(borrowed_column.has_columns("positive_ints"));
    }
}
