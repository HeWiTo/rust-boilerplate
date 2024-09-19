import("../../crates/wasm/pkg")
  .then((module) => {
    module.run_app();
  })
  .catch(console.error);
