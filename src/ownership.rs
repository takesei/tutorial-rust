// Thx to the great movie to understand ownership and lifetime::
// https://www.youtube.com/watch?v=lG7YbM2AfU8&t=201s

use std::collections::HashMap;

pub fn greet_map(id: isize, name: &str) -> HashMap<isize, String> {
    let mut map = HashMap::new();
    let message = format!("Hello, {}!", name);
    map.insert(id, message);
    map
}
