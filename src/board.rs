use crate::bits::{nonzero, upper_bit};
use packed_simd::*;
use rand::Rng;
use std::mem;

#[derive(Debug, Clone, Eq, Hash)]
pub struct Board {
    me: u64,
    opp: u64,
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.me == other.me && self.opp == other.opp
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            me: (1 << 28) | (1 << 35),
            opp: (1 << 27) | (1 << 36),
        }
    }

    pub fn new_random(discs: u32) -> Board {
        if discs < 4 || discs > 64 {
            panic!("Invalid amount of discs");
        }

        let mut board = Board::new();
        let mut skips = 0;

        while board.count_discs() != discs {
            if skips == 2 {
                // Stuck. Try again.
                board = Board::new();
                skips = 0;
                continue;
            }

            if board.moves() == 0 {
                skips += 1;
                board.switch_turn();
                continue;
            }

            skips = 0;
            board = board.do_random_move();
        }

        board
    }

    pub fn print(&self, white_to_move: bool) {
        let white: u64;
        let black: u64;

        if white_to_move {
            white = self.me;
            black = self.opp
        } else {
            black = self.me;
            white = self.opp;
        }
        let moves = self.moves();

        println!("+-----------------+");
        for i in 0..64 {
            if i % 8 == 0 {
                print!("| ");
            }

            let is_black = ((black >> i) & 1) == 1;
            let is_white = ((white >> i) & 1) == 1;
            let is_move = ((moves >> i) & 1) == 1;

            match (is_black, is_white, is_move) {
                (false, false, false) => print!("  "),
                (false, false, true) => print!("- "),
                (false, true, false) => print!("\x1b[0;34m⏺\x1b[0m "),
                (true, false, false) => print!("\x1b[0;31m⏺\x1b[0m "),
                (true, true, _) => panic!("Two discs on one square"),
                (_, _, _) => panic!("Filled square is valid move"),
            }
            if i % 8 == 7 {
                print!("|\n");
            }
        }
        println!("+-----------------+");
    }

    pub fn moves(&self) -> u64 {
        let shift1 = u64x4::new(1, 7, 9, 8);
        let mask = u64x4::new(
            0x7e7e7e7e7e7e7e7eu64,
            0x7e7e7e7e7e7e7e7eu64,
            0x7e7e7e7e7e7e7e7eu64,
            0xffffffffffffffffu64,
        );
        let v_player = u64x4::splat(self.me);
        let masked_op = u64x4::splat(self.opp) & mask;
        let mut flip_l = masked_op & (v_player << shift1);
        let mut flip_r = masked_op & (v_player >> shift1);
        flip_l |= masked_op & (flip_l << shift1);
        flip_r |= masked_op & (flip_r >> shift1);
        let pre_l = masked_op & (masked_op << shift1);
        let pre_r = pre_l >> shift1;
        let shift2 = shift1 + shift1;
        flip_l |= pre_l & (flip_l << shift2);
        flip_r |= pre_r & (flip_r >> shift2);
        flip_l |= pre_l & (flip_l << shift2);
        flip_r |= pre_r & (flip_r >> shift2);
        let mut res = flip_l << shift1;
        res |= flip_r >> shift1;
        res &= u64x4::splat(!(self.me | self.opp));
        return res.or();
    }

    fn flip(&self, pos: usize) -> u64 {
        let p = u64x4::new(self.me, self.me, self.me, self.me);
        let o = u64x4::new(self.opp, self.opp, self.opp, self.opp);
        let omask = u64x4::new(
            0xFFFFFFFFFFFFFFFFu64,
            0x7E7E7E7E7E7E7E7Eu64,
            0x7E7E7E7E7E7E7E7Eu64,
            0x7E7E7E7E7E7E7E7Eu64,
        );
        let om = o & omask;
        let mask1 = u64x4::new(
            0x0080808080808080u64,
            0x7f00000000000000u64,
            0x0102040810204000u64,
            0x0040201008040201u64,
        );
        let mut mask = mask1 >> (63 - pos) as u32;
        let mut outflank = upper_bit(!om & mask) & p;
        let mut flipped = u64x4::from_cast(-i64x4::from_cast(outflank) << 1) & mask;
        let mask2 = u64x4::new(
            0x0101010101010100u64,
            0x00000000000000feu64,
            0x0002040810204080u64,
            0x8040201008040200u64,
        );
        mask = mask2 << pos as u32;
        outflank = mask & ((om | !mask) + 1) & p;
        flipped |= (outflank - nonzero(outflank)) & mask;

        flipped.or()
    }

    pub fn do_move(&self, index: usize) -> Board {
        let flipped = self.flip(index);
        if flipped == 0 {
            panic!("Invalid move");
        }
        Board {
            me: self.opp ^ flipped,
            opp: (self.me ^ flipped) | (1u64 << index),
        }
    }

    pub fn do_random_move(&self) -> Board {
        let moves = self.moves();

        if moves == 0 {
            panic!("No moves");
        }

        let mut child = self.clone();

        loop {
            let index = rand::thread_rng().gen_range(0, 64);

            if (moves >> index) & 1 == 1 {
                child = child.do_move(index as usize);
                break;
            }
        }

        child
    }

    pub fn switch_turn(&mut self) {
        mem::swap(&mut self.opp, &mut self.me);
    }

    pub fn children(&self) -> Vec<Board> {
        let mut moves = self.moves();
        let mut children: Vec<Board> = Vec::new();

        while moves != 0 {
            let index = moves.trailing_zeros() as usize;
            children.push(self.do_move(index));
            moves &= !(1 << index)
        }
        children
    }

    pub fn exact_score(&self) -> i32 {
        let me_count = self.me.count_ones() as i32;
        let opp_count = self.opp.count_ones() as i32;

        if me_count > opp_count {
            return 64 - (2 * opp_count);
        }
        if me_count < opp_count {
            return -64 + (2 * me_count);
        }
        return 0;
    }

    pub fn has_moves(&self) -> bool {
        self.moves() != 0
    }

    pub fn count_discs(&self) -> u32 {
        (self.me | self.opp).count_ones()
    }

    pub fn corner_difference(&self) -> i32 {
        let corner_mask = 1 << 0 | 1 << 7 | 1 << 56 | 1 << 63;
        let me_corners = (self.me & corner_mask).count_ones() as i32;
        let opp_corners = (self.opp & corner_mask).count_ones() as i32;

        me_corners - opp_corners
    }

    pub fn potential_moves_difference(&self) -> i32 {
        let left_mask = 0x7F7F7F7F7F7F7F7F;
        let right_mask = 0xFEFEFEFEFEFEFEFE;

        let mut me_potential_moves = (self.opp & left_mask) << 1;
        me_potential_moves |= (self.opp & right_mask) >> 1;
        me_potential_moves |= (self.opp & left_mask) << 9;
        me_potential_moves |= (self.opp & right_mask) >> 9;
        me_potential_moves |= (self.opp & right_mask) << 7;
        me_potential_moves |= (self.opp & left_mask) >> 7;
        me_potential_moves |= self.opp << 8;
        me_potential_moves |= self.opp >> 8;

        me_potential_moves &= !(self.me | self.opp);

        let mut opp_potential_moves = (self.me & left_mask) << 1;
        opp_potential_moves |= (self.me & right_mask) >> 1;
        opp_potential_moves |= (self.me & left_mask) << 9;
        opp_potential_moves |= (self.me & right_mask) >> 9;
        opp_potential_moves |= (self.me & right_mask) << 7;
        opp_potential_moves |= (self.me & left_mask) >> 7;
        opp_potential_moves |= self.me << 8;
        opp_potential_moves |= self.me >> 8;

        opp_potential_moves &= !(self.me | self.opp);

        let me_potential_move_count = me_potential_moves.count_ones() as i32;
        let opp_potential_moves_count = opp_potential_moves.count_ones() as i32;

        me_potential_move_count - opp_potential_moves_count
    }
}

