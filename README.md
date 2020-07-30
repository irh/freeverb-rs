# freeverb-rs

A Rust implementation of the Freeverb algorithm.

## About Freeverb

Freeverb was originally written in C++ by "Jezar at Dreampoint", and was released into the public domain in June 2000. It is now widely used in various incarnations in multiple software packages.

- [Analysis of the Freeverb algorithm](https://ccrma.stanford.edu/~jos/pasp/Freeverb.html)
- [More information and a link to original C++ source](http://freeverb3vst.osdn.jp/sites.shtml)

## About freeverb-rs

This implementation of Freeverb in Rust is an almost direct conversion of the original source, created as a demonstration project for a [talk I gave about Rust at the Audio Developer Conference 2018](https://www.youtube.com/watch?v=Yom9E-67bdI). The code has been updated since then, so if you want to follow along with the talk then take a look at the [the adc-2018 branch](/irh/freeverb-rs/tree/adc-2018).

There are a couple of (intentional) differences to the original implementation:
- delay line buffers are dynamically allocated for simplicity. This may have a performance impact, and once generic constants are available I would like to make the buffer static.
- 64 bit processing is used internally whereas the original is 32 bit. I might make the sample type configurable at some point, but for now 64 bit processing seemed like a sensible default.

## Folder structure

`freeverb/`

This contains the core implementation of Freeverb, with a simple interface.

`app_gtk`

A very basic audio+GUI application that runs the `freeverb::Freeverb` processor.

You will need `gtk` installed on your system for this to work.

`audio_module`

This contains a (very) experimental generic module+parameter library, which I really only added as an excuse to explore approaches to polymorphism. The `audio_module` approach is currently only used by `app_gtk`, it might go away in the future, or maybe I'll decide I like it and continue to work on it. At this point I don't know!

`freeverb_module`

The `freeverb` processor wrapped up as an `AudioModule`, currently only used by `app_gtk`.

`app_juce`

A very basic JUCE application that runs the `freeverb::Freeverb` processor via a statically linked library.

`clib`

A static library that provides C bindings to the `freeverb::Freeverb` processor, used by app_juce.

`wasm`

A library that provides a `wasm-bindgen` interface to the `freeverb::Freeverb` processor.

Also in the folder is a small web application that runs the `wasm` processor.


