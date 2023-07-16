/* tslint:disable */
/* eslint-disable */
/**
*
*Compiles Melody source code to a regular expression
*
*# Errors
*
*Throws an error if a compilation error is encountered
*
*# Example
*
*```js
*const source = `
*  <start>;
*
*  option of "v";
*
*  capture major {
*    some of <digit>;
*  }
*
*  ".";
*
*  capture minor {
*    some of <digit>;
*  }
*
*  ".";
*
*  capture patch {
*    some of <digit>;
*  }
*
*  <end>;
*`;
*
*try {
*  const output = compiler(source);
*  new RegExp(output).test("v1.1.1"); // true
*} catch (error) {
*  // handle compilation error
*}
*```
* @param {string} source
* @returns {string}
*/
export function compiler(source: string): string;
/**
*/
export function main(): void;
