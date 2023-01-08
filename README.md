# Parametric L-systems
An implementation of parametric L-systems for the Fractals course at TU Wien.

Check it out [TODO]().

## Install build dependencies
* [Install Rust](https://www.rust-lang.org/tools/install)
* [Install wasm-pack](https://rustwasm.github.io/wasm-pack/installer)

## Build
Within the project root directory, run
```bash
wasm-pack build --target web
```

## Run
* Install a [browser that supports WebGPU](https://github.com/gpuweb/gpuweb/wiki/Implementation-Status).
* If necessary, enable WebGPU in the browser (e.g. on Linux: Chromium with flags `--enable-vulkan --enable-unsafe-webgpu`)
* Navigate to `index.html` using the browser with WebGPU enabled.
