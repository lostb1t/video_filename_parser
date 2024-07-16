use logos::{Logos};
use strum_macros::Display;


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

#[derive(Debug, Clone, PartialEq, Eq, Display)]
enum VideoSourceKind {
    Workprint,
    Cam,
    Telesync,
    Telecine,
    R5,
    HDRip,
    PPVRip,
    Preair,
    TVRip,
    DSR,
    SDTV,
    DVDscr,
    BDscr,
    Webrip,
    HDTV,
    Webdl,
    DVDrip,
    Remux,
    BluRay,
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
enum VideoColorRangeKind {
    #[strum(serialize = "8Bit")]
    C8Bit,
    #[strum(serialize = "10Bit")]
    C10Bit,
    HDRplus,
    HDR,
    DolbyVision,
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
enum AudioCodecKind {
    MP3,
    AAC,
    #[strum(serialize = "DD5.1")]
    DD51,
    AC3,
    #[strum(serialize = "DD+5.1")]
    DDPLUS51,
    FLAC,
    DTSHD,
    DTS,
    TRUEHD
}

#[derive(Logos, Debug, PartialEq, Clone, Display)]
#[logos(subpattern year = r"(1[89]|20)\d\d")]
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

    // Source
    #[regex(r"(?:b[dr][\W_]?rip|blu[\W_]?ray(?:[\W_]?rip)?)", |_| VideoSourceKind::BluRay)]
    #[regex(r"bluray?", |_| VideoSourceKind::BluRay)]
    #[regex(r"web(?:[\W_]?(dl|hd))?", |_| VideoSourceKind::Webdl)]
    VideoSource(VideoSourceKind),

    // Color Range
    #[regex(r"8[^\w]?bits?|hi8p?", |_| VideoColorRangeKind::C8Bit)]
    #[regex(r"10[^\w]?bits?|hi10p?", |_| VideoColorRangeKind::C10Bit)]
    #[regex(r"hdr(10)?[^\w]?(\+|p|plus)", |_| VideoColorRangeKind::HDRplus)]
    #[regex(r"hdr([^\w]?10)?", |_| VideoColorRangeKind::HDR)]
    #[regex(r"(dolby[^\w]?vision|dv|dovi)", |_| VideoColorRangeKind::DolbyVision)]
    VideoColorRange(VideoColorRangeKind),

    // Audio Codec
    #[regex(r"mp3", |_| AudioCodecKind::MP3)]
    #[regex(r"aac", |_| AudioCodecKind::AAC)]
    #[regex(r"dd5.1", |_| AudioCodecKind::DD51)]
    #[regex(r"ac3", |_| AudioCodecKind::AC3)]
    #[regex(r"dd[p+]", |_| AudioCodecKind::DDPLUS51)]
    #[regex(r"flac", |_| AudioCodecKind::FLAC)]
    #[regex(r"dts[\W_]?hd(?:[\W_]?ma)?", |_| AudioCodecKind::DTSHD)]
    #[regex(r"dts", |_| AudioCodecKind::DTS)]
    #[regex(r"truehd", |_| AudioCodecKind::TRUEHD)]
    AudioCodec(AudioCodecKind),

    // Audio Channels
    // TODO: Needs work
    #[regex(r"([765].?[01])")]
    AudioChannels,

    #[regex(r"(?&year)+", |lex| lex.slice().parse())]
    Year(i32),

    #[token(".")]
    Period,
    // r'(?:series|season|s)\s?(\d{1,4})(?:\s(?:.*\s)?)?(?:episode|ep|e|part|pt)\s?(\d{1,3}|%s)(?:\s?e?(\d{1,2}))?'
    // % roman_numeral_re,
    // r'(?:series|season)\s?(\d{1,4})\s(\d{1,3})\s?of\s?(?:\d{1,3})',
    // r'(\d{1,2})\s?x\s?(\d+)(?:\s(\d{1,2}))?',
    // r'(\d{1,3})\s?of\s?(?:\d{1,3})',
    // r'(?:episode|e|ep|part|pt)\s?(\d{1,3}|%s)' % roman_numeral_re,
    // r'part\s(%s)' % '|'.join(map(str, english_numbers)),
    // #[regex(r"(?i)s?(?P<short>\d+) ?[ex]|(?:season)(?:[^\d]|$)(?P<long>\d+)|s(?P<dash>\d+) - \d+|\.s(?P<collection>\d){1,2}\.", |lex| lex.slice().parse())]
    #[regex(r"S\d\dE\d\d", |lex| lex.slice().parse())]
    Episode(i32),
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct ParseResult {
    title: String,
    year: Option<i32>,
    episode: Option<i32>,
    video_codec: Option<VideoCodecKind>,
    video_resolution: Option<VideoResolutionKind>,
    video_color_range: Option<String>,
    audio_codec: Option<AudioCodecKind>,
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

// TODO implement an delimiter: Regex(@"(\s|\.|,|_|-|=|\|)+"

fn parse(v: &String) -> ParseResult {
    let s = &v.clone().to_lowercase();
    let mut lex = Token::lexer(&s);
    let mut result = ParseResult::new(v.clone());
    
    while let Some(token) = lex.next() {
        match token {
            Token::VideoCodec(value) => {
                result.video_codec = Some(value);
            }
            Token::VideoResolution(value) => {
                result.video_resolution = Some(value);
            }
            Token::VideoSource(value) => {
                result.source = Some(value);
            }
            Token::AudioCodec(value) => {
                result.audio_codec = Some(value);
            }
            Token::AudioCodec(value) => {
                result.audio_codec = Some(value);
            }
            Token::Year(value) => {
                result.year = Some(value);
            }
            Token::Episode(value) => {
                result.episode = Some(value);
            }
            _ => (),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(test)]
    use pretty_assertions::{assert_eq};

    #[test]
    fn generic_test() {
        let entries: Vec<ParseResult> = vec![ParseResult {
            file_name: "Tenet 2020 2160p UHD Webdl DTS-HD MA 5.1 x265-LEGi0N".to_string(),
            year: Some(2020),
            video_codec: Some(VideoCodecKind::H265),
            video_resolution: Some(VideoResolutionKind::R2160P),
            source: Some(VideoSourceKind::Webdl),
            audio_codec: Some(AudioCodecKind::DTSHD),
            ..ParseResult::default()
        },
        ParseResult {
            file_name: "Tenet.2020.2160p.UHD.Webdl.dd5.1.x265-LEGi0N".to_string(),
            year: Some(2020),
            video_codec: Some(VideoCodecKind::H265),
            video_resolution: Some(VideoResolutionKind::R2160P),
            source: Some(VideoSourceKind::Webdl),
            audio_codec: Some(AudioCodecKind::DD51),
            ..ParseResult::default()
        },
        ParseResult {
            file_name: "Sons.of.Anarchy.S03.720p.BluRay.CLUEREWARD".to_string(),
            video_resolution: Some(VideoResolutionKind::R720P),
            source: Some(VideoSourceKind::BluRay),
            ..ParseResult::default()
        }];
        for entry in entries {
            let result = parse(&entry.file_name);
            assert_eq!(
                result,
                entry
            );
        }
    }
}
