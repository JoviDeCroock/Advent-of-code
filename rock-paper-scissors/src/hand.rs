#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Hand {
  Rock,
  Paper,
  Scissors,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Result {
    Win,
    Lose,
    Draw,
}

pub trait Beats {
  fn beats(&self) -> Self;
}

impl Beats for Hand {
  fn beats(&self) -> Self {
    match *self {
      Hand::Rock => Hand::Scissors,
      Hand::Paper => Hand::Rock,
      Hand::Scissors => Hand::Paper,
    }
  }
}

