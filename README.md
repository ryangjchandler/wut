# Wut

Wut is a small programming language that compiles into WebAssembly Text Format (WAT) files. I'm using it as a project to better understand WebAssembly's Text Format and how languages themselves generate WebAssembly code.

By compiling a language to WebAssembly, you open up a whole new world of possibilities. You can execute the language on it's own outside of a browser by using an embedded WASM runtime, or you can compile into `.wasm` files and ship those out with some JavaScript bindings to execute code on the front-end.

## Roadmap

* [x] Get basic `.wat` generation working.
* [ ] Add host bindings for a generic `print()` function.
* [ ] Add static types.
* [ ] Add support for strings.
* [ ] Add support for arrays.
