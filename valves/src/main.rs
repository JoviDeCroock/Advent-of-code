// use std::fs;
// use std::collections::HashMap;

// fn main() {
//     let input = fs::read_to_string("./src/input.txt")
//         .expect("Could not read input");
//     let lines = input.split_terminator("\n");

//     const MAX_TIME: usize = 30;

//     type Valve<'a> = (usize, Vec<&'a str>);
//     // let mut sensors: Vec<(Point, Point, isize)> = Vec::new();
//     let mut valves: HashMap<&str, Valve> = HashMap::new();

//     // Build valves network
//     for line in lines {
//         let sides: Vec<&str> = line.split_terminator("valve").collect();
//         let terms: Vec<&str> = sides[0].split_terminator(' ').collect();
//         let valve_name: &str = terms[1];
//         let flow_rate: usize = terms[4].split_terminator(['=', ';']).collect::<Vec<&str>>()[1].parse().unwrap();
//         let leads_to: Vec<&str> = sides[1].trim_start_matches("s ").trim().split_terminator(", ").collect();
//         valves.insert(valve_name, (flow_rate, leads_to));
//     }

//     type Choice<'a> = (u32, usize, &'a str, HashMap<&'a str, bool>);
//     // type Choice<'a> = (u32, usize, &'a str);
//     let mut choices: HashMap<(usize, &str), Choice> = HashMap::new();
//     let first_choice = (0, 0, "AA", HashMap::new());
//     choices.insert((0, "AA"), first_choice);

//     // Simulate all possible choices
//     for minute in 0..MAX_TIME {
//         println!("choices {}", choices.len());
//         let mut new_choices = HashMap::new();
//         for (_, choice) in &choices {
//             let valid = &valves.get(choice.2).unwrap().1;
//             // Start building new states
//             let new_time = choice.0 + 1;
//             let mut new_release = choice.1;
//             // Count up new released pressure
//             // for (v, _) in &choice.3 {
//             //     new_release += valves.get(v).unwrap().0;
//             // }
//             // Travel down a path
//             for v in valid {
//                 let new_valve = *v; // Valve we moved to.
//                 let new_state = choice.3.clone(); // No new valves were opened.
//                 // let new_choice = (new_time, new_release, new_valve);
//                 let new_choice = (new_time, new_release, new_valve, new_state);
//                 new_choices.insert((new_release, new_valve), new_choice);
//             }
//             // Open current valve
//             let new_valve = choice.2; // Valve we are at.
//             if !choice.3.contains_key(new_valve) {
//                 new_release += &valves.get(choice.2).unwrap().0 * (MAX_TIME - minute - 1);
//                 let mut new_state = choice.3.clone(); // Open the valve.
//                 new_state.insert(new_valve, true);
//                 let new_choice = (new_time, new_release, new_valve, new_state);
//                 // let new_choice = (new_time, new_release, new_valve);
//                 // new_choices.insert(new_choice);
//                 new_choices.insert((new_release, new_valve), new_choice);
//             }
//         }
//         choices = new_choices;
//     }

//     let mut m = 0;
//     for (_, choice) in choices {
//         if choice.1 > m {
//             m = choice.1;
//         }
//     }


//     // println!("X: {}", found_x);
//     // println!("Y: {}", found_y);
//     println!("Part 1: {}", m);
// }

use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::Chain;
use std::slice::Iter;

