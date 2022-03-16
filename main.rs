use colored::Colorize;
use rand::seq::SliceRandom;
use serde_json::Value;
use std::fs;
use std::io;
use std::io::Write;

type Word = [char; 5];

trait AsWord {
  fn as_word(&self) -> Result<Word, ()>;
}

impl AsWord for String {
  fn as_word(&self) -> Result<Word, ()> {
    let mut result = [0 as char; 5];

    if self.len() != 5 {
      return Err(());
    }

    self
      .chars()
      .zip(result.iter_mut())
      .for_each(|(c, square)| *square = c);

    Ok(result)
  }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Color {
  White,
  Gray,
  Yellow,
  Green,
}

fn print_colored(c: &Color, s: &str) {
  match c {
    Color::White => print!("{}", s.on_white()),
    Color::Gray => print!("{}", s.on_truecolor(127, 127, 127)),
    Color::Yellow => print!("{}", s.on_yellow()),
    Color::Green => print!("{}", s.on_green()),
  }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Square {
  color: Color,
  letter: char,
}

type Row = [Square; 5];

trait PrintWordle {
  fn print_wordle(&self);
}

impl PrintWordle for Row {
  fn print_wordle(&self) {
    for square in self.iter() {
      print_colored(&square.color, "┌───┐");
      print!("  ");
    }
    println!("");

    for square in self.iter() {
      let mut boxed = "│ ".to_owned();
      boxed.push_str(&square.letter.to_string());
      boxed.push_str(" │");

      print_colored(&square.color, &boxed);
      print!("  ");
    }
    println!("");

    for square in self.iter() {
      print_colored(&square.color, "└───┘");
      print!("  ");
    }
    println!("");
  }
}

fn score_guess(target: &Word, guess: &Word) -> Row {
  let mut remaining = target.map(|c| Some(c));

  let mut result = guess.map(|letter| Square {
    color: Color::White,
    letter,
  });

  for (i, square) in result.iter_mut().enumerate() {
    if target[i] == guess[i] {
      square.color = Color::Green;
      remaining[i] = None;
    }
  }

  for (i, square) in result.iter_mut().enumerate() {
    if square.color == Color::White {
      if let Some(pos) = remaining.iter().position(|&c| c == Some(guess[i])) {
        square.color = Color::Yellow;
        remaining[pos] = None;
      }
    }
  }

  for square in result.iter_mut() {
    if square.color == Color::White {
      square.color = Color::Gray;
    }
  }

  result
}

fn load_wordlist(path: &str) -> Vec<String> {
  let file = fs::read_to_string(path).expect("Could not open wordlist");

  let data: Value = serde_json::from_str(&file).expect("Could not parse wordlist");

  let words: Vec<String> = data
    .as_array()
    .expect("Wordlist is not an array")
    .iter()
    .map(|word| word.as_str().expect("Word is not a string").to_uppercase())
    .collect();

  words
}

fn prompt() -> String {
  print!("{}", "> ");
  io::stdout().flush().unwrap();

  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Could not read line");

  input.trim().to_uppercase()
}

fn main() {
  let allowed = load_wordlist("allowlist.json");
  let targetlist = load_wordlist("targetlist.json");

  let mut target: Word;
  let mut guess: Word;

  let mut input: String;

  loop {
    println!("Welcome to WORDLE! Enter a five letter guess...");

    target = targetlist
      .choose(&mut rand::thread_rng())
      .unwrap()
      .clone()
      .as_word()
      .expect("Wordlist yielded word of invalid length");

    loop {
      input = prompt();

      if input.len() != 5 {
        println!("Please enter a five letter guess.");
        continue;
      }

      if input.chars().any(|c| !c.is_ascii_alphabetic()) {
        println!("Please enter only US English letters.");
        continue;
      }

      if !allowed.contains(&input) {
        println!("Not in wordlist.");
        continue;
      }

      guess = input.as_word().unwrap();

      let scored = score_guess(&target, &guess);
      scored.print_wordle();

      if guess == target {
        println!("You win!");
        break;
      }
    }

    println!("Play again? (y/n)");

    input = prompt();

    if input.to_lowercase() == "n" {
      break;
    }
  }
}
