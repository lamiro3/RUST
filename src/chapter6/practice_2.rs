// [Option 열거형]

// rust에는 null value가 없기 때문에, Option<T>를 이용해 값의 존재/부재의 개념을 정의할 수 있음.

// <T> : Generic Type Parameter -> 10장에서 배울 예정 -> T에 Type 명시하면 됨.

fn main(){
    let x:i8 = 3;
    let y:Option<i8> = Some(4);

    let sum = x + y;

    // 위와 같은 경우는 컴파일 불가 | i8과 Option<i8>은 엄연히 다른 dtype임.
}