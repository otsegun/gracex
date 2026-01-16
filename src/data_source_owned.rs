// There are three options to implement DataSource:
// 1. DataSource returns borrowed data from the original external data (deal with lifetimes)
// 2. DataSource owns the data (we copy the data)
// 3. DataSource trait borrows the data from DataSource object (its lifetime is dependent on self)

use crate::data_sources::{DataError, OwnedColumnSource};

// returns owned data, static lifetime
pub trait DataSourceOwned {
    fn get_numeric_column(&self, name: &str) -> Result<Vec<f64>, DataError>;

    fn n_rows(&self) -> usize;

    fn has_columns(&self, name: &str) -> bool;
}

// Trait implementations
impl DataSourceOwned for OwnedColumnSource {
    fn get_numeric_column(&self, name: &str) -> Result<Vec<f64>, DataError> {
        if name == self.name {
            return Ok(self.data.clone());
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

#[cfg(test)]
mod tests {
    use super::*; // Imports everything from the outer file

    #[test]
    // test data source owned for owned_column_source
    fn test_data_source_owned_for_owned_column_source() {
        let test_data: Vec<f64> = vec![1.0, 2.0, 3.0];

        let owned_data_column = OwnedColumnSource {
            name: "positive_ints".to_string(),
            data: test_data.clone(),
        };
        let answer = owned_data_column
            .get_numeric_column("positive_ints")
            .unwrap();

        assert_eq!(answer, test_data);
    }
}
