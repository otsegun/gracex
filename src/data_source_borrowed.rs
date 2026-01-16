// lifetime pinned to the existence of the original data
use crate::data_sources::{BorrowedColumnSource, DataError};

pub trait DataSourceBorrowed<'a> {
    fn get_numeric_column(&self, name: &str) -> Result<&'a [f64], DataError>;

    fn n_rows(&self) -> usize;

    fn has_columns(&self, name: &str) -> bool;
}

impl<'a> DataSourceBorrowed<'a> for BorrowedColumnSource<'a> {
    fn get_numeric_column(&self, name: &str) -> Result<&'a [f64], DataError> {
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

mod tests {
    use super::*; // Imports everything from the outer file

    #[test]
    // test data source owned for owned_column_source
    fn test_data_source_borrowed_for_borrowed_column_source() {
        let test_data: Vec<f64> = vec![1.0, 2.0, 3.0];

        let borrowed_column = BorrowedColumnSource {
            name: "positive_ints".to_string(),
            data: &test_data,
        };
        let numeric_column = borrowed_column.get_numeric_column("positive_ints").unwrap();

        assert_eq!(numeric_column, &test_data);

        let number_rows = borrowed_column.n_rows();
        assert_eq!(number_rows, test_data.len());

        assert!(borrowed_column.has_columns("positive_ints"));
    }
}
