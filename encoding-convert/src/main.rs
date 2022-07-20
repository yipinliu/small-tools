use clap::Parser;
use clap::ArgEnum;
use encoding::EncoderTrap;
use encoding::{DecoderTrap, EncodingRef, all};
use encoding::decode;
use encoding::label::encoding_from_whatwg_label;

#[derive(Parser)]
#[clap(author="yipinliu", version="0.0.1")]
struct Opts {
    #[clap(value_parser)]
    filename: String,
    #[clap(parse(try_from_str=parse_encoding))]
    dst_encoding: Box<EncodingRef>,
    #[clap(short='s', long="src", parse(try_from_str=parse_encoding))]
    src_encoding: Option<Box<EncodingRef>>,
    /// It will overwrite the input file if this option is not specified.
    #[clap(short='o', value_parser)]
    output_filename: Option<String>,
    /// The operation while invalid sequences are meet
    #[clap(arg_enum, value_parser, default_value_t=DecodeMode::Strict)]
    decode_mode: DecodeMode,
    /// The operation while invalid sequences are meet
    #[clap(arg_enum, value_parser, default_value_t=EncodeMode::Strict)]
    encode_mode: EncodeMode
}

fn parse_encoding(label: &str) -> Result<Box<EncodingRef>, &'static str> {
    encoding_from_whatwg_label(label).map(Box::new)
        .ok_or("Fail to parse encoding from input label")
}

#[derive(Debug)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum DecodeMode {
    /// Immediately fails on errors.
    /// Corresponds to WHATWG "fatal" error algorithm.
    Strict,
    /// Replaces an error with a U+FFFD (decoder).
    /// Corresponds to WHATWG "replacement" error algorithm.
    Replace,
    /// Silently ignores an error, effectively replacing it with an empty sequence.
    Ignore,
}

#[derive(Debug)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum EncodeMode {
    /// Immediately fails on errors.
    /// Corresponds to WHATWG "fatal" error algorithm.
    Strict,
    /// Replaces an error with `?` in given encoding.
    /// Note that this fails when `?` cannot be represented in given encoding.
    /// Corresponds to WHATWG "URL" error algorithms.
    Replace,
    /// Silently ignores an error, effectively replacing it with an empty sequence.
    Ignore,
    /// Replaces an error with XML numeric character references (e.g. `&#1234;`).
    /// The encoder trap fails when NCRs cannot be represented in given encoding.
    /// Corresponds to WHATWG "<form>" error algorithms.
    NcrEscape,
}

fn main() {
    let opts: Opts = Opts::parse();
    let decode_trap = match opts.decode_mode {
        DecodeMode::Strict => {DecoderTrap::Strict}
        DecodeMode::Replace => {DecoderTrap::Replace}
        DecodeMode::Ignore => {DecoderTrap::Replace}
    };
    let encode_trap = match opts.encode_mode {
        EncodeMode::Strict => EncoderTrap::Strict,
        EncodeMode::Replace => EncoderTrap::Replace,
        EncodeMode::Ignore => EncoderTrap::Ignore,
        EncodeMode::NcrEscape => EncoderTrap::NcrEscape,
    };
    let output_filename = opts.output_filename.as_ref().unwrap_or(&opts.filename);
    let fallback_decoding = opts.src_encoding
        .as_ref()
        .map(|box_ref| box_ref.as_ref())
        .unwrap_or(&(all::ERROR as EncodingRef));

    let content = std::fs::read(&opts.filename);
    let content = match content {
        Ok(x) => {x}
        Err(err) => {
            println!("Fail to read file: {}", err);
            return;
        }
    };
    let de_data = decode(&content, decode_trap, *fallback_decoding);
    println!("The encoding used to decode the file is {}", de_data.1.name());
    let data = match de_data.0 {
        Ok(data) => data,
        Err(_) => {
            println!("Fail to decode the file content!");
            return;
        }
    };
    let en_data = opts.dst_encoding.encode(&data, encode_trap);
    let en_data = match en_data {
        Ok(data) => data,
        Err(_) => {
            println!("Fail to encode the data to {}", opts.dst_encoding.name());
            return;
        }
    };
    std::fs::write(output_filename, &en_data)
        .expect("Fail to write file");
}

