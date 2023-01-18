use logos::{Logos, Lexer};
use strum_macros::Display;



#[derive(Logos, Debug, PartialEq, Clone, Display)]
enum Token {
    #[error(logos::skip)]
    Error,

    // Codec
    #[regex(r"[hx].?264")]
    H264,
    #[regex(r"[hx].?265|hevc")]
    H265,

    // Resolution
    #[strum(serialize = "1080P")]
    #[regex(r"(1920x)?1080p?x?([56]0)?")]
    R1080P,
    #[strum(serialize = "2160P")]
    #[regex(r"((3840x)?2160p?x?([56]0)?)|4k")]
    R2160P,

    // Source
    #[regex(r"(?:b[dr][\W_]?rip|blu[\W_]?ray(?:[\W_]?rip)?)")]
    Bluray,
    #[regex(r"web(?:[\W_]?(dl|hd))?")]
    Webdl,
    #[strum(serialize = "8bit")]
    #[regex(r"8[^\w]?bits?|hi8p?")]
    C8Bit,

    // Audio
    #[strum(serialize = "MP3")]
    #[regex(r"mp3")]
    Mp3,
    //#[regex(r"flac")]
    //#[regex(r"flac%s?", |lex| lex.slice().parse())]
    #[regex(r"flac")]
    Flac,

    // Audio Channels
    // TODO: Needs work
    #[regex(r"([765].?[01])")]
    AudioChannels,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct ParseResult {
    title: String,
    video_codec: Option<String>,
    video_resolution: Option<String>,
    video_color_range: Option<String>,
    audio_codec: Option<String>,
    audio_channels: Option<String>,
    source: Option<String>,
    file_name: String,
}


impl ParseResult {
    pub fn new(file_name: String) -> Self {
        ParseResult {
            file_name,
            ..ParseResult::default()
        }
    }
}

// impl PartialEq for FileInfo {
//     fn eq(&self, other: &Self) -> bool {
//         self.path == other.path
//     }
// }

// TODO implement an delimiter: Regex(@"(\s|\.|,|_|-|=|\|)+"

fn parse(v: &String) -> ParseResult {
    let s = v.clone().to_lowercase();
    let mut lex = Token::lexer(&s);

    let mut result = ParseResult::new(s.clone());

    while let Some(token) = lex.next() {
        if token == Token::Error {
            continue;
        }

        let token_string = Some(token.to_string());

        match token {
            Token::H265 | Token::H264 => {
                result.video_codec = token_string;
            }
            Token::R1080P | Token::R2160P => {
                result.video_resolution = token_string;
            }
            Token::Bluray | Token::Webdl => {
                result.source = token_string;
            }
            Token::C8Bit => {
                result.video_color_range = token_string;
            }
            Token::Mp3 | Token::Flac => {
                // dbg!(lex.slice());
                result.audio_codec = token_string;
            }
            Token::AudioChannels => {
                result.audio_channels = Some(lex.slice().to_string());
            }
            _ => (),
        }
        // dbg!(token);
        // dbg!(VideoCodecKind::R1080P);
        // dbg!(token);
        // println!("{:?}", token);
    }
    result
}

fn main() {
    //let df = example().unwrap();
    //let result = parse("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1920x1080_H.265_FLAC][1234ABCD].mkv".to_string());
    // let result = parse("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon[1920x1080_H.265_FLAC_5.1_bluray][1234ABCD].mkv".to_string());
    // dbg!(result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(test)]
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn movies() {
        let title = "Tenet 2020 2160p UHD BluRay DTS-HD MA 5.1 DV x265-LEGi0N".to_string();
        let result = parse(&title);

        assert_eq!(
            result,
            ParseResult { title: title, ..ParseResult::default() }
        );
    }
}