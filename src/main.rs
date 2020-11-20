#[derive(Debug)]
struct Board {
    me: u64,
    opp: u64,
}

impl Board {
    fn new() -> Self {
        Board {
            me: (1 << 27) | (1 << 36),
            opp: (1 << 28) | (1 << 35),
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

        println!("+-----------------+");
        for i in 0..64 {
            if i % 8 == 0 {
                print!("| ");
            }

            let is_black = ((black >> i) & 1) == 1;
            let is_white = ((white >> i) & 1) == 1;

            match (is_black, is_white) {
                (false, false) => print!("- "),
                (false, true) => print!("\x1b[0;31m⏺\x1b[0m "),
                (true, false) => print!("\x1b[0;34m⏺\x1b[0m "),
                (true, true) => panic!("Two discs on one square"),
            }
            if i % 8 == 7 {
                print!("|\n");
            }
        }
        println!("+-----------------+");
    }
}

fn main() {
    let board = Board::new();
    board.print(false);
}
