use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    /* add key:value pairs to the hash map */
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    /* retrieve the values */
    let key = String::from("Blue");
    let score = scores.get(&key).copied().unwrap_or(0);
    
    /* Iterate over a hash map */
    for (key, value) in &scores {
        println!("Key: {}, Value: {}", key, value);
    }

    /* Hash maps and ownership */
    let field_name = String::from("Favourite colour");
    let field_value = String::from("Blue");
    let mut map: HashMap<String, String> = HashMap::new();
    /* field_name and field_value will become invalid after the below; they are owned by the hashmap */
    map.insert(field_name, field_value);
    for (key, value) in &map{
        println!("{key}, {value}");
    }
    /* This will overwrite the value in the hash map */
    map.insert(String::from("Favourite colour"), String::from("Green"));
    for (key, value) in &map{
        println!("{key}, {value}");
    }

    /* check if a key exists before inserting else return a mutable reference to the value for the
     * corresponding key */
    let mut team_size = HashMap::new();
    team_size.insert(String::from("Blue"), 10);
    /* Will return a mutable reference to the value 10 */
    team_size.entry(String::from("Blue")).or_insert(50);
    /* will insert the value 50 */
    team_size.entry(String::from("Yellow")).or_insert(50);
    println!("{:?}", team_size);

    /* Update a key's value based on the old value */
    let text = "Hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

