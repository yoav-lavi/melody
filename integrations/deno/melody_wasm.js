let wasm;

const cachedTextDecoder = new TextDecoder("utf-8", {
  ignoreBOM: true,
  fatal: true,
});

cachedTextDecoder.decode();

let cachedUint8Memory0;
function getUint8Memory0() {
  if (cachedUint8Memory0.byteLength === 0) {
    cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
  }
  return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
  return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
  if (heap_next === heap.length) heap.push(heap.length + 1);
  const idx = heap_next;
  heap_next = heap[idx];

  heap[idx] = obj;
  return idx;
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = new TextEncoder("utf-8");

const encodeString =
  typeof cachedTextEncoder.encodeInto === "function"
    ? function (arg, view) {
        return cachedTextEncoder.encodeInto(arg, view);
      }
    : function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
          read: arg.length,
          written: buf.length,
        };
      };

function passStringToWasm0(arg, malloc, realloc) {
  if (realloc === undefined) {
    const buf = cachedTextEncoder.encode(arg);
    const ptr = malloc(buf.length);
    getUint8Memory0()
      .subarray(ptr, ptr + buf.length)
      .set(buf);
    WASM_VECTOR_LEN = buf.length;
    return ptr;
  }

  let len = arg.length;
  let ptr = malloc(len);

  const mem = getUint8Memory0();

  let offset = 0;

  for (; offset < len; offset++) {
    const code = arg.charCodeAt(offset);
    if (code > 0x7f) break;
    mem[ptr + offset] = code;
  }

  if (offset !== len) {
    if (offset !== 0) {
      arg = arg.slice(offset);
    }
    ptr = realloc(ptr, len, (len = offset + arg.length * 3));
    const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
    const ret = encodeString(arg, view);

    offset += ret.written;
  }

  WASM_VECTOR_LEN = offset;
  return ptr;
}

let cachedInt32Memory0;
function getInt32Memory0() {
  if (cachedInt32Memory0.byteLength === 0) {
    cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
  }
  return cachedInt32Memory0;
}

function getObject(idx) {
  return heap[idx];
}

function dropObject(idx) {
  if (idx < 36) return;
  heap[idx] = heap_next;
  heap_next = idx;
}

function takeObject(idx) {
  const ret = getObject(idx);
  dropObject(idx);
  return ret;
}
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
export function compiler(source) {
  try {
    const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
    const ptr0 = passStringToWasm0(
      source,
      wasm.__wbindgen_malloc,
      wasm.__wbindgen_realloc
    );
    const len0 = WASM_VECTOR_LEN;
    wasm.compiler(retptr, ptr0, len0);
    var r0 = getInt32Memory0()[retptr / 4 + 0];
    var r1 = getInt32Memory0()[retptr / 4 + 1];
    var r2 = getInt32Memory0()[retptr / 4 + 2];
    var r3 = getInt32Memory0()[retptr / 4 + 3];
    var ptr1 = r0;
    var len1 = r1;
    if (r3) {
      ptr1 = 0;
      len1 = 0;
      throw takeObject(r2);
    }
    return getStringFromWasm0(ptr1, len1);
  } finally {
    wasm.__wbindgen_add_to_stack_pointer(16);
    wasm.__wbindgen_free(ptr1, len1);
  }
}

async function load(module, imports) {
  if (typeof Response === "function" && module instanceof Response) {
    if (typeof WebAssembly.instantiateStreaming === "function") {
      try {
        return await WebAssembly.instantiateStreaming(module, imports);
      } catch (e) {
        if (module.headers.get("Content-Type") != "application/wasm") {
          console.warn(
            "`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",
            e
          );
        } else {
          throw e;
        }
      }
    }

    const bytes = await module.arrayBuffer();
    return await WebAssembly.instantiate(bytes, imports);
  } else {
    const instance = await WebAssembly.instantiate(module, imports);

    if (instance instanceof WebAssembly.Instance) {
      return { instance, module };
    } else {
      return instance;
    }
  }
}

function getImports() {
  const imports = {};
  imports.wbg = {};
  imports.wbg.__wbindgen_error_new = function (arg0, arg1) {
    const ret = new Error(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
  };

  return imports;
}

function initMemory(imports, maybe_memory) {}

function finalizeInit(instance, module) {
  wasm = instance.exports;
  init.__wbindgen_wasm_module = module;
  cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
  cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);

  return wasm;
}

function initSync(bytes) {
  const imports = getImports();

  initMemory(imports);

  const module = new WebAssembly.Module(bytes);
  const instance = new WebAssembly.Instance(module, imports);

  return finalizeInit(instance, module);
}

async function init(input) {
  if (typeof input === "undefined") {
    input = new URL("melody_wasm_bg.wasm", import.meta.url);
  }
  const imports = getImports();

  if (
    typeof input === "string" ||
    (typeof Request === "function" && input instanceof Request) ||
    (typeof URL === "function" && input instanceof URL)
  ) {
    input = fetch(input);
  }

  initMemory(imports);

  const { instance, module } = await load(await input, imports);

  return finalizeInit(instance, module);
}

export { initSync };
export default init;
