use std::fs;

use hand::{Hand, Result, Beats};

mod hand;

const POINTS_FOR_ROCK: i32 = 1;
const POINTS_FOR_PAPER: i32 = 2;
const POINTS_FOR_SCISSORS: i32 = 3;

const POINTS_FOR_LOSS: i32 = 0;
const POINTS_FOR_DRAW: i32 = 3;
const POINTS_FOR_WIN: i32 = 6;

fn main() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    let move_matrix = contents.split('\n').collect::<Vec<&str>>();

    // Second column is your move
   let score = move_matrix.iter().fold(0, |acc, item| {
        let game = item.chars().collect::<Vec<char>>();

        let opponent_move = translate_move(game.get(0).unwrap()).unwrap();
        let my_move = translate_move(game.get(2).unwrap()).unwrap();

        let outcome = get_outcome_for_game(&my_move, &opponent_move);
        let total_score = get_score_for_move(&my_move) + get_score_for_outcome(&outcome);

        acc + total_score
    });

    println!("Final score {:?}", score);

    // Second column means what the outcome should be
    let score = move_matrix.iter().fold(0, |acc, item| {
        let game = item.chars().collect::<Vec<char>>();

        let opponent_move = translate_move(game.get(0).unwrap()).unwrap();
        let expected_outcome = translate_expected_outcome(game.get(2).unwrap()).unwrap();

        let total_score = get_score_for_outcome(&expected_outcome) + get_score_for_move(&get_move_for_outcome(&expected_outcome, &opponent_move));

        acc + total_score
    });

    println!("Final score {:?}", score);
}

fn translate_move(input: &char) -> Option<Hand> {
    match input {
        'A' | 'X' => Some(Hand::Rock),
        'B' | 'Y' => Some(Hand::Paper),
        'C' | 'Z' => Some(Hand::Scissors),
        _ => None
    }
}

fn translate_expected_outcome(input: &char) -> Option<Result> {
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

fn get_score_for_move(input: &Hand) -> i32 {
    match input {
        Hand::Rock => POINTS_FOR_ROCK,
        Hand::Paper => POINTS_FOR_PAPER,
        Hand::Scissors => POINTS_FOR_SCISSORS,
    }
}

fn get_outcome_for_game(my_move: &Hand, opponent_move: &Hand) -> Result {
    let own_winning_hand = Beats::beats(my_move);
    let opponent_winning_hand = Beats::beats(opponent_move);

    if my_move == &opponent_winning_hand {
        Result::Lose
    } else if opponent_move == &own_winning_hand {
        Result::Win
    } else {
        Result::Draw
    }
}

fn get_move_for_outcome(outcome: &Result, opponent_move: &Hand) -> Hand {
    let opponent_winning_hand = Beats::beats(opponent_move);
    let own_winning_hand = Beats::beats(&opponent_winning_hand);

    match outcome {
        Result::Lose => opponent_winning_hand,
        Result::Win => own_winning_hand,
        Result::Draw => *opponent_move
    }
}
