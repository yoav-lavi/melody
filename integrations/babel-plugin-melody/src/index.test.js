const { describe, it } = require('node:test');
const { default: pluginTester } = require('babel-plugin-tester');
const melodyPlugin = require('./index.js');

globalThis.describe = describe;
globalThis.it = it;

pluginTester({
  plugin: melodyPlugin,
  name: 'babel-plugin-melody',
  describe: '',
  tests: [
    {
      code: `const regex = new RegExp(/*melody*/'2 to 3 of "na";')`,
      output: 'const regex = new RegExp("(?:na){2,3}");',
    },
    {
      code: 'const regex = new RegExp(/*melody*/`2 to 3 of "na";`)',
      output: 'const regex = new RegExp("(?:na){2,3}");',
    },
    {
      code: `const regex = new RegExp(/*melody*/"2 to 3 of 'na';")`,
      output: 'const regex = new RegExp("(?:na){2,3}");',
    },
  ],
});
