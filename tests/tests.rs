use triggerlang::{parse_triggers, parse_triggers_to_ast};

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

#[test]
fn test_empty_input_fails() {
    assert!(parse_triggers("").is_err());
}

#[test]
fn test_whitespace_only_fails() {
    assert!(parse_triggers(" \n\t     \t   \t     \t ").is_err());
}

#[test]
fn test_minimal_fields_trigger() {
    let input = r#"
            trigger smallTrigger {
                on: player_join
                description: "small"
                action: test()
            };
        "#;
    assert!(parse_triggers(input).is_ok());
}

#[test]
fn test_minimal_fields_trigger_fails() {
    let input = r#"
            trigger smallTrigger {
                description: "small"
                action: test()
            };
        "#;
    assert!(parse_triggers(input).is_err());
}

#[test]
fn test_ast_generation() {
    let input = r#"
            trigger TestTrigger {
                on: player_join
                description: "Test"
                condition: x == 1
                action: test()
            };
        "#;
    let ast = parse_triggers_to_ast(input);
    assert!(ast.is_ok());
    let ast = ast.unwrap();
    assert_eq!(ast.triggers.len(), 1);
    assert_eq!(ast.triggers[0].name, "TestTrigger");
}

#[test]
fn test_not_equal_operator() {
    let input = r#"
        trigger NotEqual {
            on: player_join
            description: "not equal test"
            condition: player.score != 100
            action: test()
        };
    "#;
    assert!(parse_triggers(input).is_ok());
}

#[test]
fn test_less_than_operator() {
    let input = r#"
        trigger LessThan {
            on: player_join
            description: "lt test"
            condition: player.score < 50
            action: test()
        };
    "#;
    assert!(parse_triggers(input).is_ok());
}

#[test]
fn test_less_or_equal_operator() {
    let input = r#"
        trigger LessOrrEqual {
            on: player_join
            description: "lte test"
            condition: player.score <= 20
            action: test()
        };
    "#;
    assert!(parse_triggers(input).is_ok());
}
