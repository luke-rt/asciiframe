<h1 align="center">ASCIIFRAME</h1>
<p align="center">A CLI tool for viewing videos in the terminal with ASCII<p>
<p align="center">
    <img src="https://img.shields.io/github/last-commit/seggfault/asciiframe?style=for-the-badge" alt="Last Commit">
    <img src="https://img.shields.io/github/license/seggfault/asciiframe?style=for-the-badge" alt="MIT license">
    <img src="https://img.shields.io/crates/v/asciiframe?style=for-the-badge" alt="crates.io version">
    <img src="https://img.shields.io/github/workflow/status/seggfault/asciiframe/CICD?style=for-the-badge" alt="CICD">
    <img src="https://img.shields.io/github/languages/code-size/seggfault/asciiframe?style=for-the-badge" alt="File size">
</p>

## How To Use
### Usage
```sh
$ asc [OPTIONS] --file <FILE>
```

### Options
File
```
-f, --file <FILE
```
Specify the input video file to be convertered

Output
```
-o, --output <OUTPUT>
```
Optional filename for asciiframe to write rendered output to, which can be run later to display the video

## Installation
### Source

Clone the repository then install it with
```
cargo install --path .
```
Make sure `~/.cargo/bin` is in your path

## Contributing
### Building from source
To build from source, you'll need `opencv` and `clang`

## About
### Authors and Contributors
- [ltgr](https://github.com/ltgr)(Luke T)

### License
[MIT License](https://github.com/ltgr/turbo/blob/master/LICENSE)
