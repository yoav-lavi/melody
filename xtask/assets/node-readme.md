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

const source = `1 to 5 of "A";`;

try {
  const output = compiler(source);
  // do something with output
} catch (error) {
  // do something with error
}
```

## Links

- [Language Documentation](https://yoav-lavi.github.io/melody/book/)