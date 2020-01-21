const filenameBase = "test_plugin";

let filenameSuffix = ".so";
let filenamePrefix = "lib";

if (Deno.build.os === "win") {
  filenameSuffix = ".dll";
  filenamePrefix = "";
}
if (Deno.build.os === "mac") {
  filenameSuffix = ".dylib";
}

const filename = `../target/${Deno.args[0]}/${filenamePrefix}${filenameBase}${filenameSuffix}`;
const plugin = Deno.openPlugin(filename);

// Construct Web GPU interface

/// <reference path="../third_party/gpuweb/types/dist/index.d.ts" />
// @deno-types="../third_party/gpuweb/types/dist/index.d.ts"
const api: GPU = {
}
export default api
