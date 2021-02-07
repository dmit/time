//! Error parsing an input into a [`Parsed`](crate::parsing::Parsed) struct

use core::fmt;

/// An error that occurred while parsing the input into a [`Parsed`](crate::parsing::Parsed) struct.
#[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntermediateParse {
    /// A string literal was not what was expected.
    #[non_exhaustive]
    InvalidLiteral,
    /// A dynamic component was not valid.
    InvalidComponent(&'static str),
}

impl fmt::Display for IntermediateParse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLiteral => f.write_str("a character literal was not valid"),
            Self::InvalidComponent(name) => {
                write!(f, "the '{}' component could not be parsed", name)
            }
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "std")))]
impl std::error::Error for IntermediateParse {}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
impl From<IntermediateParse> for crate::Error {
    fn from(original: IntermediateParse) -> Self {
        Self::IntermediateParse(original)
    }
}
