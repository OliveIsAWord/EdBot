# EdBot 0.0.1
The backend for the EdCord server bot, written in Rust using the Serenity and Tokio libraries. New behaviors can be implemented with Traits.

## Behavior Traits
- `MessageResponse` can respond to any message with text sent by the bot.
- `Command` can run on any message that begins with the bot's prefix. (TODO)
- `SlashCommand` can run on use of Dscord's specialized slash commands. (TODO)
- TODO
