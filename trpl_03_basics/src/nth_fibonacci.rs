// 0 1 1 2 3 5 8 13 21 34 ...
pub fn fib(n: u32) -> u32 {
    let mut count: u32 = 0;
    let mut first: u32 = 0;
    let mut second: u32 = 1;

    while count < n {
        let sum: u32 = first + second;
        first = second;
        second = sum;
        count += 1;
    }
    first
}

pub fn fib_rec(n: u32) -> u32 {
    fn rec(n: u32, first: u32, second: u32) -> u32 {
        if n == 0 { first }
        else { rec(n - 1, second, first + second) }
    }
    rec(n, 0, 1)
}