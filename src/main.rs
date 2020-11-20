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

    fn print(&self) {
        println!("+-----------------+");
        for i in 0..64 {
            if i % 8 == 0 {
                print!("| ");
            }
            match ((self.me >> i) & 1, (self.opp >> i) & 1) {
                (0, 0) => print!("- "),
                (0, 1) => print!("x "),
                (1, 0) => print!("o "),
                (1, 1) => panic!("Two discs on one square"),
                (_, _) => panic!("Masking went wrong"),
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
    board.print()
}
