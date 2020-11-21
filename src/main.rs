mod board;
mod bot;

fn main() {
    let mut board = board::Board::new();
    let bot = bot::Bot::new(9);

    let mut turn = false;
    board.print(turn);

    loop {
        if !board.has_moves() {
            board.switch_turn();
            turn = !turn;
            if !board.has_moves() {
                break;
            }
        }

        board = bot.do_move(&board);
        turn = !turn;
        board.print(turn);
    }
}
