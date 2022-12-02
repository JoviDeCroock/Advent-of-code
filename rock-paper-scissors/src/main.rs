use std::fs;

// TODO: refactor enums...
const POINTS_FOR_ROCK: i32 = 1;
const POINTS_FOR_PAPER: i32 = 2;
const POINTS_FOR_SCISSORS: i32 = 3;

const POINTS_FOR_LOSS: i32 = 0;
const POINTS_FOR_DRAW: i32 = 3;
const POINTS_FOR_WIN: i32 = 6;

fn get_move(input: &char) -> String {
    match input {
        'A' | 'X' => String::from("Rock"),
        'B' | 'Y' => String::from("Paper"),
        'C' | 'Z' => String::from("Scissors"),
        _ => String::from("None")
    }
}

fn get_score_for_move(input: &str) -> i32 {
    match input {
        "Rock" => POINTS_FOR_ROCK,
        "Paper" => POINTS_FOR_PAPER,
        "Scissors" => POINTS_FOR_SCISSORS,
        _ => 0
    }
}

fn get_expected_outcome_for_game(input: &char) -> String {
    match input {
        'X' => String::from("Loss"),
        'Y' => String::from("Draw"),
        'Z' => String::from("Win"),
        _ => String::from("None")
    }
}

fn get_score_for_outcome(input: &str) -> i32 {
    match input {
        "Loss" => POINTS_FOR_LOSS,
        "Draw" => POINTS_FOR_DRAW,
        "Win" => POINTS_FOR_WIN,
        _ => 0
    }
}

fn get_score_for_game(my_move: &str, opponent_move: &str) -> i32 {
    match opponent_move {
        "Rock" => {
            match my_move {
                "Rock" => POINTS_FOR_DRAW,
                "Paper" => POINTS_FOR_WIN,
                "Scissors" => POINTS_FOR_LOSS,
                _ => 0,
            }
        },
        "Paper" => {
            match my_move {
                "Rock" => POINTS_FOR_LOSS,
                "Paper" => POINTS_FOR_DRAW,
                "Scissors" => POINTS_FOR_WIN,
                _ => 0,
            }
        },
        "Scissors" => {
            match my_move {
                "Rock" => POINTS_FOR_WIN,
                "Paper" => POINTS_FOR_LOSS,
                "Scissors" => POINTS_FOR_DRAW,
                _ => 0,
            }
        },
        _ => 0
    }
}

fn get_move_for_outcome(outcome: &str, opponent_move: &str) -> String {
    match opponent_move {
        "Rock" => {
            match outcome {
                "Draw" => String::from("Rock"),
                "Win" => String::from("Paper"),
                "Loss" => String::from("Scissors"),
                _ => String::from("None"),
            }
        },
        "Paper" => {
            match outcome {
                "Draw" => String::from("Paper"),
                "Win" => String::from("Scissors"),
                "Loss" => String::from("Rock"),
                _ => String::from("None"),
            }
        },
        "Scissors" => {
            match outcome {
                "Draw" => String::from("Scissors"),
                "Win" => String::from("Rock"),
                "Loss" => String::from("Paper"),
                _ => String::from("None"),
            }
        },
        _ => String::from("None"),
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

        let opponent_move = get_move(game.get(0).unwrap());
        let my_move = get_move(game.get(2).unwrap());

        let total_score = get_score_for_move(&my_move) + get_score_for_game(&my_move, &opponent_move);

        score = score + total_score;
    });

    println!("Final score {:?}", score);

    // Second column means what the outcome should be
    let mut score = 0;
    move_matrix.iter().for_each(|item| {
        let game = item.chars().collect::<Vec<char>>();

        let opponent_move = get_move(game.get(0).unwrap());
        let expected_outcome = get_expected_outcome_for_game(game.get(2).unwrap());

        let total_score = get_score_for_outcome(&expected_outcome) + get_score_for_move(&get_move_for_outcome(&expected_outcome, &opponent_move));

        score = score + total_score;
    });

    println!("Final score {:?}", score);
}
