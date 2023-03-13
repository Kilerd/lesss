const { addBeforeLoader, loaderByName } = require('@craco/craco');
const cracoWasm = require("craco-wasm")
module.exports = {
  plugins: [cracoWasm()]
};