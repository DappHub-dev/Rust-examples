fn main() {
    let s = gives_ownership();

    let s1 = String::from("hi");
    let s2 = takes_and_gives_back(s1);
    println!("{}, {}", s, s2);
    let (s3, len) = calculate_length(s);
    println!("{}, {}", s3, len);
}

fn gives_ownership() -> String{
    let s = String::from("hello");
    s
}

fn takes_and_gives_back(s: String) -> String{
    s
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}