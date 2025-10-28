# freeverb-rs

A Rust implementation of the Freeverb algorithm.

## About Freeverb

Freeverb was originally written in C++ by "Jezar at Dreampoint", and was released into the public domain in June 2000. It is now widely used in various incarnations in multiple software packages.

- [Analysis of the Freeverb algorithm](https://ccrma.stanford.edu/~jos/pasp/Freeverb.html)
- [More information and a link to original C++ source](https://freeverb3-vst.sourceforge.io/sites.shtml)

## About freeverb-rs

This implementation of Freeverb in Rust is an almost direct conversion of the original source, created as a demonstration project for a [talk I gave about Rust at the Audio Developer Conference 2018](https://www.youtube.com/watch?v=Yom9E-67bdI). The code has been updated since then, so if you want to follow along with the talk then take a look at the `adc-2018` branch.

A difference from the original implementation is that delay line buffers are dynamically allocated so that lengths can be adjusted for sample rates other than 44.1kHz.

## Repo structure

[`crates/freeverb/`](./crates/freeverb)

This contains the core implementation of Freeverb, with a simple interface.

[`crates/audio_module`](./crates/audio_module)

This contains a (very) experimental generic module+parameter library, which I really only added as an excuse to explore approaches to polymorphism. The `audio_module` approach is currently only used by `app_gtk`, it might go away in the future, or maybe I'll decide I like it and continue to work on it. At this point I don't know!

[`crates/freeverb_module`](./crates/freeverb_module)

The `freeverb` processor wrapped up as an `AudioModule`, currently only used by `app_gtk`.

[`crates/clib`](./crates/clib)

A static library that provides C bindings to the `freeverb::Freeverb` processor, used by app_juce.

[`examples/app_gtk`](./examples/app_gtk)

A very basic audio+GUI application that runs the `freeverb::Freeverb` processor.

You will need `gtk4` installed on your system for this to work.

[`examples/app_juce`](./examples/app_juce)

A very basic JUCE application that runs the `freeverb::Freeverb` processor via a statically linked library.

[`examples/wasm`](./examples/wasm)

A library that provides a `wasm-bindgen` interface to the `freeverb::Freeverb` processor.

Also in the folder is a small web application that runs the `wasm` processor.
