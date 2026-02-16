use comrak::adapters::SyntaxHighlighterAdapter;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{self, Write};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::html::{IncludeBackground, styled_line_to_highlighted_html};
use syntect::parsing::{SyntaxDefinition, SyntaxSet};
use syntect::util::LinesWithEndings;

pub struct CustomHighlighter {
    syntax_set: SyntaxSet,
    theme: Theme,
}

impl CustomHighlighter {
    pub fn new(theme_name: &str) -> Self {
        let mut builder = SyntaxSet::load_defaults_newlines().into_builder();

        let nix_def = include_str!("../../nix-syntax.yml");
        let def = SyntaxDefinition::load_from_str(nix_def, true, Some("nix")).unwrap();
        builder.add(def);

        let syntax_set = builder.build();

        let theme_set = ThemeSet::load_defaults();
        let theme = theme_set
            .themes
            .get(theme_name)
            .expect("Theme not found")
            .clone();

        Self { syntax_set, theme }
    }
}

impl SyntaxHighlighterAdapter for CustomHighlighter {
    fn write_highlighted(
        &self,
        output: &mut dyn Write,
        lang: Option<&str>,
        code: &str,
    ) -> fmt::Result {
        let syntax = lang
            .and_then(|l| self.syntax_set.find_syntax_by_token(l))
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        let mut highlighter = HighlightLines::new(syntax, &self.theme);

        for line in LinesWithEndings::from(code) {
            let ranges = highlighter
                .highlight_line(line, &self.syntax_set)
                .map_err(|_| fmt::Error)?;

            let html = styled_line_to_highlighted_html(&ranges, IncludeBackground::No)
                .map_err(|_| fmt::Error)?;

            write!(output, "{}", html)?;
        }

        Ok(())
    }

    fn write_pre_tag(
        &self,
        output: &mut dyn Write,
        mut attributes: HashMap<&'static str, Cow<'_, str>>,
    ) -> fmt::Result {
        let style = "background-color: #2d2d2d;";
        attributes.insert("style", Cow::Borrowed(style));
        comrak::html::write_opening_tag(output, "pre", attributes)
    }

    fn write_code_tag(
        &self,
        output: &mut dyn Write,
        attributes: HashMap<&'static str, Cow<'_, str>>,
    ) -> fmt::Result {
        comrak::html::write_opening_tag(output, "code", attributes)
    }
}
