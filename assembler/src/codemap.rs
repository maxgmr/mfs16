/// A unique identifier pointing to a substring in some file.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Span(usize);
impl Span {
    /// Returns the special "dummy" span, which matches anything.
    ///
    /// For testing/internal use only!
    pub fn dummy() -> Self {
        Self(0)
    }
}
