use rust_1_brc::t008_bittwiddeling_parsing_extreme;

mod t006_memmap;
fn main() {
    println!("Hello, world!");
    let file = "../1brc/measurements_10_000.txt";
    let res = t008_bittwiddeling_parsing_extreme::one_brc(file);
    println!("{}", res);
}
