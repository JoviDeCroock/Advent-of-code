use std::collections::HashMap;

use nom::{
    IResult,
    sequence::{separated_pair, terminated, preceded},
    character::complete::{digit1, multispace0, alphanumeric0},
    combinator::map_res,
    bytes::complete::tag, multi::separated_list1,
};

#[derive(Debug)]
struct Game<'a> {
    pub id: u32,
    pub rolls: Vec<Vec<(u32, &'a str)>>,
}

fn parse_dice(input: &str) -> IResult<&str, Vec<(u32, &str)>> {
    let (input, dice) = separated_list1(
        tag(", "),
        separated_pair(
            map_res(digit1, |s: &str| s.parse::<u32>()),
            multispace0,
            alphanumeric0,
        ),
    )(input)?;

    Ok((input, dice))
}

fn parse_game<'a>(input: &'a str) -> IResult<&str, Game<'a>> {
    let (input, id) = preceded(
        multispace0,
        terminated(
            preceded(
                tag("Game"),
                preceded(multispace0, map_res(digit1, |s: &str| s.parse::<u32>())),
            ),
            tag(": "),
        ),
    )(input)?;

    let (input, rolls) = separated_list1(
        tag("; "),
        parse_dice
    )(input)?;

    Ok((input, Game { id, rolls }))
}

fn main() {
    let input = include_str!("../input.txt");

    let mut amount_of_dice = HashMap::new();
    amount_of_dice.insert("red", 12);
    amount_of_dice.insert("green", 13);
    amount_of_dice.insert("blue", 14);

    let mut total_p1 = 0;
    let mut total_p2 = 0;
    input.split("\n").for_each(|line| {
        let game = parse_game(line);
        let mut min_required_red = 0;
        let mut min_required_blue = 0;
        let mut min_required_green = 0;
        match game {
            Ok((_, game)) => {
                let mut failed_game = false;
                println!("Game {:?}", game);
                for roll in game.rolls {
                    for roll in roll.iter() {
                        let max_amount = amount_of_dice.get_mut(roll.1).unwrap();

                        match roll.1 {
                            "red" => {
                                if roll.0 > min_required_red {
                                    min_required_red = roll.0;
                                }
                            },
                            "blue" => {
                                if roll.0 > min_required_blue {
                                    min_required_blue = roll.0;
                                }
                            },
                            "green" => {
                                if roll.0 > min_required_green {
                                    min_required_green = roll.0;
                                }
                            },
                            _ => {}
                        }

                        if roll.0 > *max_amount {
                            failed_game = true;
                        }
                    }
                }

                total_p2 += min_required_red * min_required_blue * min_required_green;
                if !failed_game {
                    total_p1 += game.id;
                }
            },
            _ => {
                println!("failed to parse line {:?}", line);
            }
        }
    });

    println!("Part 1: {}", total_p1);
    println!("Part 2: {}", total_p2);
}