use crate::components::common::piecetype::PieceType;
use crate::components::common::svgpos::SvgPos;
use crate::components::molecules::destination::{Destination, Level};
use crate::components::molecules::flatpiece::FlatPiece;
use crate::components::molecules::lastmove::{LastMove, MoveType};
use crate::components::svgs::bugs::Bugs;
use crate::stores::gamestate::GameStateStore;
use web_sys;
use yew::prelude::*;
use yewdux::prelude::*;
use hive_lib::state::LastTurn;

#[function_component(PlayBoard)]
pub fn playboard() -> Html {
    let window = web_sys::window().unwrap();
    let height = window.inner_height().unwrap().as_f64().unwrap();
    let width = window.inner_width().unwrap().as_f64().unwrap();
    let vb = format! {"{} {} {} {}", -0.2*width, -0.2*height, width*0.4, height*0.4};

    let (store, _dispatch) = use_store::<GameStateStore>();

    let mut center_offset = (0.0, 0.0);
    if let Some(position) = store.position {
        if let Some(pieces) = store.state.board.board.get(&position) {
            center_offset = SvgPos::center_offset(pieces.len());
        }
    }

    html! {
        <>
        {"History: "} {store.state.history.to_string()}
        <svg viewBox={vb} >
            <Bugs />
            {
                if let LastTurn::Move(from, to) = store.state.last_turn {
                    html_nested! {
                        <>
                            <LastMove pos={from} m={MoveType::From}/>
                            <LastMove pos={to} m={MoveType::To}/>
                        </>
                    }
                } else {
                    html_nested! {
                    <>
                    </>
                    }
                }
            }
            // TODO: one for before one for after
            {
                for store.target_postitions.iter().map(|pos| {
                    html_nested! {
                        <Destination position={pos.clone()} level={Level::Low} />
                    }
                })
            }
            {
                for store.state.board.level_iter().map(|(pos, piece, lvl, more)| {
                    let piecetype = if more {
                        PieceType::Covered
                    } else {
                        PieceType::Board
                    };
                    html_nested! {
                        <FlatPiece piece={piece} position={pos} center_offset={SvgPos::center_offset(lvl)} piecetype={piecetype} />
                    }
                })
            }
            {
                for store.target_postitions.iter().map(|pos| {
                    html_nested! {
                        <Destination position={pos.clone()} level={Level::High} />
                    }
                })
            }
            // this shows the piece at the new destination
            if store.active.is_some() && store.position.is_some() {
                <FlatPiece piece={store.active.unwrap()} position={store.position.unwrap()} center_offset={center_offset} piecetype={PieceType::Spawn} />
            }
        </svg>
        </>
    }
}
