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
