#[derive(Debug)]
struct Board {
    me: u64,
    opp: u64,
}

impl Board {
    fn new() -> Self {
        Board {
            me: (1 << 28) | (1 << 35),
            opp: (1 << 27) | (1 << 36),
        }
    }

    fn print(&self, white_to_move: bool) {
        let white: u64;
        let black: u64;

        if white_to_move {
            white = self.me;
            black = self.opp
        }
        else {
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

    fn moves(&self) -> u64 {
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
}

fn main() {
    let board = Board::new();
    board.print(false);
}
