//! In Rust, `Option<T>` is not Ffi-Safe,
//! so we use the new type `Optional<T>`
//! as a substitute for passing parameters,
//! and perform type conversion when needed.

pub use Optional::*;

#[repr(u64)]
#[derive(Copy, Clone)]
/// # Optional<T>
/// In Rust, `Option<T>` is not Ffi-Safe,
/// so we use the new type `Optional<T>`
/// as a substitute for passing parameters,
/// and perform type conversion when needed.
pub enum Optional<T> {
    /// # Something(T)
    /// Some value of type `T`.
    Something(T),
    /// # Nothing
    /// No value.
    Nothing,
}

impl<T> From<Option<T>> for Optional<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => Something(v),
            None => Nothing,
        }
    }
}

impl<T> Into<Option<T>> for Optional<T> {
    fn into(self) -> Option<T> {
        match self {
            Something(v) => Some(v),
            Nothing => None,
        }
    }
}
