use crate::board::Board;

pub struct Bot {
    search_depth: u32,
}

impl Bot {
    pub fn new(search_depth: u32) -> Self {
        Bot {
            search_depth: search_depth,
        }
    }

    pub fn do_move(&self, board: &Board) -> Board {
        let children = board.children();

        if children.len() == 0 {
            panic!("No children")
        }

        let mut best_child = children[0].clone();
        let mut alpha = -64000;
        let beta = 64000;

        for (i, child) in children.iter().enumerate() {
            let heuristic = -self.alpha_beta(child, -beta, -alpha, self.search_depth);
            println!("Child {} / {}: {}", i + 1, children.len(), heuristic);
            if heuristic > alpha {
                alpha = heuristic;
                best_child = child.clone();
            }
        }

        best_child
    }

    fn alpha_beta(&self, board: &Board, mut alpha: i32, beta: i32, depth: u32) -> i32 {
        if depth == 0 {
            return self.heuristic(board);
        }

        let children = board.children();

        if children.len() == 0 {
            let mut passed = board.clone();
            passed.switch_turn();
            if passed.children().len() == 0 {
                return 1000 * board.exact_score();
            }
            return -self.alpha_beta(&passed, -beta, -alpha, depth);
        }

        for child in children.iter() {
            let heuristic = -self.alpha_beta(&child, -beta, -alpha, depth - 1);
            if heuristic >= beta {
                return beta;
            }
            if heuristic > alpha {
                alpha = heuristic;
            }
        }

        alpha
    }

    fn heuristic(&self, board: &Board) -> i32 {
        5 * board.corner_difference() + board.potential_moves_difference()
    }
}
