#[macro_use]
extern crate seed;
use seed::prelude::*;
use std::i32;

fn genereate_random_number() -> i32 {
    let secret_number = (js_sys::Math::random() * 100.0) as i32;

    secret_number
}

// Model

struct SecretNumber {
    pub secret_number: i32,
    pub text: String,
    pub guess: i32
}

impl Default for SecretNumber {
    fn default() -> Self {
        SecretNumber {
            secret_number: genereate_random_number(),
            text: String::from("You haven't made a guess yet!"),
            guess: 0
        }
    }
}


// Update

#[derive(Clone)]
enum Msg {
    GenerateNumber,
    SetGuess(String),
    MakeGuess
}


fn update(msg: Msg, model: &mut SecretNumber, _: &mut Orders<Msg>) {
    let secret_number = genereate_random_number();

    match msg {
        Msg::GenerateNumber => {
            model.text = String::from("You haven't made a guess yet!");
            model.secret_number = secret_number
        },
        Msg::SetGuess(val) => model.guess = match val.trim().parse::<i32>() {
            Ok(val) => {
                if val > 100 {
                    100
                } else {
                    val
                }
            },
            Err(_) => {
                // model.text = String::from("ERROR! Bad Input!");
                0
            },
        },
        Msg::MakeGuess => {
            if model.guess < model.secret_number {
                model.text = String::from("Too Low!")
            } else if model.guess > model.secret_number {
                model.text = String::from("Too High!")
            } else {
                model.text = String::from("You guessed the number!")
            }
        }
    }
}


// View

fn view(model: &SecretNumber) -> El<Msg> {
    div![
        h3!["Enter a number between 1 and 100!"],
        input![
            input_ev(Ev::Input, |new_value| Msg::SetGuess(new_value)),
            attrs!{At::Type => "number"; At::Min => 1; At::Max => 100; At::Value => {
                    if model.guess == 0 {
                        String::from("")
                    } else {
                        String::from(format!("{}", model.guess))
                    }
                }
            }
        ],
        button![
            simple_ev(Ev::Click, Msg::MakeGuess),
            "GUESS"
        ],
        p![
            model.text
        ],
        hr![],
        button![
            simple_ev(Ev::Click, Msg::GenerateNumber),
            "Generate new secret number"
        ],
    ]
}

#[wasm_bindgen]
pub fn render() {
    seed::App::build(SecretNumber::default(), update, view)
        .finish()
        .run();

}