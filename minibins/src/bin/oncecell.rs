use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::OnceCell;

fn global_date() -> &'static Mutex<HashMap<i32, String>> {
    static INSTANCE: OnceCell<Mutex<HashMap<i32, String>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(12, "Spica".to_string());
        Mutex::new(m)
    })
}

fn main() {}