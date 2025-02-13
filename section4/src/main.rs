fn main() {
    // String 타입의 메모리 할당
    let mut s = String::from("Hello");
    s.push_str(", World!");
    println!("s = {}", s);

    /*
    소유권 규칙
    1. Rust에서 모든 값은 소유자가 있다.
    2. 한 시점에 딱 하나의 소유자만 있을 수 있다.
    3. 소유자의 범위가 끝나면, 값도 사라진다.
     */

    {
        let s1 = String::from("Hello"); // Heap
                                        // let s2 = s1; // S2로 소유권이 넘어가게 됨. S1은 소유권을 잃음
                                        // println!("s2 = {s2}");
                                        // println!("s1 = {s1}"); // 그래서 컴파일 오류

        let s2 = s1.clone();
        println!("s2 = {s2}");
        println!("s1 = {s1}"); // clone()이므로 OK
    }

    {
        let x = 3;
        let y = x; // 기본 데이터 타입은 Copy

        let s = String::from("헬로");
        string_length(s); // 소유권 이전
                          // println!("s: {s}"); // Error

        let s2 = String::from("헬로2");
        let r = string_length2(s2);
        println!("return: {}", r);
    }
}

fn string_length(s: String) {
    println!("문자열의 길이는 {}", s.len());
}

fn string_length2(s: String) -> String {
    println!("문자열의 길이는 {}", s.len());
    s
}
