# Thesis VST

## Summary

The centralised repository for all Rust DSP effects created for my thesis.

All major effects seen in this repository are based off the airwindows suite
of processing plugins (which will be credited in all relevant plugins).

## Repository Outline

Various components of this repository are divided across multiple crates. This
will provide a brief outline of them:

- `benches` is a crate that provides benchmarks across every processing
  algorithm and plugin present in this repository. It imports every available
  plugin crate, and then supplies benchmark tests for each of them.
- `xtask` is a thin wrapper around the *nih-plug* `xtask` crate, and provides
  and easy way to access the building and bundling functionality this crate
  provides. The commands provided by xtask are exposed by the alias provided
  in `.cargo/config.toml`.
- `plugin-utils` provides utility functions that are shared between plugins.
  This is currently mainly used when certain `unsafe` functionality must be
  exposed, and is minimised through proper wrapper functions.
- `plugin-mute` is the Mute VST3 plugin.

## Licensing Notices

As this plugin suite is built on top of Steinberg's VST-3 technologies, it is
appropriately licensed under the GPLv3 licence.

The code is built on top of a number of other existing products, which are
credited in the [notices](notices/) directory:

1. **Airwindows**, by Chris Johnson. Certain plugin code is directly derived from
   the Airwindows algorithms, which is licensed under the MIT licence. Please
   find the relevant licence [here](notices/airwindows_LICENCE).
2. **nih-plug**, by Robbert Vanderhelm. This suite is built on top of the nih-plug
   framework, which is licensed under the ISC licence. Please
   find the relevant licence [here](notices/nih_plug_LICENCE).
