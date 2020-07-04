const js = import("./node_modules/@metarobert/rust-wasm/rust_wasm.js");
js.then(js => {
  js.system_report();
});