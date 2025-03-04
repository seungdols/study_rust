// enum Result<T, E> {
//     OK(T),
//     Err(E),
// }

use std::fs::File;
use std::io::Error;
use std::io::Read;

fn read_username() -> Result<String, Error> {
    // let file_result = File::open("hello.txt");

    // let mut file = match file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut username = String::new();
    // match file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }

    // ?을 사용하기
    // let file_result = File::open("hello.txt")?; // ?가 에러 전파하기의 축약 표현
    // let mut username = String::new();
    // file.read_to_string(&mut username)?;
    // Ok(username)


    // 더 축약 표현
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

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
    // let file = File::create("hello.txt").expect("파일을 열 수 없습니다.");

    let username = read_username();
    println!("username= {:?}", username);
}
