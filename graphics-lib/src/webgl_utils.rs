use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

pub fn resize_canvas_to_display_size(context: &WebGlRenderingContext, canvas: &HtmlCanvasElement) -> bool {
    let display_width = canvas.client_width();
    let display_height = canvas.client_height();

    let need_resize = display_width != canvas.width() as i32 || display_height != canvas.height() as i32;

    if need_resize {
        canvas.set_width(display_width as u32);
        canvas.set_height(display_height as u32);
    }
    
    context.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    need_resize
}