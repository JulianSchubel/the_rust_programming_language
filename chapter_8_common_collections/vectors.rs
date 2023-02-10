fn main () {
    /* u32 vector */
    let v2: Vec<u32> = Vec::new();
    /* Initialized i32 vector */
    let v3: Vec<i32> = vec![1,2,3];
    /* Iterate over an immutable references to vector */
    println!("Immutable iteration over a vector:");
    for i in &v3 {
        /* elements are immutable */
        println!("{i}");
    }
    /* Iterate over mutable reference to a vector */
    let mut v3: Vec<i32> = vec![1,2,3];
    println!("Mutable iteration over a vector with mutation:");
    for i in &mut v3{
        /* elements are mutable */
        /* Changing the objects in the vector */
        *i += 2;
        println!("{i}");
    }

    /* Iterate over a vector with enumeration */
    for (i, j) in v3.iter().enumerate() {
        println!("{i}, {j}");
    }

    
    let mut v4: Vec<i32> = Vec::new();
    v4.push(5);
    v4.push(6);
    v4.push(7);
    v4.push(8);

    /* Access vector with get require an a option as this could be null */
    let x: Option<&i32> = v4.get(2);
    if let Some(value) = x {
        println!("received reference to i32 value of {}", value);
    } else {
        println!("received reference of unexpected type");
    }

    /* Vector with different types using an enum */
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3), 
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];
    
    for i in &row {
        match i {
            SpreadsheetCell::Int(x) => println!("{x}"),
            SpreadsheetCell::Text(y) => println!("{y}"),
            SpreadsheetCell::Float(z) => println!("{z}"),
            _other => println!("unexpected value received"),             
        }
    }
}
