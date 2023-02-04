fn main() {
    /* tuples */
    let tuple1 : (u8,u8,u8) = (255,255,255);
    let tuple2 : (u32, f64, u8) = (500, 6.4, 1);
    /* Can destructure (unpack) a tuple */
    let (x,y,z) = tuple2;
    println!("The value of y is {y}");
    /* tuple access */
    let x = tuple2.0;
    println!("The value of x is {x}");

    /* arrays */
    let a = [1,2,3,4,5];
    let b : [i32; 5] = [1,2,3,4,5];
    /* Can initialize an array to have the same value for every element */
    /* below will contain 5 elements initialized to 3 */
    let c = [3;5];
    /* array access */
    let five = a[4];
    println!("The fifth element of the array is {five}");
}

