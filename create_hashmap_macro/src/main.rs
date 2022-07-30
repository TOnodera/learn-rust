macro_rules! hashmap {
    ($($key: expr => $val: expr),*) => {{
        let mut tmp = std::collections::HashMap::new();
        $(
            tmp.insert($key,$val);
        ) *
        tmp
    }};
}
fn main() {
    let week = hashmap!(
        "mon" => "月曜日",
        "tue" => "火曜日",
        "wed" => "水曜日",
        "thu" => "木曜日",
        "fir" => "金曜日",
        "sat" => "土曜日",
        "sun" => "日曜日"
    );
    for (k, v) in week {
        println!("key={},val={}", k, v);
    }
}
