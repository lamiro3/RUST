enum Coins {
    Penny,
    Nickel,
    Dime,
    Quater
}

fn value_in_cents(coins: Coins) -> i32 {
    match coins {
        // [match 표현식] : 열거형과 열거형의 variant를 패턴으로 사용함.
        Coins::Penny => {
            println!("1 penny군요! 축하합니다 :)"); // 이렇게 되면 Coins::Penny가 참조될 때 마다 match 표현식에 의해 실행됨
            1
        },
        Coins::Nickel => 5,
        Coins::Dime => 10,
        Coins::Quater => 25,
    }
}

fn main() {
    let penny = value_in_cents(Coins::Penny);
    println!("{}", penny);
}