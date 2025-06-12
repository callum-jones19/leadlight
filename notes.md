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
4. Add licence notices for Airwindows and NIH Plug
5. Decide on workspace organisation
   1. Workspaces vs modules.
      1. I think we want to do workspaces - at least for the plugins. That will
      let us compile every plugin individually, if we want to do that. E.G., we
      want to distribute specific plugins, without bundling them all into a single
      solution.
      2. Either way, we essentially will have a lib module for the plugin,
      which is then exposed in the built lib.rs with a `nih_export_vst3!()`.
      We can also use `nih_export_standalone!()` to create a version of the
      plugin that can be run as a standalone GUI.
      3. An nih_plug plugin consists of an implementation of the `Plugin` trait,
      and then an `nih_export` macro call to actually expose that plugin
      implementation.
      4. For now, I'm going to leave everything in the same module. That's because
      for the moment, we're only dealing with one plugin - there is no need to
      overengineer this, because assuming that I make sure the code is relatively
      decoupled.
   2. Consider how we should make the GUI for the plugins - can we just let the
      DAW take care of that? Or, do we need to use something like `iced`/`egui`

