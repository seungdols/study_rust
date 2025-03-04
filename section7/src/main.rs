// enum Result<T, E> {
//     OK(T),
//     Err(E),
// }

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // let file = File::open("hello.txt");

    // let file = match file {
    //     Ok(f) => println!("file opening is sucess!"),
    //     Err(e) => match e.kind() {
    //         ErrorKind::NotFound => File::create("hello.txt"),
    //        _=> panic!("file opening failed: {:?}", e),
    //     },
    // };

    // Use unwrap()
    // let file = File::open("hello.txt").unwrap();
    // println!("file opening is sucess!");

    // Use expect()
    let file = File::create("hello.txt").expect("파일을 열 수 없습니다.");
}
