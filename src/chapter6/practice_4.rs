fn plus_one(x:Option<i32>) -> Option<i32> {
    match x {
        None => None, // None은 반드시 명시해줘야 함 | 근데 작성 못했더라도 RUST가 알아서 알려줌( 너 이거 빼먹었다고 )
        Some(i) => Some(i+1)
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    /*
        plus_one func에 five가 그대로 대응되기 때문에 Some(5+1), 즉 Some(6)이 그대로 반환됨.
     */
    let none = plus_one(None);
}