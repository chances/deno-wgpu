import { decode } from "./encoding.ts";

function decodeResponse(response: Uint8Array): any[] {
  return JSON.parse(decode(response))
}

export default function promisify(
  op: Deno.PluginOp, control: Uint8Array, zeroCopy?: ArrayBufferView | null
): Promise<any> {
  return new Promise<Uint8Array>((resolve) => {
    try {
      const syncResponse = op.dispatch(control, zeroCopy);
      if (syncResponse) {
        resolve(syncResponse);
      }
      op.setAsyncHandler((response) => {
        resolve(response);
      });
    }
    catch (e) {
      console.error(e);
      throw new Deno.DenoError(Deno.ErrorKind.NoAsyncSupport, "Promisify-ed plugin op failed");
    }
  }).then(decodeResponse);
}
