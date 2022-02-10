use crate::api;
use crate::api::Charstate;
use crate::api::GameState;
use crate::api::Guess;
use crate::api::Guessstate;
use crate::words;
use rand::Rng;
pub struct Game {
    correctword: Vec<u8>,
    guesses: Vec<Guess>,
}

impl Game {
    pub fn new() -> Self {
        let mut words =
            words::Words::new(String::from(r#"words\english_words_original_wordle.txt"#));
        words.sort_words();
        return Self {
            correctword: words.words[rand::thread_rng().gen_range(0..words.words.len())]
                .as_bytes()
                .to_vec(),
            guesses: Vec::new(),
        };
    }
    fn start(&mut self) -> (GameState, usize) {
        for i in 1..self.correctword.len() + 1 {
            let mut guess = api::get_user_input("Type your Guess!\n");
            if guess == "cheat!" {
                println!(
                    "You are cheating! The correct word is {}",
                    String::from_utf8(self.correctword.clone()).unwrap()
                );
                guess = api::get_user_input("");
            }
            let guess_correctness =
                api::check_word(&guess.as_bytes().to_vec(), &self.correctword).unwrap();
            self.update_guesses(guess, guess_correctness);
            println!(
                "{}",
                api::translate_to_string(&self.guesses[i - 1].correctness)
            );
            match self.guesses[i - 1].check_correctness() {
                Guessstate::Correct => return (GameState::Won, i),
                Guessstate::False => {}
            }
        }

        (GameState::Lost, 69)
    }
    fn update_guesses(&mut self, guessstring: String, guesscorrectness: Vec<Charstate>) {
        let guess = Guess {
            raw: guessstring,
            correctness: guesscorrectness,
        };
        self.guesses.push(guess);
    }

    pub fn play(&mut self) {
        let result = self.start();
        match result.0 {
            GameState::Lost => {
                println!(
                    "You lost! The correct Word would have been '{}'! Try again!",
                    String::from_utf8(self.correctword.clone()).unwrap(),
                )
            }
            GameState::Won => {
                println!("You won with {} tries! Play again!", result.1)
            }
        }
    }
}
