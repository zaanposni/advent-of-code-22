// returns 1 for draw, 2 for win, 3 for loss
fn duel(_me: &str, _opponent: &str) -> i32 {
    if (_me == "rock" && _opponent == "scissors") || (_me == "scissors" && _opponent == "paper") || (_me == "paper" && _opponent == "rock") {
        return 2;
    } else if _me == _opponent {
        return 1;
    } else {
        return 3;
    }
}

// 1: draw, 2: win, 3: loss
fn choose_best_hand(_opponent: &str, mode: i32) -> &str {
    if mode == 1 {
        return _opponent;
    } else if mode == 2 {
        if _opponent == "rock" {
            return "paper";
        } else if _opponent == "paper" {
            return "scissors";
        } else {
            return "rock";
        }
    } else {
        if _opponent == "rock" {
            return "scissors";
        } else if _opponent == "paper" {
            return "rock";
        } else {
            return "paper";
        }
    }
}

fn code_to_hand(_code: &str) -> &str {
    match _code {
        "A" => "rock",
        "X" => "rock",
        "B" => "paper",
        "Y" => "paper",
        "C" => "scissors",
        "Z" => "scissors",
        _ => "invalid",
    }
}

// 1: draw, 2: win, 3: loss
fn code_to_mode(_code: &str) -> i32 {
    match _code {
        "X" => 3,
        "Y" => 1,
        "Z" => 2,
        _ => -1,
    }
}

pub fn calculate_score(_input: &str) {
    let mut score = 0;
    let lines = _input.lines();

    for line in lines {
        let mut words = line.split_whitespace();
        let opponent = code_to_hand(words.next().unwrap());
        let me = choose_best_hand(opponent, code_to_mode(words.next().unwrap()));

        if me == "invalid" || opponent == "invalid" {
            println!("Invalid input");
            return;
        }

        let result = duel(me, opponent);
        if result == 2 {
            score += 6;
        } else if result == 1 {
            score += 3;
        }

        if me == "rock" {
            score += 1;
        } else if me == "paper" {
            score += 2;
        } else {
            score += 3;
        }
    }

    println!("Score: {}", score);
}
