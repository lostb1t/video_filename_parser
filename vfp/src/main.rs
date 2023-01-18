use logos::{Lexer, Logos};
use strum_macros::Display;

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct Element {
//     kind: VideoCodec
// }

#[derive(Debug, Clone, PartialEq, Eq, Display)]
enum VideoCodecKind {
    Divx,
    Xvid,
    Vp9,
    #[strum(serialize = "h265")]
    H265,
    #[strum(serialize = "h264")]
    H264,
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
enum VideoResolutionKind {
    #[strum(serialize = "360p")]
    R360P,
    #[strum(serialize = "480p")]
    R480P,
    #[strum(serialize = "540p")]
    R540P,
    #[strum(serialize = "576P")]
    R576P,
    #[strum(serialize = "720P")]
    R720P,
    #[strum(serialize = "1080p")]
    R1080P,
    #[strum(serialize = "2160p")]
    R2160P,
}

// #[derive(Debug, Clone, PartialEq, Eq, Display)]
// enum VideoSourceKind {
//     Workprint,
//     Cam,
//     TS,
//     TC,
//     R5,
//     HDrip,
//     PPVRip,
//     Preair,
//     TVRip,
//     Dsr,
//     Sdtv,
//     BluRay,
//     Webdl,
//     #[strum(serialize = "8Bit")]
//     C8Bit,
// }

// see https://github.com/scttcper/video-filename-parser/blob/master/src/source.ts
#[derive(Debug, Clone, PartialEq, Eq, Display)]
enum VideoSourceKind {
    Workprint,
    Cam,
    TS,
    TC,
    PPV,
    BluRay,
    Webdl,
    #[strum(serialize = "8Bit")]
    C8Bit,
}

// _sources = [
//     QualityComponent('source', 10, 'workprint', modifier=-8),
//     QualityComponent('source', 20, 'cam', '(?:hd)?cam', modifier=-7),
//     QualityComponent('source', 30, 'ts', '(?:hd)?ts|telesync', modifier=-6),
//     QualityComponent('source', 40, 'tc', 'tc|telecine', modifier=-5),
//     QualityComponent('source', 50, 'r5', 'r[2-8c]', modifier=-4),
//     QualityComponent('source', 60, 'hdrip', r'hd[\W_]?rip', modifier=-3),
//     QualityComponent('source', 70, 'ppvrip', r'ppv[\W_]?rip', modifier=-2),
//     QualityComponent('source', 80, 'preair', modifier=-1),
//     QualityComponent('source', 90, 'tvrip', r'tv[\W_]?rip'),
//     QualityComponent('source', 100, 'dsr', r'dsr|ds[\W_]?rip'),
//     QualityComponent('source', 110, 'sdtv', r'(?:[sp]dtv|dvb)(?:[\W_]?rip)?'),
//     QualityComponent('source', 120, 'dvdscr', r'(?:(?:dvd|web)[\W_]?)?scr(?:eener)?', modifier=0),
//     QualityComponent('source', 130, 'bdscr', 'bdscr(?:eener)?'),
//     QualityComponent('source', 140, 'webrip', r'web[\W_]?rip'),
//     QualityComponent('source', 150, 'hdtv', r'a?hdtv(?:[\W_]?rip)?'),
//     QualityComponent('source', 160, 'webdl', r'web(?:[\W_]?(dl|hd))?'),
//     QualityComponent('source', 170, 'dvdrip', r'dvd(?:[\W_]?rip)?'),
//     QualityComponent('source', 175, 'remux'),
//     QualityComponent('source', 180, 'bluray', r'(?:b[dr][\W_]?rip|blu[\W_]?ray(?:[\W_]?rip)?)'),

// fn codec(lex: &mut Lexer<Token>) -> Option<VideoCodec> {
//     let slice = lex.slice();
//     Some(VideoCodec::H265)
// }

#[derive(Logos, Debug, PartialEq, Clone, Display)]
enum Token {
    #[error(logos::skip)]
    Error,

    // Codec
    #[regex(r"divx", |_| VideoCodecKind::Divx)]
    #[regex(r"xvid", |_| VideoCodecKind::Xvid)]
    #[regex(r"vp9", |_| VideoCodecKind::Vp9)]
    #[regex(r"[hx].?264", |_| VideoCodecKind::H264)]
    #[regex(r"[hx].?265|hevc", |_| VideoCodecKind::H265)]
    VideoCodec(VideoCodecKind),

    // Resolution
    #[regex(r"360p?", |_| VideoResolutionKind::R360P)]
    #[regex(r"480p?", |_| VideoResolutionKind::R480P)]
    #[regex(r"540p?", |_| VideoResolutionKind::R540P)]
    #[regex(r"576p?", |_| VideoResolutionKind::R576P)]
    #[regex(r"(1280x)?720(p|hd)?x?([56]0)?", |_| VideoResolutionKind::R720P)]
    #[regex(r"(1920x)?1080p?x?([56]0)?", |_| VideoResolutionKind::R1080P)]
    #[regex(r"((3840x)?2160p?x?([56]0)?)|4k", |_| VideoResolutionKind::R2160P)]
    VideoResolution(VideoResolutionKind),

    // // Source
    #[regex(r"(?:b[dr][\W_]?rip|blu[\W_]?ray(?:[\W_]?rip)?)", |_| VideoSourceKind::BluRay)]
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
    video_codec: Option<VideoCodecKind>,
    video_resolution: Option<VideoResolutionKind>,
    video_color_range: Option<String>,
    audio_codec: Option<String>,
    audio_channels: Option<String>,
    source: Option<VideoSourceKind>,
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
    let mut result = ParseResult::new(v.clone());

    while let Some(token) = lex.next() {
        if token == Token::Error {
            continue;
        }

        //let token_string = Some(token.to_string());
        //let token_string = Some(token.into());

        match token {
            Token::VideoCodec(value) => {
                // result.video_codec = Some(value.to_string());
                result.video_codec = Some(value);
            }
            Token::VideoResolution(value) => {
                //result.video_resolution = Some(value.to_string());
                result.video_resolution = Some(value);
            }
            Token::VideoSource(value) => {
                //result.source = Some(value.to_string());
                result.source = Some(value);
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
        let title = "Tenet 2020 2160p UHD Webdl DTS-HD MA 5.1 DV x265-LEGi0N".to_string();
        let result = parse(&title);

        assert_eq!(
            result,
            ParseResult {
                file_name: title,
                video_codec: Some(VideoCodecKind::H265),
                video_resolution: Some(VideoResolutionKind::R2160P),
                source: Some(VideoSourceKind::Webdl),
                ..ParseResult::default()
            }
        );
    }
}
