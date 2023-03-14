#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32
}

// 구조체 내 함수 정의 -> impl 문 활용

/*
    연관 함수는 새로운 구조체의 인스턴스를 반환해주는 생성자
    - 연관함수란? : param에 self가 없으면서 impl 내에 정의된 함수
 */

impl Rectangle {
    fn area(&self) -> u32 {
        self.width*self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn newSq(_size: u32) -> Rectangle {
        Rectangle {
            width:_size, height:_size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30, height: 40
    };

    let rect2 = Rectangle {
        width: 20, height: 20
    };

    let rect3 = Rectangle {
        width: 30, height: 50
    };

    let sq = Rectangle::newSq(30);


    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Sq Info: {:#?}", sq);
}