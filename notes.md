# Notes

## Setting up this repository

NIH-plug provides a template that can quickly get a repo set up for dev
work, but I would rather just do it myself from scratch so I know how each part
interacts with everything else.

These are the steps I followed:

1. Apply COPYING to the repository, which gives it the GPLv3 licensing.
2. Add nih_plug as a dependency of the project. We do this by adding
   nih_plug = { git = "https://github.com/robbert-vdh/nih-plug.git", features = ["assert_process_allocs"] }
   to the `Cargo.toml`. It is not on Cargo (Rust's package manager), so we add
  the dependency directly from Github.
3. Set up the project settings in `Cargo.toml`.
