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
    return self.guesses.contains(letter)
      || (self.answer.is_some() && self.answer != Some(*letter));
  }

  pub fn set_answer(&mut self, letter: &char) {
    self.answer = Some(*letter);
  }

  pub fn add_guess(&mut self, letter: &char) {
    if !self.is(letter) && !self.guesses.contains(letter) {
      self.guesses.push(*letter);
    }
  }
}

struct Guesses {
  in_word: Vec<char>,
  not_in_word: Vec<char>,
  positions: [Position; 5],
}

impl Guesses {
  pub fn new() -> Self {
    Self {
      in_word: vec![],
      not_in_word: vec![],
      positions: [
        Position::new(),
        Position::new(),
        Position::new(),
        Position::new(),
        Position::new(),
      ],
    }
  }

  pub fn is(&self, letter: &char, position: usize) -> bool {
    self.positions[position].is(letter)
  }

  pub fn is_not(&self, letter: &char, position: usize) -> bool {
    self.positions[position].is_not(letter) || self.not_in_word.contains(letter)
  }

  pub fn might_be(&self, letter: &char) -> bool {
    self.in_word.contains(letter)
  }

  pub fn set_answer(&mut self, letter: &char, position: usize) {
    self.positions[position].set_answer(letter);
    self.add_known_letter(letter, position);
  }

  pub fn add_known_letter(&mut self, letter: &char, position: usize) {
    self.positions[position].add_guess(letter);
    if !self.in_word.contains(letter) {
      self.in_word.push(*letter)
    }
  }

  pub fn add_incorrect_letter(&mut self, letter: &char) {
    if !self.not_in_word.contains(letter) {
      self.not_in_word.push(*letter)
    }
  }
}

pub struct Game {
  answer: Vec<char>,
  guesses: Guesses,
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
      guesses: Guesses::new(),
    }
  }

  pub fn check_input(&self, letter: &char, position: usize) -> Letter {
    if self.guesses.is(letter, position) {
      return Letter::Green(*letter);
    }

    if self.guesses.is_not(letter, position) {
      return Letter::Red(*letter);
    }

    if self.guesses.might_be(letter) {
      return Letter::Yellow(*letter);
    }

    Letter::White(*letter)
  }

  pub fn check_guess(&mut self, letter: &char, position: usize) -> Letter {
    if self.answer[position] == *letter {
      self.guesses.set_answer(letter, position);
      return Letter::Green(*letter);
    }

    if self.answer.contains(letter) {
      self.guesses.add_known_letter(letter, position);
      return Letter::Yellow(*letter);
    }

    self.guesses.add_incorrect_letter(letter);
    return Letter::White(*letter);
  }
}
