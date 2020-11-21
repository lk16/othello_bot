mod board;

fn main() {
    let board = board::Board::new();
    board.print(false);

    let children = board.children();
    for child in children.iter() {
        child.print(true);
    }
}
