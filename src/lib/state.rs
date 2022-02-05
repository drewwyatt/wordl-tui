struct Position {
  answer: Option<char>,
  guesses: Vec<char>,
}

impl Position {
  pub fn new() -> Self {
    Self {
      answer: None,
      guesses: vec![],
    }
  }

  pub fn is(&self, letter: &char) -> bool {
    return self.answer == Some(*letter);
  }

  pub fn is_not(&self, letter: &char) -> bool {
    return self.guesses.contains(letter);
  }

  pub fn set_answer(&mut self, letter: &char) {
    self.answer = Some(*letter);
  }

  pub fn add_guess(&mut self, letter: &char) {
    if !self.guesses.contains(letter) {
      self.guesses.push(*letter);
    }
  }
}

pub struct Game {
  answer: Vec<char>,
  known_letters: Vec<char>,
  positions: [Position; 5],
}

pub enum Letter {
  Green(char),
  Red(char),
  Yellow(char),
  White(char),
}

impl Game {
  pub fn new(answer: &str) -> Self {
    Self {
      answer: answer.chars().collect(),
      known_letters: vec![],
      positions: [
        Position::new(),
        Position::new(),
        Position::new(),
        Position::new(),
        Position::new(),
      ],
    }
  }

  pub fn check_input(&self, letter: &char, position: usize) -> Letter {
    if self.positions[position].is(letter) {
      return Letter::Green(*letter);
    }

    if self.positions[position].is_not(letter) {
      return Letter::Red(*letter);
    }

    if self.known_letters.contains(letter) {
      return Letter::Yellow(*letter);
    }

    Letter::White(*letter)
  }

  pub fn check_guess(&mut self, letter: &char, position: usize) -> Letter {
    if self.answer[position] == *letter {
      self.positions[position].set_answer(letter);
      if !self.known_letters.contains(letter) {
        self.known_letters.push(*letter);
      }

      return Letter::Green(*letter);
    }

    self.positions[position].add_guess(letter);

    if self.answer.contains(letter) {
      if !self.known_letters.contains(letter) {
        self.known_letters.push(*letter);
      }

      return Letter::Yellow(*letter);
    }

    return Letter::White(*letter);
  }
}
