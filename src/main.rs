fn main() {
    // Testing for loop fibonacci
    println!("For-loop fibonacci calculation:");
    test_fibonacci(fibonacci);

    // Testing recursive fibonacci
    println!("Recursive fibonacci calculation:");
    test_fibonacci(fibonacci_recursive);

}

fn fibonacci(n: u32) -> u32 {
    // Fibonacci calculation using for-loop.
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

fn fibonacci_recursive(n: u32) -> u32 {
    // Fibonacci calculation using recursion.
    if n == 1 || n == 2 {
        return 1
    }
    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

fn test_fibonacci(fibonacci: fn(u32) -> u32) {
    /*
    * For a given fibonacci calculator, print the
    * first 10 fibonacci numbers.
    *
    * Param: fibonacci: fn(u32) -> u32
     */
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
