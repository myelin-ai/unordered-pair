# Changelog

## 0.2.4
- Add `into_ordered_tuple`.
- Derive `Default`.
- Implement `From<UnorderedPair<T>>` instead of `Into<UnorderedPair>`. The `Into` impl now comes from the [blanket impl](https://doc.rust-lang.org/src/core/convert/mod.rs.html#541-552).

## 0.2.3
- Add support for Serialization/Deserialization using serde.

## 0.2.2
- Derive Copy

## 0.2.1
- Stable rust support

## 0.2.0
- Derive Eq

## 0.1.1
- Update readme

## 0.1.0
Initial release
