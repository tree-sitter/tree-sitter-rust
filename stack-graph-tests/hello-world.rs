fn fib(
    n: i64
) -> i64 {
    if n <= 1 {
//     ^ defined: 2
        1
    } else {
        fib(n - 1) * n
//      ^ defined: 1
//          ^ defined: 2
//                   ^ defined: 2
    }
}