//
// #[allow(non_camel_case_types)]
// #[derive(Debug)]
// #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
// enum EncodingValue {
//     ASCII,
//     CP437,
//     ARMSCII_8,
//     IBM866,
//     ISO_8859_1,
//     ISO_8859_2,
//     ISO_8859_3,
//     ISO_8859_4,
//     ISO_8859_5,
//     ISO_8859_6,
//     ISO_8859_7,
//     ISO_8859_8,
//     ISO_8859_10,
//     ISO_8859_13,
//     ISO_8859_14,
//     ISO_8859_15,
//     ISO_8859_16,
//     KOI8_R,
//     KOI8_U,
//     MAC_ROMAN,
//     WINDOWS_874,
//     WINDOWS_1250,
//     WINDOWS_1251,
//     WINDOWS_1252,
//     WINDOWS_1253,
//     WINDOWS_1254,
//     WINDOWS_1255,
//     WINDOWS_1256,
//     WINDOWS_1257,
//     WINDOWS_1258,
//     MAC_CYRILLIC,
//     UTF_8,
//     UTF_16LE,
//     UTF_16BE,
//     WINDOWS_949,
//     EUC_JP,
//     WINDOWS_31J,
//     ISO_2022_JP,
//     GBK,
//     GB18030,
//     HZ,
//     BIG5_2003,
//     X_USER_DEFINED,
//     ISO_8859_8_I,
//     REPLACEMENT,
// }
//
// impl Sized for EncodingValue {}
//
// impl Into<EncodingRef> for EncodingValue {
//     fn into(self) -> EncodingRef {
//         match self {
//             EncodingValue::ASCII => {all::ASCII}
//             EncodingValue::CP437 => {all::CP437}
//             EncodingValue::ARMSCII_8 => {all::ARMSCII_8}
//             EncodingValue::IBM866 => {all::IBM866}
//             EncodingValue::ISO_8859_1 => {all::ISO_8859_1}
//             EncodingValue::ISO_8859_2 => {all::ISO_8859_2}
//             EncodingValue::ISO_8859_3 => {all::ISO_8859_3}
//             EncodingValue::ISO_8859_4 => {all::ISO_8859_4}
//             EncodingValue::ISO_8859_5 => {all::ISO_8859_5}
//             EncodingValue::ISO_8859_6 => {all::ISO_8859_6}
//             EncodingValue::ISO_8859_7 => {all::ISO_8859_7}
//             EncodingValue::ISO_8859_8 => {all::ISO_8859_8}
//             EncodingValue::ISO_8859_10 => {all::ISO_8859_10}
//             EncodingValue::ISO_8859_13 => {all::ISO_8859_13}
//             EncodingValue::ISO_8859_14 => {all::ISO_8859_14}
//             EncodingValue::ISO_8859_15 => {all::ISO_8859_15}
//             EncodingValue::ISO_8859_16 => {all::ISO_8859_16}
//             EncodingValue::KOI8_R => {all::KOI8_R}
//             EncodingValue::KOI8_U => {all::KOI8_U}
//             EncodingValue::MAC_ROMAN => {all::MAC_ROMAN}
//             EncodingValue::WINDOWS_874 => {all::WINDOWS_874}
//             EncodingValue::WINDOWS_1250 => {all::WINDOWS_1250}
//             EncodingValue::WINDOWS_1251 => {all::WINDOWS_1251}
//             EncodingValue::WINDOWS_1252 => {all::WINDOWS_1252}
//             EncodingValue::WINDOWS_1253 => {all::WINDOWS_1253}
//             EncodingValue::WINDOWS_1254 => {all::WINDOWS_1254}
//             EncodingValue::WINDOWS_1255 => {all::WINDOWS_1255}
//             EncodingValue::WINDOWS_1256 => {all::WINDOWS_1256}
//             EncodingValue::WINDOWS_1257 => {all::WINDOWS_1257}
//             EncodingValue::WINDOWS_1258 => {all::WINDOWS_1258}
//             EncodingValue::MAC_CYRILLIC => {all::MAC_CYRILLIC}
//             EncodingValue::UTF_8 => {all::UTF_8}
//             EncodingValue::UTF_16LE => {all::UTF_16LE}
//             EncodingValue::UTF_16BE => {all::UTF_16BE}
//             EncodingValue::WINDOWS_949 => {all::WINDOWS_949}
//             EncodingValue::EUC_JP => {all::EUC_JP}
//             EncodingValue::WINDOWS_31J => {all::WINDOWS_31J}
//             EncodingValue::ISO_2022_JP => {all::ISO_2022_JP}
//             EncodingValue::GBK => {all::GBK}
//             EncodingValue::GB18030 => {all::GB18030}
//             EncodingValue::HZ => {all::HZ}
//             EncodingValue::BIG5_2003 => {all::BIG5_2003}
//             EncodingValue::X_USER_DEFINED => {all::whatwg::X_USER_DEFINED}
//             EncodingValue::ISO_8859_8_I => {all::whatwg::ISO_8859_8_I}
//             EncodingValue::REPLACEMENT => {all::whatwg::REPLACEMENT}
//         }
//     }
// }