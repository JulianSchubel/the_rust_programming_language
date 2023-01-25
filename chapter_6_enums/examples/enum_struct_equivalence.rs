enum Message {
    Quit,
    /* Two named fields, x and y */
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32)
}

/* Unit struct */
struct QuitMessage;
/* Normal struct */
struct Move {
    x: i32,
    y: i32
};
/* Tuple struct */
struct Write(String);
/* Tuple struct */
struct ChangeColor(i32,i32,i32);

fn main() {
    
}
