use rand::Rng;

struct Letters {
    active: bool,
    letters: Vec<String>,
    alphabet: Vec<String>
}

impl Letters {
    fn start(mut self, word: &str) {
        self.active = true;
        let mut next_char: char;
        println!("start! {}", self.active);
        for i in word.chars() {
            if i == ' ' {
                next_char = i;
            } else {
                next_char = ' ';
            }
            self.letters.push(Letter {

            })
            println!("letter: {}", next_char);
        } 
    }
}

struct Letter {
    char: char,
    next: char,
    speed: u32,
    total: u32,
    duration: u32,
    animating: bool,
    isDead: bool
}

impl Letter {
    fn animate(self) {
    }
}

fn generateLetter(next: char) -> Letter {

    let mut rng = rand::thread_rng();

    let letter = Letter {
        char: ' ',
        next: next,          
        speed: rng.gen(),
        total: 0,
        duration: 2000,
        animating: true,
        isDead: false
    }

    letter.animate();
    return letter;
}



fn main() {
    let letters = Letters {
        active: true,
        letters: vec![String::from("A")],
        alphabet: vec![String::from("A"), String::from("B"), String::from("C")]
    };
    letters.start("C");
    // println!("active: {}", letters.active);
    // println!("letters: {}", letters.letters[0]);
    // println!("alphabet: {}", letters.alphabet[0]);
}