fn main() {
    let input = fs::read_to_string("./src/input.txt")
        .expect("Could not read input");
    let lines = input.split_terminator("\n");

    const MAX_TIME: usize = 26;

    type Valve<'a> = (usize, Vec<&'a str>);
    // let mut sensors: Vec<(Point, Point, isize)> = Vec::new();
    let mut valves: HashMap<&str, Valve> = HashMap::new();

    // Build valves network
    for line in lines {
        let sides: Vec<&str> = line.split_terminator("valve").collect();
        let terms: Vec<&str> = sides[0].split_terminator(' ').collect();
        let valve_name: &str = terms[1];
        let flow_rate: usize = terms[4].split_terminator(['=', ';']).collect::<Vec<&str>>()[1].parse().unwrap();
        let leads_to: Vec<&str> = sides[1].trim_start_matches("s ").trim().split_terminator(", ").collect();
        valves.insert(valve_name, (flow_rate, leads_to));
    }

    type Choice<'a> = (usize, &'a str, HashSet<&'a str>, &'a str);
    // type Choice<'a> = (u32, usize, &'a str);
    let mut choices: HashMap<(usize, &str, &str), Choice> = HashMap::new();
    let first_choice = (0, "AA", HashSet::new(), "AA");
    choices.insert((0, "AA", "AA"), first_choice);

    const TIME_DELTA: usize = 1;
    const OFFSET: usize = 0;

    // Simulate all possible choices
    'outer: for minute in OFFSET..MAX_TIME + OFFSET {
        println!("{} - choices {}", minute, choices.len());
        let mut new_choices = HashMap::new();
        for (_, choice) in &choices {
            let i_stay = [choice.1];
            let e_stay = [choice.3];
            let valid: Chain<Iter<_>, Iter<_>> = valves.get(choice.1).unwrap().1.iter().chain(i_stay.iter());
            let valid_e: Chain<Iter<_>, Iter<_>> = valves.get(choice.3).unwrap().1.iter().chain(e_stay.iter());
            // We have opened everything.
            if choice.2.len() == valves.len() {
                println!("Skipped");
                break 'outer;
            }
            // Travel down a path
            for v in valid {
                let this_valid_e = valid_e.clone();
                for e in this_valid_e {
                    // Start building new states
                    let mut new_release = choice.0;
                    let new_valve = *v; // Valve we moved to.
                    let new_e_valve = *e; // Elephant moved to.
                    let mut new_state = choice.2.clone(); // No new valves were opened (yet).
                    let new_choice;
                    // BOTH OPEN VALVE
                    if new_e_valve == choice.3 && new_valve == choice.1 {
                        // Open current valve
                        if !new_state.contains(new_valve) || !choice.2.contains(new_e_valve) {
                            new_release = choice.0;
                            if !new_state.contains(new_valve) {
                                new_release += &valves.get(new_valve).unwrap().0 * (MAX_TIME - minute - TIME_DELTA);
                                new_state.insert(new_valve);
                            }
                            if !new_state.contains(new_e_valve) {
                                new_release += &valves.get(new_e_valve).unwrap().0 * (MAX_TIME - minute - TIME_DELTA);
                                new_state.insert(new_e_valve);
                            }
                        }
                    }
                    // I OPEN VALVE
                    else if new_valve == choice.1 {
                        // Open current valve
                        new_release = choice.0;
                        if !new_state.contains(new_valve) {
                            new_release += &valves.get(new_valve).unwrap().0 * (MAX_TIME - minute - TIME_DELTA);
                            new_state.insert(new_valve);
                        }
                    }
                    // ELEPHANT OPENS VALVE
                    else if new_e_valve == choice.3 {
                        // Open current valve
                        new_release = choice.0;
                        if !new_state.contains(new_e_valve) {
                            new_release += &valves.get(new_e_valve).unwrap().0 * (MAX_TIME - minute - TIME_DELTA);
                            new_state.insert(new_e_valve);
                        }
                    }
                    new_choice = (new_release, new_valve, new_state, new_e_valve);
                    // Me vs El in a position is equivalent.
                    if !new_choices.contains_key(&(new_release, new_valve, new_e_valve)) && !new_choices.contains_key(&(new_release, new_e_valve, new_valve)) {
                    new_choices.insert((new_release, new_valve, new_e_valve), new_choice);
                    }
                }
            }
        }
        choices = new_choices;
    }

    let mut m = 0;
    for (_, choice) in choices {
        if choice.0 > m {
            m = choice.0;
        }
    }


    // println!("X: {}", found_x);
    // println!("Y: {}", found_y);
    println!("Part 2: {}", m);
}

