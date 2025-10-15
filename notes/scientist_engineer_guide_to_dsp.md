# The Scientist and Engineers Guide to DSP

## Acknowledgment

These notes are taken from _The Scientist and Engineer's Guide to Digital
Signal Processing, 2nd edn_, by Steven W. Smith. A copy is available
for free digitally [here](https://ia801301.us.archive.org/23/items/GuideToDigitalSignalProcessing/Guide%20To%20Digital%20Signal%20Processing.pdf).

## DSP Software

Number precision:

- Floating point numbers: the gaps between adjacent numbers vary over the represented number range.
  - Large numbers have large gaps between them. Small numbers have small gaps between them.

## Linear Systems

What is a linear system?

- **Homogeneity**: 
- **Additivity**: 
- **Shift invariance**:

Two important properties of linear systems:

1. Static linearity:
2. Sinusoidal fluidity:

Special properties of linearity:

- 

**Superposition**: Signal being processed is broken into simple components, each component processed individually, results reunited.

- Only possible with *linear* systems.

Decompositions:

- **Impulse decomposition**: Break N samples signal into N component signals, each containing N samples.
  Remainder of values are 0. _A single nonzero point in a string of zeros is an impulse_.
  - Important because allows signals to be examined *one sample at a time*.
  - Knowing how a system responds to an impulse, system's output can be calculated
    for a given input. This is called **convolution**.
  - Maybe try to conceptualise it like this:
    - You have a discrete sampled signal. Say, 20 samples in the signal we are looking at.
    - This sampled signal is plotted on a graph, where the x axis has 20 discrete points (duh).
    - Individually, grab the value of each sample.
    - For each sample, create a new graph, which also has 20 discrete points on its x axis.
    - Place each sample on its respective graph, and then fill in each other of the 19 remaining 
      points with 0.
    - Thus, you end up with 20 graphs, each with a single point with a non-0 value, and 19
      zeroed values. For each graph, the point with a value is the impulse.
  - This lets us analyze signals one sample at a time.
- **Fourier decomposition**: *Very* mathematical. TODO

## Convolution

Some basic terms:

- **Delta function**: a *normalized* impulse. I.e., sample number zero has a value of one,
  all other samples have a value of zero. Also called the *unit impulse*.
  - Any impulse can be represented as a shifted & scaled delta function.
  - Also referred to as $\sigma[n]$
- **Impulse response**: Signal that exits a system when a delta function (unit impulse)
  is the input.
  - Usually given the symbol **h[n]**.
  - **If you know a system's impulse response, you know how it will react to *any* impulse**.
  - That is, if you know a system's impulse response, you know *everything* about a linear
    system's characteristics.
  - To re-phrase this definition somewhat: to get the impulse response, we need to think to ourselves:
    we have a system that is doing some sort of transformation to its input. In this case,
    the impulse response is the output that system would give us were the delta function to be
    our input. Or,
    - $\sigma[n]$, then passed _into_ our system, gives $h[n]$ as an output signal.
    - So, the impulse response is a description of what a normalized impulse will look like after
      it has exited the system.
    - Mathematically: $x[n] \circledast h[n] = y[n]$, where $x[n]$ is our input signal and $y[n]$
      is our output signal.

Impulse response has different names in different contexts:

- Filters = filter kernel/convolution kernel/kernel.

Convolution is a formal mathematical operation: takes two signals, produces a third signal.

- Describes the relationship between the input signal, output signal and impulse response.
- Convolution is represented by a star formally: $\circledast$.

**Shifting and scaling** the delta function:

- Consider a signal $a[n]$. It is composed of _all zeros_, except for sample
  8 which has a value of $-3$.
- This is the same as a delta function shifted right 8 and multiplied by -3.
- Mathematically: $a[n] = -3 \sigma [n - 8]$.
  - If you want an alternate way to this about why we say $-8$ to shift the delta function 8 units to the right, 
    imagine we have our finger on point 8 of the delta function. It has a value of $0$. So, we think how can we get
    this to equal the value that is at index 0 (and hence "shift" the graph)? Well, we intuitively just subtract 8.
- This notation can (and is) used as a convenient notation for describing the position and amplitude. If, e.g., we are
  considering an _impulse_ from an impulse decomposition at index 4 with amplitude 4.3, then we can represent the impulse
  as $4.3 \sigma [n - 4]$ (a shifted and scaled delta function). The _output_ component of this can be represented as
  $4.3h[n - 4]$ (a shifted and scaled impulse response).

For example, a low-pass filter would look like this:

![lowpass-convolution](./assets/low_pass_filter_convolution_equation.png)

Length of the output signal = `length of the input signal` + `length of impulse response - 1`

- This makes intuitive sense, if we consider it in the step-by-step given below. 
1. Convolution means that we will apply the impulse response to each decomposed impulse of
   the input signal. Let's say, for this example, that we have an input signal 20 samples long. For the sake of this
   example, I'm going to refer to the samples as being on a grid (x is sample num, y is amplitude of the sample).
2. Once we get to the 20th decomposed sample, we want to apply the impulse response to it.
3. Let's say our impulse response is 5 samples long. Sample 1 of the impulse response is applied to position 20 of
   the input signal.
4. But then sample 2 of the impulse response needs to go somewhere. So, this is applied to sample 21 of our input signal
   (which has amplitude 0, because the input signal has been padded to make it 'longer').
5. This continues until every sample of the impulse response has been applied onto our output signal.
6. Therefore, we end up with an output signal consisting of $20 + 5 - 1 = 24$ samples ($-1$ because the first impulse
   response sample on the final iteration is applied to input sample 20, not 21).

So this explains the _process_ of convolution, but what are the _mathematics_ of convolution?

The textbook notes that there are two ways of thinking about the mathematics of convolution: you can think about it
from the perspective of the input signal, or the perspective of the output signal.

POV of the input signal = how does each sample in the input signal contribute to many points in the output signal:

- We can do this in two possible ways
  1. ![convolution-isa1](./assets/convolution_input_side_algorithm_example1.png)
  2. ![convolution-isa2](./assets/convolution_input_side_algorithm_example2.png)
- These both give identical outputs. This shows us that convolution is **commutative**.