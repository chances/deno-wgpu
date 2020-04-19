/**
 * Returns the result of running UTF-8's encoder.
 */
export const encode = (new TextEncoder()).encode;
/**
 * Returns the result of running UTF-8's decoder.
 */
export const decode = (new TextDecoder()).decode;
