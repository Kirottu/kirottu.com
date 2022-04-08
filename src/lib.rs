// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

mod metaballz;

use metaballz::{marching_squares, Metaball};
// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        metaballz: vec![
            Metaball::new(400.0, 400.0, 100.0, 0.0, -5.0),
            Metaball::new(1000.0, 400.0, 150.0, -5.0, 0.0),
            Metaball::new(1000.0, 600.0, 50.0, 5.0, 10.0),
            Metaball::new(500.0, 100.0, 50.0, 0.0, 5.0),
        ],
        grid_size: 20,
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
pub struct Model {
    metaballz: Vec<Metaball>,
    grid_size: u32,
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

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        C!["main-div"],
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
