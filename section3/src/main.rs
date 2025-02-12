
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
}

