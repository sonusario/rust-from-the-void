/*
    const X: &str = "-1";

    let y: Integer<{X.len()}> = Integer::from_str(X).unwrap();
    println!("{:?}", y.string());
*/


use Digit::*;

#[derive(PartialEq)]
pub enum Digit {
    Negative,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Digit {
    fn from_char(c: char) -> Result<Digit, String> {
        match c {
            '-' => Ok(Negative),
            '0' => Ok(Zero),
            '1' => Ok(One),
            '2' => Ok(Two),
            '3' => Ok(Three),
            '4' => Ok(Four),
            '5' => Ok(Five),
            '6' => Ok(Six),
            '7' => Ok(Seven),
            '8' => Ok(Eight),
            '9' => Ok(Nine),
            _ => Err(format!("Expected digit, found '{c}'.")),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Negative => '-',
            Zero => '0',
            One => '1',
            Two => '2',
            Three => '3',
            Four => '4',
            Five => '5',
            Six => '6',
            Seven => '7',
            Eight => '8',
            Nine => '9',
        }
    }
}

pub struct Integer<const SIZE: usize> {
    digits: [Digit; SIZE],
}

impl<const SIZE: usize> Integer<{ SIZE }> {
    fn from(digits: [Digit; SIZE]) -> Result<Integer<{ SIZE }>, &'static str> {
        match &digits[..] { // All this to express -? [1-9] [0-9]* | 0
            [Negative, Zero] => Err("Zero cannot be negative"),
            [Negative] => Err("NULL cannot be negative"),
            [] => Err("No digits"),
            dgts @ [Zero, ..] | dgts @ [Negative, Zero, ..]
                if dgts != [Zero] && dgts != [Negative, Zero] =>
            {
                Err("Leading zero")
            }
            [_, rest @ ..] if rest.iter().fold(false, |_, d| d == &Negative) => {
                Err("Expected digit found '-'.")
            }
            _ => Ok(Integer { digits }),
        }
    }

    pub fn from_str(s: &'static str) -> Result<Integer<{ SIZE }>, &'static str> {
        if s.len() > SIZE {
            return Err("Too many digits");
        }

        Integer::from(
            s.chars()
                .map(Digit::from_char)
                .collect::<Result<Vec<_>, _>>()
                .unwrap()
                .try_into()
                .ok()
                .unwrap(),
        )
    }

    pub fn string(&self) -> String {
        self.digits.iter().map(Digit::to_char).collect()
    }

    //pub fn add<const OTHER_SIZE: usize>(&self, other: Integer<{OTHER_SIZE}>) -> 
}