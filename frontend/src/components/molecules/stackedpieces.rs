use crate::components::common::piecetype::PieceType;
use crate::components::molecules::flatpiece::FlatPiece;
use crate::components::common::svgpos::SvgPos;
use hive_lib::{piece::Piece, position::Position};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct StackedPiecesProps {
    pub pieces: Vec<Piece>,
    pub position: Position,
    pub piecetype: PieceType,
}

#[function_component(StackedPieces)]
pub fn stackedpieces(props: &StackedPiecesProps) -> Html {
    let len = props.pieces.len() - 1;
    html! {
        for props.pieces.iter().enumerate().map(|(i, piece)|{
            let piecetype = if i == len {
                // last piece
                props.piecetype.clone()
            } else {
                PieceType::Covered
            };
            let center_offset = SvgPos::center_offset(i);

            html_nested! {
                <FlatPiece piece={piece.clone()} position={props.position.clone()} center_offset={center_offset} piecetype={piecetype} />
            }
        })
    }
}
