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

    {
        let s = String::from("헬로");

        let len = calc_length(&s); // 소유권을 잠시 빌려줌

        println!("'{}'의 길이는 {}입니다.", s, len);
    }

    {
        let s = String::from("Hello");

        append_word(&mut s); // mutable reference를 만들면, 추가 참조를 만들 수 없다.

        println!("s = {}", s);
    }

    {
        let mut s = String::from("Hello");

        let r1 = &s;
        let r2 = &s;

        println!("r1 = {}, r2 = {}", r1, r2);

        let r3 = &mut s;
        // println!("r1 = {}, r2 = {}, r3 = {}", r1, r2, r3); // Error

        println!("r3 = {}",r3); // OK - 범위가 겹치지 않아 가능 (실제 r1, r2의 참조가 r3 사용시점에는 참조 범위가 1개여서 가능)
        // println!("r1 = {}",r1); // Error
    }
}

fn string_length(s: String) {
    println!("문자열의 길이는 {}", s.len());
}

fn string_length2(s: String) -> String {
    println!("문자열의 길이는 {}", s.len());
    s
}

fn calc_length(s: &String) -> (usize) {
    let length = s.len();
    length
}

fn append_word(s: &mut String) {
    s.push_str("World!");
}
