use crate::board::Board;
use std::time::Instant;

pub struct Bot {
    search_depth: u32,
    nodes: u64,
}

impl Bot {
    pub fn new(search_depth: u32) -> Self {
        Bot {
            search_depth: search_depth,
            nodes: 0,
        }
    }

    pub fn do_move(&mut self, board: &Board) -> Board {
        let start = Instant::now();

        let children = board.children();

        if children.len() == 0 {
            panic!("No children")
        }

        let mut best_child = children[0].clone();
        let mut alpha = -64000;
        let beta = 64000;

        for (i, child) in children.iter().enumerate() {
            let heuristic = -self.pvs(child, -beta, -alpha, self.search_depth);
            if heuristic > alpha {
                alpha = heuristic;
                best_child = child.clone();
            }

            let duration = start.elapsed().as_secs_f32();
            println!(
                "Child {:2}/{:2}: {:6} | {:9} nodes in {:4.2} sec = {:9} nodes/sec",
                i + 1,
                children.len(),
                heuristic,
                self.nodes,
                duration,
                ((self.nodes as f32) / duration) as i32
            );
        }

        self.nodes = 0;

        best_child
    }

    fn pvs(&mut self, board: &Board, mut alpha: i32, beta: i32, depth: u32) -> i32 {
        self.nodes += 1;

        if depth == 0 {
            return self.heuristic(board);
        }

        let mut children = board.children();

        if children.len() == 0 {
            let mut passed = board.clone();
            passed.switch_turn();
            if passed.children().len() == 0 {
                return 1000 * board.exact_score();
            }
            return -self.pvs(&passed, -beta, -alpha, depth);
        }

        children.sort_by(|lhs, rhs| self.heuristic(lhs).cmp(&self.heuristic(rhs)));

        for (i,child) in children.iter().enumerate() {
            let mut heuristic;
            if i == 0 {
                heuristic = -self.pvs(&child, -beta, -alpha, depth - 1);
            } else {
                heuristic = -self.null_window(&child, -(alpha+1), depth-1);
                if heuristic > alpha {
                    heuristic = -self.pvs(&child, -beta, -heuristic, depth - 1);
                }
            }


            if heuristic >= beta {
                return beta;
            }
            if heuristic > alpha {
                alpha = heuristic;
            }
        }

        alpha
    }

    fn null_window(&mut self, board: &Board, alpha: i32, depth: u32) -> i32 {
        self.nodes += 1;

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
            return -self.null_window(&passed, -(alpha+1), depth);
        }

        for child in children.iter() {
            let heuristic = -self.null_window(&child, -(alpha+1), depth - 1);
            if heuristic > alpha {
                return alpha + 1;
            }
        }

        alpha
    }

    fn heuristic(&self, board: &Board) -> i32 {
        5 * board.corner_difference() + board.potential_moves_difference()
    }
}



#[cfg(test)]
mod tests {
    use crate::board::tests::generate_test_boards;
    use crate::bot::Board;
    use super::Bot;

    impl Bot {
        fn minimax(&mut self, board: &Board, depth: u32, is_max: bool) -> i32 {

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
                return self.minimax(&passed, depth, !is_max);
            }

            if is_max {
                let mut best = -64000;
                for child in children.iter() {
                    let heuristic = self.minimax(&child, depth - 1, !is_max);
                    if heuristic > best {
                        best = heuristic;
                    }
                }
                return best
            }


            let mut best = 64000;
            for child in children.iter() {
                let heuristic = self.minimax(&child, depth - 1, !is_max);
                if heuristic < best {
                    best = heuristic;
                }
            }
            return best
        }

        fn alpha_beta(&mut self, board: &Board, mut alpha: i32, beta: i32, depth: u32) -> i32 {
            self.nodes += 1;

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

    }


    #[test]
    fn test_alphabeta() {
        let boards = generate_test_boards();
        let depth = 3;
        let mut bot = Bot::new(depth);

        for board in boards.iter() {
            let minimax = bot.minimax(board, depth, true);
            let alpha_beta = bot.alpha_beta(board, -64000, 64000, depth);

            if minimax != alpha_beta {
                board.print(true);
                println!("minimax: {}", minimax);
                println!("alpha_beta: {}", alpha_beta);
            }
        }
    }

    #[test]
    fn test_pvs() {
        let boards = generate_test_boards();
        let depth = 4;
        let mut bot = Bot::new(depth);

        for board in boards.iter() {
            let alpha_beta = bot.alpha_beta(board, -64000, 64000, depth);
            let pvs = bot.pvs(board, -64000, 64000, depth);

            if alpha_beta != pvs {
                board.print(true);
                println!("alpha_beta: {}", alpha_beta);
                println!("pvs: {}", pvs);
            }
        }
    }

}
