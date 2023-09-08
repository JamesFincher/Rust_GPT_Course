fn add_five(num: u32) -> u32 {
    num + 5
}

fn main() {
    let mut x: u32 = 50;
    println!("{}", x);

    let y: u32 = add_five(x);
    println!("y is {}", y);

    x = 70;
    print!("x is {}", x);
}
