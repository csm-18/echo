fn main() {
    let args: Vec<String> = std::env::args().collect();
    let args = &args[1..];
    println!("{:?}", args);
}
