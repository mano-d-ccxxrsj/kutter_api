use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::collections::HashSet;

use crate::entities::banned_word_entity::BannedWord;
use crate::entities::user_flag_entity::{NewUserFlag, UserFlag};
use crate::ports::moderation::content_moderation_port::ContentModerationPort;
use crate::ports::repositories::banned_word_repository_port::BannedWordRepositoryPort;
use crate::ports::repositories::user_flag_repository_port::UserFlagRepositoryPort;
use crate::services::types::{
    NoopContentModerationService, ProfanityContentModerationService,
    ToggleContentModerationService,
};

impl NoopContentModerationService {
    pub fn new() -> NoopContentModerationService {
        NoopContentModerationService {}
    }
}

impl<ContentModeration> ToggleContentModerationService<ContentModeration>
where
    ContentModeration: ContentModerationPort,
{
    pub fn new(
        enabled: bool,
        content_moderation: ContentModeration,
    ) -> ToggleContentModerationService<ContentModeration> {
        ToggleContentModerationService { enabled, content_moderation }
    }
}

impl<BannedWordRepository, UserFlagRepository>
    ProfanityContentModerationService<BannedWordRepository, UserFlagRepository>
where
    BannedWordRepository: BannedWordRepositoryPort,
    UserFlagRepository: UserFlagRepositoryPort,
{
    pub fn new(
        banned_words: BannedWordRepository,
        user_flags: UserFlagRepository,
    ) -> ProfanityContentModerationService<BannedWordRepository, UserFlagRepository> {
        ProfanityContentModerationService { banned_words, user_flags }
    }

    fn remove_spaces(text: &str) -> String {
        let mut result: String = String::new();
        let mut characters: std::str::Chars = text.chars();

        loop {
            let character: char = match (& mut characters).next() {
                Some(found) => found,
                None => break,
            };

            if !character.is_whitespace() {
                (&mut result).push(character);
            }
        }

        result
    }

    fn remove_separators(text: &str) -> String {
        let mut result: String = String::new();
        let mut characters: std::str::Chars = text.chars();

        loop {
            let character: char = match (& mut characters).next() {
                Some(found) => found,
                None => break,
            };

            if !Self::separator(character) {
                (&mut result).push(character);
            }
        }

        result
    }

    fn separator(character: char) -> bool {
        if character.is_whitespace() || character.is_ascii_punctuation() {
            return true;
        }

        match character {
            '\u{3000}'..='\u{303F}'
            | '\u{FF00}'..='\u{FF65}'
            | '\u{2000}'..='\u{206F}' => true,
            _ => false,
        }
    }

    fn text_contains_east_asian(text: &str) -> bool {
        let mut characters: std::str::Chars = text.chars();

        loop {
            let character: char = match (& mut characters).next() {
                Some(found) => found,
                None => break,
            };

            if Self::east_asian_character(character) {
                return true;
            }
        }

        false
    }

    fn east_asian_character(character: char) -> bool {
        match character {
            '\u{3040}'..='\u{30FF}'
            | '\u{3400}'..='\u{4DBF}'
            | '\u{4E00}'..='\u{9FFF}'
            | '\u{AC00}'..='\u{D7AF}'
            | '\u{F900}'..='\u{FAFF}' => true,
            _ => false,
        }
    }

    fn split_into_words(text: &str) -> Vec<String> {
        let mut words: Vec<String> = Vec::new();
        let mut current_word: String = String::new();
        let mut characters: std::str::Chars = text.chars();

        loop {
            let character: char = match (& mut characters).next() {
                Some(found) => found,
                None => break,
            };

            if character.is_whitespace() {
                if !current_word.is_empty() {
                    (&mut words).push(current_word);
                    current_word = String::new();
                }
            } else {
                (&mut current_word).push(character);
            }
        }

        if !current_word.is_empty() {
            (&mut words).push(current_word);
        }

        words
    }

    fn normalize_latin_like(text: &str) -> String {
        let mut result: String = String::new();
        let mut characters: std::str::Chars = text.chars();

        loop {
            let character: char = match (& mut characters).next() {
                Some(found) => found,
                None => break,
            };

            let normalized: Option<String> = Self::normalize_latin_like_character(character);

            if let Some(found) = normalized {
                (&mut result).push_str(&found);
            }
        }

        result
    }

    fn normalize_latin_like_character(character: char) -> Option<String> {
        let lower: char = character.to_ascii_lowercase();

        let normalized: &str = match lower {
            'a' => "a",
            'b' => "b",
            'c' => "c",
            'd' => "d",
            'e' => "e",
            'f' => "f",
            'g' => "g",
            'h' => "h",
            'i' => "i",
            'j' => "j",
            'k' => "k",
            'l' => "l",
            'm' => "m",
            'n' => "n",
            'o' => "o",
            'p' => "p",
            'q' => "o",
            'r' => "r",
            's' => "s",
            't' => "t",
            'u' => "u",
            'v' => "v",
            'w' => "w",
            'x' => "x",
            'y' => "y",
            'z' => "z",
            '0' => "o",
            '1' => "i",
            '2' => "s",
            '3' => "e",
            '4' => "a",
            '5' => "s",
            '6' => "g",
            '7' => "l",
            '8' => "b",
            '9' => "g",
            '@' => "a",
            '#' => "h",
            '$' => "s",
            '!' => "i",
            '|' => "i",
            '-' | '_' | '\\' | '/' => " ",
            '\u{00E0}' | '\u{00E1}' | '\u{00E2}' | '\u{00E3}' | '\u{00E4}' => "a",
            '\u{00C0}' | '\u{00C1}' | '\u{00C2}' | '\u{00C3}' | '\u{00C4}' => "a",
            '\u{00E8}' | '\u{00E9}' | '\u{00EA}' | '\u{00EB}' => "e",
            '\u{00C8}' | '\u{00C9}' | '\u{00CA}' | '\u{00CB}' => "e",
            '\u{00EC}' | '\u{00ED}' | '\u{00EE}' | '\u{00EF}' => "i",
            '\u{00CC}' | '\u{00CD}' | '\u{00CE}' | '\u{00CF}' => "i",
            '\u{00F2}' | '\u{00F3}' | '\u{00F4}' | '\u{00F5}' | '\u{00F6}' => "o",
            '\u{00D2}' | '\u{00D3}' | '\u{00D4}' | '\u{00D5}' | '\u{00D6}' => "o",
            '\u{00F9}' | '\u{00FA}' | '\u{00FB}' | '\u{00FC}' => "u",
            '\u{00D9}' | '\u{00DA}' | '\u{00DB}' | '\u{00DC}' => "u",
            '\u{00E7}' | '\u{00C7}' => "c",
            '\u{0410}' | '\u{0430}' => "a",
            '\u{0411}' | '\u{0431}' => "b",
            '\u{0412}' | '\u{0432}' => "v",
            '\u{0413}' | '\u{0433}' => "g",
            '\u{0414}' | '\u{0434}' => "d",
            '\u{0415}' | '\u{0435}' | '\u{0401}' | '\u{0451}' => "e",
            '\u{0416}' | '\u{0436}' => "zh",
            '\u{0417}' | '\u{0437}' => "z",
            '\u{0418}' | '\u{0438}' | '\u{0419}' | '\u{0439}' => "i",
            '\u{041A}' | '\u{043A}' => "k",
            '\u{041B}' | '\u{043B}' => "l",
            '\u{041C}' | '\u{043C}' => "m",
            '\u{041D}' | '\u{043D}' => "n",
            '\u{041E}' | '\u{043E}' => "o",
            '\u{041F}' | '\u{043F}' => "p",
            '\u{0420}' | '\u{0440}' => "r",
            '\u{0421}' | '\u{0441}' => "s",
            '\u{0422}' | '\u{0442}' => "t",
            '\u{0423}' | '\u{0443}' => "u",
            '\u{0424}' | '\u{0444}' => "f",
            '\u{0425}' | '\u{0445}' => "h",
            '\u{0426}' | '\u{0446}' => "ts",
            '\u{0427}' | '\u{0447}' => "ch",
            '\u{0428}' | '\u{0448}' => "sh",
            '\u{0429}' | '\u{0449}' => "sh",
            '\u{042B}' | '\u{044B}' => "y",
            '\u{042D}' | '\u{044D}' => "e",
            '\u{042E}' | '\u{044E}' => "yu",
            '\u{042F}' | '\u{044F}' => "ya",
            _ => return None,
        };

        Some(String::from(normalized))
    }

    fn normalize_east_asian(text: &str) -> String {
        let mut result: String = String::new();
        let mut characters: std::str::Chars = text.chars();

        loop {
            let character: char = match (& mut characters).next() {
                Some(found) => found,
                None => break,
            };

            if !Self::separator(character) {
                let mut lowercase_characters: std::char::ToLowercase = character.to_lowercase();

                loop {
                    let lowercase: char = match (& mut lowercase_characters).next() {
                        Some(found) => found,
                        None => break,
                    };

                    (&mut result).push(lowercase);
                }
            }
        }

        result
    }

    fn compress(text: &str) -> String {
        let mut result: String = String::new();
        let mut previous_character: Option<char> = None;
        let mut characters: std::str::Chars = text.chars();

        loop {
            let character: char = match (& mut characters).next() {
                Some(found) => found,
                None => break,
            };

            if Some(character) != previous_character || character.is_whitespace() {
                (&mut result).push(character);
            }

            previous_character = Some(character);
        }

        result
    }

    fn normalize_latin_like_and_compress(text: &str) -> String {
        let normalized: String = Self::normalize_latin_like(text);
        let compressed: String = Self::compress(&normalized);

        compressed
    }

    fn matched_words(text: &str, forbidden_words: &HashSet<String>) -> Vec<String> {
        if Self::text_contains_east_asian(text) {
            return Self::matched_east_asian_words(text, forbidden_words);
        }

        Self::matched_latin_like_words(text, forbidden_words)
    }

    fn matched_latin_like_words(text: &str, forbidden_words: &HashSet<String>) -> Vec<String> {
        let mut matched_words: Vec<String> = Vec::new();
        let words: Vec<String> = Self::split_into_words(text);
        let mut word_index: usize = 0;

        loop {
            if word_index >= words.len() {
                break;
            }

            let word: &String = &words[word_index];
            let normalized_word: String = Self::normalize_latin_like_and_compress(word.trim());

            if forbidden_words.contains(&normalized_word) && !matched_words.contains(&normalized_word) {
                (&mut matched_words).push(normalized_word);
            }

            word_index += 1;
        }

        let normalized_text: String = Self::normalize_latin_like_and_compress(text);
        let continuous_text: String = Self::remove_spaces(&normalized_text);
        let forbidden_word_values: Vec<String> = forbidden_words.iter().cloned().collect();
        let mut forbidden_word_index: usize = 0;

        loop {
            if forbidden_word_index >= forbidden_word_values.len() {
                break;
            }

            let forbidden_word: &String = &forbidden_word_values[forbidden_word_index];

            if !matched_words.contains(forbidden_word)
                && (normalized_text.contains(forbidden_word.as_str())
                    || continuous_text.contains(forbidden_word.as_str()))
            {
                (&mut matched_words).push(forbidden_word.clone());
            }

            forbidden_word_index += 1;
        }

        matched_words
    }

    fn matched_east_asian_words(text: &str, forbidden_words: &HashSet<String>) -> Vec<String> {
        let mut matched_words: Vec<String> = Vec::new();
        let normalized_text: String = Self::normalize_east_asian(text);
        let continuous_text: String = Self::remove_separators(&normalized_text);
        let forbidden_word_values: Vec<String> = forbidden_words.iter().cloned().collect();
        let mut forbidden_word_index: usize = 0;

        loop {
            if forbidden_word_index >= forbidden_word_values.len() {
                break;
            }

            let forbidden_word: &String = &forbidden_word_values[forbidden_word_index];

            if continuous_text.contains(forbidden_word.as_str()) && !matched_words.contains(forbidden_word) {
                (&mut matched_words).push(forbidden_word.clone());
            }

            forbidden_word_index += 1;
        }

        matched_words
    }

    fn joined_words(words: &[String]) -> String {
        let mut joined: String = String::new();
        let mut first_word: bool = true;
        let mut word_index: usize = 0;

        loop {
            if word_index >= words.len() {
                break;
            }

            let word: &String = &words[word_index];

            if first_word {
                first_word = false;
            } else {
                (&mut joined).push_str(", ");
            }

            (&mut joined).push_str(word.as_str());
            word_index += 1;
        }

        joined
    }
}

