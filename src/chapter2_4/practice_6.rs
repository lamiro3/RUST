fn first_word(s: &String) -> &str { // &str -> "스트링 슬라이스" dType !!
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // 공백 문자를 찾았다면,
            return &s[..i]; // "스트링 슬라이스 이용" ~ 첫 번째 단어의 참조자를 반환
        }
    }

    &s[..] // 공백 문자를 찾지 못했다면, 문자열 전체에 대한 참조자를 반환
}

fn main(){
    let mut s = String::from("Hello world!");
    let word = first_word(&s); // 공백이 위 문자열의 5번 index에 위치해있기 때문에 word가 5로 초기화됨 

    println!("{}", word);
    s.clear(); // 문자열 비워버림 -> 이로 인해 가변 참조자를 갖기 위한 시도를 하게됨.
            // * 참조자를 이용할 땐, 반드시 여러개의 불변 참조자 / 한 개의 가변 참조자, 이 둘 중 하나여야만 함.
            // 따라서 clear() func이 먼저 실행되고 난 뒤, 그 뒤에  첫 번째 단어를 출력하게 되면, 위 규칙을 위배하게 됨.
            // -> 결과적으로 오류 발생
    // println!("{}", word);

    // 여기서부터, word는 여전히 5라는 값을 가지고 있지만, 이 값을 유용하게 사용할 문자열이 사라졌기 때문에 의미가 없음.
    // 위와 같은 문제를 해결하기 위해 RUST에서는 "스트링 슬라이스"를 이용한다.
}