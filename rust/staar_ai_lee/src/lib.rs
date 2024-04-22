#[derive(Debug)]
pub struct ParsedParagraph {
    num_of_quotes: usize,
    num_of_sentences: usize,
    num_of_paragraphs: usize,
    split_text: Vec<String>, // One element per paragraph
}

pub struct RawParagraph(String);

#[derive(Debug)]
pub enum Error {
    ParserError,
}

#[derive(Debug)]
pub enum WhyFail {
    MismatchedQuotes,
    TextEvidence,
    Sentences,
    Paragraphs,
}

#[derive(Debug)]
pub struct Pass;

fn num_of_occurences(haystack: &str, needle: char) -> usize {
    haystack.match_indices(needle).collect::<Vec<_>>().len()
}

impl RawParagraph {
    pub fn new(text: &str) -> Self {
        RawParagraph(text.to_string())
    }

    pub fn parse(self) -> Result<ParsedParagraph, Error> {
        let num_of_quotes = num_of_occurences(&self.0, '"');
        let num_of_sentences = num_of_occurences(&self.0, '.');
        if num_of_quotes % 2 != 0 {
            return Err(Error::ParserError);
        }

        let split_text: Vec<String> = self.0.split("\n").map(|x| x.to_string()).collect();
        let num_of_paragraphs = split_text.len();

        Ok(ParsedParagraph {
            num_of_quotes,
            num_of_sentences,
            num_of_paragraphs,
            split_text,
        })
    }
}

impl ParsedParagraph {
    pub fn passes(&self) -> Result<Pass, WhyFail> {
        if !self.num_of_quotes % 2 == 0 {
            return Err(WhyFail::MismatchedQuotes);
        }

        // Two pieces of text evidence
        if !self.num_of_quotes >= 4 {
            return Err(WhyFail::TextEvidence);
        }
        if !self.num_of_sentences >= 13 {
            return Err(WhyFail::Sentences);
        }
        if !self.num_of_paragraphs == 4 {
            return Err(WhyFail::Paragraphs);
        }
        return Ok(Pass);
    }
}
