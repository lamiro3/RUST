use std::io;

fn main(){
    let mut text = String::from("Hello");
    change(&mut text);

    // io::Stdout::from(text: String).writeln!();
}

// param을 그냥 참조자로 받게 되면, 소유권까지 넘어오는 것이 아니기 때문에
// 이를 수정하려면 반드시 가변 참조자로 받아와야 함.
fn change(t: &mut String){ 
    t.push_str(", World!");
}