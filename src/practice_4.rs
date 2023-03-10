// use std::io;

fn main(){
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    // 불변 참조자의 경우에는 컴파일 시 아무런 문제가 없음.
    let r3 = &mut s;
    // 가변 참조자의 경우, 문제가 생겨야하는데 왜 안생기지..?
}

// 불변 참조자 - 사라짐