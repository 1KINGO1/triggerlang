## triggerlang
triggerlang parses a custom trigger language
inspired by Battlemetrics trigger system

### Each trigger defines:
- an event type (`on`), such as player join or score change
- a logical condition (`condition`) combining comparisons with `&&`, `||`, `!`
- and an action (`action`) to execute when the condition is true

The parser reads a file or string row containing one or more triggers.
Constructs an Abstract Syntax Tree (AST) representing the trigger structure, 
and produces structured output that can be used for event handling, notifications, or game server automation.

### Parsing process:
1. read the trigger file.
2. for each `trigger` block:
    - identify the trigger name.
    - parse the `on` event type.
    - parse the `description` field.
    - parse the logical expression in `condition`.
    - parse the `action` string, including variable interpolations.
3. build an AST representing the parsed triggers.
4. the resulting AST can be used by rust code to execute actions or further analyze triggers.

### Cli commands:
1. Show help
```
cargo run -- help
```
or
```
cargo run -- help parse
```

2. Credits
```
cargo run -- credits
```

3. Parse file
```
cargo run -- parse example.tl
```
or with AST
```
cargo run -- parse example.tl --ast
```


### Example:
```
trigger JoinTrigger {
  on: player_join
  description: "Player joined a game"
  condition: player.is_new == true && player.is_banned == false
  action: send_message("Welcome!")
};
```

