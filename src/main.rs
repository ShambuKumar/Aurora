mod defs;
mod eval;
mod search;
use cozy_chess::*;
use crate::search::SearchTree;


fn main() {
    /*
        // Some code to generate moves for a given position
     
    */
    let board = Board::default();
    let mut tree = SearchTree::new(board);
    tree.search(5);
    
}
