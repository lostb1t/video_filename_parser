use pyo3::prelude::*;
use logos::Logos;

// enum VideoCodecKind {
//     X265,
//     X264,
//     H264,
//     H265,
//     WMV,
//     XVID,
//     DVDR,
// }

// struct VideoCodecRegex {
//     regex: String
//     kind: VideoCodecKind
// }

// enum VideoCodecKind {
//     X265 (VideoCodecDef),
// }

#[derive(Debug)]
enum VideoCodecKind {
    H265,
}

#[derive(Debug)]
enum ResolutionKind {
    R1080P,
}

#[derive(Debug)]
struct VideoCodecRegex {
    regex: String,
    kind: VideoCodecKind,
}


#[pyclass]
#[derive(Logos, Debug, PartialEq, Clone)]
enum Token {
    #[error(logos::skip)]
    Error,
    #[regex(r"[hx].?265|hevc")]
    H265,
    // #[regex(r"[hx].?264|hevc")]
    // H264,
    #[regex(r"(1920x)?1080p?x?([56]0)?")]
    R1080P,
    #[regex(r"((3840x)?2160p?x?([56]0)?)|4k")]
    R2160P,
    #[regex(r"(?:b[dr][\W_]?rip|blu[\W_]?ray(?:[\W_]?rip)?)")]
    Bluray,
    // #[regex(r"\[TaigaSubs\]")]
    // Title,
}

// impl fmt::Display for Token {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Token::R1080P => write!(f, "1080p"),
//         }
//     }
// }

#[derive(Debug, Default, Clone)]
#[pyclass]
struct ParseResult {
    #[pyo3(get, set)]
    title: String,
    #[pyo3(get, set)]
    video_codec: Option<Token>,
    #[pyo3(get, set)]
    file_name: String,
}

#[pymethods]
impl ParseResult {
    #[new]
    pub fn new(file_name: String) -> Self {
        ParseResult {file_name, ..ParseResult::default()}
    }
}

// impl Default for ParseResult {
//     fn default() -> ParseResult {
//         ParseResult {
//             title: Default::default(),
//             videoCodec: Default::default(),
//         }
//     }
// }

#[pyfunction]
fn parse(v: String) -> PyResult<ParseResult> {
    let s = v.clone().to_lowercase();
    let lex = Token::lexer(&s);

    let mut result = ParseResult::new(s.clone());
    for token in lex {
        if token == Token::Error {
            continue;
        }
        result.video_codec = Some(token);
        // dbg!(token);
        // println!("{:?}", token);
    }
    Ok(result)
}

#[pymodule]
fn video_filename_parser(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<ParseResult>()?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    Ok(())
}

// fn main() -> PolarsResult<()> {
//     //let df = example().unwrap();
//     let result = parse("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1920x1080_H.265_FLAC][1234ABCD].mkv".to_string());
//     dbg!(result);
//     Ok(())
// }
