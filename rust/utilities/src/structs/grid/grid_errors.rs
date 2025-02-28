use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum GridError {
    #[error("String has no line breaks.")]
    NoLineBreak,
    #[error(r"String uses Windows-style \r\n line breaks. Strip \r before making Grid.")]
    ContainsCarriageReturns,
    #[error(r"Line breaks occur at uneven intervals, indicating that string does not represent an even grid.")]
    UnevenLineBreaks,
    #[error("Supplied backing data does not divide evenly by the supplied dimensions.")]
    UnevenDimensions,
    #[error("Supplied backing data is empty.")]
    Empty,
    #[error("Index exceeds dimensions.")]
    OutOfRange,
}
