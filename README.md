# Rust Analyzer Issue

This is a repository to reproduce a bug in Rust Analyzer.

## Steps to build

1. Clone this repository
2. Install `cargo`, `bindgen` and `meson`
3. Run `meson setup build` in the root directory of the repository
4. Run `meson compile -C build` to build the project
5. The runnalbe should be placed at `build/myexe`

```bash
./build/myexe
# Output: Sum is 30
```
