use wasm_bindgen::{JsCast, JsValue};
use web_sys::WebGlRenderingContext;


pub fn initialize_webgl_context() -> Result<(WebGlRenderingContext, web_sys::HtmlCanvasElement), JsValue> {
    let window = web_sys::window().unwrap();
    
    let document = window.document().unwrap();

    let canvas = document.get_element_by_id("glCanvas").unwrap();
    
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl")? // without ? does not compile
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;
    
    //context.enable(WebGlRenderingContext::BLEND);
    //context.blend_func(WebGlRenderingContext::SRC_ALPHA, WebGlRenderingContext::ONE_MINUS_SRC_ALPHA);
    context.clear_color(0.25, 0.25, 0.75, 1.); 
    //context.clear_depth(1.);
    
    Ok((context, canvas))
}