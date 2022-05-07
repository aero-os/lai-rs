## `lai-rs`

Rusty and *safe* bindings for the [Lightweight AML Interpreter](https://github.com/managarm/lai) (LAI). As an AML interpreter, LAI is used by OS kernels to implement support for ACPI.

## Porting
LAI has a few standardized `host` functions that the kernel needs to implement; most of them are optional and the crate provides default implementations for them. The `host` functions are a part of the `Host` trait which the kernel is required to implement.

## Initializing Host
```rust
struct LaiHost;

impl lai::Host for LaiHost {
    // host functions...
}

let lai_host = Arc::new(LaiHost);
lai::init(lai_host);

// After this point, the host has been successfully initialized and
// can do whatever it wants to do with LAI!
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, 
as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
