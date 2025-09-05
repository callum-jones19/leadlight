# Term 2 Break Notes

## Changes

We are no longer going to do Baxandall as the first effect. I still plan to
do it as one of the second/third effects planned in ThesisC, but I'm moving it
back. There are a couple of really crucial reasons.

1. There just isn't a heap of easily-accessible resources on Baxnadall EQ online.
   A lot of it also assumes prior knowledge of "more basic" EQ algorithms, which
   means it makes less sense to start with Baxandall.
2. Airwindows' Baxandall algorithm is built off other algorithms, namely biquad
   filters.
   1. Understanding this algorithm therefore has the prerequisite that you understand
      biquad filters in the first place. While I could research biquad and then apply
      that knowledge to understanding the Airwindows BX, it seems to make more sense
      to simply make Biquad the first thing to implement. This also makes more sense,
      given that a) Chris has a full livestream developing on of the Biquad filters
      in Airwindows, which will give very valuable insight into both the algorithm
      itself, along with the parts of this algorithm that he then builds on top of
      to create other algorithms (in particular Baxandall).
   2. The quirks of how he codes BX will presumably be present in his other
      filters too. If I can implement my own version of one of these, then
      it should be much clearer where these are being adopted in the other plugins.
3. Biquad filters are (from what I can tell) very fundamental to DAP. Because of this,
   there are a wealth of resources online (and I assume in books, etc.) from which I
   can learn a lot. Again, it comes back to the idea that if I'm learning all this
   relatively foundational stuff, it seems a bit pointless not to make that what I actually
   code and implement first.
