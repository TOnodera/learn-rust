use std::{collections::HashMap, hash::Hash};
fn main() {
    let tuki = [
        "睦月",
        "如月",
        "弥生",
        "卯月",
        "皐月",
        "水無月",
        "文月",
        "葉月",
        "長月",
        "神無月",
        "霜月",
        "師走",
    ];
    let mut tuki_map: HashMap<&str, usize> = HashMap::new();
    for (i, v) in tuki.iter().enumerate() {
        tuki_map.insert(v, i + 1);
    }

    for (key, val) in tuki_map.iter() {
        println!("{}{}", key, val);
    }
}
