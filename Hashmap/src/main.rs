use std::collections::HashMap;

fn group_by_key(pairs: Vec<(&str, i32)>) -> HashMap<&str, Vec<i32>> {
    let mut map: HashMap<&str, Vec<i32>> = HashMap::new();
    for (k, v) in pairs {
        map.entry(k).or_default().push(v);
    }
    map
}

fn main() {
    let data = vec![
        ("apple", 1),
        ("banana", 2),
        ("apple", 3),
        ("banana", 4),
        ("cherry", 5),
    ];
    
    let grouped = group_by_key(data);
    println!("{:?}", grouped);
}

