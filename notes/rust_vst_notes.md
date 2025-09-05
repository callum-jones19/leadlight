# Rust VST Notes

## Rust VST 3 Bindings

What are the TODOs inside the codebase?

- The repository’s README comments that where elements of the API binding is incomplete, there is a TODO in the relevant section of code. We can therefore find these missing elements by searching the codebase for TODOs.
- The repository README also mentions that ‘the crate intentionally omits anything not part of the COM-compatible API.
- There are actually two crates within this repository:
  - Vst3-sys is the main one that we are using
  - There is also a sub-crate, called vst3-com, that the main crate is built around
    and uses.
  - Com is a fork of com-rs
    - Com-rs has now been deprecated
    - COM is a Microsoft platform-independent system for “creating binary software 
      components that can interact.” (From the MS docs)
      - “To understand COM (and therefore all COM-based technologies), it is crucial
        to understand that it is not an object-oriented language but a standard.”
      - “The only language requirement for COM is that code is generated in a language 
      that can create structures of pointers and, either explicitly or implicitly, call 
      functions through pointers”
      - “COM defines the essential nature of a COM object. In general, a software object 
      is made up of a set of data and the functions that manipulate the data. A COM object 
      is one in which access to an object's data is achieved exclusively through one or 
      more sets of related functions. These function sets are called interfaces, and the 
      functions of an interface are called methods. Further, COM requires that the only 
      way to gain access to the methods of an interface is through a pointer to the 
      interface”

VST3-sys therefore creates its compatability with VST3 native by using the COM interface to make Rust bindings that call and execute the native C++ VST parts.
To understand what is going on in the VST3 Rust bindings, we need to understand the architecture of the core VST3 module around which the Rust library is being built.
•	From the Steinberg VST 3 technical documentation:
  - VST 3 is based on a technology called VST Module Architecture (VST-MA).
o	A VST3 effect has two parts
