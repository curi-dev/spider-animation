use std::f64::consts::PI;
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

pub fn deg_to_rad(angle_in_deg: f32) -> f32 {
    return angle_in_deg * PI as f32 / 180.;
}

pub fn rad_to_deg(angle_in_rad: f32) -> f32 {
    return angle_in_rad * 180. / PI as f32;
}

// pub fn normalize(v: [f32; 3]) {
//     let length = Math.sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]);
//     // make sure we don't divide by 0.
//     if length > 0.00001 {
//       return [v[0] / length, v[1] / length, v[2] / length];
//     } else {
//       return [0, 0, 0];
//     }
//   }