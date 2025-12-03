# Leadlight

## Summary

A suite of DSP VST/CLAP plugins written in Rust.

All major effects seen in this repository are based off the Airwindows suite
of processing plugins (which will be credited in all relevant plugins).

## Repository Outline

Various components of this repository are divided across multiple crates. This
ill provide a brief outline of them:

- `benches` is a crate that provides benchmarks across every processing
  algorithm and plugin present in this repository. It imports every available
  plugin crate, and then supplies benchmark tests for each of them. We do this
  in a separate crate, rather than supplying a bench function within each
  module, so that we can only initialise Criterion as a dependency in one place,
  and then not need to ensure it is imported as a dev dependency universally.
  This might be subject to change if the benches start to bloat out, or if it
  ends up making more sense to hold them within each plugin.
- `xtask` is a thin wrapper around the *nih-plug* `xtask` crate, and provides
  and easy way to access the building and bundling functionality this crate
  provides. The commands provided by xTask are exposed by the alias provided
  in `.cargo/config.toml`.
- `plugin-utils` provides utility functions that are shared between plugins.
  This is currently mainly used when certain `unsafe` functionality must be
  exposed, and is minimised through proper wrapper functions.
- `plugin-mute` is the Mute VST3 plugin.

## Getting Started

The repository has a few system-level dependencies. The following should be
sufficient on Ubuntu-based Linux systems. If there are dependencies
missing from this list, or you believe there is an extraneous dependency,
please file an issue:

```bash
apt install build-essential pkg-config libasound2-dev libgl-dev libx11-xcb-dev libjack-dev
```

There is currently one optional dependency, `gnuplot`. This is the default plotting tool for
Criterion-rs (the benchmarking tool), however if `gnuplot` is not found, Criterion will
fall back to the `plotters` crate backend. This optional dependency can be installed via

```bash
apt install gnuplot
```

Once all dependencies are installed, you can check everything is working as
intended by running the autotests:

```bash
cargo test
```

To build a plugin from source, execute the following command:

```bash
cargo xtask bundle {plugin-name}
```

where `plugin-name` refers to the name of the plugin crate, as defined in its
`Cargo.toml`.

To run the benchmarks for each module, execute:

```bash
cargo bench
```

The results of this benchmark will be accessible as an HTML-formatted interactive report,
available at the subdirectory `target/criterion/report/index.html`.

## Licensing Notices

As this plugin suite is built on top of Steinberg's VST-3 technologies, it is
appropriately licensed under the GPLv3 licence.

The code is built on top of a number of other existing products, which are
credited in the [notices](notices/README.md) directory:

1. **Airwindows**, by Chris Johnson. Certain plugin code is directly derived from
   the Airwindows algorithms, which is licensed under the MIT licence. Please
   find the relevant licence [here](notices/airwindows_LICENCE).
2. **nih-plug**, by Robbert Vanderhelm. This suite is built on top of the nih-plug
   framework, which is licensed under the ISC licence. Please
   find the relevant licence [here](notices/nih_plug_LICENCE).
