use std::collections::HashMap;
pub fn mode(vector: &Vec<isize>) -> Option<isize> {
        if vector.len() < 1 {
            return None
        }
        let mut map: HashMap<isize, usize> = HashMap::new();
        for i in vector {
            let count = map.entry(*i).or_insert(0);
            *count += 1;     
        }
        let mut max: Option<isize> = None;
        let mut occured: usize = 0;
        for (k, v) in map {
            if v > occured {
                occured = v;
                max = Some(k); 
            }
        }
       max 
    }

pub fn median(vector: &Vec<isize>) -> Option<f32> {
    if vector.len() < 1 {
        return None
    }
    let mut median: Option<f32> = None;
    let mut v: Vec<isize> = Vec::new();
    for i in vector {
        v.push(*i)
    }
    /* Median is defined for sorted values */
    v.sort();
    if vector.len() % 2 == 0 {
        if let Some(x) = vector.get(vector.len() / 2) {
            median = Some(*x as f32);
        }
        if let Some(y) = vector.get((vector.len() / 2) - 1) {
           median = Some((median.unwrap() + *y as f32) / 2.0);
        }
    } else {
       if let Some(x) = vector.get(vector.len() / 2) {
            median = Some(*x as f32);
       }
    }
    median
}

#[cfg(test)]
mod tests {
    use crate::averages;
    #[test]
    fn test_mode() {
        let v: Vec<isize> = vec![-1,5,5,5,-18,-1,23,12];
        let result = averages::mode(&v);
        match result {
            Some(x) => assert_eq!(x, 5),
            other => assert_eq!(other, None),
        }
    }

    #[test]
    fn test_median() {
        let v: Vec<isize> = vec![1,2,3];
        let result = averages::median(&v);
        match result {
            Some(x) => assert_eq!(x, 2 as f32),
            other => assert_eq!(other, None),
        }
    }

}
