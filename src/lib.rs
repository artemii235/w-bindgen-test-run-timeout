use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::console;

#[wasm_bindgen(module = "/async_sleep.js")]
extern "C" {
    fn async_sleep(ms: u32) -> Promise;
}

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn will_fail() {
    console::log_1(&"Start my long integration test".into());
    JsFuture::from(async_sleep(21000)).await.unwrap();
    console::log_1(&"My long integration test finished successfully".into());
}
