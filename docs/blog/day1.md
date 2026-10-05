# leaning Rust day1

한번 러스트를 배워보자

## 1. Variable and Print
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

## 2. Input!!!!

이를 통해서 인풋을 받을 수 있는 변수가 생성이 가능함, input은 std::io로 가져올 수 있음
> 추가로 rust는 스네이크 케이스를 사용해야함, 별도로 카멜, 파스칼 사용시 경고를 띄움(설정을 통해서 변경가능)
```rust
// main.rs
fn main() {
    let mut input_val = String::new();

    std::io::stdin()
        .read_line(&mut input_val);

    print!("You entered: {}", input_val);
}
```

## 3. Match

Match표현식은 러스트가 자랑하는 강력한 표현식이다.
switch문과 굉장이 비슷한데, 한번 아래 코드를 돌려보자
```rust 
// main.rs
use std::io;

fn main() {
    println!("Please enter an integer: ");

    let mut input_val = String::new();

    io::stdin()
        .read_line(&mut input_val)
        .expect("Failed to read line");

    match input_val.trim().parse::<i32>() {
        Ok(num) => println!("You entered: {}", num),
        Err(_) => println!("Please enter a valid integer."),
    }
}
```
위 코드 처럼, input을 받고 값을 비교할때 match를 많이 사용함, 많은 언어들이 보통 switch나 if로 비교를 진행할텐데, rust에서는 match 표현식으로 처리를 할 수 있음.

### 스위치 처리
아래처럼 보통은 '<' '>'를 이용해서 처리하는 비교식을 아래 "{int}..={int}" 표현으로 처리할 수 있는걸 확인할 수 있음
```rust
 fn score_to_grade(score: i32) -> &'static str {
    match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        0..=59 => "F",
        _ => "Invalid score",
    }
}
```

### 튜플 처리
match만 사용할 수 있는게 아니라 if문도 혼용해서 사용가능
```rust
fn position_check(x: i32, y: i32) -> &'static str {
    match (x, y) {
        (0, 0) => "Origin",
        (0, _) => "Y-axis",
        (_, 0) => "X-axis",
        (x, y) if x > 0 && y > 0 => "Quadrant I",
        (x, y) if x < 0 && y > 0 => "Quadrant II",
        (x, y) if x < 0 && y < 0 => "Quadrant III",
        (x, y) if x > 0 && y < 0 => "Quadrant IV",
        _ => "Unknown position",
    }
}
```

### @ 바인딩을 통해 변수 할당가능
@ 바인딩을 사용하면, n을 입력 받고, x에 할당을 하여 변수 이용가능
```rust
fn classify(n: i32) {
    match n {
        x @ 1..=9 => println!("한 자리 수: {x}"),
        x @ 10..=99 => println!("두 자리 수: {x}"),
        _ => println!("기타"),
    }
}
```

### 만약 치는게 귀찮다면?? matches!
match로 하면 코드가 보기 이상해지는 경우도 있다, 이 경우에는 matches매크로를 이용해서 단순하게 이용가능함
```rust
// main.rs
fn main () {
    let score = Some(95);

    if matches!(score, Some(v) if v >= 70) {
        println!("합격");
    }
}
```

## Result and expect??
### Result 사용해보기
Result는 일반적인 변수보단, rust에서 기본적으로 지원하는 enum이라고한다.
```
pub enum Result<T, E> {
  /// Contains the success value
  #[lang = "Ok"]
  #[stable(feature = "Rust1", since = "1.0.0")]
  Ok(#[stable(feature = "Rust1", since = "1.0.0")] T),
  /// Contains the error value
  #[lang = "Err"]
  #[stable(feature = "Rust1", since = "1.0.0")]
  Err(#[stable(feature = "Rust1", since = "1.0.0")] E),
}
```
조금 복잡해 보이지만, ok, err로 나뉘어져있고, 그저 실행이 되었나 안되었나를 보기위한 타입으로 보인다.
아래 코드를 보면 이해가 될 것이다.
```rust
// main.rs
fn is_i32<T>(n: T) -> Result<T, &'static str> {
    if type_of(&n) == "i32" {
        Ok(n)
    } else {
        Err("Not an i32")
    }
}

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn main(){
    let a = "adsf";
    let b: i32 = 5;

    println!("a is i32? {:?}\n", is_i32(a));
    println!("b is i32? {:?}\n", is_i32(b));
}
```
i32인지 확인하는 함수를 통해서, i32를 확인하고, 결과를 보는 코드인다. 아래로 출력이 되는데, Err과 OK로 나눠서 보내지게 되는걸 확인할 수 있다.
```
a is i32? Err("Not an i32")
b is i32? Ok(5)
```
### expect
expect는 어떠한 함수에서 에러가 발생했을때 예외를 처리해주는 메서드다. 위에 result 타입의 코드를 이용해서 한번 어떻게 쓰는지 확인해보자.
```rust
// main.rs
fn is_i32<T>(n: T) -> Result<T, &'static str> {
    if type_of(&n) == "i32" {
        Ok(n)
    } else {
        Err("Not an i32")
    }
}

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn main(){
    let a = "adsf";
    let b: i32 = 5;

    let a_result = is_i32(a).expect("ERROR!!!!");
    let b_result = is_i32(b).expect("ERROR!!!!");

    println!("a is i32? {:?}\n", a_result);
    println!("b is i32? {:?}\n", b_result);
}
```
결과를 보면?
```
thread 'main' (2721225) panicked at src/main.rs:17:30:
ERROR!!!!: "Not an i32"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
약간의 throw와 assert같은 느낌이 든다, 에러 발생시 바로 expect에서 지정한 문자열이 출력이 되고, 코드가 멈추고 트레이스할 수 있게 나타나는걸 확인할 수 있다.

## how to use for, while, loop ?
## for and while
for와 while은 보통의 python 언어와 비슷한걸 알 수 있다.
### for
```rust
fn main(){
    let arr = [10, 20, 30];
    for element in arr.iter() {
        println!("{}", element);
    }
}
/*
10
20
30
*/
```

### while
```rust
// main.rs
fn main(){
    let mut n = 3;
    while n > 0{
        println!("n is {}", n);
        n -= 1;
    }
    println!("DONE!");
}
/*
n is 3
n is 2
n is 1
DONE!
*/
```

### loop
loop의 경우 신기하게 for, while과 다른 느낌이 간한데, 변수에 선언이 가능하며, "break"에다가 변수 반환을 시킬 수 있다. 아래 예제 코드를보자.
```rust 
// main.rs
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // 20을 반환하고 루프 탈출
        }
    };
    println!("Result: {}", result);
}

