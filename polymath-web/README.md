# Polymath Javascript Bindings

Polymath is an ASCIIMath implementation with the capabilities to produce MathML markup that can be rendered by many browsers.

It's implemented in Rust and compiled to WASM. This package provides an ECMASrcipt Module that can be loaded through a bundler or nodejs.

## Usage

For webpack the following config options have to be set:

```json
experiments: {
  asyncWebAssembly: true
},
```
For node pass this command line argument:

```bash
--experimental-wasm-modules
```

