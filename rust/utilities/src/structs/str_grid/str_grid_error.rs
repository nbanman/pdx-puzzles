use thiserror::Error;

#[derive(Error, Debug)]
pub enum StrGridError {
    #[error("String has no line breaks.")]
    NoLineBreak,
    #[error(r"String uses Windows-style \r\n line breaks. Strip \r before making Grid.")]
    ContainsCarriageReturns,
    #[error(r"Line breaks occur at uneven intervals, indicating that string does not represent an even grid.")]
    UnevenWidth,
    #[error("This struct only supports ascii characters.")]
    IsUTF,
}