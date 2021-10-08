fn main() {
    let requested_roll = "1d12";
    
    let roll_result = roll_request(requested_roll);
}

fn roll_request(&requested_roll) -> {
    let roll_request_list = parse_roll_request(&requested_roll);
    for roll in roll_request_list {
        roll_result = roll_
    }
}

fn parse_roll_request(request_string: &str) -> Option<T>{

    None
}

struct Die {
    sides: u8,
}

trait dice {
    fn simple_roll() -> Option<RollResult>;
    fn complex_roll() -> Option<RollResultList>;
}

struct SimpleRoll {
    die: Die,
    roll_result: u16,
}

struct RollResult {
    total_roll_result: u16,
    
}

struct RollRequestList {
    number_unique_dice: u16,
    roll_requests: Vec<roll_request_composition>,
}

struct RollRequestComposition {
    number_rolls: u8,
    die_size: u8,
    roll_options: RollOptions,
}

enum RollOptions {
    Once,
    ReRollOnceLessThan: u8,
    ReRollManyLessThan: u8,
    ReRollOnceGreaterThan: u8,
    ReRollManyGreaterThan: u8,
}