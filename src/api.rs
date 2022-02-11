#[derive(Debug, Clone)]
pub enum Charstate {
    None,
    Some,
    Correct,
}

pub enum GameState {
    Won,
    Lost,
}

pub struct Guess {
    pub raw: String,
    pub correctness: Vec<Charstate>,
}

impl Guess {
    pub fn check_correctness(&mut self) -> Guessstate {
        let mut count = 0;
        for i in self.correctness.clone() {
            if let Charstate::Correct = i {
                count += 1
            }
        }
        if count == self.correctness.len() {
            Guessstate::Correct
        } else {
            Guessstate::False
        }
    }
}
pub enum Guessstate {
    Correct,
    False,
}

pub fn check_word(word1: &Vec<u8>, word2: &Vec<u8>) -> Result<Vec<Charstate>, &'static str> {
    if word1.len() != word2.len() {
        return Err("Provided a String with wrong lengths! Try another word!");
    }
    let mut res: Vec<Charstate> = Vec::new();
    for i in 0..word1.len() {
        if word1[i] == word2[i] {
            res.push(Charstate::Correct);
        } else {
            let mut counter = 0;
            for j in word2 {
                if &word1[i] == j {
                    res.push(Charstate::Some);
                    break;
                } else {
                    counter += 1;
                }
            }
            if word2.len() == counter {
                res.push(Charstate::None);
            }
        }
    }

    Ok(res)
}

pub fn get_user_input(msg: &str) -> String {
    let mut input = String::new();
    print!("{msg}");
    std::io::stdin().read_line(&mut input).unwrap();
    input.remove(input.len() - 1);
    input.remove(input.len() - 1);
    input
}

pub fn translate_to_string(results: &Vec<Charstate>) -> String {
    let mut res = String::new();
    for i in results {
        match i {
            Charstate::Correct => res.push('🟩'),
            Charstate::Some => res.push('🟨'),
            Charstate::None => res.push('🟫'),
        }
    }

    res
}

pub fn handle_guess(
    res: Result<Vec<Charstate>, &'static str>,
    correctword: &Vec<u8>,
) -> Vec<Charstate> {
    let guess = match res {
        Ok(guess) => guess,
        Err(e) => {
            println!("{e}");
            let new_guess = get_user_input("Type your Guess!\n").as_bytes().to_vec();
            handle_guess(check_word(&new_guess, correctword), correctword)
        }
    };
    guess
}
