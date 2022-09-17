Conditional compilation using boolean expression syntax, rather than *any()*,
*all()*, *not()*.
<br>

## Summary

Rust's `cfg` and `cfg_attr` conditional compilation attributes use a restrictive
domain-specific language for specifying configuration predicates. The syntax is
described in the *[Conditional compilation]* page of the Rust reference. The
reason for this syntax as opposed to ordinary boolean expressions was to
accommodate restrictions that old versions of rustc used to have on the grammar
of attributes.

However, all restrictions on the attribute grammar were lifted in Rust 1.18.0 by
[rust-lang/rust#40346]. This crate explores implementing conditional compilation
using ordinary boolean expressions
instead:&ensp;`&&`,&ensp;`||`,&ensp;`!`&ensp;as usual in Rust syntax.
