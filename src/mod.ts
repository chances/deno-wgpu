import promisify from "./promisify.ts";
import { encode } from "./encoding.ts";

const filenameBase = "wgpu_deno";

let filenamePrefix = "lib";
let filenameSuffix = ".so";

if (Deno.build.os === "win") {
  filenamePrefix = "";
  filenameSuffix = ".dll";
}
if (Deno.build.os === "mac") {
  filenameSuffix = ".dylib";
}

const filename = `./target/${Deno.args[0] ||
  "debug"}/${filenamePrefix}${filenameBase}${filenameSuffix}`;
const plugin = Deno.openPlugin(filename).ops;
// Promisify the ops
const ops = Object.keys(plugin).reduce(
  (promisifiedOps, opName) => {
    promisifiedOps[opName] = (...args) => promisify(plugin[opName], encode(JSON.stringify(args)));
    return promisifiedOps;
  },
  {} as {
    [name: string]: (...args: any[]) => ReturnType<typeof promisify>;
  }
);

console.log("Ops:", Deno.inspect(ops));

function notImplemented(opName: string) {
  return Promise.reject(
    new Deno.DenoError(Deno.ErrorKind.OpNotAvailable, `${opName} op not implemented`)
  );
}

// Construct Web GPU interface
export default {
  requestAdapter(options?: Object): Promise<Object> {
    options = options ? { ...options } : {};
    return ops.requestAdapter(options);
  }
};

export * from "../third_party/gpuweb/types/src/constants.ts";
