accel-derive
=============

[![Crate](http://meritbadge.herokuapp.com/accel-derive)](https://crates.io/crates/accel-derive)
[![docs.rs](https://docs.rs/accel-derive/badge.svg)](https://docs.rs/accel-derive)

Procedural-macro crate for `#[kernel]`

Requirements
------------

Install nightly-2020-01-02 toolchain

```
rustup toolchain add nightly-2020-01-02 --profile minimal
```

and [rust-ptx-linker](https://github.com/denzp/rust-ptx-linker)

```
cargo install ptx-linker -f
```

#[accel_derive::kernel]
------------------------

`#[kernel]` function will be converted to two part:

- Device code will be compiled into PTX assembler
- Host code which call the generated device code (PTX asm) using `accel::module` API

### Generate PTX assember

```rust
use accel_derive::kernel;

#[kernel]
#[dependencies("accel-core" = "0.2.0-alpha")]  // Strings in `()` will be parsed as TOML
pub unsafe fn add(a: *const f64, b: *const f64, c: *mut f64, n: usize) {
    let i = accel_core::index();
    if (i as usize) < n {
        *c.offset(i) = *a.offset(i) + *b.offset(i);
    }
}
```

will generate a crate with a device code

```rust
#[no_mangle]
pub unsafe extern "ptx-kernel" fn add(a: *const f64, b: *const f64, c: *mut f64, n: usize) {
    let i = accel_core::index();
    if (i as usize) < n {
        *c.offset(i) = *a.offset(i) + *b.offset(i);
    }
}
```

and `Cargo.toml`

```toml
[dependencies]
accel-core = "0.2.0-alpha"
```

This crate will be compiled into PTX assembler using `nvptx64-nvidia-cuda` target.
Genearated crates will be placed on `$HOME/.cache/accel-derive` (determined by [dirs::cache_dir](https://docs.rs/dirs/2.0.2/dirs/fn.cache_dir.html)).
`nvptx64-nvidia-cuda` target does not support libstd yet.
You need to write codes in `no_std` manner.

### Kernel launcher

On the other hand, corresponding host code will also generated:

```rust
use ::accel::{Grid, Block, kernel::void_cast, module::Module};

pub fn add(grid: Grid, block: Block, a: *const f64, b: *const f64, c: *mut f64, n: usize) {
    thread_local!{
        static module: Module = Module::from_str("{{PTX_STRING}}").expect("Load module failed");
    }
    module.with(|m| {
        let mut kernel = m.get_kernel("add").expect("Failed to get Kernel");
        let mut args = [void_cast(&a)), void_cast(&b), void_cast(&c)];
        unsafe {
            kernel.launch(args.as_mut_ptr(), grid, block).expect("Failed to launch kernel")
        };
    })
}
```

where `{{PTX_STRING}}` is a PTX assembler string compiled from the device code.
This can be called like following:

```rust
let grid = Grid::x(1);
let block = Block::x(n as u32);
add(grid, block, a.as_ptr(), b.as_ptr(), c.as_mut_ptr(), n);
```
