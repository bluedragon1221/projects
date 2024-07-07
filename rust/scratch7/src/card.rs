#[derive(Debug)]
pub struct Card {
    value: i32,
}

#[derive(Debug)]
pub enum CardError {
    OutOfBounds(i32),
    ParseError(String),
}

impl Card {
    pub fn new(num: i32) -> Result<Self, CardError> {
        if num < 1 || num > 13 {
            Err(CardError::OutOfBounds(num))
        } else {
            Ok(Self { value: num })
        }
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }

    pub fn from_str(inp: &str) -> Result<Self, CardError> {
        if let Ok(num) = inp.parse::<i32>() {
            Ok(Card::new(num)?)
        } else {
            match inp {
                "J" => Ok(Self { value: 11 }),
                "Q" => Ok(Self { value: 12 }),
                "K" => Ok(Self { value: 13 }),
                e => Err(CardError::ParseError(e.into())),
            }
        }
    }

    fn to_str(&self) -> String {
        match self.value {
            1..=10 => self.value.to_string(),
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            _ => unreachable!(),
        }
    }

    pub fn fmt(&self) -> String {
        format!("[{}]", self.to_str())
    }
}
