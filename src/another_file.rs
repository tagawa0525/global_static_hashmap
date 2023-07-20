#[path = "./my_map.rs"]
pub mod my_map;

use my_map::MY_MAP;

pub fn print_my_map() {
    println!("another_file : {:?}", MY_MAP);
}
