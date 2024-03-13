/* When a vector is dropped, all of its contents are also dropped */
/* Cannot hold a reference to an element in a vector an add an item to the vector, as the vector
 * may need to be resized which would invalidate the reference */
fn main () {
    /* u32 vector */
    /* Have to type annotate here as the compiler won't otherwise know what the type of the
     * elements are */
    let v2: Vec<u32> = Vec::new();
    /* Initialized i32 vector */
    /* vec! macro creates a new vector containing the provided values. Type is inferred from the
     * given values */
    let v3: Vec<i32> = vec![1,2,3];

    /* iterate over a vector not implementing the copy trait will cause ownership to be transferred
     * to the into_iterator values by copy */
    /* println!("iteration with values by copy");
    for i in v3 {
        println!("{i}");
    }*/

    /* Iterate over immutable references to each element in a vector */
    println!("Immutable iteration over a vector:");
    /* i is a reference to i32 */
    for i in &v3 {
        /* elements are immutable */
        println!("{i}");
    }
    /* Iterate over mutable references to each element in a vector */
    let mut v3: Vec<i32> = vec![1,2,3];
    println!("Mutable iteration over a vector with mutation:");
    for i in &mut v3{
        /* elements are mutable */
        /* Because i is an i32 reference need to dereference the objects in the vector */
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

    /* Access vector with get require an option as this could be null */
    /* note that .get() returns a reference to the element */
    let x: Option<&i32> = v4.get(2);
    if let Some(value) = x {
        println!("received reference to i32 value of {}", value);
    } else {
        println!("received reference of unexpected type");
    }

    /* Can also access an element of a vector by reference as follows */
    /* note however that this will cause the program to cash if we attempt to access a non-existent
     * index */
    let y: &i32 = &v3[2];

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
