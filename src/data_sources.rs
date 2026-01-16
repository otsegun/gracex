use polars::error::PolarsError;

// error handling
#[derive(Debug)]
pub enum DataError {
    ColumnNotFound(String),
    TypeMismatch(String),
    SeriesConvertionFailure(String),
    PolarsError(String),
}

impl From<PolarsError> for DataError {
    fn from(err: PolarsError) -> Self {
        DataError::PolarsError(err.to_string())
    }
}

// Data Sources
pub struct OwnedColumnSource {
    pub name: String,
    pub data: Vec<f64>,
}

pub struct BorrowedColumnSource<'a> {
    pub name: String,
    pub data: &'a [f64],
}
