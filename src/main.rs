mod my_map;
use my_map::{init_my_map, MY_MAP};

mod another_file;
use another_file::print_my_map;

fn main() {
    init_my_map();
    println!("main : {:?}", MY_MAP);

    print_my_map();
}
