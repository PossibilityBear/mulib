mod parse_svg;
use parse_svg::*;

pub fn main () {
    println!("Hello from leptos_svg");
    let svg = read_svg(&FilePath("../public/play.svg".to_string()));
    println!("{}", svg);
}