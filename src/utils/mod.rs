#[cfg(feature = "server")]
mod highlighter;
#[cfg(feature = "server")]
pub use highlighter::CustomHighlighter;
