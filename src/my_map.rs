use std::collections::HashMap;
use std::sync::OnceLock;

pub static MY_MAP: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

pub fn init_my_map() {
    let map = MY_MAP.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("one", "uno");
        m.insert("two", "dos");
        m.insert("three", "tres");
        m
    });
    println!("my_map : {:?}", map);
    println!("my_map : {:?}", MY_MAP);
}
