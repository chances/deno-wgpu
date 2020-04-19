#[macro_use]
extern crate deno_core;
extern crate futures;
extern crate futures_executor;
extern crate serde;
extern crate serde_json;

use deno_core::CoreOp;
use deno_core::Op;
use deno_core::PluginInitContext;
use deno_core::{Buf, ZeroCopyBuf};
use futures::future::FutureExt;
use futures_executor::block_on;
use serde::{Deserialize, Serialize};

// References:
// https://github.com/eliassjogreen/deno_webview/blob/e706cf83bd7230e528afef07c6aa8ea669eb48e9/src/lib.rs
// https://github.com/eliassjogreen/deno_webview/blob/master/src/lib.rs
//
// https://github.com/denoland/deno/blob/47a580293eb5c176497264f1c3f108bf6b2c480d/test_plugin/src/lib.rs
// https://github.com/denoland/deno/blob/master/test_plugin/src/lib.rs

// Generated modules
// deno --allow-read --allow-write --allow-run scripts/idl.ts
mod enums;
mod params;

use winit::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::{Window, WindowId},
};

fn init(context: &mut dyn PluginInitContext) {
  context.register_op("testSync", Box::new(op_test_sync));
  context.register_op("requestAdapter", Box::new(op_request_adapter));
}
init_fn!(init);

// TODO: Post about this project to https://github.com/denoland/deno/issues/1629 and a plugin wip issue, (i.e. a la https://github.com/denoland/deno/issues/4481)

#[derive(Serialize)]
struct OpResponse<T> {
  err: Option<String>,
  ok: Option<T>,
}

// TODO: Generate struct decls given WebGPU WebIDL definitions
// https://crates.io/crates/webidl

#[derive(Serialize)]
struct RequestAdapterResult {
  id: u32,
}

fn serialize_response<T>(response: Result<T, String>) -> Buf where T: Serialize {
  let response: OpResponse<T> = match response {
    Err(message) => OpResponse {
      err: Some(message),
      ok: None
    },
    Ok(data) => OpResponse {
      err: None,
      ok: Some(data)
    }
  };
  serde_json::to_vec(&response).unwrap().into_boxed_slice()
}

pub fn op_test_sync(data: &[u8], zero_copy: Option<ZeroCopyBuf>) -> CoreOp {
  if let Some(buf) = zero_copy {
    let data_str = std::str::from_utf8(&data[..]).unwrap();
    let buf_str = std::str::from_utf8(&buf[..]).unwrap();
    println!(
      "Hello from plugin. data: {} | zero_copy: {}",
      data_str, buf_str
    );
  }
  let result = b"test";
  let result_box: Buf = Box::new(*result);
  Op::Sync(result_box)
}

use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashMap;

// https://stackoverflow.com/a/27826181/1363247
lazy_static! {
  static ref WINDOWS: Mutex<Vec<Window>> = Mutex::new(Vec::new());
  static ref ADAPTERS: Mutex<HashMap<WindowId, wgpu::Adapter>> = Mutex::new(HashMap::new());
}

// TODO: Consider changing this to an op that creates the window w/ an adapter
pub fn op_request_adapter(data: &[u8], zero_copy: Option<ZeroCopyBuf>) -> CoreOp {
  let fut = async move {
    // TODO: Deserialize the params data
    // let data_str = std::str::from_utf8(&data[..]).unwrap().to_string();

    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    let window_id = window.id();
    let surface = wgpu::Surface::create(&window);

    let mut windows = WINDOWS.lock().unwrap();
    windows.push(window);

    let satisfactory_backends = wgpu::BackendBit::from_bits(
      wgpu::BackendBit::PRIMARY.bits() | wgpu::BackendBit::SECONDARY.bits(),
    )
    .unwrap();
    let adapter_options = wgpu::RequestAdapterOptions {
      power_preference: wgpu::PowerPreference::Default,
      compatible_surface: Some(&surface),
    };

    let future_adapter =
      wgpu::Adapter::request(&adapter_options, satisfactory_backends).map(|maybe_adapter| {
        let adapter_or_err = match maybe_adapter {
          None => {
            Err(String::from("Could not find satisfactory adapter"))
          },
          Some(adapter) => {
            let mut adapters = ADAPTERS.lock().unwrap();
            adapters.insert(window_id, adapter);
            let adapters_count = adapters.len() as u32;
            Ok(RequestAdapterResult { id: adapters_count })
          }
        };
        Ok(serialize_response(adapter_or_err))
      });
    block_on(future_adapter)
  };

  Op::Async(fut.boxed())
}

// TODO: Add window resizing ops
