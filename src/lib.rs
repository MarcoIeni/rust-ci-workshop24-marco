pub fn add() {
    let var1 = Some(1);
    match var1 {
        Some(num) => println!("{num}"),
        None => {},
    }
}
