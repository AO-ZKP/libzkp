
let imports = {};
imports['wasi_snapshot_preview1'] = require('wasi_snapshot_preview1');
let wasm;

const path = require('path').join(__dirname, 'zkp_bg.wasm');
const bytes = require('fs').readFileSync(path);

const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
wasm = wasmInstance.exports;
module.exports.__wasm = wasm;

