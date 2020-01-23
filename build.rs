fn main() {
  cc::Build::new()
    .file("src/adapter.c")
    .file("src/device.c")
    .include("src")
    .compile("libwgpu_deno_c_interop.a");
}
