pub fn reply(message: &str) -> &str {

    // Trim the message
    let message = message.trim();

    // If there is no message
    if message == "" {
        return "Fine. Be that way!"
    }

    // Check to see if message is made of characters
    if message.contains(char::is_alphabetic) {
        // Check if yelling and if question
        if message.ends_with("?") && message.to_uppercase() == message {
            return "Calm down, I know what I'm doing!";
        }
        // Check if just yelling
        if message.to_uppercase() == message {
            return "Whoa, chill out!";
        }
    }

    // Check to see if it is a question
    if message.ends_with("?") {
        return "Sure.";
    }

    // Standard edgy teen response
    "Whatever."
}
