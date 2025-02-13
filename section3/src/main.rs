
// 상수 - mut 붙일 수 없음.
const PI: f32 = 3.141592;

fn main() {
    // mut 을 붙이면, 가변 변수가 됨.
    // let mut x = 3;
    // println!("x: {x}");
    // x = 7;
    // println!("x: {x}");

    println!("PI 상수 값은 {PI}입니다.");

    let x = 3;
    println!("x의 값은 {x} 입니다");
    let x = x + 1; // variables shadowing
    println!("x의 값은 {x} 입니다");

    {
        let x = x * 2; // variables shadowing
        println!("안쪽 범위에서 x의 값은 {x} 입니다");
    }
    println!("x의 값은 {x} 입니다");


    let add = 3 + 8;
    let sub = 26.5 - 2.1;
    let mul = 7 * 20;
    let quotient = 12.0 / 3.14;
    let truncated  = 7 / 5; // 결과는 1
    let remainder = 46 % 5;

    let t = true;
    let f: bool = false;

    let c = 'A';
    let z: char = '가';

    // tuple
    let t = (32, true, 1.41);
    let (x,y,z) = t;
    println!("{x},{y},{z} 입니다.");

    // unit tuple = void type of other language
    // ()

    let x = [1,2,3,4,5];
    x[0]; // 1
    x[4]; // 5

    let threes = [3; 100];
    let last = threes[99];
    println!("{last}");

}

