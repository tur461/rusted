fitos
====
This crate provides a fast conversion of integer primitives to decimal strings.
The implementation comes straight from [libcore] but avoids the performance
penalty of going through [`core::fmt::Formatter`].

## Example

```rust
fn main() {
    let mut buffer = fitos::Buffer::new();
    let printed = buffer.format(128u64);
    assert_eq!(printed, "128");
}
```