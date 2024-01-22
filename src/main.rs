use fenreader::board::{Color, Piece};

fn main() {
    fenreader::test();
    println!("Hello, world!");

    let _ctest = Color::White;
    let ptest = Piece::Rook;
    println!("My piece is a {} !", ptest);
}
