trigger WelcomePlayer {
  on: player_join
  description: "Welcome new players to the server"
  condition: player.is_new == true
  action: send_message("Welcome to our server!")
  action: give_reward("starter_pack")
};

trigger BanCheck {
  on: player_join
  description: "Check if player is banned"
  condition: player.is_banned == true
  action: kick_player("You are banned")
  action: log_event("Banned player attempted to join")
};

trigger GoodbyePlayer {
  on: player_leave
  description: "Say goodbye to leaving players"
  action: send_message("Thanks for playing!")
};

trigger HighScoreReward {
  on: player_score_change
  description: "Reward players who reach high scores"
  condition: player.score >= 1000 && !player.has_received_reward
  action: send_message("Congratulations on reaching 1000 points!")
  action: give_reward("gold_medal")
  action: set_flag("has_received_reward")
};

trigger AdminMessage {
  on: message_receive
  description: "Process messages from admins"
  condition: (message.sender.is_admin == true || message.sender.is_moderator == true) && !message.is_empty
  action: broadcast_message("Admin announcement")
  action: log_message("admin_log")
};