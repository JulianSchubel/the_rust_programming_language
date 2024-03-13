fn main() {
    /* string from string literal */
    let s1 = String::from("hello");
    /* with the _to_string() method */
    let literal = ", world!";
    /* the .to_string() method is available to all types that implement the Display trait */
    let s2 = literal.to_string();
    /* concat using format! */
    println!("{}", format!("{}{}", s1,s2));
    /* concat using +: note s1 is invalidated */
    /* signature of + is fn add(self, &T) -> Self */
    let s3 = s1 + &s2;
    println!("{}", s3);
    /* indexing into a string */
    /* is a slice of the entirety of s3 */
    let s4 = &s2[..];
    println!("{}", s4);
    /* iterate over unicode scalar values */
    for c in s3.chars() {
        println!("{c}");
    }

    /* iterate over raw bytes of the string */
    for b in s3.bytes() {
        println!("{b}");
    }
}
