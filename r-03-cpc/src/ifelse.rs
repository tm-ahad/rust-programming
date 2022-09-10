
pub fn guess(num: i8) -> String {

    if num == 0
    {
        return "Paper".to_string();
    }

    if num == 1 {
        return "Scissor".to_string();
    }

    if num == 2 {
        return "Rock".to_string();
    }

    else {
        return String::from("");
    }
}