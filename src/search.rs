use cozy_chess::{Board, Move, PieceMoves};
pub(crate) struct SearchTree{
 root_node: SearchNode
    }
pub(crate) struct SearchNode{
    board: Board,
    children: Vec<SearchNode>,
    moves: Vec<Move>
}
impl SearchTree {
    pub fn new(board: Board) -> SearchTree {
        SearchTree{root_node: SearchNode{board, children: Vec::new(), moves:Vec::new()}}
    }
    fn create_children(parent_node:&mut SearchNode, depth: i32, current_depth: i32){
        if current_depth > depth {
            return;
        }
        parent_node.board.generate_moves(|moves| {
            parent_node.moves.extend(moves);
            false
        });
        for i in parent_node.moves.iter_mut(){
            let mut new_board: Board =  parent_node.board.clone();
            new_board.play(*i);
            parent_node.children.push(SearchNode{board: new_board , children: Vec::new(), moves:Vec::new()}
            );
            Self::create_children(parent_node.children.last_mut().unwrap(), depth, current_depth+1);
         
        }
    }
    pub(crate) fn search(&mut self, depth:i32){
     Self::create_children(&mut self.root_node, depth, 1);
    }
}