#[cfg(test)]
mod tests {

    use super::Board;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    fn generate_test_boards() -> Vec<Board> {
        let mut boards = Vec::new();

        for y in 0..8 {
            for x in 0..8 {
                let mut board = Board { me: 0, opp: 0 };
                board.me |= 1 << (y * 8 + x);

                // for each direction
                for dy in -1..2 {
                    for dx in -1..2 {
                        if (dy == 0) && (dx == 0) {
                            continue;
                        }
                        board.opp = 0;

                        // for each distance
                        for d in 1..7 {
                            // check if me can still flip within othello boundaries
                            let py = y + (d + 1) * dy;
                            let px = x + (d + 1) * dx;

                            if (py < 0) || (py > 7) || (px < 0) || (px > 7) {
                                break;
                            }

                            let qy = y + (d * dy);
                            let qx = x + (d * dx);

                            board.opp |= 1 << (qy * 8 + qx);

                            boards.push(board.clone());
                        }
                    }
                }
            }
        }

        boards.push(Board::new());

        // random reachable boards with 4-64 discs
        for _ in 0..10 {
            for discs in 4..65 {
                boards.push(Board::new_random(discs));
            }
        }

        boards
    }

    impl Board {
        fn naive_count_discs(&self) -> u32 {
            let mut count: u32 = 0;
            for i in 0..64 {
                if (self.me | self.opp) >> i & 1 == 1 {
                    count += 1
                }
            }
            count
        }

