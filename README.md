# Unified EFM32 device support crate

The goal of this project is to unify support for all EFM32 parts into one crate.
In order to try to achieve this, the original SVD files have been modified and
combined into a small set of files. Unfortunately, it isn't possible for
svd2rust to actually generate a single crate supporting all devices, nor is it
possible to create a mega-crate that combines multiple svd2rust crates together.
Issues arise with things such as the global vector table. Therefore, currently
this crate is effectively a "meta-crate" that produces a tool that outputs a new
crate when it is run.

For example, to generate a crate for the EFM32HG350F64 device, use
```sh
./target/debug/efm32 efm32hg350f64 >lib.rs
```

Note that "the point" of this repository is the maintenance of the files under
`svd/` and not the actual tool itself (which is just a thin wrapper around
svd2rust).

# Currently supported devices
* All EFM32HG parts

# Big thanks to
* [svd2rust](https://github.com/japaric/svd2rust)
