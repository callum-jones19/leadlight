# Airwindows Biquad+ Ariwindows Notes

## Sources

Notes in this segment come from the following resources:

- https://www.youtube.com/watch?v=PFHtgtQ6ex0&list=PL843zqX5EuA5xzpnuG-6jmKvjvdrDpjoe&index=31
- https://www.airwindows.com/biquadplus/

## Blog summary

Biquad plus is:

- The regular, 'simpler' implementation of biquad **plus** zipper noise suppression.
- This addition comes at the cost of a bit more CPU overhead & inefficiency.

## General Biquad Notes

TODO

## Airwindows Implementation Notes

Again, to reiterate, this is based on **Biquad Plus**.

- This plugin is itself based on the "standard" Airwindows Biquad, so understanding this one
  should be relatively useful across the board.

Setting up BiqP:

- Not needing additional sliders - just setting it up like BQ then
  adding the smoothing.
- New enum:
  - All the things that go into the biquad array.
- Check the main CPP file to get the min/max values for input sliders.
- The smoothing part in this algorithm is the way that Z-Plane filters
  dont have zipper noise, and these biquads don't have zipper noise:
  - So, we will define the coefficients of the biquads, and then for
    every sample, we will not only do those co-efficients, but we will
    quickyl generate an interpolation between a beginning and an end section.
    This should in theory smooth things out.
