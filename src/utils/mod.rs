use chrono::Utc;
use time::macros::format_description;
use anyhow::Result;
use rand::Rng;
use lazy_static::lazy_static;
use regex::Regex;

pub mod config;
pub mod file;


// Rustfmt really wants to put some of my comments at the end of
// lines.
#[rustfmt::skip]
const TO_STRIP: [char; 28] = [
    '\u{00ad}', 
	// &iexcl and &iquest.
    '\u{00a1}', '\u{00bf}', 
	// Angle quotes.
    '\u{00ab}', '\u{00bb}', '\u{2039}', '\u{203a}', 
	// Curly quotes.
    '\u{2018}', '\u{2019}', '\u{201a}', '\u{201b}', '\u{201c}', '\u{201d}', '\u{201e}', '\u{201f}', '\u{2022}',
    // &copy, &reg, &deg, &hellip, and &trade.
    '\u{00a9}', '\u{00ae}', '\u{00b0}', '\u{2026}', '\u{2122}', 
	// Acute accents.
    '\u{00b4}', '\u{02ca}', '\u{0301}', '\u{0341}', 
	// Grave accent, macron, caron.
    '\u{0300}', '\u{0304}', '\u{030c}',
];

const TO_REWRITE: [&str; 10] = [
    "&nbsp;", "&#160;", "&ndash;", "&8211;", "&mdash;", "&#8212;", "\u{00a0}", "\u{2013}", "\u{2014}", "-",
];

macro_rules! big_collection {
    ( $ty:ident, $fnn:ident ) => {
        #[inline]
        fn $fnn() -> String {
            r"(".to_string()
                + &($ty
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join(r"|"))
                + &r")+".to_string()
        }
    };
}

macro_rules! mk_workspace {
	( $v:ident, $( $tag:ident, $rep:literal, )* ) => {
		$(let $v = $tag.replace_all(&$v, $rep); )*
	}
}

macro_rules! extra_lazy {
	( $( $y:ident, $r:expr, )* ) => {
		lazy_static! {
			$(static ref $y: Regex = Regex::new($r).unwrap();)*
		}
	}
}

big_collection!(TO_STRIP, mk_strip);
big_collection!(TO_REWRITE, mk_rewrite);

const SCRIPT_AND_STYLE: &str = r"(<script[^>]*?>.*?</script>|<style[^>]*?>.*?</style>)";

/// Sanitize a string and return an array of the atomized words, all
/// lowercased.  This function is here because there are other uses
/// for slugified titles than just as slugs, and clients may want to
/// limit the length of a slug, remove stopwords or just "a|an|the"
/// language articles, or other modifications.
pub fn sanitize_and_split(title: &str) -> Vec<String> {
    #[rustfmt::skip]
    extra_lazy! {
        STRIP_DANGEROUS_TAGS,
        SCRIPT_AND_STYLE,
        REMOVE_TAGS, r"<[^>]*?>",
        REMOVE_SOFT_PUNCT, &mk_strip(),
        REWRITE_SOFT_PUNCT, &mk_rewrite(),
        REMOVE_REMAINING_ENTITIES, r"&.+?;",
		REWRITE_ACCEPTABLE_PUNCT, r"[\.\?!;:_@\r\n]+",
        REMOVE_REMAINING_PUNCT, r"[^%\p{Alphabetic}0-9 -]+",
    }

    let workspace = title.to_string().to_lowercase();

    #[rustfmt::skip]
    mk_workspace!(
		workspace,
        STRIP_DANGEROUS_TAGS, "",
        REMOVE_TAGS, "",
        REMOVE_SOFT_PUNCT, "",
        REWRITE_SOFT_PUNCT, "-",
        REMOVE_REMAINING_ENTITIES, "",
        REWRITE_ACCEPTABLE_PUNCT, "-",
		REMOVE_REMAINING_PUNCT, "",
    );

    workspace
        .split(|c| c == ' ' || c == '-')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

/// Sanitize a string and return the string lowercased with a single
/// hyphen between the words.
pub fn slugify(title: &str) -> String {
    sanitize_and_split(title).join("-")
}


pub fn generate_unique_id() -> String {
    let timestamp = Utc::now().timestamp_millis();
    let random_bytes: [u8; 4] = rand::thread_rng().gen();
    let random_hex = random_bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    format!("{}{}", timestamp, random_hex)
}

pub fn now() -> Result<String> {
    let now = time::OffsetDateTime::now_utc();
    let x = now.format(format_description!("[year]-[month]-[day]"))?;
    Ok(x)
}


pub fn number_to_string(num: i64) -> String {
    let mut string = format!("{}", num);
    while string.len() < 4 {
      string.insert(0, '0');
    }
    string
}