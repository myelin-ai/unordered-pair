//! This crate provides a tuple struct for an unordered pair
//! ## Crate Features
//! - `serde`: Enables serde support for [`UnorderedPair`].

#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    missing_docs,
    clippy::doc_markdown,
    clippy::unimplemented
)]

use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// A tuple struct representing an unordered pair
#[derive(Debug, Copy, Clone, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnorderedPair<T>(pub T, pub T);

impl<T: Ord> UnorderedPair<T> {
    /// Transforms the `UnorderedPair<T>` into a `(T,T)`.
    /// The tuple's components are always in the same order, smallest to largest.
    ///
    /// # Examples
    ///
    /// ```
    /// use unordered_pair::UnorderedPair;
    ///
    /// let pair = UnorderedPair(1,2);
    /// let rev = UnorderedPair(2,1);
    ///
    /// let tuple_pair = pair.into_ordered_tuple();
    /// let tuple_rev = rev.into_ordered_tuple();
    ///
    /// assert_eq!(tuple_pair, (1,2));
    /// assert_eq!(tuple_rev, (1,2));
    /// ```
    pub fn into_ordered_tuple(self) -> (T, T) {
        let UnorderedPair(first, second) = self;

        match first.cmp(&second) {
            Ordering::Greater => (second, first),
            _ => (first, second),
        }
    }
}

impl<T> From<(T, T)> for UnorderedPair<T> {
    fn from(tuple: (T, T)) -> UnorderedPair<T> {
        UnorderedPair(tuple.0, tuple.1)
    }
}

impl<T> From<UnorderedPair<T>> for (T, T) {
    fn from(pair: UnorderedPair<T>) -> (T, T) {
        (pair.0, pair.1)
    }
}

/// Compares two pairs while disregarding the order of the contained items
impl<T> PartialEq for UnorderedPair<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &UnorderedPair<T>) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

/// Computes the same hash regardless of the order of the contained items
impl<T> Hash for UnorderedPair<T>
where
    T: Ord + Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let UnorderedPair(first, second) = self;

        match first.cmp(second) {
            Ordering::Greater => {
                second.hash(state);
                first.hash(state);
            }
            _ => {
                first.hash(state);
                second.hash(state);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_eq_different_internal_order() {
        let pair = UnorderedPair(5, 7);
        let rev = UnorderedPair(7, 5);
        assert_eq!(pair, rev);
    }

    #[test]
    fn partial_eq_same_internal_order() {
        let pair1 = UnorderedPair(5, 7);
        let pair2 = UnorderedPair(5, 7);
        assert_eq!(pair1, pair2);
    }

    #[test]
    fn partial_eq_nan() {
        let pair1 = UnorderedPair(f32::NAN, 1.3);
        let pair2 = UnorderedPair(1.3, f32::NAN);
        assert_ne!(pair1, pair2);
    }

    #[test]
    fn hash_different_internal_order() {
        use std::collections::hash_map::DefaultHasher as Hasher;

        let hash_pair = {
            let pair = UnorderedPair(5, 7);
            let mut hasher = Hasher::new();
            pair.hash(&mut hasher);
            hasher.finish()
        };

        let hash_rev = {
            let pair = UnorderedPair(7, 5);
            let mut hasher = Hasher::new();
            pair.hash(&mut hasher);
            hasher.finish()
        };

        assert_eq!(hash_rev, hash_pair);
    }
}
