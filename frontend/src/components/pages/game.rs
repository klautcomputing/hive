use gloo::console::log;
use gloo_net::http::Request;
use hive_lib::color::Color;
use hive_lib::piece::Piece;
use crate::components::organisms::board::FBoard;
use crate::components::organisms::reserve::Reserve;
use hive_lib::{history::History, bug::Bug};
use hive_lib::state::State;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[function_component(Game)]
pub fn home() -> Html {
    let history = use_state(|| History::default());

    let get_game = {
        let history = history.clone();
        Callback::from(move |_| {
            let history = history.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::get("http://127.0.0.1:8081/history")
                    .send()
                    .await
                    .unwrap()
                    .json::<History>()
                    .await
                    .unwrap();
                history.set(resp);
            });
        })
    };

    let state = use_mut_ref(|| State::default());

    let turn = use_state(|| 0);

    let next_move = {
        let state = state.clone();
        let turn = turn.clone();
        let history = history.clone();
        Callback::from(move |_| {
            let state = state.clone();
            if let Some((piece, position)) = history.moves.get(*turn) {
                state.borrow_mut().play_turn(&piece, &position);
                turn.set(*turn + 1);
                log!("Next clicked, turn: ", *turn);
            }
        })
    };

    html! {
        <div>
            <h1>
                {"Game"}
            </h1>
            <div>
                <button onclick={next_move}>{"Next"}</button>
            </div>
            <div>
                <Reserve board={state.borrow_mut().board.clone()} color={Color::Black} zoom=1/>
            </div>
            <div>
                <Reserve board={state.borrow_mut().board.clone()} color={Color::White} zoom=1/>
            </div>
            <div>
                <FBoard board={state.borrow_mut().board.clone()} zoom=1/>
            </div>
            <div>
                <button onclick={get_game}>{"Get game"}</button>
            </div>
            <div id="history">
                <ul class="item-list">
                    { history.moves.clone().iter().map(|(piece, pos)| html!{ <li> { format!("{} {}", piece, pos) } </li> }).collect::<Html>() }
                </ul>
            </div>
            <div id="board">
                {state.borrow_mut().board.clone()}
            </div>
        </div>
    }
}