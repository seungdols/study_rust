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
}
