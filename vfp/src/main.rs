use logos::{Lexer, Logos};
use strum_macros::Display;

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct Element {
//     kind: VideoCodec
// }

#[derive(Debug, Clone, PartialEq, Eq, Display)]
enum VideoCodecKind {
    #[strum(serialize = "h265")]
    H265,
    #[strum(serialize = "h264")]
    H264,
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
enum VideoResolutionKind {
    #[strum(serialize = "1080p")]
    R1080P,
    #[strum(serialize = "2160p")]
    R2160P,
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
enum VideoSourceKind {
    Bluray,
    Webdl,
    #[strum(serialize = "8Bit")]
    C8Bit,
}

// fn codec(lex: &mut Lexer<Token>) -> Option<VideoCodec> {
//     let slice = lex.slice();
//     Some(VideoCodec::H265)
// }

#[derive(Logos, Debug, PartialEq, Clone, Display)]
enum Token {
    #[error(logos::skip)]
    Error,

    // Codec
    #[regex(r"[hx].?264", |_| VideoCodecKind::H264)]
    #[regex(r"[hx].?265|hevc", |_| VideoCodecKind::H265)]
    VideoCodec(VideoCodecKind),

    // Resolution
    #[regex(r"(1920x)?1080p?x?([56]0)?", |_| VideoResolutionKind::R1080P)]
    #[regex(r"((3840x)?2160p?x?([56]0)?)|4k", |_| VideoResolutionKind::R2160P)]
    VideoResolution(VideoResolutionKind),

    // // Source
    // #[regex(r"bluray|blu-ray", |_| VideoSourceKind::Bluray)]
    #[regex(r"(?:b[dr][\W_]?rip|blu[\W_]?ray(?:[\W_]?rip)?)", |_| VideoSourceKind::Bluray)]
    #[regex(r"web(?:[\W_]?(dl|hd))?", |_| VideoSourceKind::Webdl)]
    #[regex(r"8[^\w]?bits?|hi8p?", |_| VideoSourceKind::C8Bit)]
    VideoSource(VideoSourceKind),

    // // Audio
    // #[strum(serialize = "MP3")]
    // #[regex(r"mp3")]
    // Mp3,
    // //#[regex(r"flac")]
    // //#[regex(r"flac%s?", |lex| lex.slice().parse())]
    // #[regex(r"flac")]
    // Flac,

    // // Audio Channels
    // // TODO: Needs work
    // #[regex(r"([765].?[01])")]
    // AudioChannels,
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

        //let token_string = Some(token.to_string());
        //let token_string = Some(token.into());

        match token {
            Token::VideoCodec(value) => {
                result.video_codec = Some(value.to_string());
            }
            Token::VideoResolution(value) => {
                result.video_resolution = Some(value.to_string());
            }
            Token::VideoSource(value) => {
                result.source = Some(value.to_string());
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
    let result = parse(&"[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon[1920x1080_H.265_FLAC_5.1_blu-ray][1234ABCD].mkv".to_string());
    dbg!(result);
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
            ParseResult {
                title: title,
                ..ParseResult::default()
            }
        );
    }
}
