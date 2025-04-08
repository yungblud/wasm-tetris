use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext;

// JS에서 호출 할 수 있도록 설정
#[wasm_bindgen]
pub fn init_webgl() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No global `window` exists")?;
    let document = window.document().ok_or("Should have a document on window")?;
    let canvas = document
        .get_element_by_id("gameCanvas")
        .ok_or("Canvas not found")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let gl = canvas
        .get_context("webgl")?
        .ok_or("Failed to get WebGL context")?
        .dyn_into::<WebGlRenderingContext>()?;

    // 캔버스 배경색 설정
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    Ok(())
}

#[wasm_bindgen]
pub fn draw_block(x: f32, y: f32) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("no global window")?;
    let document = window.document().ok_or("no document")?;
    let canvas = document
        .get_element_by_id("gameCanvas")
        .ok_or("no canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let gl = canvas
        .get_context("webgl")?
        .ok_or("no webgl")?
        .dyn_into::<WebGlRenderingContext>()?;

    gl.clear_color(0.1, 0.1 + x, 0.4 + y, 1.0);
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    Ok(())
}