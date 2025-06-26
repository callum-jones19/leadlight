# Notes

## General comments as I go

Maybe could be good to contribute to documentation as I go. This could be either
through writing guides, or just improving a lot of the default docs. Would make
a lot of sense, and could be an easy part of the thesis development too.

## Setting up this repository

NIH-plug provides a template that can quickly get a repo set up for dev
work, but I would rather just do it myself from scratch so I know how each part
interacts with everything else.

These are the steps I followed:

1. Apply COPYING to the repository, which gives it the GPLv3 licensing.
2. Add nih_plug as a dependency of the project. We do this by adding
   `nih_plug = { git = "https://github.com/robbert-vdh/nih-plug.git", features = ["assert_process_allocs"] }`
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
      overengineer this, because assuming that I will make sure the code is relatively
      decoupled. That way, as it grows, I can just reorganize in whichever way
      makes the most sense.
   2. Consider how we should make the GUI for the plugins - can we just let the
      DAW take care of that? Or, do we need to use something like `iced`/`egui`
6. How do we actually link the VST3 binaries to the workspace? And, how is the
   final binary formatted? Is it a VST3 file? How do we get to that?
   1. Let's start by just trying to compile the examples in the vst3-sys repo.
      1. So, crossreferencing the built example, and the Airwindows Consolidated
         VST3 code, it turns out a VST3 "file" is actually a folder. This contains the
         `.so` file, and a JSON that contains information about the plugin. Note,
         .so is just the Linux version (Windows would be `.dll`, etc). This just
         is a **dynamic system library**. So, a VST3 file is essentially a bundled
         dynamic system library, along with a metadata information file. So, for
         something to be a VST3 file, we need to compile it to a DSL, and then move
         that file into a correct VST3 folder structure.
   2. The other question is then, do the actual VST3 binaries need to be linked into the
      compiled .so file?
         1. My current understanding is no, and that this is because of the fact that the
            vst3-sys library uses the COM API to actually interact with the underlying
            VST interface. This might mean that it doesn't actually need to bundle the native VST3
            libraries with it.
7. We start by making a boilerplate plugin implementation. This is
   not actually going to contain anything useful processing-wise, but
   it lets me get a hands on understanding of what each plugin that
   I write needs to look like, and what functions it exposes.
      1. In this case, we create a Plugin struct, and a Plugin Parameters
      struct. We then impl the nih-plugin `Pugin` trait for the Plugin.
      To satisfy this, we need to implement a `process(...)` and a
      `params(...)` function for the plugin. `process` deals with the actual
      processing algorithm. `params` returns an atomically referenced copy of
      the plugin's parameters. Interestingly, this `Arc` means that we can be
      sure that the plugin won't suffer from race conditions in multi-threaded
      applications.
8. With this in mind, we want to now make a testing suite, that takes the
   processing algorithm, and tests that the output is what we expect it to be.
      1. We also want to test performance of the library (see the ThesisA report
         ). To do this, we will use a performance benchnarking library, such as
         `criterion-rs`.
      2. I can't apply a test directly to the implemented `process(...)` function.
         This is because it needs a reference to the processing context, which
         the testing suite won't have access to (as far as I'm aware). So,
         to get around this, we will apply the testing function to a processing
         algorithm which takes in just the buffers.
9. In the tests, we make a new default Buffer, and then set the slices to a set
   of voilerplate values. To do this, we need to use an unsafe section of code,
   however I pulled this directly from the built-in nih-plug tests, so I'm relatively
   confident that they will not pose an issue (though I want to check this later).
   1. The test just creates a two-channel buffer filled with samples of value 5.0
   2. The test then asserts that all the samples in the buffer have this value.
   3. We then run the processing algorithm on this buffer, and run a second
      assertion all the values have now been set to 0.0.
   4. I also want to really reconsider how that unsafe is used. My current idea
      is to wrap it in a helper function called something like
      `createTestBuffer() -> Buffer`. That way I can limit the presence
      of `unsafe` to a single line of code in the repository. Even though it's
      only used in the testing modules, I definitely don't want it to exist
      in every test.
   5. This is now done. It expects you to pass in a reference to a `Vec` representation
      of the buffer, which is then fed into the unsafe code that creates a Buffer
      and feeds in the slices to it. That way, we don't have to deal with lifetimes,
      etc., and we also get more flexibility between tests in what length of buffer
      we pass in.
10. At this stage, the compiled .so/.dll/etc. file actually works. However, in
    order for it to work, it needs to be renamed into a .vst3 file, and then DAWs
    will recognise it and allow you to import it. This seems clunky though - we
    want to figure out a proper way to automatically compile it into a proper
    VST3 file.
    1. See [this](https://steinbergmedia.github.io/vst3_dev_portal/pages/Technical+Documentation/Locations+Format/Plugin+Format.html?highlight=.vst3%20file#for-the-windows-platform)
       link for an explanation of how we can actually bundle this into a VST3 file.
    2. I want to briefly pause this though to focus a bit more on testing.
