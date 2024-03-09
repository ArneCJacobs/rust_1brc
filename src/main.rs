// use std::fs::read_to_string;
//
// use diffy::{create_patch, PatchFormatter};


// const FILE_LOCATION: &str = "/Users/steam/git/1brc/measurements_10_000.txt";

mod t001_basic;
mod t002_fixed_point;
mod t003_branchless_parsing;

fn main() {
    // let mask: i32 = -1;
    // println!("{:#x} {:#x}", mask, mask as usize);
    // let baseline = read_to_string("./data/sol_10_000.txt").expect("Could not read file");
    //
    // let expected = t001_basic::one_brc(FILE_LOCATION);
    // // println!("{}", res);
    // let actual = t002_fixed_point::one_brc(FILE_LOCATION);
    // // let actual = t003_branchless_parsing::one_brc(FILE_LOCATION);
    // // println!("{}", res);
    // let patch = create_patch(&baseline, &expected);
    //
    // // let patch = create_patch(&expected, &actual);
    // let f = PatchFormatter::new().with_color();
    // println!("{}", f.fmt_patch(&patch));
}
