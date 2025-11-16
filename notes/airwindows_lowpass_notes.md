## OverallScale
I think this is to compensate for when the sampleRate is not 44100, while
this filter has been designed with this sampleRate as the 'starting point'.

`overallScale = 1 / 44100 * sampleRate`

According to some of the comments made in the Lowpass2 filter video,
the purpose of this overallScale calculation was to try to keep the same
numerical values for each setting relatively consistent between different
sample rates. Apparently though this was a little bugged at high sample rates.

## Lowpass "frequency"
Starts at 1 as default
Lower the number, lower the lowpass threshold frequency
A

```
Factor to apply the IIR
iirAmount   (A^2 + A)          1
					= --------- * ---------------
					      2        _overallScale
(default)    1 + 1           1
          = ------- *  --------------
						   2        overallScale

					= (1 / overallScale)

(**assuming** oS=1)

					= 1
```

In the default config (as above), the output shouldn't be attenuated
at all - the lowpass threshold is set to be the Nyquist frequency.

## accumulating sample
iirSampleA

## Current input sample from buffer
inputSample

Assume that hard/soft = 0. Gives us the following offset.
offset = 1

```c
new_iirSample =  (prev_iirSample * (1 - iirAmount))
 			   + (inputSample * iirAmount)
```

Now for some examples:

At the Nyquist frequency:

```c
iirSample = (iirSample * (1 - iirAmount)) + (inputSample * iirAmount)
				  = (iirSample * (1 - 1)) + (inputSample * 1)
					= (iirSample * 0) + inputSample
					= inputSample
```

At A (param one) = 0.5:

```c
iirAmount = (A^2 + A)/(2 * overallScale)
          = (A^2 + A)/(2)
          = (0.5^2 + 0.5) / 2
          = (0.25 + 0.5) / 2
					= (0.75) / 2
					= 0.375

iirSample = (iirSample * (1 - iirAmount)) + (inputSample * iirAmount)
					= (iirSample * (1 - 0.375)) + (inputSample * 0.375)
					= (iirSample * 0.625) + (inputSample * 0.375)

```

So, essentially this is a sort of moving-average time-domain IIR filter.
It takes the input sample, and then combines it (weighted) with the previous
IIR Sample value. The smaller the iirAmount (i.e., the lower the threshold
frequency), then the larger the weight of the previous iirSample, and the
smaller the weight of the new inputSample. In other words, the wider the
moving average's window.
