# nd-jpegli-rs
A `setjmp`-shim-based wrapper around jpegli.

## Warning
This is probably slower than the [jpegli](https://docs.rs/jpegli/latest/jpegli/) crate due to the way it wraps api calls.
However, this crate still works if `panic = "abort"`.
Do note that jpegli's errors seem terrible compared to libjpeg's, 
so proper error handling might not even be needed/possible for your use case.
In particular, error codes and the error message table have been removed.

## Custom Jpegli Libraries
This library currently does NOT support custom Jpegli libraries.
This is because the `boolean` data type varies per platform,
and the workarounds in place for the Rust bindings will break if used with an external library.

## References
 * https://github.com/libjpeg-turbo/libjpeg-turbo/blob/51d021bf0168ee2d6ad79f70248a88b7f57156d0/libjpeg.txt
 * https://github.com/rust-lang/rust/issues/39016
 * https://github.com/libjpeg-turbo/libjpeg-turbo/blob/cbdc20fbf45b4595506331157c8e0a13bde0d401/example.c
 * https://github.com/mozilla/mozjpeg/blob/6c9f0897afa1c2738d7222a0a9ab49e8b536a267
 * https://github.com/libjpeg-turbo/libjpeg-turbo/blob/cbdc20fbf45b4595506331157c8e0a13bde0d401
 * https://github.com/Automattic/node-canvas/blob/f138b3a667c50935d3e1ffa69ed0fb56f0f2877c/src/Image.cc
 * https://github.com/cloudflare/jpegtran/blob/81d87b4c6ce469031663287b788539d11d5e3b3a/jpeglib.h
 * https://stackoverflow.com/questions/5874215/what-is-rpcndr-h
 * https://stackoverflow.com/questions/45135/why-does-the-order-in-which-libraries-are-linked-sometimes-cause-errors-in-gcc
 * https://stackoverflow.com/questions/3735804/undefined-reference-to-operator-new
 * https://stackoverflow.com/questions/45047508/error-unknown-type-name-constexpr-during-make-in-mac-os-x
 