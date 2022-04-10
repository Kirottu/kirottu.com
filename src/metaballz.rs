use crate::Model;
use seed::{
    prelude::{
        web_sys::{console, CanvasRenderingContext2d},
        *,
    },
    *,
};
use web_sys::HtmlImageElement;

#[derive(Clone, Copy)]
pub struct Metaball {
    pub x: f64,
    pub y: f64,
    pub r: f64,
    pub x_change: f64,
    pub y_change: f64,
    pub x_offset: f64,
    pub y_offset: f64,
}

impl Metaball {
    pub fn new(x: f64, y: f64, r: f64, x_change: f64, y_change: f64) -> Self {
        Self {
            x,
            y,
            r,
            x_change,
            y_change,
            x_offset: 0.0,
            y_offset: 0.0,
        }
    }
}

impl From<crate::CurrentMetaball> for Metaball {
    fn from(current_metaball: crate::CurrentMetaball) -> Self {
        Self {
            x: current_metaball.x.parse().unwrap_or(0.0),
            y: current_metaball.y.parse().unwrap_or(0.0),
            r: current_metaball.r.parse().unwrap_or(0.0),
            x_change: current_metaball.x_change.parse().unwrap_or(0.0),
            y_change: current_metaball.y_change.parse().unwrap_or(0.0),
            x_offset: 0.0,
            y_offset: 0.0,
        }
    }
}

pub fn marching_squares(model: &mut Model) {
    let _timer = crate::timer::Timer::new("Marching Squares");
    let canvas = canvas("metaballz").unwrap();
    let image = HtmlImageElement::new().unwrap();
    image.set_src("assets/harold.png");

    let width = window().inner_width().unwrap().as_f64().unwrap() as u32;
    let height = window().inner_height().unwrap().as_f64().unwrap() as u32;

    canvas.set_width(width);
    canvas.set_height(height);

    let grid = (width / model.grid_size, height / model.grid_size);

    let ctx = canvas_context_2d(&canvas);

    ctx.set_stroke_style(&JsValue::from_str("#cc00ff"));
    ctx.set_line_width(5.0);

    ctx.set_shadow_offset_x(0.0);
    ctx.set_shadow_offset_y(0.0);
    ctx.set_shadow_color("#cc00ff");
    ctx.set_shadow_blur(20.0);

    for i in 0..grid.0 {
        for j in 0..grid.1 {
            let mut p1 = (
                (i * model.grid_size) as f64,
                (j * model.grid_size) as f64,
                0.0,
            );
            let mut p2 = (
                (i * model.grid_size + model.grid_size) as f64,
                (j * model.grid_size) as f64,
                0.0,
            );
            let mut p3 = (
                (i * model.grid_size) as f64,
                (j * model.grid_size + model.grid_size) as f64,
                0.0,
            );
            let mut p4 = (
                (i * model.grid_size + model.grid_size) as f64,
                (j * model.grid_size + model.grid_size) as f64,
                0.0,
            );
            p1.2 = metaballz_condition(p1.0, p1.1, &model.metaballz);
            p2.2 = metaballz_condition(p2.0, p2.1, &model.metaballz);
            p3.2 = metaballz_condition(p3.0, p3.1, &model.metaballz);
            p4.2 = metaballz_condition(p4.0, p4.1, &model.metaballz);

            // Order thing
            // p1 p2
            // p3 p4

            let square_config = [p1.2 > 1.0, p2.2 > 1.0, p3.2 > 1.0, p4.2 > 1.0];

            match &square_config {
                // Single corners
                // #/
                // /-
                &[true, false, false, false] => draw_edge(
                    &ctx,
                    p1.0,
                    lerp(p1.1, p3.1, p1.2, p3.2),
                    lerp(p1.0, p2.0, p1.2, p2.2),
                    p1.1,
                ),
                // --
                // #-
                &[false, false, true, false] => draw_edge(
                    &ctx,
                    lerp(p3.0, p4.0, p3.2, p4.2),
                    p3.1,
                    p1.0,
                    lerp(p3.1, p1.1, p3.2, p1.2),
                ),
                // --
                // -#
                &[false, false, false, true] => draw_edge(
                    &ctx,
                    lerp(p3.0, p4.0, p3.2, p4.2),
                    p3.1,
                    p2.0,
                    lerp(p4.1, p2.1, p4.2, p2.2),
                ),
                // -#
                // --
                &[false, true, false, false] => draw_edge(
                    &ctx,
                    p2.0,
                    lerp(p2.1, p4.1, p2.2, p4.2),
                    lerp(p2.0, p1.0, p2.2, p1.2),
                    p2.1,
                ),

                // Halved
                // ##
                // --
                &[true, true, false, false] => draw_edge(
                    &ctx,
                    p1.0,
                    lerp(p1.1, p3.1, p1.2, p3.2),
                    p2.0,
                    lerp(p2.1, p4.1, p2.2, p4.2),
                ),
                // -#
                // -#
                &[false, true, false, true] => draw_edge(
                    &ctx,
                    lerp(p2.0, p1.0, p2.2, p1.2),
                    p1.1,
                    lerp(p4.0, p3.0, p4.2, p3.2),
                    p3.1,
                ),
                // --
                // ##
                &[false, false, true, true] => draw_edge(
                    &ctx,
                    p1.0,
                    lerp(p3.1, p1.1, p3.2, p1.2),
                    p2.0,
                    lerp(p4.1, p2.1, p4.2, p2.2),
                ),
                // #-
                // #-
                &[true, false, true, false] => draw_edge(
                    &ctx,
                    lerp(p1.0, p2.0, p1.2, p2.2),
                    p1.1,
                    lerp(p3.0, p4.0, p3.2, p4.2),
                    p3.1,
                ),

                // One corner outside
                // #-
                // ##
                &[true, false, true, true] => draw_edge(
                    &ctx,
                    p2.0,
                    lerp(p4.1, p2.1, p4.2, p2.2),
                    lerp(p1.0, p2.0, p1.2, p2.2),
                    p2.1,
                ),
                // ##
                // #-
                &[true, true, true, false] => draw_edge(
                    &ctx,
                    p2.0,
                    lerp(p4.1, p2.1, p4.2, p2.2),
                    lerp(p3.0, p4.0, p3.2, p4.2),
                    p4.1,
                ),
                // ##
                // -#
                &[true, true, false, true] => draw_edge(
                    &ctx,
                    p1.0,
                    lerp(p1.1, p3.1, p1.2, p3.2),
                    lerp(p4.0, p3.0, p4.2, p3.2),
                    p3.1,
                ),
                // -#
                // ##
                &[false, true, true, true] => draw_edge(
                    &ctx,
                    p1.0,
                    lerp(p3.1, p1.1, p3.2, p1.2),
                    lerp(p2.0, p1.0, p2.2, p1.2),
                    p1.1,
                ),
                // Full ones
                // ##
                // ##
                &[true, true, true, true] => {
                    if model.harold {
                        ctx.draw_image_with_html_image_element_and_dw_and_dh(
                            &image,
                            p1.0,
                            p1.1,
                            model.grid_size as f64,
                            model.grid_size as f64,
                        )
                        .unwrap();
                    }
                }
                // --
                // --
                &[false, false, false, false] => {}
                _ => {
                    console::log_1(&JsValue::from_str("WTF"));
                }
            }
        }
    }

    ctx.stroke();
}

