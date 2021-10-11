use rand::Rng;

fn main() {
    let requested_roll = "1d12+d4 +8+ 2d6".to_string();

    let parsed_roll = parse_request(&requested_roll);
    let parsed_roll_list = parsed_roll.unwrap();

    let mut roll_result= Vec::new();
    let mut mod_result = 0u8;

    for token in parsed_roll_list.request_list {
        if let RequestToken::Dice(x) = token {
            roll_result.push(roll(x.number_of_dice, x.number_of_die_sides));
        }
        else if let RequestToken::Modifier(x) = token {
            mod_result =  x.value;
        }
    }
}

fn roll(num_dice: u16, num_sides: u8) -> RollResult{
    let mut rng = rand::thread_rng();
    let mut roll_result = RollResult { total: 0, };

    for _i in 0..=num_dice {
        roll_result.total += rng.gen_range(1..=num_sides) as u16;
    }
    roll_result
}

struct RollResult {
    total: u16,
    // request_token: RequestToken,
}

#[derive(Debug, Clone)]
struct DiceThrowDescription {
    number_of_dice: u16,
    number_of_die_sides: u8,
}

#[derive(Debug, Clone)]
struct DiceThrowModifier {
    operator: ModifierOperators,
    value: u8,
}

#[derive(Debug, Clone)]
struct RollRequest {
    request_list: Vec<RequestToken>,
}

#[derive(Debug, Clone)]
enum ModifierOperators {
    Add,
    _Subtract,
    _Multiply,
    _Divide,
}

#[derive(Debug, Clone)]
enum RequestToken {
    Dice(DiceThrowDescription),
    Modifier(DiceThrowModifier),
    Error,
}


fn roll_request(request_string: &str) -> Option<RollResult> {
    println!("{}:  roll_request", request_string);
    None
}

fn parse_request(request_string: &str) -> Option<RollRequest> {
    let tokens = tokenize(request_string.to_owned());
    Some(RollRequest{request_list:tokens})
}

fn tokenize(mut request_string: String) -> Vec<RequestToken> {
    request_string.retain(|c| !c.is_whitespace());
    let general_token_list: Vec<&str> = request_string.rsplit('+').collect();
    let mut request_token_list: Vec<RequestToken> = Vec::new();
    for general_token in general_token_list {
        let request_token = make_request_token(general_token);
        request_token_list.push(request_token);
    }
    request_token_list
}

fn make_request_token(in_token: &str) -> RequestToken {
    let token_parts = in_token.split('d').collect::<Vec<&str>>();

    match token_parts.len() {
        1 => return RequestToken::Modifier(DiceThrowModifier {operator: ModifierOperators::Add,
            value: token_parts[0].parse().unwrap()}),
        2 => return RequestToken::Dice(DiceThrowDescription {number_of_dice: token_parts[0].parse().unwrap_or(1),
            number_of_die_sides: token_parts[1].parse().unwrap()}),
        _ => return RequestToken::Error,
        };
}