        fn naive_switch_turn(&mut self) {
            let tmp = self.me;
            self.me = self.opp;
            self.opp = tmp;
        }

        fn naive_flip(&self, index: usize) -> u64 {
            if (self.me | self.opp) >> index & 1 == 1 {
                return 0;
            }
            let mut flipped: u64 = 0;
            for dx in -1..2 {
                for dy in -1..2 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let mut s = 1;
                    loop {
                        let curx = (index % 8) as i32 + (dx * s);
                        let cury = (index / 8) as i32 + (dy * s);
                        if curx < 0 || curx >= 8 || cury < 0 || cury >= 8 {
                            break;
                        }

                        let cur = 8 * cury + curx;

                        if (self.opp >> cur) & 1 == 1 {
                            s += 1;
                        } else {
                            if ((self.me >> cur) & 1 == 1) && s >= 2 {
                                for p in 1..s {
                                    let f = index as i32 + (p * (8 * dy + dx));
                                    flipped |= 1 << f;
                                }
                            }
                            break;
                        }
                    }
                }
            }
            flipped
        }

        fn naive_moves(&self) -> u64 {
            let mut moves: u64 = 0;

            for index in 0..64 {
                if self.naive_flip(index) != 0 {
                    moves |= 1 << index;
                }
            }
            moves
        }

        fn naive_do_move(&self, index: usize) -> Board {
            let flipped = self.naive_flip(index);
            Board {
                me: self.opp ^ flipped,
                opp: self.me | flipped | 1 << index,
            }
        }

        fn naive_children(&self) -> Vec<Board> {
            let mut children = Vec::new();
            let moves = self.moves();
            for index in 0..64 {
                if (moves >> index) & 1 == 1 {
                    children.push(self.do_move(index));
                }
            }
            children
        }

        fn naive_exact_score(&self) -> i32 {
            let me_count = self.me.count_ones() as i32;
            let opp_count = self.opp.count_ones() as i32;

            if me_count > opp_count {
                return 64 - 2 * opp_count;
            } else if opp_count > me_count {
                return -64 + 2 * me_count;
            }
            0
        }

        fn naive_corner_difference(&self) -> i32 {
            let indexes = vec![0, 7, 56, 63];

            let mut diff: i32 = 0;
            for index in indexes.iter() {
                if (self.me >> index) & 1 == 1 {
                    diff += 1;
                }
                if (self.opp >> index) & 1 == 1 {
                    diff -= 1;
                }
            }
            diff
        }

