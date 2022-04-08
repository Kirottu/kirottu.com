// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::{*, web_sys::console}, *};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model { 
        metaballz: vec![
            Metaball::new(400.0, 400.0, 100.0, 0.0, -10.0), 
            Metaball::new(1000.0, 400.0, 150.0, -10.0, 0.0), 
            Metaball::new(1000.0, 600.0, 50.0, 10.0, 10.0),
            Metaball::new(500.0, 100.0, 50.0, 0.0, 10.0),
        ], 
        grid_size: 5 
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    metaballz: Vec<Metaball>,
    grid_size: u32,
}

struct Metaball {
    x: f64,
    y: f64,
    r: f64,
    x_change: f64,
    y_change: f64,
}

impl Metaball {
    fn new(x: f64, y: f64, r: f64, x_change: f64, y_change: f64) -> Self {
        Self { x, y, r, x_change, y_change }
    }
}


// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    Deadvance,
    Advance,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Advance => {
            for metaball in &mut model.metaballz {
                metaball.x += metaball.x_change;
                metaball.y += metaball.y_change;
            }
            marching_squares(model);
        }
        Msg::Deadvance => {
            for metaball in &mut model.metaballz {
                metaball.x -= metaball.x_change;
                metaball.y -= metaball.y_change;
            }
            marching_squares(model);

        }
    }
}

