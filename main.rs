use colored::Colorize;

#[derive(PartialEq, Eq, Copy, Clone)]
struct Word {
  letters: [char; 5],
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Color {
  White,
  Gray,
  Yellow,
  Green,
}

#[derive(PartialEq, Eq, Copy, Clone)]
struct Square {
  color: Color,
  letter: char,
}

#[derive(PartialEq, Eq, Copy, Clone)]
struct Row {
  squares: [Square; 5],
}

fn score_guess(target: &Word, guess: &Word) -> Row {
  // Set up the guess as an array of Optional
  let mut remaining = target.letters.map(|c| Some(c));

  // Initialize the result
  let mut row = Row {
    squares: guess.letters.map(|letter| Square {
      color: Color::White,
      letter: letter,
    }),
  };

  // Color the green squares
  for (i, square) in row.squares.iter_mut().enumerate() {
    if target.letters[i] == guess.letters[i] {
      square.color = Color::Green;
      remaining[i] = None;
    }
  }

  // Color the yellow squares
  for (i, square) in row.squares.iter_mut().enumerate() {
    if square.color == Color::Green {
      continue;
    }

    let pos = remaining.iter().position(|&c| c == Some(guess.letters[i]));

    if let Some(pos) = pos {
      square.color = Color::Yellow;
      remaining[pos] = None;
    }
  }

  // Color remaining squares gray
  for square in row.squares.iter_mut() {
    if square.color == Color::Green {
      continue;
    }

    if square.color == Color::Yellow {
      continue;
    }

    square.color = Color::Gray;
  }

  row
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
  // Print the top border
  for square in word.squares.iter() {
    print_colored(&square.color, "┌───┐");

    print!("  ");
  }

  println!("");

  // Print the letter
  for square in word.squares.iter() {
    let mut boxed = "│ ".to_owned();
    boxed.push_str(&square.letter.to_string());
    boxed.push_str(" │");

    print_colored(&square.color, &boxed);

    print!("  ");
  }

  println!("");

  // Print the bottom border
  for square in word.squares.iter() {
    print_colored(&square.color, "└───┘");

    print!("  ");
  }

  println!("");
}

fn main() {
  println!("Hello, wordle!");

  let target = Word {
    letters: ['A', 'R', 'R', 'A', 'Y'],
  };

  let guess = Word {
    letters: ['F', 'U', 'R', 'R', 'Y'],
  };

  let scored = score_guess(&target, &guess);

  print_row(&scored);
}
