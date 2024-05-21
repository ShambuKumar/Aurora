mod defs;
mod eval;
mod search;
use cozy_chess::*;
use crate::search::SearchTree;


fn main() {
    let board = Board::default();
    let mut tree = SearchTree::new(board);
    SearchTree::search(&mut tree.root_node, 5, 1);
}
