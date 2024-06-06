use console::Term;

#[derive(Debug, thiserror::Error)]
enum GuesserError {
    #[error("There was an error with the console crate?")]
    ConsoleError(#[from] std::io::Error),

    #[error("Unexpected Keypress: {0}")]
    UnexpectedKey(char),
}

enum Comp {
    TooHigh,
    TooLow,
    JustRight,
}

impl Comp {
    fn from_char(character: char) -> Result<Comp, GuesserError> {
        match character {
            'h' => Ok(Comp::TooHigh),
            'l' => Ok(Comp::TooLow),
            '=' => Ok(Comp::JustRight),
            e => Err(GuesserError::UnexpectedKey(e)),
        }
    }
}

trait GuessState {}
struct NoGuess {}
struct HasGuess {}
impl GuessState for HasGuess {}
impl GuessState for NoGuess {}

struct Bot<G: GuessState> {
    marker: std::marker::PhantomData<G>,
    guess_range: (i32, i32),
    guess: Option<i32>,
}

impl<G: GuessState> Bot<G> {
    fn new(lower: i32, higher: i32) -> Self {
        Self {
            marker: std::marker::PhantomData,
            guess_range: (lower, higher),
            guess: None,
        }
    }
}

impl Bot<NoGuess> {
    fn do_guess(&self) -> Bot<HasGuess> {
        let guess = (self.guess_range.0 + self.guess_range.1) / 2;
        Bot::<HasGuess> {
            marker: std::marker::PhantomData,
            guess_range: self.guess_range,
            guess: Some(guess),
        }
    }
}

impl Bot<HasGuess> {
    #[must_use]
    fn check_guess(&self, comp: Comp) -> Bot<NoGuess> {
        let mut guess_range = self.guess_range;
        match comp {
            Comp::TooHigh => guess_range.0 = self.guess.unwrap(),
            Comp::TooLow => guess_range.1 = self.guess.unwrap(),
            Comp::JustRight => todo!(),
        }

        Bot::<NoGuess> {
            marker: std::marker::PhantomData,
            guess_range,
            guess: None,
        }
    }

    fn get_guess(&self) -> i32 {
        self.guess.unwrap()
    }
}

fn main() {
    let term = Term::stdout();
    let mut bot = Bot::<NoGuess>::new(0, 100);

    println!("You're thinking of a number between 0 and 100");

    loop {
        let n_bot = bot.do_guess();
        println!("{}", n_bot.get_guess());
        let i_read_a_char = term.read_char().expect("Failed to read char");
        let comp = Comp::from_char(i_read_a_char).expect("Unexpected Character");
        bot = n_bot.check_guess(comp);
    }
}
