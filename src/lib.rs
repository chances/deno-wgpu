#[macro_use]
extern crate deno_core;
extern crate futures;
extern crate futures_executor;

// References:
// https://github.com/eliassjogreen/deno_webview/blob/e706cf83bd7230e528afef07c6aa8ea669eb48e9/src/lib.rs
// https://github.com/eliassjogreen/deno_webview/blob/master/src/lib.rs
//
// https://github.com/denoland/deno/blob/47a580293eb5c176497264f1c3f108bf6b2c480d/test_plugin/src/lib.rs
// https://github.com/denoland/deno/blob/master/test_plugin/src/lib.rs

use deno_core::CoreOp;
use deno_core::Op;
use deno_core::PluginInitContext;
use deno_core::{Buf, ZeroCopyBuf};
use futures::future::FutureExt;
use futures_executor::block_on;
// use std::convert::TryInto;

use winit::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::Window,
};

fn init(context: &mut dyn PluginInitContext) {
  context.register_op("testSync", Box::new(op_test_sync));
  context.register_op("requestAdapter", Box::new(op_request_adapter));
}
init_fn!(init);

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

// https://stackoverflow.com/a/27826181/1363247
lazy_static! {
  static ref ADAPTERS: Mutex<Vec<wgpu::Adapter>> = Mutex::new(Vec::new());
}

// TODO: Consider changing this to an op that creates the window w/ an adapter
pub fn op_request_adapter(data: &[u8], zero_copy: Option<ZeroCopyBuf>) -> CoreOp {
  let fut = async move {
    // TODO: Deserialize the params data
    // let data_str = std::str::from_utf8(&data[..]).unwrap().to_string();

    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    // let size = window.inner_size();
    let surface = wgpu::Surface::create(&window);
    let satisfactory_backends = wgpu::BackendBit::from_bits(
      wgpu::BackendBit::PRIMARY.bits() | wgpu::BackendBit::SECONDARY.bits(),
    )
    .unwrap();
    let adapter_options = wgpu::RequestAdapterOptions {
      power_preference: wgpu::PowerPreference::Default,
      compatible_surface: Some(&surface),
    };

    let future_adapter =
      wgpu::Adapter::request(&adapter_options, satisfactory_backends).map(|adapter| {
        // TODO: Check for None adapter
        let mut adapters = ADAPTERS.lock().unwrap();
        adapters.push(adapter.unwrap());
        let adapters_count = adapters.len() as u8;
        let result = std::slice::from_ref::<u8>(&adapters_count);
        let result_box: Buf = result.into();
        Ok(result_box)
      });
    // .ok_or(())
    // .ok_or(b"No satisfactory graphics device found")
    let result = block_on(future_adapter);
    result
  };

  Op::Async(fut.boxed())
}
