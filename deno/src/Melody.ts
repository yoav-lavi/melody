import init, { compiler } from "./pkg/melody_wasm.js";
const initialCode = `/* matches the batman theme tune */
    16 of "na";

    2 of match {
      <space>;
      "batman";
    }
`;

function debug() {
  return function (
    _target: unknown,
    _propertyKey: string,
    descriptor: PropertyDescriptor,
  ) {
    const originalMethod = descriptor.value;
    descriptor.value = function (
      code: string,
      _options: { debug: boolean } = { debug: false },
    ) {
      if (_options.debug) {
        console.log(`
input:
--------------------------------------------------
${code || initialCode}
--------------------------------------------------
output: 
--------------------------------------------------
${originalMethod.call(this, code)}
--------------------------------------------------
      `);
      }
      return originalMethod.apply(this, [code]);
    };
    return descriptor;
  };
}
export class Melody {
  #code: string;
  #initalized = false;
  constructor(code?: string) {
    this.#code = code || initialCode;
  }
  /**
   * initalizes Melody
  */
  async init(): Promise<void> {
    await init();
    this.#initalized = true;
  }
  /**
   * compiles the code
   * @param code the code to compile
   * @param options the options to compile with
   * @returns the compiled code
   * @throws if the code is not initalized
   * 
  */
  @debug()
  compile(
    code: string = this.#code,
    _options: { debug: boolean } = { debug: false },
  ): string {
    this.#code = code;
    if (!this.#initalized) {
      throw new  Error(`
      Melody was not initalized
      --------------------------------------------------
      use Melody.init() to initalize Melody
      `);
    }
    return compiler(this.#code)
  }
}
