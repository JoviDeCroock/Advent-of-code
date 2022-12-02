use std::fs;

use hand::{Hand, Result, Beats};

mod hand;

const POINTS_FOR_ROCK: i32 = 1;
const POINTS_FOR_PAPER: i32 = 2;
const POINTS_FOR_SCISSORS: i32 = 3;

const POINTS_FOR_LOSS: i32 = 0;
const POINTS_FOR_DRAW: i32 = 3;
const POINTS_FOR_WIN: i32 = 6;

fn get_move(input: &char) -> Option<Hand> {
    match input {
        'A' | 'X' => Some(Hand::Rock),
        'B' | 'Y' => Some(Hand::Paper),
        'C' | 'Z' => Some(Hand::Scissors),
        _ => None
    }
}

fn get_score_for_move(input: &Hand) -> i32 {
    match input {
        Hand::Rock => POINTS_FOR_ROCK,
        Hand::Paper => POINTS_FOR_PAPER,
        Hand::Scissors => POINTS_FOR_SCISSORS,
    }
}

fn get_expected_outcome_for_game(input: &char) -> Option<Result> {
    match input {
        'X' => Some(Result::Lose),
        'Y' => Some(Result::Draw),
        'Z' => Some(Result::Win),
        _ => None
    }
}

fn get_score_for_outcome(input: &Result) -> i32 {
    match input {
        Result::Lose => POINTS_FOR_LOSS,
        Result::Draw => POINTS_FOR_DRAW,
        Result::Win => POINTS_FOR_WIN,
    }
}

fn get_outcome_for_game(my_move: &Hand, opponent_move: &Hand) -> Result {
    let beaten_move = Beats::beats(my_move);
    if my_move == opponent_move {
        Result::Draw
    } else if opponent_move == &beaten_move {
        Result::Win
    } else {
        Result::Lose
    }
}

fn get_move_for_outcome(outcome: &Result, opponent_move: &Hand) -> Option<Hand> {
    match opponent_move {
        Hand::Rock => {
            match outcome {
                Result::Draw => Some(Hand::Rock),
                Result::Win => Some(Hand::Paper),
                Result::Lose => Some(Hand::Scissors),
            }
        },
        Hand::Paper => {
            match outcome {
                Result::Draw => Some(Hand::Paper),
                Result::Win => Some(Hand::Scissors),
                Result::Lose => Some(Hand::Rock),
            }
        },
        Hand::Scissors => {
            match outcome {
                Result::Draw => Some(Hand::Scissors),
                Result::Win => Some(Hand::Rock),
                Result::Lose => Some(Hand::Paper),
            }
        },
    }
}

fn main() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    let move_matrix = contents.split("\n").collect::<Vec<&str>>();

    // Second column is your move
    let mut score = 0;
    move_matrix.iter().for_each(|item| {
        let game = item.chars().collect::<Vec<char>>();

        let opponent_move = get_move(game.get(0).unwrap()).unwrap();
        let my_move = get_move(game.get(2).unwrap()).unwrap();

        let outcome = get_outcome_for_game(&my_move, &opponent_move);
        let total_score = get_score_for_move(&my_move) + get_score_for_outcome(&outcome);

        score = score + total_score;
    });

    println!("Final score {:?}", score);

    // Second column means what the outcome should be
    let mut score = 0;
    move_matrix.iter().for_each(|item| {
        let game = item.chars().collect::<Vec<char>>();

        let opponent_move = get_move(game.get(0).unwrap()).unwrap();
        let expected_outcome = get_expected_outcome_for_game(game.get(2).unwrap()).unwrap();

        let total_score = get_score_for_outcome(&expected_outcome) + get_score_for_move(&get_move_for_outcome(&expected_outcome, &opponent_move).unwrap());

        score = score + total_score;
    });

    println!("Final score {:?}", score);
}
