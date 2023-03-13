use reqwest;
use reqwest::header::COOKIE;

static AOC_SESSION_COOKIE: &str = env!("AOC_SESSION_COOKIE");

pub enum PuzzlePart {
    Part1,
    Part2,
}

pub struct Input {
    pub year: u32,
    pub day: u8,
    pub text: String,
}

impl Input {
    pub fn try_get(year: u32, day: u8) -> Result<Self, reqwest::Error> {
        assert!(day > 0 && day <= 25, "day must be between 1 and 25");

        let client = reqwest::blocking::Client::new();
        let text = client
            .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
            .header(COOKIE, format!("session={AOC_SESSION_COOKIE}"))
            .send()?
            .error_for_status()?
            .text()?;

        Ok(Self { year, day, text })
    }

    // fn parse(FnOnce<>) ->
}

// pub fn submit(year: u32, day: u8, part: u8) -> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_input() {
        let input = Input::try_get(2022, 1).unwrap();
        assert_eq!(input.year, 2022);
        assert_eq!(input.day, 1);
        assert!(input.text.len() > 0);
    }
}
