// TODO(ry) Re-enable this test on windows. It is flaky for an unknown reason.
#![cfg(not(windows))]

use deno::test_util::*;
use std::process::Command;

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

#[cfg(debug_assertions)]
const BUILD_VARIANT: &str = "debug";

#[cfg(not(debug_assertions))]
const BUILD_VARIANT: &str = "release";

#[test]
fn basic() {
  // let mut build_plugin_base = Command::new("cargo");
  // let mut build_plugin =
  //   build_plugin_base.arg("build").arg("-p").arg("test_plugin");
  // if BUILD_VARIANT == "release" {
  //   build_plugin = build_plugin.arg("--release");
  // }
  // let _build_plugin_output = build_plugin.output().unwrap();
  let output = deno_cmd()
    .arg("--allow-plugin")
    .arg("tests/gpu.spec.ts")
    .arg(BUILD_VARIANT)
    .output()
    .unwrap();
  let stdout = std::str::from_utf8(&output.stdout).unwrap();
  let stderr = std::str::from_utf8(&output.stderr).unwrap();
  if !output.status.success() {
    println!("stdout {}", stdout);
    println!("stderr {}", stderr);
  }
  assert!(output.status.success());
  let expected = if cfg!(target_os = "windows") {
    "Hello from plugin. data: test | zero_copy: test\nPlugin Sync Response: test\r\nHello from plugin. data: test | zero_copy: test\nPlugin Async Response: test\r\n"
  } else {
    "Hello from plugin. data: test | zero_copy: test\nPlugin Sync Response: test\nHello from plugin. data: test | zero_copy: test\nPlugin Async Response: test\n"
  };
  assert_eq!(stdout, expected);
  assert_eq!(stderr, "");
}