fn marching_squares(model: &mut Model) {
    let canvas = canvas("metaballz").unwrap();
    
    let width = window().inner_width().unwrap().as_f64().unwrap() as u32;
    let height = window().inner_height().unwrap().as_f64().unwrap() as u32;
    
    canvas.set_width(width);
    canvas.set_height(height);

    let grid = (width / model.grid_size, height / model.grid_size);

    let ctx = canvas_context_2d(&canvas);
    
    ctx.set_stroke_style(&JsValue::from_str("#FFFFFF"));

    
    for i in 0..grid.0 {
        for j in 0..grid.1 {
            let mut p1 = (i * model.grid_size, j * model.grid_size, 0.0);
            let mut p2 = (i * model.grid_size + model.grid_size, j * model.grid_size + model.grid_size, 0.0);
            let mut p3 = (i * model.grid_size + model.grid_size, j * model.grid_size, 0.0);
            let mut p4 = (i * model.grid_size, j * model.grid_size + model.grid_size, 0.0);

            p1.2 = metaballz_condition(p1.0, p1.1, &model.metaballz);
            p2.2 = metaballz_condition(p2.0, p2.1, &model.metaballz);
            p3.2 = metaballz_condition(p3.0, p3.1, &model.metaballz);
            p4.2 = metaballz_condition(p4.0, p4.1, &model.metaballz);
            

            // Check if the points are all the same. If they are they are effectively useless
            if !((p1.2 < 1.0 && p2.2 < 1.0 && p3.2 < 1.0 && p4.2 < 1.0) || (p1.2 > 1.0 && p2.2 > 1.0 && p3.2 > 1.0 && p4.2 > 1.0)) {
                console::log_1(&JsValue::from_str("Le point"));
                // Check for if a single corner is inside the metaball
                if p1.2 > 1.0 && p2.2 < 1.0 && p3.2 < 1.0 && p4.2 < 1.0 {
                    ctx.move_to((p1.0 + model.grid_size / 2) as f64, p1.1 as f64);
                    ctx.line_to(p1.0 as f64, (p1.1 + model.grid_size / 2) as f64);
                } else if p1.2 < 1.0 && p2.2 > 1.0 && p3.2 < 1.0 && p4.2 < 1.0 {
                    ctx.move_to((p1.0 + model.grid_size) as f64, (p1.1 + model.grid_size / 2) as f64);
                    ctx.line_to((p1.0 + model.grid_size / 2) as f64, (p1.1 + model.grid_size) as f64);
                } else if p1.2 < 1.0 && p2.2 < 1.0 && p3.2 > 1.0 && p4.2 < 1.0 {
                    ctx.move_to((p1.0 + model.grid_size / 2) as f64, p1.1 as f64);
                    ctx.line_to((p1.0 + model.grid_size) as f64, (p1.1 + model.grid_size / 2) as f64);
                } else if p1.2 < 1.0 && p2.2 < 1.0 && p3.2 < 1.0 && p4.2 > 1.0 {
                    ctx.move_to(p1.0 as f64, (p1.1 + model.grid_size / 2) as f64);
                    ctx.line_to((p1.0 + model.grid_size / 2) as f64, (p1.1 + model.grid_size) as f64);
                }
                // Check for 2 points inside the metaball
                else if p1.2 > 1.0 && p2.2 < 1.0 && p3.2 > 1.0 && p4.2 < 1.0 {
                    ctx.move_to(p1.0 as f64, (p1.1 + model.grid_size / 2) as f64);
                    ctx.line_to((p1.0 + model.grid_size) as f64, (p1.1 + model.grid_size / 2) as f64);
                } else if p1.2 < 1.0 && p2.2 > 1.0 && p3.2 < 1.0 && p4.2 > 1.0 {
                    ctx.move_to(p1.0 as f64, (p1.1 + model.grid_size / 2) as f64);
                    ctx.line_to((p1.0 + model.grid_size) as f64, (p1.1 + model.grid_size / 2) as f64);
                } else if p1.2 > 1.0 && p2.2 < 1.0 && p3.2 < 1.0 && p4.2 > 1.0 {
                    ctx.move_to((p1.0 + model.grid_size / 2) as f64, p1.1 as f64);
                    ctx.line_to((p1.0 + model.grid_size / 2) as f64, (p1.1 + model.grid_size) as f64);
                } else if p1.2 < 1.0 && p2.2 > 1.0 && p3.2 > 1.0 && p4.2 < 1.0 {
                    ctx.move_to((p1.0 + model.grid_size / 2) as f64, p1.1 as f64);
                    ctx.line_to((p1.0 + model.grid_size / 2) as f64, (p1.1 + model.grid_size) as f64);
                }
                // Check for if 3 points are inside the metaball
                else if p1.2 < 1.0 && p2.2 > 1.0 && p3.2 > 1.0 && p4.2 > 1.0 {
                    ctx.move_to((p1.0 + model.grid_size / 2) as f64, p1.1 as f64);
                    ctx.line_to(p1.0 as f64, (p1.1 + model.grid_size / 2) as f64);
                } else if p1.2 > 1.0 && p2.2 < 1.0 && p3.2 > 1.0 && p4.2 > 1.0 {
                    ctx.move_to((p1.0 + model.grid_size) as f64, (p1.1 + model.grid_size / 2) as f64);
                    ctx.line_to((p1.0 + model.grid_size / 2) as f64, (p1.1 + model.grid_size) as f64);
                } else if p1.2 > 1.0 && p2.2 > 1.0 && p3.2 < 1.0 && p4.2 > 1.0 {
                    ctx.move_to((p1.0 + model.grid_size / 2) as f64, p1.1 as f64);
                    ctx.line_to((p1.0 + model.grid_size) as f64, (p1.1 + model.grid_size / 2) as f64);
                } else if p1.2 > 1.0 && p2.2 > 1.0 && p3.2 > 1.0 && p4.2 < 1.0 {
                    ctx.move_to(p1.0 as f64, (p1.1 + model.grid_size / 2) as f64);
                    ctx.line_to((p1.0 + model.grid_size / 2) as f64, (p1.1 + model.grid_size) as f64);
                }
            }
        }
    }
    ctx.stroke();

}

fn metaballz_condition(x: u32, y: u32, metaballz: &Vec<Metaball>) -> f64 {
    let mut total: f64 = 0.0;

    for metaball in metaballz {
        total += metaball.r/((x as f64 - metaball.x) * (x as f64 - metaball.x) + (y as f64 - metaball.y) * (y as f64 - metaball.y)).sqrt();
    }

    return total;
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        h1!["Metaballz!", C!["screaming-den"]],
        button![ev(Ev::Click, |_| Msg::Advance), "Advance"],
        button![ev(Ev::Click, |_| Msg::Deadvance), "De-advance"],
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
