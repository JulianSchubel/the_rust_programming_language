use statistics::averages;

fn main() {
    let v: Vec<isize> = vec![1,22,2,2,52456,71];
    if let Some(x) = averages::mode(&v) {
        println!("{x}");
    }
}

