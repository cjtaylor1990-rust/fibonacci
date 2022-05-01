fn main() {
    test_fibonacci();
}

fn fibonacci(n: u32) -> u32 {
    let mut previous = 1;
    let mut current = 1;

    let mut temp;
    for _n in 3..n+1 {
        temp = current;
        current = current + previous;
        previous = temp;
    }
    current
}

fn test_fibonacci() {
    println!("First: {}", fibonacci(1));
    println!("Second: {}", fibonacci(2));
    println!("Third: {}", fibonacci(3));
    println!("Fourth: {}", fibonacci(4));
    println!("Fifth: {}", fibonacci(5));
    println!("Sixth: {}", fibonacci(6));
    println!("Seventh: {}", fibonacci(7));
    println!("Eighth: {}", fibonacci(8));
    println!("Ninth: {}", fibonacci(9));
    println!("Tenth: {}", fibonacci(10));
}
