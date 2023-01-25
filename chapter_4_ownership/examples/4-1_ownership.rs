fn main() {
    let mut s = String::from("");
    /* push_str() appends a literal to a String */
    s.push_str(", world!");
    println!("{s}");
    let mut x = 5;
    let y = x;
    x = 7;
    println!("x:{x}, y:{y}");
}