fn draw_edge(ctx: &CanvasRenderingContext2d, x1: f64, y1: f64, x2: f64, y2: f64) {
    ctx.move_to(x1, y1);
    ctx.line_to(x2, y2);
}

fn lerp(x1: f64, x2: f64, v1: f64, v2: f64) -> f64 {
    let result = x1 + (1.0 - v1) / (v2 - v1) * (x2 - x1);
    /*console::log_1(&JsValue::from_str(&format!(
        "Final value: {}, difference from original: {}, min_value: {}, max_value: {}",
        result,
        result - x1,
        x1,
        x2
    )));*/
    result
}

fn metaballz_condition(x: f64, y: f64, metaballz: &Vec<Metaball>) -> f64 {
    let mut total: f64 = 0.0;

    for metaball in metaballz {
        let _x = x - (metaball.x + metaball.x_offset);
        let _y = y - (metaball.y + metaball.y_offset);
        total += metaball.r / (_x * _x + _y * _y).sqrt();
        //total += fast_inv_sqrt((_x * _x + _y * _y) as f32) as f64 * metaball.r;
    }

    return total;
}

// Quake 3 fast inverse square root because why not
#[allow(dead_code)]
fn fast_inv_sqrt(val: f32) -> f32 {
    let val2 = val * 0.5;
    const THREE_HALFS: f32 = 1.5;
    let mut i = val.to_bits();
    i = 0x5f3759df - (i >> 1);
    let mut y = f32::from_bits(i);
    y = y * (THREE_HALFS - (val2 * y * y));

    y
}
