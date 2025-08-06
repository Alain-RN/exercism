pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();

    if trimmed.is_empty() {
        return "Fine. Be that way!";
    }

    let is_question = trimmed.ends_with('?');
    let has_letter = trimmed.chars().any(|c| c.is_alphabetic());
    let is_yelling = has_letter && trimmed.chars().all(|c| !c.is_lowercase());
    
    if is_yelling && is_question {
        "Calm down, I know what I'm doing!"
    } else if is_question {
        "Sure."
    } else if is_yelling {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
