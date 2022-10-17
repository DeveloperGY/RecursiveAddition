use std::io;

fn rec_add(a: i32, b: i32) -> i32
{
    if b == 1
    {
        return a;
    }

    return a + rec_add(a, b - 1);
}

fn main() {
    let mut a: String = String::new();
    let mut b: String = String::new();
    print!("A: ");
    io::stdin().read_line(&mut a).unwrap();
    let a: i32 = a.trim().parse().expect("e");
    print!("B: ");
    io::stdin().read_line(&mut b).unwrap();
    let b: i32 = b.trim().parse().expect("e");
    println!("A * B = {}", rec_add(a, b));
    
}