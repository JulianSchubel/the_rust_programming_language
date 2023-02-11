use std::collections::HashMap;

fn mode(vector: &Vec<isize>) -> isize {
    let mut map: HashMap<isize, usize> = HashMap::new();
    for i in vector {
        let count = map.entry(*i).or_insert(0);
        *count += 1;     
    }
    let mut max: isize = 0;
    let mut occured: usize = 0;
    for (k, v) in map {
        if v > occured {
            occured = v;
            max = k; 
        }
    }
    max
}

fn main() {
    let v: Vec<isize> = vec![-1,5,5,5,-18,-1,23,12];
    let mode = mode(&v);
    println!("{}", mode);
}
