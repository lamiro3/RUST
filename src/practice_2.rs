extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

// 숫자 예측하기 예제

fn main() {
    println!("Guess the number!");
    let res = rand::thread_rng().gen_range(1, 101); 
    //thread_rng() func -> 현재 thread에서의 정수생성기(Random Number Generator)

    loop {
        // 입력받는 문자열을 저장할 변수는 가변변수로 설정해야한다. 따라서 "mut"를 함께 작성해야한다.
        let mut pre = String::new();

        //이 부분이 도대체 뭘 의미하는 것인지 더 알아봐야할듯요.
        io::stdin().read_line(&mut pre)
            .expect("Failed to read a line...");

        //trim() func -> 입력받은 문자열의 양쪽 끝 공백을 제거하는 역할
        //parse() func -> 문자열을 numerical 형태로 변환해주는 역할
        //앞서 가변변수 pre를 선언했지만, RUST에서는 "shadowing"을 지원한다. <추후에 알게 될 예정>

        // let pre: u32 = pre.trim().parse()
        //     .expect("Plz type your answer: ");

        let pre: u32 = match pre.trim().parse() {
            Ok(num) => num,     // 숫자일 떄 -> 그대로 출력
            Err(_) => {     // 숫자가 아닐 떄 -> 경고 메시지 출력 후, 해당 iteration pass
                println!("Only a number >_<");
                continue;
            }
        };

        println!("Your answer: {}", pre);

        //match 표현식
        match pre.cmp(&res) {
            Ordering::Less      => println!("Too small :("),
            Ordering::Greater   => println!("Too big :("),
            Ordering::Equal     => {
                println!("Nice :)");
                break;
            }
        }
    }
}