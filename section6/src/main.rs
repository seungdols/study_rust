#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
enum Message {
    StartGame,
    WinPoint { who: String },
    ChangePlayerName(String),
}

// enum Option<T> {
//     None,
//     Some(T)
// }

struct RGB(u8, u8, u8);
fn color_to_rgb(color: Color) -> RGB {
    match color {
        Color::Red => RGB(255, 0, 0),
        Color::Green => RGB(0, 255, 0),
        Color::Blue => RGB(0, 0, 255),
    }
}

fn handle_message(message: &Message) {
    match message {
        Message::StartGame =>
            println!("게임을 시작합니다."),

        Message::WinPoint { who } =>
            println!("{}의 득점", who),

        Message::ChangePlayerName(name) => println!("플레이어의 이름 변경 => {}", name),

        _ => println!("아직 구현 하지 않은 메시지"),
    }
}

fn increment(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {

    let red = Color::Red;
    let green = Color::Green;
    let blue = Color::Blue;

    println!("Hello, world!");
    println!("red = {:?}", red);
    println!("green = {:?}", green);
    println!("blue = {:?}", blue);
    println!("red == green => {}", red == green);

    let m1 = Message::StartGame;
    let m2 = Message::WinPoint {
        who: String::from("seungdols"),
    };
    let m3 = Message::ChangePlayerName(String::from("seungdols"));

    println!("m1 = {:?}", m1);
    println!("m2 = {:?}", m2);
    println!("m3 = {:?}", m3);

    let some_number = Some(2);
    let absent_number: Option<i32> = None;
    println!("some_number = {:?}", some_number);
    println!("absent_number = {:?}", absent_number);

    // some_number + 1; // Error
    let x = Some(2);
    println!("{:?}", increment(x));
    println!("{:?}", increment(None));
}
