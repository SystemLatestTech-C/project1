// 구조체 정의
struct Numbers {
    a: i32,
    b: i32,
}
// 두 변수의 덧셈을 수행하는 메서드
impl Numbers {
    fn add(&self) -> i32 {
        self.a + self.b
    }
}
// 표준 입력에서 숫자를 읽어오는 함수
fn read_number() -> i32 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("입력을 읽지 못했습니다.");
    input.trim().parse().expect("올바른 숫자가 아닙니다.")
}
fn main() {
    // 사용자로부터 두 개의 숫자 입력 받기
    println!("첫 번째 숫자를 입력하세요:");
    let a = read_number();
    println!("두 번째 숫자를 입력하세요:");
    let b = read_number();
    // Numbers 구조체의 인스턴스 생성 및 덧셈 결과 출력
    let nums = Numbers { a, b };
    println!("{} + {} = {}", nums.a, nums.b, nums.add());
}
