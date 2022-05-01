fn main() {
    test_2();
}

fn test_2() {
    let v = vec!["Yuhei", "Kuratomi"];
    v.iter().for_each(|x| println!("{}", x));
}
