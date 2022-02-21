use crate::random::{Rng, XorShift32};

const EMOJI_MINIMUM: usize = 5;
const REPLACE_CHANCE: f64 = 0.05; // 100% babeyyy

pub struct EmojiReplace;

use crate::command_types::MessageResponse;
use serenity::async_trait;
use serenity::model::channel::Message;
#[async_trait]
impl MessageResponse for EmojiReplace {
    async fn message_response(&self, msg: &Message) -> Option<String> {
        let text = &msg.content;
        let mut rng = XorShift32(msg.timestamp.timestamp() as u32);
        if count_emoji(text) >= EMOJI_MINIMUM && rng.run_probability(REPLACE_CHANCE) {
            Some(replace(text))
        } else {
            None
        }
    }
}

use std::default::Default;
impl Default for EmojiReplace {
    fn default() -> Self {
        Self
    }
}

fn is_face_emoji(c: char) -> bool {
    // "face emoji" in this context excludes any emoji with body or hair.
    let ranges = [
        (0x1F479..=0x1F47B),
        (0x1F47D..=0x1F480),
        (0x1F600..=0x1F644),
        (0x1F910..=0x1F917),
        (0x1F920..=0x1F925),
        (0x1F927..=0x1F92F),
        (0x1F970..=0x1F976),
        (0x1F978..=0x1F97A),
        (0x1F978..=0x1F97A),
        (0x1F9D0..=0x1F9D0),
    ];
    let code = &(c as u32);
    ranges.iter().any(|range| range.contains(code))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_face() {
        assert!(is_face_emoji('🥺'));
        assert!(is_face_emoji('😏'));
        assert!(is_face_emoji('😳'));
        assert!(is_face_emoji('🤣'));
    }
    #[test]
    fn is_not_face() {
        assert!(!is_face_emoji('👋'));
        assert!(!is_face_emoji('👸'));
        assert!(!is_face_emoji('⭕'));
        assert!(!is_face_emoji('👀'));
    }
}

fn replace(s: &str) -> String {
    let replacement_char = '😐';
    s.chars()
        .map(|c| {
            if is_face_emoji(c) {
                replacement_char
            } else {
                c
            }
        })
        .collect()
}

fn count_emoji(s: &str) -> usize {
    s.chars().filter(|&c| is_face_emoji(c)).count()
}
