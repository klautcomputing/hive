use crate::components::molecules::{flatpiece::Pos, stackedpieces::StackedPieces};
use hive_lib::board::Board;
use web_sys;
use yew::prelude::*;
use yewdux::prelude::*;
use crate::stores::gamestate::GameStateStore;

#[function_component(PlayBoard)]
pub fn playboard() -> Html {
    let window = web_sys::window().unwrap();
    let height = window.inner_height().unwrap().as_f64().unwrap();
    let width = window.inner_width().unwrap().as_f64().unwrap();
    let vb = format!{"{} {} {} {}", -0.2*width, -0.2*height, width*0.4, height*0.4};

    let (store, dispatch) = use_store::<GameStateStore>();

    html!{
        <svg viewBox={vb}>
            {
                for store.state.board.board.iter().map(|(pos, pieces)| {
                    let pos = Pos::new(pos.0, pos.1);
                    html_nested! {
                        <StackedPieces pieces={pieces.clone()} pos={pos.clone()} zoom={2} size={30}/>
                    }
                })
            }
        </svg>
    }
}