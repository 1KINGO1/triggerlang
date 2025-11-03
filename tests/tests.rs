use triggerlang::parse_triggers;

#[test]
fn test_simple_trigger() {
    let input = r#"
        trigger JoinTrigger {
           on: player_join
           description: "Player joined a game"
           condition: player.is_new == true && player.is_banned == false
           action: send_message("Welcome!")
        };
    "#;

    assert!(parse_triggers(input).is_ok());
}

#[test]
fn test_trigger_without_condition() {
    let input = r#"
        trigger NoCondition {
           on: player_leave
           description: "Player left the game"
           action: send_message("Goodbye!")
        };
    "#;

    assert!(parse_triggers(input).is_ok());
}

#[test]
fn test_multiple_conditions_and_actions() {
    let input = r#"
        trigger MultiAction {
           on: player_score_change
           description: "Score event"
           condition: player.score > 100 && player.is_banned == false
           action: send_message("Congrats!")
           action: give_reward("log")
        };
    "#;

    assert!(parse_triggers(input).is_ok());
}

#[test]
fn test_trigger_with_nested_expression() {
    let input = r#"
        trigger Complex {
           on: message_receive
           description: "Message processing"
           condition: (message.sender.is_admin == true || message.text == "hello") && !user.is_banned
           action: log_message("log")
        };
    "#;

    assert!(parse_triggers(input).is_ok());
}

#[test]
fn test_multiple_triggers_in_one_file() {
    let input = r#"
        trigger First {
           on: player_join
           description: "Join event"
           action: send_message("Welcome!")
        };

        trigger Second {
           on: player_leave
           description: "Leave event"
           action: send_message("Bye!")
        };
    "#;

    assert!(parse_triggers(input).is_ok());
}

#[test]
fn test_trigger_with_numbers_and_strings_in_condition() {
    let input = r#"
        trigger NumericCheck {
           on: player_score_change
           description: "Check numeric score"
           condition: player.score >= 50 && player.name == "Taras"
           action: send_message(":D")
        };
    "#;

    assert!(parse_triggers(input).is_ok());
}

#[test]
fn test_invalid_trigger_mising_semicolon() {
    let input = r#"
        trigger MissingSemicolon {
           on: player_join
           description: "Missing semicolon"
           action: send_message("ups")
        }
    "#;

    assert!(parse_triggers(input).is_err());
}
