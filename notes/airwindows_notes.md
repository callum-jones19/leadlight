# Airwindows Notes

## GUI Controls

On the actual GUI side of things, we have 3 controls:

- treble
- bass
- output

## Baxandall

### Variable names

In `Baxandall.h`, we have some private vars:

- Floats A, B, C
  - Inside of the processing algorithm, A gets plugged into the trebleGain
    variable.
  - B gets plugged into the bassGain variable.
  - C gets plugged into the output variable.

A => trebleGain/trebleFreq
B => bassGain/bassFreq
C => output

- trebleAL[9], trebleBL[9], bassAL[9], bassBL[9] & trebleAR[9], trebleBR[9], bassAR[9], bassBR[9]
  - My current running assumption is that this is the top/bottom
    frequency controls for the treble and bass, respectively. As in,
    say we lower the treble by a value of 4.6, then the trebleA would
    be the lower side of the shelving EQ on the treble side, and the
    trebleB would be the top-end frequency required to achieve that
    shelving EQ slope.
  - This is why they're all initialised to 0.0 - as the EQ starts off
    in a totally neutral position.
  - Will want to verify this with Chris.

- bool flip

- uint32 fpd[L/R]
  - fpd = Floating Point Dither

In the processing function, we see a few more variables pop up:

- K
  - It might be the shelving cutoff? Only reason I say this is because
    its value is tied directly to the value of trebleAL and bassAL

### VST-specific code

`getChunk` and `setChunk` both refer to `A`, `B`, and `C`. What do they do?

- I have a feeling A, B, C all refer to the raw input data for each parameter:
  that is, A is the raw input of the treble control, etc.
  I think that the slider control actually ranges from 0 <=> 1, and so
  0.5 initialises it to the middle, which then gets transformed
  in the subsequent variables to be "-15.0 <=> 0 <=> 15.0".

### What to ask Chris

Why do we use functions like tan() in calculating the K value, as well
as PI.
