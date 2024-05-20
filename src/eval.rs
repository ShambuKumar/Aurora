use cozy_chess::Board;
use cozy_chess::Color::{Black, White};
use cozy_chess::Piece::{Bishop, Knight, Pawn, Queen, Rook};
use crate::defs;
fn eval(board:Board) -> i32 {
    let mut eval_score = 0;
    eval_score += board.colored_pieces(White, Pawn).len() as i32  * defs::PAWN_VAL;
    eval_score += board.colored_pieces(White, Knight).len() as i32  * defs::KNIGHT_VAL;
    eval_score += board.colored_pieces(White, Bishop).len() as i32  * defs::BISHOP_VAL;
    eval_score += board.colored_pieces(White, Rook).len() as i32  * defs::ROOK_VAL;
    eval_score += board.colored_pieces(White, Queen).len() as i32  * defs::QUEEN_VAL;
    eval_score -= board.colored_pieces(Black, Pawn).len() as i32  * defs::PAWN_VAL;
    eval_score -= board.colored_pieces(Black, Knight).len() as i32  * defs::KNIGHT_VAL;
    eval_score -= board.colored_pieces(Black, Bishop).len() as i32  * defs::BISHOP_VAL;
    eval_score -= board.colored_pieces(Black, Rook).len() as i32  * defs::ROOK_VAL;
    eval_score -= board.colored_pieces(Black, Queen).len() as i32  * defs::QUEEN_VAL;
    return eval_score
}
