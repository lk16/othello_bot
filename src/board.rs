use packed_simd::*;
use std::mem;

#[derive(Debug, Clone)]
pub struct Board {
    me: u64,
    opp: u64,
}

impl Board {
    pub fn new() -> Self {
        Board {
            me: (1 << 28) | (1 << 35),
            opp: (1 << 27) | (1 << 36),
        }
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
        let mask = self.opp & 0x7E7E7E7E7E7E7E7E;

        let mut flip_l = mask & (self.me << 1);
        flip_l |= mask & (flip_l << 1);
        let mut mask_l = mask & (mask << 1);
        flip_l |= mask_l & (flip_l << (2 * 1));
        flip_l |= mask_l & (flip_l << (2 * 1));
        let mut flip_r = mask & (self.me >> 1);
        flip_r |= mask & (flip_r >> 1);
        let mut mask_r = mask & (mask >> 1);
        flip_r |= mask_r & (flip_r >> (2 * 1));
        flip_r |= mask_r & (flip_r >> (2 * 1));
        let mut moves_set = (flip_l << 1) | (flip_r >> 1);

        flip_l = mask & (self.me << 7);
        flip_l |= mask & (flip_l << 7);
        mask_l = mask & (mask << 7);
        flip_l |= mask_l & (flip_l << (2 * 7));
        flip_l |= mask_l & (flip_l << (2 * 7));
        flip_r = mask & (self.me >> 7);
        flip_r |= mask & (flip_r >> 7);
        mask_r = mask & (mask >> 7);
        flip_r |= mask_r & (flip_r >> (2 * 7));
        flip_r |= mask_r & (flip_r >> (2 * 7));
        moves_set |= (flip_l << 7) | (flip_r >> 7);

        flip_l = mask & (self.me << 9);
        flip_l |= mask & (flip_l << 9);
        mask_l = mask & (mask << 9);
        flip_l |= mask_l & (flip_l << (2 * 9));
        flip_l |= mask_l & (flip_l << (2 * 9));
        flip_r = mask & (self.me >> 9);
        flip_r |= mask & (flip_r >> 9);
        mask_r = mask & (mask >> 9);
        flip_r |= mask_r & (flip_r >> (2 * 9));
        flip_r |= mask_r & (flip_r >> (2 * 9));
        moves_set |= (flip_l << 9) | (flip_r >> 9);

        flip_l = self.opp & (self.me << 8);
        flip_l |= self.opp & (flip_l << 8);
        mask_l = self.opp & (self.opp << 8);
        flip_l |= mask_l & (flip_l << (2 * 8));
        flip_l |= mask_l & (flip_l << (2 * 8));
        flip_r = self.opp & (self.me >> 8);
        flip_r |= self.opp & (flip_r >> 8);
        mask_r = self.opp & (self.opp >> 8);
        flip_r |= mask_r & (flip_r >> (2 * 8));
        flip_r |= mask_r & (flip_r >> (2 * 8));
        moves_set |= (flip_l << 8) | (flip_r >> 8);

        moves_set & !(self.me | self.opp)
    }

    fn upper_bit(mut x: u64x4) -> u64x4 {
        x = x | (x >> 1);
        x = x | (x >> 2);
        x = x | (x >> 4);
        x = x | (x >> 8);
        x = x | (x >> 16);
        x = x | (x >> 32);
        let lowers: u64x4 = x >> 1;
        x & !lowers
    }

    fn nonzero(x: u64x4) -> u64x4 {
        let zero = u64x4::new(0, 0, 0, 0);
        let mask = x.ne(zero);
        let one = u64x4::new(1, 1, 1, 1);
        one & u64x4::from_cast(mask)
    }

    fn flip_simd(&self, pos: usize) -> u64x4 {
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
        let mut outflank = Board::upper_bit(!om & mask) & p;
        let mut flipped = u64x4::from_cast(-i64x4::from_cast(outflank) << 1) & mask;
        let mask2 = u64x4::new(
            0x0101010101010100u64,
            0x00000000000000feu64,
            0x0002040810204080u64,
            0x8040201008040200u64,
        );
        mask = mask2 << pos as u32;
        outflank = mask & ((om | !mask) + 1) & p;
        flipped |= (outflank - Board::nonzero(outflank)) & mask;
        return flipped;
    }

    pub fn flip_unchecked(&self, pos: usize) -> u64 {
        let flips = self.flip_simd(pos);
        flips.or()
    }

    pub fn do_move(&self, index: usize) -> Board {
        let flip_bits = self.flip_unchecked(index);
        if flip_bits == 0 {
            panic!("Invalid move");
        }
        Board {
            me: self.opp ^ flip_bits,
            opp: (self.me ^ flip_bits) | (1u64 << index),
        }
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
