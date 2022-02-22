use serenity::async_trait;
use serenity::model::channel::Message;

/// Send a potential response to any chat message.
#[async_trait]
pub trait MessageResponse {
    async fn message_response(&self, msg: &Message) -> Option<String>;
}
