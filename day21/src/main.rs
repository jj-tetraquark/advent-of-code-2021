use std::env;

struct DeterministicDice {
    value : u32
}

impl DeterministicDice {
    fn roll(&mut self, times: u32) -> u32 {
        (0..times).fold(0, |acc, _| { 
            self.value = self.value % 100 + 1;
            acc + self.value
        })

    }
}

fn part1(mut player1: u32, mut player2: u32) -> u32 {
    let mut dice = DeterministicDice { value: 0 };
    let mut player1_score = 0;
    let mut player2_score = 0;
    let mut dice_rolls = 0;
    loop { 
        player1 = (player1 + dice.roll(3) - 1) % 10 + 1;
        player1_score += player1;
        dice_rolls += 3;
        if player1_score >= 1000 { break; }

        player2 = (player2 + dice.roll(3) - 1) % 10 + 1;
        player2_score += player2;
        dice_rolls += 3;
        if player2_score >= 1000 { break; }
    }

    if player1_score > player2_score {
        return player2_score * dice_rolls;
    } else {
        return player1_score * dice_rolls;
    };

}

// 3 rolls of 3
fn ways_of_getting_dice_value(value: u32) -> u64 {
    match value {
        3 => 1,
        4 => 3,
        5 => 6,
        6 => 7,
        7 => 6,
        8 => 3,
        9 => 1,
        _ => panic!()
    }
}

fn play_turn(score: u32, p1_state: u32, p2_state: u32, p1_score: u32, p2_score: u32) -> (u64, u64) {
    let mut p1_wins : u64 = 0;
    let mut p2_wins : u64 = 0;
    // dice rolls can be 3-9
    for p1_roll in 3..=9 {
        let p1_new_state = (p1_state + p1_roll - 1) % 10 + 1;
        let p1_new_score = p1_score + p1_new_state;
        let routes_to_p1_new_state = ways_of_getting_dice_value(p1_roll);
        if p1_new_score >= score {
            p1_wins += routes_to_p1_new_state;
            continue;
        }
        for p2_roll in 3..=9 {
            let p2_new_state = (p2_state + p2_roll - 1) % 10 + 1;
            let p2_new_score = p2_score + p2_new_state;
            let routes_to_p2_new_state = routes_to_p1_new_state * ways_of_getting_dice_value(p2_roll);
            if p2_new_score >= score {
                p2_wins += routes_to_p2_new_state;
                continue;
            }
            let wins = play_turn(
                score, p1_new_state, p2_new_state, p1_new_score, p2_new_score);

            p1_wins += routes_to_p2_new_state * wins.0;
            p2_wins += routes_to_p2_new_state * wins.1;
        }
    }
    return (p1_wins, p2_wins); 
}

fn main() {
    let args : Vec<_> = env::args().collect();
    assert!(args.len() == 3, "specify player starting positions as arguments");
    
    let player1_start = args[1].parse::<u32>().unwrap();
    let player2_start = args[2].parse::<u32>().unwrap();
    
    println!("Part 1: {}", part1(player1_start, player2_start));

    let part2_wins = play_turn(21, player1_start, player2_start, 0, 0);
    println!("Part2: {:?}", part2_wins);
}
