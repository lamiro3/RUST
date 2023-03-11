fn main() {
    println!("result = {}", fibonacci(4));
}

fn fibonacci(n: u32) -> u32{
    let mut ans: u32 = 1;
    for num in (2..(n+1)) {
        ans = ans * num;
    };

    ans
}