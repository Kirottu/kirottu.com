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
        current_metaball_index: None,
        harold: false,
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
    current_metaball_index: Option<usize>,
    harold: bool,
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

impl From<Metaball> for CurrentMetaball {
    fn from(metaball: Metaball) -> Self {
        Self {
            x: metaball.x.to_string(),
            y: metaball.y.to_string(),
            r: metaball.r.to_string(),
            x_change: metaball.x_change.to_string(),
            y_change: metaball.y_change.to_string(),
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

    // Harold
    ToggleHarold,

    NewMetaball,
    MetaballSelected(usize),
    MetaballUpdated,
    MetaballRemoved,
    MetaballCancel,

    // Metaball edits
    MetaballXChange(String),
    MetaballYChange(String),
    MetaballRChange(String),
    MetaballXChangeBy(String),
    MetaballYChangeBy(String),

    // Settings
    GridSizeChange(String),
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

        // Harold
        Msg::ToggleHarold => {
            model.harold = !model.harold;
            marching_squares(model);
        }

        // ---------------------------
        // General metaball operations
        // ---------------------------
        Msg::MetaballSelected(index) => {
            model.current_metaball_index = Some(index);
            model.current_metaball = model.metaballz[index].into();
        }
        Msg::MetaballUpdated => {
            if let Some(index) = model.current_metaball_index {
                model.metaballz[index] = model.current_metaball.clone().into();
            }
            model.current_metaball_index = None;
        }
        Msg::MetaballRemoved => {
            if let Some(index) = model.current_metaball_index {
                model.metaballz.remove(index);
            }
            model.current_metaball_index = None;
        }
        Msg::MetaballCancel => {
            model.current_metaball_index = None;
        }
        Msg::NewMetaball => {
            model
                .metaballz
                .push(Metaball::new(100.0, 100.0, 100.0, 0.0, 0.0));
        }

        // ---------------------------
        // Metaball edits
        // ---------------------------
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

        // Settings
        Msg::GridSizeChange(value) => {
            let grid_size = value.parse().unwrap_or(1);
            if !(grid_size < 1) {
                model.grid_size = grid_size;
            }
        }
        _ => (),
    }
}

// ------ ------
//     View
// ------ ------

fn view_metaballz(model: &Model) -> Node<Msg> {
    let mut index = 0;
    ul![
        C!["metaball-list"],
        model.metaballz.iter().map(|metaball| {
            let index_clone = index.clone();
            let item = li![
                C!["metaball-list-item"],
                button![
                    C!["metaball-list-item-button"],
                    ev(Ev::Click, move |_| Msg::MetaballSelected(index_clone)),
                    format!("Metaball {}", index)
                ]
            ];
            index += 1;
            item
        })
    ]
}

fn view_metaball_edit(model: &Model) -> Node<Msg> {
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
        div![
            input![
                id!["metaball-cancel"],
                attrs! {
                    At::Type => "button",
                    At::Value => "Cancel",
                },
                ev(Ev::Click, |_| Msg::MetaballCancel),
            ],
            input![
                id!["metaball-update"],
                attrs! {
                    At::Type => "button",
                    At::Value => "Update",
                },
                ev(Ev::Click, |_| Msg::MetaballUpdated),
            ],
            input![
                id!["metaball-remove"],
                attrs! {
                    At::Type => "button",
                    At::Value => "Remove",
                },
                ev(Ev::Click, |_| Msg::MetaballRemoved),
            ]
        ]
    ]
}

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        keyboard_ev(Ev::KeyDown, |e| {
            IF![e.key() == "ArrowRight" => Msg::Advance]
        }),
        keyboard_ev(Ev::KeyDown, |e| {
            IF![e.key() == "ArrowLeft" => Msg::Deadvance]
        }),
        canvas![C!["metaballz"], id!["metaballz"]],
        div![
            C!["main-div"],
            h1!["Metaballz!", C!["screaming-den"]],
            div![
                C!["main-controls"],
                section![
                    C!["main-controls-buttons"],
                    button![ev(Ev::Click, |_| Msg::Advance), "Advance"],
                    button![ev(Ev::Click, |_| Msg::Deadvance), "De-advance"],
                    button![ev(Ev::Click, |_| Msg::NewMetaball), "Create metaball"],
                    button![ev(Ev::Click, |_| Msg::ToggleHarold), "Harold"],
                ],
                label![
                    C!["main-controls-label"],
                    attrs! {
                        At::For => "grid-size",
                    },
                    "Grid Size"
                ],
                input![
                    id!["grid-size"],
                    attrs! {
                        At::Type => "number",
                        At::Value => model.grid_size,
                    },
                    input_ev(Ev::Input, Msg::GridSizeChange),
                ]
            ],
            IF![
                model.current_metaball_index.is_some() =>
                view_metaball_edit(model)
            ],
            view_metaballz(model),
        ]
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
