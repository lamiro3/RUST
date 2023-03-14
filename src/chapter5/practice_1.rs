#[derive(Debug)] //구조체를 출력할 수 있도록, 디버깅 하는 동안 그 값을 볼수 있게 해주는 트레잇
struct Rectangle {
    width: u32,
    height: u32
}

fn area(rect: Rectangle) -> u32 {
    return rect.width * rect.height;
}

fn main() {
    let rect1 = Rectangle {
        width: 20, height: 30
    };

    /*
        :? -> 구조체의 각 값들을 자세히 출력
        :#? -> 구조체의 각 값들을 더 자세한 format으로 출력
     */
    //println!("rect1 is {:?}", rect1);  | [OUTPUT] => rect1 is Rectangle { width: 20, height: 30 }
    println!("rect1 is {:#?}", rect1);
    /*
        [OUTPUT] =>
        rect1 is Rectangle {
            width: 20,
            height: 30,
        }
     */
}