# shaddup-rs
Shut up your Rust program('s stdout and stderr).

Works on Unix only (so far).

# Usage
```rust
use shaddup::run_quietly;

let result = run_quietly(|| {
    println!("This will not be printed");
    eprintln!("neither will this");
    123
});

assert_eq!(result.unwrap(), 123);
```

This is similar to [`gag`](https://docs.rs/gag/latest/gag/):
that crate has a different API,
based around guard handles instead of closures.
It supports Unix and Windows targets.

# Cargo features

- `allow_unsupported`: If this is enabled, and you're building on an unsupported target, then the library will be a no-op. (If this feature is not provided on an unsupported target, then this will fail to compile.)
- `no_op`: If this is enabled, then the library will be a no-op, even if the target is supported. This can be useful for debugging.