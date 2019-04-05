#[macro_use]
extern crate seed;
use seed::prelude::*;


// Model

struct Model {
    pub val: i32,
    pub coords: (i32, i32),
}

impl Default for Model {
    fn default() -> Self {
        Self {
            val: 0,
            coords: (0, 0),
        }
    }
}


// Update

#[derive(Clone)]
enum Msg {
    Increment,
    UpdateCoords(web_sys::MouseEvent),
}

fn update(msg: Msg, model: &mut Model) -> Update<Msg> {
    match msg {
        Msg::Increment => model.val += 1,
        Msg::UpdateCoords(ev) => model.coords = (ev.screen_x(), ev.screen_y()),
    }
    Render.into()
}


// View

fn new_chess_board(nb_line: i32, nb_col: i32) -> El<Msg> {
    let mut table_body = tbody![id!["chess-board-body"]];
    for i in 0..nb_line {
        let mut tr_content = tr![id![&format!("line-{}", i)]];
        for j in 0..nb_col {
            let td_content = td![
                style! {
                    "border" => "1px solid #505050";
                    // "margin-bottom" => "0px";
                    // "margin-top" => "0px";
                    "margin-left" => "auto";
                    "margin-right" => "auto";
                    "text-align" => "center";
                    "padding" => "0px";
                    "background-color" => if (i+j) % 2 == 0 {"black"} else {"white"}
                },
                div![
                    style! {
                        // "display" => "block"//;
                        // "margin" => "auto"
                        "color" => if (i+j) % 2 == 0 {"white"} else {"black"} 
                    },
                    span![format!("cell-{}×{}", i, j)]]
            ];
            tr_content.children.push(td_content);
        }
        table_body.children.push(tr_content);
    }

    table![id!["chess-board"],
           attrs!{
               "cellingspace" => "0px";
               //"border-spacing" => "0px"
               // "class" => "live"
           },
           style! {
               "border-spacing" => "0px";
               "border-collapse" => "collapse";
               "margin-left" => "auto";
               "margin-right" => "auto";
               "width" => "50rem";
               "height" => "50rem"
           },
           table_body
    ]
}

fn view(model: &Model) -> El<Msg> {
    // let outer_style = style! {
    //     "display" => "flex";
    //     "flex-direction" => "column";
    //     "text-align" => "center"
    // };
    // let hr_style = style! {
    //     "border" => "4px solid blue"
    // };

    // div![
    //     button![
    //         simple_ev(Ev::Click, Msg::Increment),
    //         format!("Hello, World × {}", model.val)
    //     ],
    //     h1![
    //         style! {
    //             "text-align" => "center"
    //         },
    //         "8 Queens Problem"],
    //     //outer_style,
    //     // hr![hr_style],
    //     new_chess_board(nb_line, nb_col),
    //     img! [
    //         attrs! {
    //             "data-src" => "src/image/queen.png";
    //             "src" => "src/image/queen.png";
    //             "height" => "50rem"
    //         }
    //     ]
    // ]

    let mut page = div! [];
    let mouse_div = div! [format!("cell-{}", model.coords.0)];
    page.children.push(mouse_div);

    let title = h1![
        style! {
            "text-align" => "center"
        },
        "8 Queens Puzzle"
    ];
    page.children.push(title);

    let nb_col  = 8;
    let nb_line = 8;
    let chess_board = new_chess_board(nb_line, nb_col);
    page.children.push(chess_board);

    let nb_queens = 8;
    for i in 0..nb_queens {
        let queen_elt = img! [
            attrs! {
                "id" => &format!("queen-{}", i);
                "data-src" => "src/image/queen.png";
                "src" => "src/image/queen.png";
                "height" => "50rem"
            }
        ];
        page.children.push(queen_elt);
    }

    page
}

fn window_events(model: &Model) -> Vec<seed::dom_types::Listener<Msg>> {
    let mut result = Vec::new();
    // if model.watching {
    result.push(mouse_ev("mousemove", |ev| Msg::UpdateCoords(ev)));
    // result.push(keyboard_ev("keydown", |ev| Msg::KeyPressed(ev)));
    // }
    result
}

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::default(), update, view)
        .window_events(window_events)
        .finish()
        .run();

}
