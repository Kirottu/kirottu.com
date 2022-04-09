// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

mod metaballz;
mod timer;

use metaballz::{marching_squares, Metaball};
use web_sys::console;
// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        metaballz: vec![],
        grid_size: 15,
        current_metaball: CurrentMetaball::new(
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        ),
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
pub struct Model {
    metaballz: Vec<Metaball>,
    grid_size: u32,
    current_metaball: CurrentMetaball,
}

#[derive(Clone)]
struct CurrentMetaball {
    x: String,
    y: String,
    r: String,
    x_change: String,
    y_change: String,
}

impl CurrentMetaball {
    fn new(x: String, y: String, r: String, x_change: String, y_change: String) -> Self {
        Self {
            x,
            y,
            r,
            x_change,
            y_change,
        }
    }
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    Deadvance,
    Advance,
    NewMetaball,
    MetaballXChange(String),
    MetaballYChange(String),
    MetaballRChange(String),
    MetaballXChangeBy(String),
    MetaballYChangeBy(String),
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
        Msg::NewMetaball => {
            if let Ok(r) = model.current_metaball.r.parse::<f64>() {
                if r > 0.0 {
                    model.metaballz.push(model.current_metaball.clone().into());
                    marching_squares(model);
                    model.current_metaball = CurrentMetaball::new(
                        String::new(),
                        String::new(),
                        String::new(),
                        String::new(),
                        String::new(),
                    );
                    console::log_1(&JsValue::from_str("New metaball added"));
                }
            }
        }
        Msg::MetaballXChange(value) => {
            model.current_metaball.x = value;
        }
        Msg::MetaballYChange(value) => {
            model.current_metaball.y = value;
        }
        Msg::MetaballRChange(value) => {
            model.current_metaball.r = value;
        }
        Msg::MetaballXChangeBy(value) => {
            model.current_metaball.x_change = value;
        }
        Msg::MetaballYChangeBy(value) => {
            model.current_metaball.y_change = value;
        }
        _ => (),
    }
}

// ------ ------
//     View
// ------ ------

fn view_metaball_input(model: &Model) -> Node<Msg> {
    form![
        C!["metaball-form"],
        label![
            C!["metaball-form-label"],
            attrs! {
                At::For => "metaball-x",
            },
            "Metaball X"
        ],
        input![
            id!["metaball-x"],
            attrs! {
                At::Type => "number",
                At::Value => model.current_metaball.x,
            },
            input_ev(Ev::Input, Msg::MetaballXChange),
        ],
        label![
            C!["metaball-form-label"],
            attrs! {
                At::For => "metaball-y",
            },
            "Metaball Y"
        ],
        input![
            id!["metaball-y"],
            attrs! {
                At::Type => "number",
                At::Value => model.current_metaball.y,
            },
            input_ev(Ev::Input, Msg::MetaballYChange),
        ],
        label![
            C!["metaball-form-label"],
            attrs! {
                At::For => "metaball-r",
            },
            "Metaball Radius"
        ],
        input![
            id!["metaball-r"],
            attrs! {
                At::Type => "number",
                At::Value => model.current_metaball.r,
            },
            input_ev(Ev::Input, Msg::MetaballRChange),
        ],
        label![
            C!["metaball-form-label"],
            attrs! {
                At::For => "metaball-x-change",
            },
            "Metaball X Change for each iteration"
        ],
        input![
            id!["metaball-x-change"],
            attrs! {
                At::Type => "number",
                At::Value => model.current_metaball.x_change,
            },
            input_ev(Ev::Input, Msg::MetaballXChangeBy),
        ],
        label![
            C!["metaball-form-label"],
            attrs! {
                At::For => "metaball-y-change",
            },
            "Metaball Y Change for each iteration"
        ],
        input![
            id!["metaball-y-change"],
            attrs! {
                At::Type => "number",
                At::Value => model.current_metaball.y_change,
            },
            input_ev(Ev::Input, Msg::MetaballYChangeBy),
        ],
        input![
            attrs! {
                At::Type => "button",
                At::Value => "Create Metaball",
            },
            ev(Ev::Click, |_| { Msg::NewMetaball }),
        ]
    ]
}

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        C!["main-div"],
        h1!["Metaballz!", C!["screaming-den"]],
        button![ev(Ev::Click, |_| Msg::Advance), "Advance"],
        button![ev(Ev::Click, |_| Msg::Deadvance), "De-advance"],
        view_metaball_input(model),
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
