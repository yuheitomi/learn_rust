use serde_json::json;

fn main() {
    let hanako = json!({
        "name": "hanako",
        "age": 8,
        "favorites": {
            "food": ["apple", "melon"]
        }
    });

    println!("{:?}", hanako);

    println!("hanako[name]: {}", hanako["name"]);

    let boxed = Box::new(1);
    let val = *boxed;
    println!("{}", val);
}
