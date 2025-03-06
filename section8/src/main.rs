fn smallest<T: std::cmp::PartialOrd>(list: &[T]) -> &T{
    let mut smallest = &list[0];

    for item in list {
        if item < smallest {
            smallest = item;
        }
    }

    smallest
}
// fn smallest_i32(list: &[i32]) -> &i32 {
//     let mut smallest = &list[0];

//     for item in list {
//         if item < smallest {
//             smallest = item;
//         }
//     }

//     smallest
// }

// fn smallest_char(list: &[char]) -> &char {
//     let mut smallest = &list[0];

//     for item in list {
//         if item < smallest {
//             smallest = item;
//         }
//     }

//     smallest
// }

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

trait Greet {
    fn greeting(&self) -> String;
}

#[derive(Debug)]
enum Pet {
    Dog,
    Cat,
    Tiger,
}

impl Greet for Pet {
    fn greeting(&self) -> String {
        match self {
            Pet::Dog => String::from("멍멍"),
            Pet::Cat => String::from("야옹"),
            Pet::Tiger => String::from("어흥"),
        }
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    active: bool,
}

impl Greet for Person {
    fn greeting(&self) -> String {
        String::from("안녕하세요")
    }
}

// fn meet(one: &impl Greet, another: &impl Greet) {
//     println!("첫번째가 인사합니다: {}", one.greeting());
//     println!("두번째가 인사합니다: {}", another.greeting()));
// }

use std::fmt::Debug;
use std::fmt::Display;

// fn meet<T: Greet + Debug, U: Greet + Display>(one: &T, another: &U) {
//     println!("첫번째가 인사합니다: {}", one.greeting());
//     println!("두번째가 인사합니다: {}", another.greeting());
// }

fn meet<T, U>(one: &T, another: &U)
where T: Greet + Debug,
      U: Greet + Display
{
    println!("첫번째가 인사합니다: {}", one.greeting());
    println!("두번째가 인사합니다: {}", another.greeting());
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{}", self.name)
        f.write_str(&self.name.as_str())
    }
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

struct ImportantPart<'a> {
    part: &'a str,
}

fn lifetime_in_struct() {
    let sentence = String::from("안녕하세요. 러스트의 참조 수명에 대해 알아볼게요.");
    let first_sentence = sentence.split('.')
    .next()
    .expect("마침표를 찾을 수 없어요!");

    let i = ImportantPart { part: first_sentence };
    println!("{}", i.part);
}

fn static_lifetime() {
    let s: &'static str = "I have a static lifetime.";
}

fn main() {
    let numbers = vec![3,4,1,6,8,10];
    let result = smallest(&numbers);
    println!("가장 작은 수는 {}", result);

    let chars = vec!['홍', '길', '동'];
    let result = smallest(&chars);
    println!("가장 작은 문자는 {}", result);

    let result = smallest(&["홍길도", "둘리", "도우너"]);
    println!("가장 작은 문자열은 {}", result);

    let p1 = Point { x: 2, y:3 };
    let p2 = Point { x: 5.0, y: 1.0};
    println!("p1 = {:?}, p2 = {:?}", p1, p2);

    let cat = Pet::Cat;
    let gildong = Person {
        name: String::from("홍길동"),
        active: true,
    };
    meet(&cat, &gildong);

    let s1 = String::from("가나다");
    let s2 = "하나둘셋";

    let res = longest(s1.as_str(), s2);
    println!("가장 긴 문자열은 {}", res);
}
