<div align="center">

  <h1><code>Rust/wasm Chaotic Attractor Module</code></h1>

<strong>An Rust based innerloop module for my javascript chaotic attractor web app <a href="https:https://github.com/dmaynard/chaos-screen-saver">chaos-screen-saver</a>.</strong>

</div>

## About

This is a Rust Implementation of the same chaotic attractor implemented in javascript [here](https://github.com/dmaynard/attractor-iterator) .

The idea is to use the chaotic attractor as a kind of visual benchmark and let the user choose between the ES6 version and the Rust-wasm version.

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build --scope davidsmaynard
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish --access=public
```

## 🔋 Batteries Included

- [`wasm-bindgen`]git fi(https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
- [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
- [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
```
