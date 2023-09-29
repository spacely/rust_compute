fn main() {
    println!("Hello, world!");
    let q = 90;
    let  x = 5;
    const ABC: u32 = 7;
    println!("x is {}",x);

    let tuple = (45,5,6);
    println!("{}",tuple.0);
    let my_array = [10,5,6];
    println!("{}",my_array[0]);

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value within this place is {}",x);
    }
    println!("X is now {}",x);
    println!("Constant is {}",ABC);

    let  space = "    ";
    let space = space.len();
    println!("Space size {}",space);
    println!("A is of value {}",q);
    
}
