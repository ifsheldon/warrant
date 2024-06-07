# warrant

A guard macro that is like `guard` in Swift.
It helps to better express code logic without mind twists when checking conditions.

It implements what I termed "procedural warranty".

## Usages

Add `warrant = "0.1.0"` to your `Cargo.toml`.

**Before**

```rust
// if some condition is not satisfied, early return.
let condition = is_satisfied();
if !condition {
    return;
}
// proceed
```

**After**

```rust
use warrant::warrant;
let condition = is_satisfied();
warrant!(condition, else {
    return;
});
// proceed
```

`warrant::warrant` is also aliased as `warrant::guard` if you come from Swift and prefer `guard`.

## License

[MIT](./LICENSE)

## Additional References

* Pattern Matched Guard: consider just use `if-let-else`
* "Structural Warranty": to enforce a condition on a struct, [nutype](https://github.com/greyblake/nutype) is a good
  one.
