fn fibonacci(num: u32) {
    fn fib(left: u32, right: u32, more: u32) {
        if more <= 0 { return; }
        let next = left + right;
        println!("{}", next);
        fib(right, next, more - 1);
    }

    println!("0");
    println!("1");
    fib(0, 1, num);
}
