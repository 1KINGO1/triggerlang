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

### Grammar: 
```pest
WHITESPACE = _{ " " | "\t" | "\r" | "\n" }

file = { SOI ~ WHITESPACE* ~ trigger ~ (WHITESPACE* ~ trigger)* ~ WHITESPACE* ~ EOI }

trigger = { "trigger" ~ ident ~ "{" ~ trigger_body ~ "}" ~ ";" }

trigger_body = {
    (field_on ~ field_description
    | field_description ~ field_on)
    ~ field_condition?
    ~ field_action*
}

field_on = { "on" ~ ":" ~ event_type }
field_description = { "description" ~ ":" ~ string }
field_condition = { "condition" ~ ":" ~ expr }
field_action = { "action" ~ ":" ~ func_call }

event_type = { "player_join" | "player_leave" | "player_score_change" | "message_receive" }

expr = { atom ~ ((and | or) ~ expr)* }
atom = { not* ~ (comparison | func_call | ident | "(" ~ expr ~ ")") }
comparison = { ident ~ (eq | neq | gte | lte | gt | lt) ~ value }

func_call = { ident ~ "(" ~ (arg_list)? ~ ")" }
arg_list = { (value | ident) ~ ("," ~ (value | ident))* }

string = { "\"" ~ (!"\"" ~ ANY)* ~ "\"" }
ident = @{ (ASCII_ALPHA | "_") ~ (ASCII_ALPHANUMERIC | "_" | ".")* }
eq = { "==" }
neq = { "!=" }
gt = { ">" }
lt = { "<" }
gte = { ">=" }
lte = { "<=" }
not = { "!" }
and = { "&&" }
or = { "||" }
boolean = { "true" | "false" }
number = { ASCII_DIGIT+ ~ ("." ~ ASCII_DIGIT+)? }
value = { boolean | number | string | ident }
```

