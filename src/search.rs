use cozy_chess::{Board, Move, Square};

use crate::eval;

pub(crate) struct SearchTree {
    pub root_node: SearchNode,
    pub root_moves: Vec<Move>,
    pub root_move_eval: Vec<i32>,
}
#[derive(Clone)]
pub(crate) struct SearchNode {
    board: Board,
    children: Vec<SearchNode>,
    moves: Vec<Move>,
    pub eval: i32,
    pub root_move: Move,
}
impl SearchTree {
    pub fn new(board: &Board) -> SearchTree {
        let mut tree = SearchTree {
            root_node: SearchNode {
                board: board.clone(),
                children: Vec::new(),
                moves: Vec::new(),
                eval: eval::eval(&board),
                root_move: Move {
                    from: Square::A1,
                    to: Square::A1,
                    promotion: None,
                },
            },
            root_moves: Vec::new(),
            root_move_eval: Vec::new(),
        };
        tree.root_node.board.generate_moves(|moves| {
            tree.root_moves.extend(moves);
            false
        });
        tree.root_move_eval = vec![0; tree.root_moves.len()];
        tree
    }
    pub fn search(&mut self, parent_node: &mut SearchNode, depth: i32, current_depth: i32) {
        if current_depth > depth {
            if parent_node.eval
                < self.root_move_eval[self
                    .root_moves
                    .iter()
                    .position(|&r| r == parent_node.root_move)
                    .unwrap()]
            {
                self.root_move_eval[self
                    .root_moves
                    .iter()
                    .position(|&r| r == parent_node.root_move)
                    .unwrap()] = parent_node.eval;
            }
            return;
        }

        parent_node.board.generate_moves(|moves| {
            parent_node.moves.extend(moves);
            false
        });

        for i in parent_node.moves.iter_mut() {
            let mut new_board: Board = parent_node.board.clone();
            new_board.play(*i);
            let root_move: Move;
            // Assign each node a move which it originally originated from. If the depth is 1, aka. the Position directly after the move, the move is assigned, otherwise the child inherits the parents root move
            if current_depth == 1 {
                root_move = i.clone();
            } else {
                root_move = parent_node.root_move;
            }
            parent_node.children.push(SearchNode {
                board: new_board.clone(),
                children: Vec::new(),
                moves: Vec::new(),
                eval: eval::eval(&new_board),
                root_move,
            });
            Self::search(
                self,
                parent_node.children.last_mut().unwrap(),
                depth,
                current_depth + 1,
            );
        }
    }
}