#[async_trait]
impl ContentModerationPort for NoopContentModerationService {
    async fn validate(&self, _: i32, _: &str, _: &str, _: &str, _: &str) -> Result<(), String> {
        Ok(())
    }
}

#[async_trait]
impl<ContentModeration> ContentModerationPort for ToggleContentModerationService<ContentModeration>
where
    ContentModeration: ContentModerationPort,
{
    async fn validate(
        &self,
        user_id: i32,
        field: &str,
        action: &str,
        target: &str,
        content: &str,
    ) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }

        self
            .content_moderation
            .validate(user_id, field, action, target, content)
            .await
    }
}

#[async_trait]
impl<BannedWordRepository, UserFlagRepository> ContentModerationPort
    for ProfanityContentModerationService<BannedWordRepository, UserFlagRepository>
where
    BannedWordRepository: BannedWordRepositoryPort,
    UserFlagRepository: UserFlagRepositoryPort,
{
    async fn validate(
        &self,
        user_id: i32,
        field: &str,
        action: &str,
        target: &str,
        content: &str,
    ) -> Result<(), String> {
        if content.trim().is_empty() {
            return Ok(());
        }

        let banned_words: Vec<BannedWord> = self
            .banned_words
            .find_active()
            .await
            .map_err(|error| error.to_string())?;

        let mut forbidden_words: HashSet<String> = HashSet::new();
        let east_asian_text: bool = Self::text_contains_east_asian(content);
        let mut banned_word_index: usize = 0;

        loop {
            if banned_word_index >= banned_words.len() {
                break;
            }

            let banned_word: &BannedWord = &banned_words[banned_word_index];
            let normalized_word: String = if east_asian_text {
                Self::normalize_east_asian(&banned_word.word)
            } else {
                Self::normalize_latin_like_and_compress(&banned_word.word)
            };

            if !normalized_word.is_empty() {
                (&mut forbidden_words).insert(normalized_word);
            }

            banned_word_index += 1;
        }

        let matched_words: Vec<String> = Self::matched_words(content, &forbidden_words);

        if matched_words.is_empty() {
            return Ok(());
        }

        let created_at: DateTime<Utc> = Utc::now();
        let user_flag: NewUserFlag = NewUserFlag {
            user_id,
            field: field.to_string(),
            action: action.to_string(),
            target: target.to_string(),
            attempted_text: content.to_string(),
            matched_words: Self::joined_words(&matched_words),
            details: None,
            created_at,
        };

        let _: UserFlag = self
            .user_flags
            .create(&user_flag)
            .await
            .map_err(|error| error.to_string())?;

        Err(String::from("Content violates platform rules"))
    }
}