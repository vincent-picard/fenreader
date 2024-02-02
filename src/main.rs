use fenreader::board::piece::Piece;
use fenreader::board::color::Color;

fn main() {
    println!("Hello, world!");

    let _ctest = Color::White;
    let ptest = Piece::Rook;
    println!("My piece is a {} !", ptest);
}
