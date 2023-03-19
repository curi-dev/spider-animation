use wasm_bindgen::{JsCast, JsValue};
use web_sys::WebGlRenderingContext;


pub fn initialize_webgl_context() -> Result<(WebGlRenderingContext, web_sys::HtmlCanvasElement), JsValue> {
    
    let window = web_sys::window().unwrap();
    
    let document = window.document().unwrap();

    let canvas = document.get_element_by_id("glCanvas").unwrap();
    
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;


    let gl = canvas
        .get_context("webgl")? // without ? does not compile
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;
       
    gl.clear_color(1., 1., 0.5, 1.); 
    gl.enable(WebGlRenderingContext::CULL_FACE);
    gl.enable(WebGlRenderingContext::DEPTH_TEST);
 
    Ok((gl, canvas))
}

