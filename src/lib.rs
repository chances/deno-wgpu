#[macro_use]
extern crate deno_core;
extern crate futures;

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
use std::convert::TryInto;

fn init(context: &mut dyn PluginInitContext) {
  context.register_op("testSync", Box::new(op_test_sync));
  context.register_op("testAsync", Box::new(op_test_async));
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

pub fn op_test_async(data: &[u8], zero_copy: Option<ZeroCopyBuf>) -> CoreOp {
  let data_str = std::str::from_utf8(&data[..]).unwrap().to_string();
  let fut = async move {
    if let Some(buf) = zero_copy {
      let buf_str = std::str::from_utf8(&buf[..]).unwrap();
      println!(
        "Hello from plugin. data: {} | zero_copy: {}",
        data_str, buf_str
      );
    }
    let (tx, rx) = futures::channel::oneshot::channel::<Result<(), ()>>();
    std::thread::spawn(move || {
      std::thread::sleep(std::time::Duration::from_secs(1));
      tx.send(Ok(())).unwrap();
    });
    assert!(rx.await.is_ok());
    let result = b"test";
    let result_box: Buf = Box::new(*result);
    Ok(result_box)
  };

  Op::Async(fut.boxed())
}

struct AdapterCollection {
  pub v: Vec<wgpu::Adapter>,
}
static Adapters: AdapterCollection = AdapterCollection {
  v: Vec::new(),
};

// pub fn op_request_adapter(data: &[u8], zero_copy: Option<PinnedBuf>) -> CoreOp {
//   let data_str = std::str::from_utf8(&data[..]).unwrap().to_string();
//   let fut = async move {
//     // TODO: Deserialize the params data
//     let satisfactoryBackends = wgpu::BackendBit::from_bits(
//       wgpu::BackendBit::PRIMARY.bits() | wgpu::BackendBit::SECONDARY.bits()
//     ).unwrap();
//     wgpu::Adapter::request(
//       &wgpu::RequestAdapterOptions {
//           power_preference: wgpu::PowerPreference::Default,
//           backends: satisfactoryBackends,
//       },
//     )
//     .map(|adapter| {
//       Adapters.v.push(adapter);
//       let result = std::slice::from_ref::<u8>(&(Adapters.v.len() as u8));
//       let result_buf: Buf = Box::new(*result);
//       result_buf
//     })
//     .ok_or(())
//     // .ok_or(b"No satisfactory graphics device found")
//   };

//   Op::Async(fut.boxed())
// }
