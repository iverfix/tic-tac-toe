pub mod board;
pub mod client;

fn main() {
    let mut board = board::Board::default();
    board.play(1, 1);
    board.print();

    client::start();
}
