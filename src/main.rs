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
    for _n in 2..n+1 {
        temp = current;
        current = current + previous;
        previous = temp;
    }
    current
}

fn fibonacci_recursive(n: u32) -> u32 {
    // Fibonacci calculation using recursion.
    if n == 0 || n == 1 {
        return 1
    }
    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

fn test_fibonacci(fn_to_test: fn(u32) -> u32) {
    /*
    * For a given fibonacci calculator, print the
    * first 10 fibonacci numbers (counting in 0 index).
    *
    * For the i-th number, the printed line will be,
    * f(n = i) = fibonacci(i).
    *
    * For example, for the 6th fibonacci number,
    * f(n = 5) = 8.
    *
    * Param: fn_to_test: fn(u32) -> u32
     */
    for n in 0..10 {
        println!("f(n = {}) = {}", n, fn_to_test(n));
    }
}
