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
            match i {
                Charstate::Correct => count += 1,
                _ => {}
            }
        }
        if count == self.correctness.len() {
            return Guessstate::Correct;
        } else {
            return Guessstate::False;
        }
    }
}
pub enum Guessstate {
    Correct,
    False,
}

pub fn check_word(word1: &Vec<u8>, word2: &Vec<u8>) -> Result<Vec<Charstate>, &'static str> {
    if word1.len() != word2.len() {
        return Err("Provided words with different lenghts!");
    }
    let mut res: Vec<Charstate> = Vec::new();
    for i in 0..word1.len() {
        if word1[i] == word2[i] {
            res.push(Charstate::Correct);
        } else {
            let mut counter = 0;
            for j in 0..word2.len() {
                if word1[i] == word2[j] {
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
