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
    words: words::Words,
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
            words,
        };
    }
    fn start(&mut self) -> (GameState, usize) {
        for i in 1..self.correctword.len() + 1 {
            let test = api::test_handle_guess(
                &self.words,
                String::from_utf8(self.correctword.clone()).unwrap(),
            );

            self.update_guesses(test.0, test.1);

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
