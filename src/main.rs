use clap::{Arg, Command};
use serde::Serialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use ttf_parser;

struct FontAnalyzer {
    data: Vec<u8>,
}

impl FontAnalyzer {
    fn new(path: &Path) -> Result<Self, std::io::Error> {
        let mut file = File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        Ok(FontAnalyzer { data })
    }

    fn analyze(&self) -> Result<FontInfo, FontAnalysisError> {
        let face =
            ttf_parser::Face::parse(&self.data, 0).map_err(|_| FontAnalysisError::InvalidFormat)?;

        let mut family_names = Vec::new();
        for name in face.names() {
            if name.name_id == ttf_parser::name_id::FULL_NAME && name.is_unicode() {
                if let Some(family_name) = name.to_string() {
                    let language = name.language();
                    family_names.push(format!(
                        "{} ({}, {})",
                        family_name,
                        language.primary_language(),
                        language.region()
                    ));
                }
            }
        }

        let post_script_name = face
            .names()
            .into_iter()
            .find(|name| name.name_id == ttf_parser::name_id::POST_SCRIPT_NAME && name.is_unicode())
            .and_then(|name| name.to_string());

        Ok(FontInfo {
            family_names,
            post_script_name,
            units_per_em: face.units_per_em(),
            ascender: face.ascender(),
            descender: face.descender(),
            line_gap: face.line_gap(),
            global_bounding_box: face.global_bounding_box(),
            number_of_glyphs: face.number_of_glyphs(),
            underline_metrics: face.underline_metrics(),
            x_height: face.x_height(),
            weight: face.weight(),
            width: face.width(),
            is_regular: face.is_regular(),
            is_italic: face.is_italic(),
            is_bold: face.is_bold(),
            is_oblique: face.is_oblique(),
            strikeout_metrics: face.strikeout_metrics(),
            subscript_metrics: face.subscript_metrics(),
            superscript_metrics: face.superscript_metrics(),
            permissions: face.permissions(),
            is_variable: face.is_variable(),
        })
    }
}

#[derive(Debug, Serialize)]
struct FontInfo {
    family_names: Vec<String>,
    post_script_name: Option<String>,
    units_per_em: u16,
    ascender: i16,
    descender: i16,
    line_gap: i16,
    #[serde(skip_serializing)]
    global_bounding_box: ttf_parser::Rect,
    number_of_glyphs: u16,
    #[serde(skip_serializing)]
    underline_metrics: Option<ttf_parser::LineMetrics>,
    x_height: Option<i16>,
    #[serde(skip_serializing)]
    weight: ttf_parser::Weight,
    #[serde(skip_serializing)]
    width: ttf_parser::Width,
    is_regular: bool,
    is_italic: bool,
    is_bold: bool,
    is_oblique: bool,
    #[serde(skip_serializing)]
    strikeout_metrics: Option<ttf_parser::LineMetrics>,
    #[serde(skip_serializing)]
    subscript_metrics: Option<ttf_parser::ScriptMetrics>,
    #[serde(skip_serializing)]
    superscript_metrics: Option<ttf_parser::ScriptMetrics>,
    #[serde(skip_serializing)]
    permissions: Option<ttf_parser::Permissions>,
    is_variable: bool,
}

#[derive(Debug)]
enum FontAnalysisError {
    InvalidFormat,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("Font Analyzer")
        .version("1.0")
        .author("Your Name")
        .about("Analyzes TTF and OTF font files")
        .arg(
            Arg::new("FILE")
                .help("The font file to analyze")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("json")
                .short('j')
                .long("json")
                .help("Output in JSON format")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let path = Path::new(matches.get_one::<String>("FILE").unwrap());
    let analyzer = FontAnalyzer::new(path)?;

    match analyzer.analyze() {
        Ok(info) => {
            if matches.get_flag("json") {
                println!("{}", serde_json::to_string_pretty(&info)?);
            } else {
                println!("Family names: {:?}", info.family_names);
                println!("PostScript name: {:?}", info.post_script_name);
                println!("Units per EM: {:?}", info.units_per_em);
                println!("Ascender: {}", info.ascender);
                println!("Descender: {}", info.descender);
                println!("Line gap: {}", info.line_gap);
                println!("Global bbox: {:?}", info.global_bounding_box);
                println!("Number of glyphs: {}", info.number_of_glyphs);
                println!("Underline: {:?}", info.underline_metrics);
                println!("X height: {:?}", info.x_height);
                println!("Weight: {:?}", info.weight);
                println!("Width: {:?}", info.width);
                println!("Regular: {}", info.is_regular);
                println!("Italic: {}", info.is_italic);
                println!("Bold: {}", info.is_bold);
                println!("Oblique: {}", info.is_oblique);
                println!("Strikeout: {:?}", info.strikeout_metrics);
                println!("Subscript: {:?}", info.subscript_metrics);
                println!("Superscript: {:?}", info.superscript_metrics);
                println!("Permissions: {:?}", info.permissions);
                println!("Variable: {:?}", info.is_variable);
            }
        }
        Err(e) => println!("Analysis failed: {:?}", e),
    }

    Ok(())
}
