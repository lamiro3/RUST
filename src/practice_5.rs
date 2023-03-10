// 댕글링 참조자

fn main(){
    let ref_to_nothing = dangle(); 
    // (4) 그렇다면 변수 ref_to_nothing은 이미 무효화된 String의 참조자를 받게 됨. -> 이러한 경우를 "댕글링 참조자"
}

fn dangle() -> &String { // (5) RUST에선 이러한 경우, 오류 메시지를 통해 알려줌.
    let mut s = String::from("text"); // (1) 가변 String 변수 s 할당
    &s // (2) String s의 참조자 반환
}
// (3) [주의할 점!!]: 여기서 s는 dangle()가 종료된 후, 스코프를 벗어나기 때문에 메모리에서 사라짐