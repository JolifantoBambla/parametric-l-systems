# Parametric L-systems
An implementation of parametric L-systems for the Fractals course at TU Wien.

On Windows, MacOS, and ChromeOS, you can check it out directly at [jolifantobambla.github.io/parametric-l-systems](https://jolifantobambla.github.io/parametric-l-systems/) in Google Chrome (from version 94 to 114) thanks to the [WebGPU origin trial](https://developer.chrome.com/origintrials/#/view_trial/118219490218475521).

## Build
Install build dependencies:
* [Install Rust](https://www.rust-lang.org/tools/install)
* [Install wasm-pack](https://rustwasm.github.io/wasm-pack/installer)

Then, within the project root directory, run
```bash
wasm-pack build --target web
```

## Run
Install requirements:
* Install a [browser that supports WebGPU](https://github.com/gpuweb/gpuweb/wiki/Implementation-Status), and if necessary, enable WebGPU in the browser (e.g., on Linux: Chromium with flags `--enable-vulkan --enable-unsafe-webgpu`).
* Install a HTTP server (e.g., [Python 3](https://www.python.org/downloads/) and its [http.server module](https://docs.python.org/3/library/http.server.html), or the [NodeJS](https://nodejs.org/en/) module [http-server](https://www.npmjs.com/package/http-server))

Then, within the project root directory
* Serve the project using an HTTP server (e.g., `python -m http.server`, or `http-server`)
* Navigate to the project's `index.html` using the browser with WebGPU enabled (e.g., `localhost:8000` when using the Python HTTP server defaults, or `localhost:8080` when using `http-server`'s defaults)

## User documentation
The documentation can be found [here](DOCUMENTATION.md), or directly under the `Documentation` tab on the website (either served locally, or at [jolifantobambla.github.io/parametric-l-systems](https://jolifantobambla.github.io/parametric-l-systems/))

## Test scenes
The following test scenes can be found [here](https://github.com/JolifantoBambla/parametric-l-systems/tree/main/scenes/):
| Name                                             | Description                                                                                                                                                                                     | Source                                                                                                                                   |
| ------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| abop-fig-1-19.json                               | A 3D Hilbert curve with 64 point light sources.                                                                                                                                                 | [Algorithmic beaty of plants, Fig. 1.19](http://algorithmicbotany.org/papers/abop/abop.pdf)                           |
| abop-fig-2-6.json                                | Honda's tree model.                                                                                                                                                                             | [Algorithmic beaty of plants, Fig. 2.6d](http://algorithmicbotany.org/papers/abop/abop.pdf)                           |
| forest-abop-fig-2-6.json                         | All four trees shown in Fig. 2.6 in the book.                                                                                                                                                   | [Algorithmic beaty of plants, Fig. 2.6](http://algorithmicbotany.org/papers/abop/abop.pdf)                           |
| abop-fig-2-7.json                                | Aono and Kunii's tree model.                                                                                                                                                                     | [Algorithmic beaty of plants, Fig. 2.7d](http://algorithmicbotany.org/papers/abop/abop.pdf)                           |
| forest-abop-fig-2-7.json                          | All four trees shown in Fig. 2.7 in the book.                                                                                                                                                   | [Algorithmic beaty of plants, Fig. 2.7](http://algorithmicbotany.org/papers/abop/abop.pdf)                           |
| tree-prusinkiewicz.json                           | A tree model as shown in Fig. 8g in the paper.                                                                                                                                                 | [L-systems: from the Theory to Visual Models of Plants, Fig. 8g](http://algorithmicbotany.org/papers/sigcourse.2003/2-1-lsystems.pdf) |
| growing-tree-prusinkiewicz.json                   | Four instances of the tree model as shown in Fig. 8g in the paper with different numbers of iterations.                                                                                 | [L-systems: from the Theory to Visual Models of Plants, Fig. 8g](http://algorithmicbotany.org/papers/sigcourse.2003/2-1-lsystems.pdf) |
| tree-prusinkiewicz-using-primitives.json          | A tree adapted from Fig. 8 in the paper s.t. it has some leaves.                                                                                                                               | [L-systems: from the Theory to Visual Models of Plants, Fig. 8](http://algorithmicbotany.org/papers/sigcourse.2003/2-1-lsystems.pdf) |
| random-forest-prusinkiewicz-using-primitives.json | Nine trees adapted from Fig. 8 in the paper s.t. it they have some leaves and use some randomness in their productions, i.e., the trees will look different every time the scene is rendered. | [L-systems: from the Theory to Visual Models of Plants, Fig. 8](http://algorithmicbotany.org/papers/sigcourse.2003/2-1-lsystems.pdf) |
| tree-prusinkiewicz-with-floor.json                | A tree model as shown in Fig. 8g in the paper and a large quad as floor.                                                                                                                       | [L-systems: from the Theory to Visual Models of Plants, Fig. 8](http://algorithmicbotany.org/papers/sigcourse.2003/2-1-lsystems.pdf) |
| tree-stochastic.json                              | A tree adapted from Fig. 8 in the paper s.t. it has uses some randomness in its productions.                                                                                                   | [L-systems: from the Theory to Visual Models of Plants, Fig. 8](http://algorithmicbotany.org/papers/sigcourse.2003/2-1-lsystems.pdf) |
