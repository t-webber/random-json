//! Traits to make structs generate some data in a formatted way.

use crate::Res;
use crate::data::Data;

/// Generate random data of the given type.
pub trait Generator<T>: Sized {
    /// Generate random data of the given type.
    fn generate(&self, data: &mut Data) -> Res<T>;
}

/// Generate random data of the given type, but with a nullable type.
pub trait NullableGenerator<T>: Sized {
    /// Generate random data of the given type, but with a nullable type.
    ///
    /// This can sometimes returns None.
    fn generate_nullable(&self, data: &mut Data) -> Res<Option<T>>;
}
