use fenreader::board::piece::Piece;
use fenreader::board::color::Color;

fn main() {
    fenreader::test();
    println!("Hello, world!");

    let _ctest = Color::White;
    let ptest = Piece::Rook;
    println!("My piece is a {} !", ptest);
}
