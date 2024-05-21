use cozy_chess::*;
use cozy_chess::util::display_uci_move;

use crate::search::SearchTree;

mod defs;
mod eval;
mod search;

fn main() {
    let mut board = Board::default();

    loop {
        let mut tree = SearchTree::new(&board);
        tree.search(&mut tree.root_node.clone(), 4, 1);
        println!("{:?}", tree.root_move_eval);
        let best_move = tree.root_moves[tree
            .root_move_eval
            .iter()
            .enumerate()
            .min_by_key(|&(_, value)| value)
            .map(|(index, _)| index)
            .unwrap()];
        board.play(best_move);
        println!("{}", display_uci_move(&board, best_move));
        if board.status() == GameStatus::Drawn {
            break;
        }
    }
}
