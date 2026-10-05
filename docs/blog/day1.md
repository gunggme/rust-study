# leaning Rust day1

한번 러스트를 배워보자

let 변수는 기본적으로 불변성을 가짐, 선언 이후에는 변형이 불가능
```rust
// main.rs
fn main() {
    let a = 9;
    print!("Hello, world! The value of let a is: {}", a);
    // ERROR!!!
    // because "let" is immutable by default!!
    // a = 30;
    // print!("Hello, world!222 The value of a is: {}", a);
}
```
하지만 mut키워드를 추가하면 가변 변수로 사용 가능함
```rust
// main.rs
fn main() {
    let mut b = 30;
    print!("Hello, world22! The value of let mut b is: {}", b);
    b = 40;
    print!("Hello, world33! The value of let mut b is: {}", b);
}
```

이를 통해서 인풋을 받을 수 있는 변수가 생성이 가능함, input은 std::io로 가져올 수 있음
> 추가로 rust는 스네이크 케이스를 사용해야함, 별도로 카멜, 파스칼 사용시 경고를 띄움(설정을 통해서 변경가능)
```rust
// main.rs
fn main() {
    let mut input_val = String::new();

    std::io::stdin()
        .read_line(&mut input_val)
        .expect("Failed to read line");

    print!("You entered: {}", input_val);
}
```

