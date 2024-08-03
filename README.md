# FC-Info-RS

## Introduction

This is a CLI tool for analyzing TTF and OTF font files.

## Installation

```bash
cargo install fc-info
```

## Usage

```bash
fc-info --help
```

```
Font Analyzer

USAGE:
    fc-info [OPTIONS] <FILE>...

ARGS:
    <FILE>...    The font file to analyze

OPTIONS:
    -j, --json    Output in JSON format
    -h, --help    Print help information
    -V, --version Print version information
```

### Examples

```bash
fc-info -j ./examples/test.ttf
```

```
Family names: ["VITRO CORE TTF (English, United States)", "비트로 코어 TTF (Korean, Korea)"]
PostScript name: Some("VITRO-CORE-TTF")
Units per EM: 1000
Ascender: 976
Descender: -353
Line gap: 0
Global bbox: Rect { x_min: -26, y_min: -468, x_max: 10077, y_max: 976 }
Number of glyphs: 3694
Underline: Some(LineMetrics { position: -190, thickness: 55 })
X height: Some(540)
Weight: Black
Width: Normal
Regular: true
Italic: false
Bold: false
Oblique: false
Strikeout: Some(LineMetrics { position: 250, thickness: 50 })
Subscript: Some(ScriptMetrics { x_size: 700, y_size: 650, x_offset: 0, y_offset: 140 })
Superscript: Some(ScriptMetrics { x_size: 700, y_size: 650, x_offset: 0, y_offset: 477 })
Permissions: Some(Restricted)
Variable: false
```

```bash
fc-info -j ./examples/test.otf -j
```

```
{
  "family_names": [
    "VITRO CORE OTF (English, United States)",
    "비트로 코어 OTF (Korean, Korea)"
  ],
  "post_script_name": "VITRO-CORE-OTF",
  "units_per_em": 1000,
  "ascender": 976,
  "descender": -353,
  "line_gap": 0,
  "number_of_glyphs": 3692,
  "x_height": 540,
  "is_regular": true,
  "is_italic": false,
  "is_bold": false,
  "is_oblique": false,
  "is_variable": false
}
```

## License

MIT
