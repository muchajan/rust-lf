use regex::Regex;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

#[derive(Debug)]
pub struct TextMetrics {
    word_count: usize,
    sentence_count: usize,
    syllable_count: usize,
    complex_word_count: usize,
    character_count: usize,
    gunning_fog_index: f64,
    flesch_kincaid_grade: f64,
    flesch_reading_ease: f64,
    smog_index: f64,
    average_words_per_sentence: f64,
    average_syllables_per_word: f64,
}

pub struct TextAnalyzer {
    word_pattern: Regex,
    sentence_pattern: Regex,
    vowel_pattern: Regex,
}

impl TextAnalyzer {
    pub fn new() -> Self {
        TextAnalyzer {
            word_pattern: Regex::new(r"\b[a-zA-Z]+\b").unwrap(),
            sentence_pattern: Regex::new(r"[.!?]+").unwrap(),
            vowel_pattern: Regex::new(r"[aeiouy]+").unwrap(),
        }
    }

    fn count_syllables(&self, word: &str) -> usize {
        let cleaned_word = word.trim_matches(|c: char| !c.is_alphabetic()).to_lowercase();
        
        // Handle special cases
        if cleaned_word.is_empty() {
            return 0;
        }
        
        let mut count = self.vowel_pattern.find_iter(&cleaned_word).count();
        
        // Adjust for common patterns
        if cleaned_word.ends_with('e') && count > 1 && !cleaned_word.ends_with("le") {
            count -= 1;
        }
        
        // Handle consecutive vowels
        let consecutive_vowels = Regex::new(r"[aeiouy]{2,}").unwrap();
        count -= consecutive_vowels.find_iter(&cleaned_word).count();
        
        // Ensure at least one syllable
        count.max(1)
    }

    fn is_complex_word(&self, word: &str, syllable_count: usize) -> bool {
        let cleaned_word = word.trim_matches(|c: char| !c.is_alphabetic()).to_lowercase();
        syllable_count >= 3 
            && !cleaned_word.ends_with("ed") 
            && !cleaned_word.ends_with("es") 
            && !cleaned_word.ends_with("ing")
    }

    pub fn analyze_text(&self, text: &str) -> TextMetrics {
        let words: Vec<&str> = self.word_pattern.find_iter(text)
            .map(|m| m.as_str())
            .collect();
        
        let word_count = words.len();
        let sentence_count = self.sentence_pattern.find_iter(text).count().max(1);
        let character_count = text.chars().filter(|c| c.is_alphabetic()).count();
        
        let mut syllable_count = 0;
        let mut complex_word_count = 0;
        
        for word in &words {
            let word_syllables = self.count_syllables(word);
            syllable_count += word_syllables;
            if self.is_complex_word(word, word_syllables) {
                complex_word_count += 1;
            }
        }

        let average_words_per_sentence = word_count as f64 / sentence_count as f64;
        let average_syllables_per_word = if word_count > 0 {
            syllable_count as f64 / word_count as f64
        } else {
            0.0
        };

        TextMetrics {
            word_count,
            sentence_count,
            syllable_count,
            complex_word_count,
            character_count,
            gunning_fog_index: self.calculate_gunning_fog(word_count, sentence_count, complex_word_count),
            flesch_kincaid_grade: self.calculate_flesch_kincaid_grade(word_count, sentence_count, syllable_count),
            flesch_reading_ease: self.calculate_flesch_reading_ease(word_count, sentence_count, syllable_count),
            smog_index: self.calculate_smog(sentence_count, complex_word_count),
            average_words_per_sentence,
            average_syllables_per_word,
        }
    }

    fn calculate_gunning_fog(&self, words: usize, sentences: usize, complex_words: usize) -> f64 {
        if words == 0 || sentences == 0 {
            return 0.0;
        }
        0.4 * ((words as f64 / sentences as f64) + 100.0 * (complex_words as f64 / words as f64))
    }

    fn calculate_flesch_kincaid_grade(&self, words: usize, sentences: usize, syllables: usize) -> f64 {
        if words == 0 || sentences == 0 {
            return 0.0;
        }
        0.39 * (words as f64 / sentences as f64) + 11.8 * (syllables as f64 / words as f64) - 15.59
    }

    fn calculate_flesch_reading_ease(&self, words: usize, sentences: usize, syllables: usize) -> f64 {
        if words == 0 || sentences == 0 {
            return 0.0;
        }
        206.835 - 1.015 * (words as f64 / sentences as f64) - 84.6 * (syllables as f64 / words as f64)
    }

    fn calculate_smog(&self, sentences: usize, complex_words: usize) -> f64 {
        if sentences < 30 {
            return 0.0; // SMOG is only valid for 30+ sentences
        }
        1.0430 * f64::sqrt(complex_words as f64 * (30.0 / sentences as f64)) + 3.1291
    }

    pub fn analyze_file(&self, filepath: &str) -> io::Result<TextMetrics> {
        let path = Path::new(filepath);
        if !path.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
        }

        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        Ok(self.analyze_text(&contents))
    }
}

fn main() {
    let analyzer = TextAnalyzer::new();
    
    // Example text
    let sample_text = "The quick brown fox jumps over the lazy dog. \
                       This is a simple sentence to demonstrate the algorithm. \
                       Extraordinary complications arise from miscellaneous circumstances. \
                       The complexity of this text should be relatively moderate.";
    
    let metrics = analyzer.analyze_text(sample_text);
    
    println!("Text Analysis Results:");
    println!("----------------------");
    println!("Word Count: {}", metrics.word_count);
    println!("Sentence Count: {}", metrics.sentence_count);
    println!("Complex Word Count: {}", metrics.complex_word_count);
    println!("Average Words per Sentence: {:.1}", metrics.average_words_per_sentence);
    println!("Average Syllables per Word: {:.1}", metrics.average_syllables_per_word);
    println!("\nReadability Scores:");
    println!("------------------");
    println!("Gunning Fog Index: {:.1}", metrics.gunning_fog_index);
    println!("Flesch-Kincaid Grade Level: {:.1}", metrics.flesch_kincaid_grade);
    println!("Flesch Reading Ease: {:.1}", metrics.flesch_reading_ease);
    println!("SMOG Index: {:.1}", metrics.smog_index);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_text() {
        let analyzer = TextAnalyzer::new();
        let metrics = analyzer.analyze_text("");
        assert_eq!(metrics.word_count, 0);
        assert_eq!(metrics.gunning_fog_index, 0.0);
    }

    #[test]
    fn test_simple_sentence() {
        let analyzer = TextAnalyzer::new();
        let metrics = analyzer.analyze_text("The cat sat on the mat.");
        assert_eq!(metrics.word_count, 6);
        assert_eq!(metrics.sentence_count, 1);
    }

    #[test]
    fn test_syllable_counting() {
        let analyzer = TextAnalyzer::new();
        assert_eq!(analyzer.count_syllables("cat"), 1);
        assert_eq!(analyzer.count_syllables("water"), 2);
        assert_eq!(analyzer.count_syllables("beautiful"), 3);
    }
}