/*
Result: 20
*/
```
이런식으로 별도 함수나, for문으로 따로 코드를 안빼고도, 한줄로 반복문을 돌려 변수값을 구할 수 있는걸 확인할 수 있다.

## rand
rand는 난수를 생성시키는 함수다. 아래 예제 코드를 보면, 어떻게 사용하는지 알 수 있다. 먼저 Cargo.toml에 아래 의존성이 필요하다.
```toml
# Cargo.toml
[dependencies]
rand = "0.8"
```
이렇게 하면, rand를 사용할 수 있고, 여러 다양한 난수가 생성되는걸 확인 할 수 있다.
```
fn main() {
    let mut a = 9;
    println!("now a is {}", a);
    a = rand::random();
    println!("now a is {}", a);
}
```
물론 범위를 설정하여, 그 사이에서 난수를 구할 수 있다.
```
use rand::Rng;

fn main() {
    let mut a = 9;
    println!("now a is {}", a);
    a = rand::thread_rng().gen_range(1..=10);
    println!("now a is {}", a);
}
```

## make Game, Number Guessing!!
이동안 배운걸로 number gussing게임을 만들어보자. 조건은 다음과 같다.
```md
- 정답은 정수로 정해져야하며, 부동소수점이나 문자열등.. i32이외의 타입이 되면 안된다.
- 5회만 도전할 수 있다.(오답 입력시 횟수 카운트)
- 정수 입력만 취급해야하며, 이외의 입력은 무효로 처리한다(도전 횟수 차감 X, 프로그램 종료)
- 만약 틀린 값이 입력되면, 더 높은 값인지, 알 수 있게 힌트를 주어야한다.
- result, expect 1회 이상 사용할것
```

내 코드!!
```rust
use rand::Rng;

fn is_correct_guess(guess: i32, correct_guesses: i32) -> bool {
    return guess == correct_guesses
}
fn lower_than_guess(guess: i32, correct_guesses: i32) -> bool {
    return guess < correct_guesses
}
fn higher_than_guess(guess: i32, correct_guesses: i32) -> bool {
    return guess > correct_guesses
}

fn main() {
    let correct_guesses = rand::thread_rng().gen_range(1..=100);
    let max_attempts = 5;
    let mut attempts = 0;
    let mut guessed_correctly = false;
    let mut guess = String::new();

    while guessed_correctly == false && attempts < max_attempts {
        println!("Please input your guess(1~100) : ");
        std::io::stdin().read_line(&mut guess).expect("Failed to read line");
        let input_guess = guess.trim().parse::<i32>().expect("Please type a number!");
        let _check = match   is_correct_guess(input_guess, correct_guesses) {
            true => {
                guessed_correctly = true;
                println!("You guessed the number correctly!");
                break;
            },
            false => {
                attempts += 1;
                if lower_than_guess(input_guess, correct_guesses) {
                    println!("Your guess is too low.");
                } else if higher_than_guess(input_guess, correct_guesses) {
                    println!("Your guess is too high.");
                }
                println!("Incorrect guess. You have {} attempts left.", max_attempts - attempts);
                // 입력이 남으니, 초기화
                guess.clear();
            }
        };
    }
    if !guessed_correctly {
        println!("You failed to guess the number. The correct number was {}", correct_guesses);
    }
}
```