<h1 align="center">ASCIIFRAME</h1>
<p align="center">Convert any video to a stream of ASCII frames<p>
<p align="center">
    <img href="https://crates.io/crates/asciiframe" src="https://img.shields.io/crates/v/asciiframe?style=for-the-badge" alt="crates.io version">
    <img href="https://github.com/luke-rt/asciiframe/actions/workflows/CICD.yml" src="https://img.shields.io/github/workflow/status/luke-rt/asciiframe/CICD?style=for-the-badge" alt="CICD">
    <img href="https://github.com/luke-rt/asciiframe/blob/main/LICENSE" src="https://img.shields.io/github/license/luke-rt/asciiframe?style=for-the-badge" alt="MIT license">
</p>
<p align="center">
    <img href="https://github.com/luke-rt/asciiframe/blob/main/docs/demo.gif" src="https://github.com/luke-rt/asciiframe/blob/main/docs/demo.gif" alt="Demo">
</p>

## How To Use
### Usage
```sh
$ asc [OPTIONS] --file <FILE>
```

### Options
File
```
-f, --file <INPUT FILE>
```
Specify the input video file to be convertered

Output
```
-o, --output <OUTPUT FILE>
```
Optional filename for asciiframe to write rendered output to, which can be run later to display the video

## Installation
Ensure `opencv` and `pkg-config` is installed!
```
cargo install asciiframe
```
Make sure `~/.cargo/bin` is in your path

MacOS:
- `brew install pkg-config opencv`
- If you get `dyld: Library not loaded: @rpath/libclang.dylib` then run `export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/usr/lib/"` first

## Library

`asciiframe` is also a package on [`crates.io`](https://crates.io/crates/asciiframe)
so you can use it in your own project

Once you've added it to your `Cargo.toml`, you can use the `asciiframe::render()` function,
which takes an input file `fin`, `width`, `height`, and finally `render_frame`, a function with the input of a string

As `asciiframe::render` iterates through the video and converts each frame to a string of ASCII
characters, it then calls `render_frame` on the string before moving onto the next video frame

As a result, you can call render and pass in a function/closure as `render_frame`.
The function thus gets called on every frame the moment it gets rendered.

For example

```rust
asciiframe::render(fin, width, height, |frame| {
	println!("{esc}c", esc = 27 as char);
	println!("{}", frame.data);
	Ok(())
});
```

This function would print out the frame to stdout the moment it gets converted to ASCII.

## Contributing

## About
### Authors and Contributors
- [luke-rt](https://github.com/luke-rt)(Luke T)

## TODO
- prebuilt binaries
- examples

### License
[MIT License](https://github.com/luke-rt/asciiframe/blob/main/LICENSE)
