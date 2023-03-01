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

pub fn get_center_of_square(vert_1: [f32; 3], vert_2: [f32; 3], vert_3: [f32; 3], vert_4: [f32; 3]) -> (f32, f32, f32) {
    let center_x = (vert_1[0] + vert_2[0] + vert_3[0] + vert_4[0]) / 4.;
    let center_y = (vert_1[1] + vert_2[1] + vert_3[1] + vert_4[1]) / 4.;
    let center_z = (vert_1[2] + vert_2[2] + vert_3[2] + vert_4[2]) / 4.;

    (center_x, center_y, center_z)
}

fn get_center_of_canvas(canvas: &HtmlCanvasElement) -> (f32, f32) {
    let center_x = (canvas.client_width() / 2) as f32;
    let center_y = (canvas.client_height() / 2) as f32;

    (center_x, center_y)
}

pub fn rad_to_deg(angle_in_rad: f32) -> f32 {
    return angle_in_rad * 180. / PI as f32;
}