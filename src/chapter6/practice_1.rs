// [enum 열거형]

use std::io;
enum Message {
    Quit,
    Move { x: i32, y: i32 }, // Move -> 익명 구조체 type
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message { // 구조체 뿐만 아니라 열거형에도 impl 활용한 메소드 정의 가능
    fn call(&self){
        // println!("{}", self::Write);  ~> 도대체 왜 오류가 뜨는 걸까요..?
    }
}

fn main(){
    let m = Message::Write(String::from("hello"));

}