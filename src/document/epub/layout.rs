use std::path::PathBuf;
use fnv::FnvHashMap;
use geom::{Point, Rectangle, Edge};
use font::{FontFamily, Font, RenderPlan};
use super::dom::Node;
use hyphenation::Language;
use color::BLACK;

pub const DEFAULT_HYPH_LANG: &str = "en";

#[derive(Debug, Clone)]
pub struct RootData {
    pub start_offset: usize,
    pub spine_dir: PathBuf,
    pub page_rect: Rectangle,
    pub rect: Rectangle,
}

#[derive(Debug, Clone)]
pub struct StyleData {
    pub display: Display,
    pub width: i32,
    pub height: i32,
    pub margin: Edge,
    pub padding: Edge,
    pub start_x: i32,
    pub end_x: i32,
    pub retain_whitespace: bool,
    pub text_align: TextAlign,
    pub text_indent: i32,
    pub line_height: i32,
    pub language: Option<String>,
    pub font_kind: FontKind,
    pub font_style: FontStyle,
    pub font_weight: FontWeight,
    pub font_size: f32,
    pub font_features: Option<Vec<String>>,
    pub color: u8,
    pub letter_spacing: i32,
    pub vertical_align: i32,
    pub uri: Option<String>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Display {
    Block,
    Inline,
}

#[derive(Debug, Clone)]
pub struct ChildArtifact {
    pub sibling_style: SiblingStyle,
    pub rects: Vec<(usize, Rectangle)>,
}

#[derive(Debug, Clone)]
pub struct SiblingStyle {
    pub padding_bottom: i32,
    pub margin_bottom: i32,
}

#[derive(Debug, Clone)]
pub struct LineStats {
    pub width: i32,
    pub merged_width: i32,
    pub glues_count: usize,
    pub started: bool,
}

impl Default for LineStats {
    fn default() -> Self {
        LineStats {
            width: 0,
            merged_width: 0,
            glues_count: 0,
            started: false,
        }
    }
}

impl Default for SiblingStyle {
    fn default() -> Self {
        SiblingStyle {
            padding_bottom: 0,
            margin_bottom: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LoopContext<'a> {
    pub parent: Option<&'a Node>,
    pub sibling: Option<&'a Node>,
    pub sibling_style: SiblingStyle,
    pub is_first: bool,
    pub is_last: bool,
}

impl<'a> Default for LoopContext<'a> {
    fn default() -> Self {
        LoopContext {
            parent: None,
            sibling: None,
            sibling_style: SiblingStyle::default(),
            is_first: false,
            is_last: false,
        }
    }
}

impl Default for StyleData {
    fn default() -> Self {
        StyleData {
            display: Display::Block,
            width: 0,
            height: 0,
            margin: Edge::default(),
            padding: Edge::default(),
            start_x: 0,
            end_x: 0,
            retain_whitespace: false,
            text_align: TextAlign::Left,
            text_indent: 0,
            line_height: 0,
            language: None,
            font_kind: FontKind::Serif,
            font_style: FontStyle::Normal,
            font_weight: FontWeight::Normal,
            font_size: 0.0,
            font_features: None,
            color: BLACK,
            letter_spacing: 0,
            vertical_align: 0,
            uri: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum InlineMaterial {
    Text(TextMaterial),
    Image(ImageMaterial),
    Glue(GlueMaterial),
    Penalty(PenaltyMaterial),
    Box(i32),
    LineBreak,
}

#[derive(Debug, Clone)]
pub struct TextMaterial {
    pub offset: usize,
    pub text: String,
    pub style: StyleData,
}

#[derive(Debug, Clone)]
pub struct ImageMaterial {
    pub offset: usize,
    pub path: String,
    pub style: StyleData,
}

#[derive(Debug, Clone)]
pub struct GlueMaterial {
    pub width: i32,
    pub stretch: i32,
    pub shrink: i32,
}

#[derive(Debug, Clone)]
pub struct PenaltyMaterial {
    pub width: i32,
    pub penalty: i32,
    pub flagged: bool,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FontKind {
    Serif,
    SansSerif,
    Monospace,
    Cursive,
    Fantasy,
}

#[derive(Debug, Copy, Clone)]
pub enum FontStyle {
    Normal,
    Italic,
}

#[derive(Debug, Copy, Clone)]
pub enum FontWeight {
    Normal,
    Bold,
}

pub struct Fonts {
    pub serif: FontFamily,
    pub sans_serif: FontFamily,
    pub monospace: FontFamily,
    pub cursive: Font,
    pub fantasy: Font,
}

impl Fonts {
    pub fn get_mut(&mut self, font_kind: FontKind, font_style: FontStyle, font_weight: FontWeight) -> &mut Font {
        match font_kind {
            FontKind::Serif => {
                match (font_style, font_weight) {
                    (FontStyle::Normal, FontWeight::Normal) => &mut self.serif.regular,
                    (FontStyle::Normal, FontWeight::Bold) => &mut self.serif.bold,
                    (FontStyle::Italic, FontWeight::Normal) => &mut self.serif.italic,
                    (FontStyle::Italic, FontWeight::Bold) => &mut self.serif.bold_italic,
                }
            },
            FontKind::SansSerif => {
                match (font_style, font_weight) {
                    (FontStyle::Normal, FontWeight::Normal) => &mut self.sans_serif.regular,
                    (FontStyle::Normal, FontWeight::Bold) => &mut self.sans_serif.bold,
                    (FontStyle::Italic, FontWeight::Normal) => &mut self.sans_serif.italic,
                    (FontStyle::Italic, FontWeight::Bold) => &mut self.sans_serif.bold_italic,
                }
            },
            FontKind::Monospace => {
                match (font_style, font_weight) {
                    (FontStyle::Normal, FontWeight::Normal) => &mut self.monospace.regular,
                    (FontStyle::Normal, FontWeight::Bold) => &mut self.monospace.bold,
                    (FontStyle::Italic, FontWeight::Normal) => &mut self.monospace.italic,
                    (FontStyle::Italic, FontWeight::Bold) => &mut self.monospace.bold_italic,
                }
            },
            FontKind::Cursive => &mut self.cursive,
            FontKind::Fantasy => &mut self.fantasy,
        }
    }
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TextAlign {
    Left,
    Right,
    Center,
    Justify,
}

#[derive(Debug, Clone)]
pub enum ParagraphElement {
    Text(TextElement),
    Image(ImageElement),
    Nothing,
}

#[derive(Debug, Clone)]
pub struct TextElement {
    pub offset: usize,
    pub language: Option<String>,
    pub text: String,
    pub plan: RenderPlan,
    pub font_features: Option<Vec<String>>,
    pub font_kind: FontKind,
    pub font_style: FontStyle,
    pub font_weight: FontWeight,
    pub font_size: u32,
    pub letter_spacing: i32,
    pub vertical_align: i32,
    pub color: u8,
    pub uri: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ImageElement {
    pub offset: usize,
    pub width: i32,
    pub height: i32,
    pub scale: f32,
    pub vertical_align: i32,
    pub display: Display,
    pub edge: Edge,
    pub path: String,
    pub uri: Option<String>,
}

#[derive(Debug, Clone)]
pub enum DrawCommand {
    Text(TextCommand),
    Image(ImageCommand),
    Marker(usize),
}

#[derive(Debug, Clone)]
pub struct TextCommand {
    pub offset: usize,
    pub position: Point,
    pub text: String,
    pub plan: RenderPlan,
    pub font_kind: FontKind,
    pub font_style: FontStyle,
    pub font_weight: FontWeight,
    pub font_size: u32,
    pub color: u8,
    pub uri: Option<String>,
    pub rect: Rectangle,
}

#[derive(Debug, Clone)]
pub struct ImageCommand {
    pub offset: usize,
    pub position: Point,
    pub scale: f32,
    pub path: String,
    pub uri: Option<String>,
    pub rect: Rectangle,
}

impl DrawCommand {
    pub fn offset(&self) -> usize {
        match *self {
            DrawCommand::Text(TextCommand { offset, .. }) => offset,
            DrawCommand::Image(ImageCommand { offset, .. }) => offset,
            DrawCommand::Marker(offset) => offset,
        }
    }
}

pub fn collapse_margins(a: i32, b: i32) -> i32 {
    if a >= 0 && b >= 0 {
        a.max(b)
    } else if a < 0 && b < 0 {
        a.min(b)
    } else {
        a + b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hyph_lang() {
        assert_eq!(hyph_lang("zh-latn-pinyin"), Some(Language::Chinese));
        assert_eq!(hyph_lang("EN"), Some(Language::EnglishUS));
        assert_eq!(hyph_lang("en-GB"), Some(Language::EnglishGB));
        assert_eq!(hyph_lang("DE-ZZZ"), Some(Language::German1996));
        assert_eq!(hyph_lang("de-CH-uuu"), Some(Language::GermanSwiss));
        assert_eq!(hyph_lang("y"), None);
    }
}

pub fn hyph_lang(name: &str) -> Option<Language> {
    HYPHENATION_LANGUAGES.get(name).or_else(|| {
        HYPHENATION_LANGUAGES.get(name.to_lowercase().as_str())
    }).or_else(|| {
        let name_lc = name.to_lowercase();
        let mut s = name_lc.as_str();
        while let Some(index) = s.rfind('-') {
            s = &s[..index];
            let opt = HYPHENATION_LANGUAGES.get(s);
            if opt.is_some() {
                return opt;
            }
        }
        None
    }).cloned()
}

lazy_static! {
pub static ref HYPHENATION_LANGUAGES: FnvHashMap<&'static str, Language> = [
    ("af", Language::Afrikaans),
    ("hy", Language::Armenian),
    ("as", Language::Assamese),
    ("eu", Language::Basque),
    ("be", Language::Belarusian),
    ("bn", Language::Bengali),
    ("bg", Language::Bulgarian),
    ("ca", Language::Catalan),
    ("zh-latn-pinyin", Language::Chinese),
    ("cop", Language::Coptic),
    ("hr", Language::Croatian),
    ("cs", Language::Czech),
    ("da", Language::Danish),
    ("nl", Language::Dutch),
    ("en-gb", Language::EnglishGB),
    ("en-us", Language::EnglishUS),
    ("en", Language::EnglishUS),
    ("eo", Language::Esperanto),
    ("et", Language::Estonian),
    ("mul-ethi", Language::Ethiopic),
    ("fi", Language::Finnish),
    ("fr", Language::French),
    ("fur", Language::Friulan),
    ("gl", Language::Galician),
    ("ka", Language::Georgian),
    ("de", Language::German1996),
    ("de-1901", Language::German1901),
    ("de-1996", Language::German1996),
    ("de-ch-1901", Language::GermanSwiss),
    ("de-ch", Language::GermanSwiss),
    ("grc", Language::GreekAncient),
    ("el-monoton", Language::GreekMono),
    ("el-polyton", Language::GreekPoly),
    ("gu", Language::Gujarati),
    ("hi", Language::Hindi),
    ("hu", Language::Hungarian),
    ("is", Language::Icelandic),
    ("id", Language::Indonesian),
    ("ia", Language::Interlingua),
    ("ga", Language::Irish),
    ("it", Language::Italian),
    ("kn", Language::Kannada),
    ("kmr", Language::Kurmanji),
    ("la", Language::Latin),
    ("la-x-classic", Language::LatinClassic),
    ("la-x-liturgic", Language::LatinLiturgical),
    ("lv", Language::Latvian),
    ("lt", Language::Lithuanian),
    ("ml", Language::Malayalam),
    ("mr", Language::Marathi),
    ("mn-cyrl", Language::Mongolian),
    ("nb", Language::NorwegianBokmal),
    ("nn", Language::NorwegianNynorsk),
    ("oc", Language::Occitan),
    ("or", Language::Oriya),
    ("pi", Language::Pali),
    ("pa", Language::Panjabi),
    ("pms", Language::Piedmontese),
    ("pl", Language::Polish),
    ("pt", Language::Portuguese),
    ("ro", Language::Romanian),
    ("rm", Language::Romansh),
    ("ru", Language::Russian),
    ("sa", Language::Sanskrit),
    ("sr-cyrl", Language::SerbianCyrillic),
    ("sh-cyrl", Language::SerbocroatianCyrillic),
    ("sh-latn", Language::SerbocroatianLatin),
    ("cu", Language::SlavonicChurch),
    ("sk", Language::Slovak),
    ("sl", Language::Slovenian),
    ("es", Language::Spanish),
    ("sv", Language::Swedish),
    ("ta", Language::Tamil),
    ("te", Language::Telugu),
    ("th", Language::Thai),
    ("tr", Language::Turkish),
    ("tk", Language::Turkmen),
    ("uk", Language::Ukrainian),
    ("hsb", Language::Uppersorbian),
    ("cy", Language::Welsh)].iter().cloned().collect();

pub static ref EM_SPACE_RATIOS: FnvHashMap<char, f32> = [
    // Em space.
    ('\u{2003}', 1.0),
    // En space.
    ('\u{2002}', 0.5),
    // Three-per-em space.
    ('\u{2004}', 0.33),
    // Four-per-em space.
    ('\u{2005}', 0.25),
    // Six-per-em space.
    ('\u{2006}', 0.16)].iter().cloned().collect();

pub static ref WORD_SPACE_RATIOS: FnvHashMap<char, f32> = [
    // Tabulation
    ('\t', 4.0),
    // No-break space
    ('\u{00A0}', 1.0),
    // Narrow no-break space
    ('\u{202F}', 0.5),
    // Thin space.
    ('\u{2009}', 0.5),
    // Hair space.
    ('\u{200A}', 0.25)].iter().cloned().collect();
}

pub const FONT_SPACES: &str = " \u{2007}\u{2008}";

pub const SPECIAL_CHARS: &str = "-–—/@";

pub struct SpecialSplitter<'a> {
    text: &'a str,
    current: usize,
    next: usize,
}

impl<'a> SpecialSplitter<'a> {
    pub fn new(text: &'a str) -> SpecialSplitter<'a> {
        SpecialSplitter {
            text,
            current: 0,
            next:  text.find(|c| SPECIAL_CHARS.contains(c))
                       .and_then(|i| text[i..].find(|c| !SPECIAL_CHARS.contains(c))
                                              .map(|j| i + j))
                       .unwrap_or(text.len()),
        }
    }
}

impl<'a> Iterator for SpecialSplitter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        if self.current == self.text.len() {
            None
        } else {
            let current = self.current;
            let next = self.next;
            self.current = self.next;
            self.next = self.text[self.current..]
                            .find(|c| SPECIAL_CHARS.contains(c))
                            .and_then(|i| self.text[self.current+i..].find(|c| !SPECIAL_CHARS.contains(c))
                                              .map(|j| self.current + i + j))
                            .unwrap_or(self.text.len());
            Some(&self.text[current..next])
        }
    }
}
