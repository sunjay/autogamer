# autogamer

An opinionated, convention over configuration game creation framework designed
for use with the [Tiled editor] and the [Python] programming language.

This is explicitly not a game engine. autogamer is a framework on top of an
existing game engine designed to allow you to create simple games directly in
the Tiled editor. Those games may then be extended using the Python programming
language.

[Tiled editor]: https://www.mapeditor.org
[Python]: https://www.python.org

## Building

Examples of different build commands:

```bash
./x.py build
./x.py build --release
./x.py build --target x86_64-pc-windows-gnu
```

Uses [Rust] and [PyO3]. You may require additional dependencies for your
particular platform.

```bash
apt install python3-dev python-dev
```

[Rust]: https://rustup.rs
[PyO3]: https://pyo3.rs
