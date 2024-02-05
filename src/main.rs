use fenreader::board::piece::Piece;
use fenreader::board::color::Color;
use fenreader::board::row::Row;

fn main() {
    println!("Hello, world!");

    let _ctest = Color::White;
    let ptest = Piece::Rook;
    println!("My piece is a {} !", ptest);

    let row = Row::new(5).unwrap();
    for s in row {
        println!("{s}");
    }
}
