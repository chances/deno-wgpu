#include <napi.h>
#include <wgpu.h>

static Napi::String Method(const Napi::CallbackInfo& info) {
  Napi::Env env = info.Env();
  return Napi::String::New(env, "Hello, world!");
}

Napi::Value requestAdapter(const Napi::CallbackInfo& info) {
  Napi::Env env = info.Env();
  Napi::Promise::Deferred deferred = Napi::Promise::Deferred::New(env);
  WGPURequestAdapterOptions adapterOptions = {
    .power_preference = WGPUPowerPreference_Default
  };
  if (info.Length() == 1 && info[0].IsObject()) {
    Napi::Object options = info[0].ToObject();
    if (options.HasOwnProperty("powerPreference") && options.Get("powerPreference").IsString()) {
      std::string powerPreference = options.Get("powerPreference").ToString().Utf8Value();
      if (powerPreference == "low-power") {
        adapterOptions.power_preference = WGPUPowerPreference_LowPower;
      } else if (powerPreference == "high-performance") {
        adapterOptions.power_preference = WGPUPowerPreference_HighPerformance;
      }
    }
  }
  WGPUAdapterId adapterId = wgpu_request_adapter(&adapterOptions);
  deferred.Resolve(Napi::Number::New(info.Env(), adapterId));

  return deferred.Promise();
}

static Napi::Object GPU(Napi::Env env) {
  Napi::Object gpu = Napi::Object::New(env);
  gpu.DefineProperty(Napi::PropertyDescriptor::Function(env, gpu, "requestAdapter", requestAdapter, napi_enumerable, nullptr));
  return gpu;
}

static Napi::Object Init(Napi::Env env, Napi::Object exports) {
  exports.Set(Napi::String::New(env, "GPU"), GPU(env));
  exports.Set(Napi::String::New(env, "hello"),
              Napi::Function::New(env, Method));
  return exports;
}

NODE_API_MODULE(hello, Init)
