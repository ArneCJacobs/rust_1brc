mod t006_memmap;
fn main() {
    println!("Hello, world!");
    let file = "/Users/steam/git/1brc/measurements_10_000.txt";
    let res = t006_memmap::one_brc(file);
    println!("{}", res);
}
