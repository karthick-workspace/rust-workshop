trait Singleton {
    fn new() -> Self
    where
        Self: Sized;
    fn get_instance() -> Self
    where
        Self: Sized;
}

struct Chessboard {}

impl Chessboard {
    const INSTANCE: Chessboard = Chessboard {};
    fn start(self) {
        println!("chess game started");
    }
}

impl Singleton for Chessboard {
    fn new() -> Self {
        Chessboard::INSTANCE
    }
    fn get_instance() -> Self {
        Chessboard::INSTANCE
    }
}

fn main() {
    let board = Chessboard::new();
    board.start();
}