        fn naive_potential_moves(&self) -> u64 {
            let mut surround_opp: u64 = 0;

            for index in 0..64 {
                if (self.opp >> index) & 1 == 1 {
                    let y = index / 8;
                    let x = index % 8;

                    if y > 0 {
                        if x > 0 {
                            surround_opp |= 1 << (index - 9);
                        }

                        surround_opp |= 1 << (index - 8);

                        if x < 7 {
                            surround_opp |= 1 << (index - 7);
                        }
                    }

                    if x > 0 {
                        surround_opp |= 1 << (index - 1);
                    }

                    if x < 7 {
                        surround_opp |= 1 << (index + 1);
                    }

                    if y < 7 {
                        if x > 0 {
                            surround_opp |= 1 << (index + 7);
                        }

                        surround_opp |= 1 << (index + 8);

                        if x < 7 {
                            surround_opp |= 1 << (index + 9);
                        }
                    }
                }
            }
            surround_opp & !(self.me | self.opp)
        }

        fn naive_potential_moves_difference(&self) -> i32 {
            let me_potential_moves_count = self.naive_potential_moves().count_ones() as i32;
            let mut clone = self.clone();
            clone.switch_turn();
            let opp_potential_moves_count = clone.naive_potential_moves().count_ones() as i32;

            me_potential_moves_count - opp_potential_moves_count
        }
    }

    #[test]
    fn test_board_do_random_move() {
        let boards = generate_test_boards();

        for board in boards.iter() {
            let children = HashSet::<Board>::from_iter(board.children());

            if children.len() == 0 {
                continue;
            }

            for _ in 0..20 {
                let child = board.do_random_move();
                assert!(children.contains(&child));
            }
        }
    }

    #[test]
    fn test_board_count_discs() {
        let boards = generate_test_boards();
        for board in boards.iter() {
            assert!(board.naive_count_discs() == board.count_discs());
        }
    }

    #[test]
    fn test_board_switch_turn() {
        let boards = generate_test_boards();
        for board in boards.iter() {
            let mut fast: Board = board.clone();
            fast.switch_turn();

            let mut naive: Board = board.clone();
            naive.naive_switch_turn();
            assert_eq!(naive, fast);
        }
    }

    #[test]
    fn test_board_flip() {
        let boards = generate_test_boards();
        for board in boards.iter() {
            let moves = board.moves();

            for index in 0..63 {
                if (moves >> index) & 1 == 1 {
                    assert_eq!(board.naive_flip(index), board.flip(index));
                }
            }
        }
    }

    #[test]
    fn test_board_moves() {
        let boards = generate_test_boards();

        for board in boards.iter() {
            assert_eq!(board.naive_moves(), board.moves());
        }
    }

    #[test]
    fn test_has_moves() {
        let boards = generate_test_boards();
        for board in boards.iter() {
            let has_moves = board.moves() != 0;
            assert_eq!(has_moves, board.has_moves());
        }
    }

    #[test]
    fn test_board_do_move() {
        let boards = generate_test_boards();
        for board in boards.iter() {
            let moves = board.naive_moves();
            for index in 0..64 {
                if (moves >> index) & 1 == 1 {
                    assert_eq!(board.naive_do_move(index), board.do_move(index));
                }
            }
        }
    }

    #[test]
    fn test_board_children() {
        let boards = generate_test_boards();
        for board in boards.iter() {
            let children_slow = HashSet::<Board>::from_iter(board.naive_children());
            let children_fast = HashSet::<Board>::from_iter(board.children());
            assert_eq!(children_slow, children_fast);
        }
    }

    #[test]
    fn test_board_exact_score() {
        let boards = generate_test_boards();
        for board in boards.iter() {
            assert_eq!(board.naive_exact_score(), board.exact_score());
        }
    }

    #[test]
    fn test_board_corner_difference() {
        let boards = generate_test_boards();
        for board in boards.iter() {
            assert_eq!(board.naive_corner_difference(), board.corner_difference());
        }
    }

    #[test]
    fn test_board_potential_moves_difference() {
        let boards = generate_test_boards();
        for board in boards.iter() {
            assert_eq!(
                board.naive_potential_moves_difference(),
                board.potential_moves_difference()
            );
        }
    }
}
