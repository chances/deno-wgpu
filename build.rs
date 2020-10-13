use std::process::Command;

// Reference: https://doc.rust-lang.org/cargo/reference/build-script-examples.html#code-generation
// `cargo build -vv` to see debug output

fn main() {
  let mut gen_idl = deno_cmd();

  gen_idl.args(&["run", "--allow-read", "--allow-write", "--allow-run", "scripts/idl.ts"]);

  let gen_idl_err = String::from("failed to generate rust interfaces from WebGPU IDL");
  let output = gen_idl.output().expect(gen_idl_err.as_str());
  if output.status.code().expect("deno status code") != 0 {
    println!("{}", std::str::from_utf8(&output.stdout).unwrap());
    println!("{}", std::str::from_utf8(&output.stderr).unwrap());
    panic!(gen_idl_err);
  }

  println!("{}", std::str::from_utf8(&output.stdout).unwrap());
  println!("cargo:rerun-if-changed=third_party/gpuweb/spec/spec/webgpu.idl");
}

fn is_program_in_path(program: &str) -> bool {
  if let Ok(path) = std::env::var("PATH") {
      for p in path.split(":") {
          let p_str = format!("{}/{}", p, program);
          if std::fs::metadata(p_str).is_ok() {
              return true;
          }
      }
  }
  false
}

fn deno_cmd() -> Command {
  assert!(is_program_in_path("deno"));
  Command::new("deno")
}
