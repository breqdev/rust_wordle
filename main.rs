use colored::Colorize;

type Word = [char; 5];

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Color {
  White,
  Gray,
  Yellow,
  Green,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Square {
  color: Color,
  letter: char,
}

type Row = [Square; 5];

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

fn print_colored(c: &Color, s: &str) {
  match c {
    Color::White => print!("{}", s.on_white()),
    Color::Gray => print!("{}", s.on_truecolor(127, 127, 127)),
    Color::Yellow => print!("{}", s.on_yellow()),
    Color::Green => print!("{}", s.on_green()),
  }
}

fn print_row(word: &Row) {
  for square in word.iter() {
    print_colored(&square.color, "┌───┐");
    print!("  ");
  }
  println!("");

  for square in word.iter() {
    let mut boxed = "│ ".to_owned();
    boxed.push_str(&square.letter.to_string());
    boxed.push_str(" │");

    print_colored(&square.color, &boxed);
    print!("  ");
  }
  println!("");

  for square in word.iter() {
    print_colored(&square.color, "└───┘");
    print!("  ");
  }
  println!("");
}

fn main() {
  let target = ['A', 'R', 'R', 'A', 'Y'];
  let guess = ['F', 'U', 'R', 'R', 'Y'];

  let scored = score_guess(&target, &guess);

  print_row(&scored);
}
