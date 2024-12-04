use std::fs::File;
use std::io;
use std::io::{BufRead, Read};
use std::str::FromStr;

/// Represents a Mul instruction
struct Mul {
    pub left: i64,
    pub right: i64,
}

impl Mul {
    pub fn from_ops(op1: String, op2: String) -> Self {
        Self {
            left: i64::from_str(&op1).unwrap(),
            right: i64::from_str(&op2).unwrap(),
        }
    }
}

/// Represents the building-up of a token
enum State {
    Empty,
    M,  // M
    Mu,  // Mu
    Mul,  // Mul
    MulParen,  // Matches mul(
    Op1(String),  // Matches mul(<number> and stores the number
    Comma(String), // Matches mul(<number>,
    Op2(String, String),  // Matches mul(<n1>,<n2> storing both numbers
    // There is no Finished state because as soon as we find ) we either discard or save token
}

impl Default for State {
    fn default() -> Self {
        Self::Empty
    }
}

/// Read input
fn read_input() -> String {
    let mut file = File::open("inputs/day3.txt").unwrap();

    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    s
}

/// Match a constant. If failed, reset state.
fn match_constant(state: &mut State, ch: &char, should_be: &char, transition_to: State) {
    if ch.eq(should_be) {
        *state = transition_to;
    } else {
        *state = State::Empty;
    }
}

/// Parse string and return all the mul operations
///
/// Iteratively read characters, building up a Mul token.
/// Pop the mul token onto the output vec if it's complete.
/// If an invalid token is found, discards current state.
fn parse(input: &str) -> Vec<Mul> {
    let mut out: Vec<Mul> = vec![];
    let mut state = State::Empty;
    // Taking it char-by-char
    for ch in input.chars() {
        // Need to use take because of borrowing rules
        match std::mem::take(&mut state) {
            // Just matching constants...
            State::Empty => match_constant(&mut state, &ch, &'m', State::M),
            State::M => match_constant(&mut state, &ch, &'u', State::Mu),
            State::Mu => match_constant(&mut state, &ch, &'l', State::Mul),
            State::Mul => match_constant(&mut state, &ch, &'(', State::MulParen ),
            // Starting to get interesting
            State::MulParen => {
                if ch.is_numeric() {
                    state = State::Op1(ch.to_string())
                } else {
                    state = State::Empty;
                }
            }
            State::Op1(mut op1) => {
                // If this char is also a number, just keep adding to it
                if ch.is_numeric() {
                    op1.push(ch);
                    state = State::Op1(op1);
                } else {
                    // Otherwise, it should be a ",".
                    // If it's neither a number nor ",", discard state
                    match_constant(&mut state, &ch, &',', State::Comma(op1));
                }
            },
            State::Comma(op1) => {
                // Here, we also check for a number or quit
                if ch.is_numeric() {
                    state = State::Op2(op1, ch.to_string());
                } else {
                    state = State::Empty;
                }
            },
            State::Op2(op1, mut op2) => {
                // Same as for op1, if this char is also a number, just keep adding to it
                if ch.is_numeric() {
                    op2.push(ch);
                    state = State::Op2(op1, op2);
                } else if ch.eq(&')') {
                    // Unlike op1, if the next thing is a ) we can pop this token and start anew
                    out.push(Mul::from_ops(op1, op2));
                    state = State::Empty;
                } else {
                    state = State::Empty;
                }
            }
        }
    }
    out
}

fn get_answer(m: &[Mul]) -> i64 {
    m.iter().map(|mul| mul.left * mul.right).sum()
}

fn main() {
    let input = read_input();
    let result = parse(&input);
    let result = get_answer(&result);
    println!("{result}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let muls = parse("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(get_answer(&muls), 161);
    }
}
