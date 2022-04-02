<p align="center">
    <img alt="Melody Logo" height="250px" src="https://user-images.githubusercontent.com/14347895/159069181-53bce5b3-a831-43f1-8c14-af6c6ed7b92b.svg">
</p>

<p align="center">
NodeJS bindings for the Melody language compiler
</p>

## Install

```sh
npm install melodyc
```
or

```sh
yarn add melodyc
```

## Usage

```js
const { compiler } = require("melodyc");

const source = `
<start>;

option of "v";

capture major {
  some of <digit>;
}

".";

capture minor {
  some of <digit>;
}

".";

capture patch {
  some of <digit>;
}

<end>;
`;

try {
  const output = compiler(source);
  let regexp = new RegExp(output);
  let match = regexp.test("v1.1.1");
  console.log({ match }); // true
} catch (error) {
  // handle compilation error
}
```

## Links

- [Language Documentation](https://yoav-lavi.github.io/melody/book/)