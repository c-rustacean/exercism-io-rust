static QUESTION: &str = "Sure.";
static YELL: &str = "Whoa, chill out!";
static YELLQUESTION: &str = "Calm down, I know what I'm doing!";
static SILENCE: &str = "Fine. Be that way!";
static ANYTHINGELSE: &str = "Whatever.";

enum BobsRepliesTo {
    Question,
    Yell,
    YellQuestion,
    Silence,
    AnythingElse,
}

fn understand_utterance(utterance: &str) -> BobsRepliesTo {
    let simple_utter = utterance
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    let (question, shouting, silence) = (
        simple_utter.ends_with("?"),
        (simple_utter.to_uppercase() == simple_utter)
            && (simple_utter.to_lowercase() != simple_utter),
        simple_utter.is_empty(),
    );

    match (question, shouting, silence) {
        (true, true, _) => BobsRepliesTo::YellQuestion,
        (true, _, _) => BobsRepliesTo::Question,
        (_, _, true) => BobsRepliesTo::Silence,
        (_, true, _) => BobsRepliesTo::Yell,
        _ => BobsRepliesTo::AnythingElse,
    }
}

pub fn reply(message: &str) -> &'static str {
    // unimplemented!("have Bob reply to the incoming message: {}", message)

    match understand_utterance(message) {
        BobsRepliesTo::Question => QUESTION,
        BobsRepliesTo::Yell => YELL,
        BobsRepliesTo::YellQuestion => YELLQUESTION,
        BobsRepliesTo::Silence => SILENCE,
        BobsRepliesTo::AnythingElse => ANYTHINGELSE,
    }
}
