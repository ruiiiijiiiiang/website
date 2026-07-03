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
        let mut real_lang = "plain";
        let mut filename: Option<&str> = None;
        let mut branch = "main";

        if let Some(l) = lang {
            let parts: Vec<&str> = l.split(':').collect();
            if !parts.is_empty() {
                real_lang = parts[0];
            }
            if parts.len() > 1 {
                filename = Some(parts[1]);
            }
            if parts.len() > 2 {
                branch = parts[2];
            }
        }

        if let Some(fname) = filename {
            let file_icon = match real_lang {
                "nix" => {
                    r#"<svg viewBox="0 0 24 24" width="12" height="12" fill="currentColor" style="display: inline-block; vertical-align: -2px; margin-right: 5px;"><path d="M7.352 1.592l-1.364.002L5.32 2.75l1.557 2.713-3.137-.008-1.32 2.34H14.11l-1.353-2.332-3.192-.006-2.214-3.865zm6.175 0l-2.687.025 5.846 10.127 1.341-2.34-1.59-2.765 2.24-3.85-.683-1.182h-1.336l-1.57 2.705-1.56-2.72zm6.887 4.195l-5.846 10.125 2.696-.008 1.601-2.76 4.453.016.682-1.183-.666-1.157-3.13-.008L21.778 8.1l-1.365-2.313zM9.432 8.086l-2.696.008-1.601 2.76-4.453-.016L0 12.02l.666 1.157 3.13.008-1.575 2.71 1.365 2.315L9.432 8.086zM7.33 12.25l-.006.01-.002-.004-1.342 2.34 1.59 2.765-2.24 3.85.684 1.182H7.35l.004-.006h.001l1.567-2.698 1.558 2.72 2.688-.026-.004-.006h.01L7.33 12.25zm2.55 3.93l1.354 2.332 3.192.006 2.215 3.865 1.363-.002.668-1.156-1.557-2.713 3.137.008 1.32-2.34H9.881Z"/></svg>"#
                }
                "age" => {
                    r#"<svg viewBox="0 0 448 512" width="11" height="12" fill="currentColor" style="display: inline-block; vertical-align: -2px; margin-right: 5px;"><path d="M144 144l0 48 160 0 0-48c0-44.2-35.8-80-80-80s-80 35.8-80 80zM80 192l0-48C80 64.5 144.5 0 224 0s144 64.5 144 144l0 48 16 0c35.3 0 64 28.7 64 64l0 192c0 35.3-28.7 64-64 64L64 512c-35.3 0-64-28.7-64-64L0 256c0-35.3 28.7-64 64-64l16 0z"/></svg>"#
                }
                "rust" | "rs" => {
                    r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 36 36" width="13" height="13" style="display: inline-block; vertical-align: -2px; margin-right: 5px;"><path fill="#A0041E" d="M6.96 20.637c.068.639-.543 1.228-1.368 1.315-.824.089-1.547-.357-1.615-.995-.068-.639.544-1.227 1.368-1.314.824-.089 1.547.356 1.615.994zm2.087 2.717c.125.818-1.756 2.544-2.576 2.669-.819.125-1.584-.438-1.708-1.257-.125-.818.58-1.14 1.398-1.265.819-.124 2.761-.965 2.886-.147zm1.783 2.104c.173.81-1.628 3.927-2.438 4.1-.811.173-1.645.146-1.817-.665-.173-.81.306-1.688 1.116-1.861.81-.174 2.966-2.384 3.139-1.574zm3.853.858c.165.811-1.338 4.354-2.15 4.519-.812.165-1.439.451-1.604-.36-.165-.812.261-1.975 1.006-2.58.644-.523 2.584-2.39(2.748)-1.579z"/><path fill="#A0041E" d="M3.925 18.869c.607.715 1.18 1.23.464 1.835-.716.606-1.747.162-2.354-.554-.605-.715-2.239-3.42-1.524-4.025.717-.606 2.809 2.029 3.414 2.744zm.33 4.88c.892.295 1.857.801 1.563 1.691-.294.891-1.328.991-2.219.698-.889-.295-3.772-1.691-3.478-2.581.295-.89 3.244-.102 4.134.192zm1.214 4.532c.905-.253 1.907-.283 2.159.619.252.903-.282 1.535-1.186 1.787-.902.251-4.342.727-4.594-.176-.251-.905 2.718-1.98 3.621-2.23zm4.348 3.188c.084-.937.644-1.669 1.577-1.585.934.085 1.258 1.025 1.173 1.96-.085.934.147 3.562-1.715 4.016-.912.221-1.121-3.46-1.035-4.391zM29.04 20.637c-.068.639-.543 1.228-1.367 1.315.824.089 1.547-.357 1.615-.995.068-.639-.544-1.227-1.367-1.314-.824-.089-1.547.356-1.615.994zm-2.087 2.717c-.125.818 1.757 2.544 2.575 2.669.819.125 1.584-.438 1.709-1.257s-.58-1.14-1.398-1.265c-.819-.124-2.761-.965-2.886-.147zm-1.783 2.104c-.173.81 1.628 3.927 2.438 4.1.81.173 1.644.146 1.816-.665.174-.81-.305-1.688-1.115-1.861-.81-.174-2.966-2.384-3.139-1.574zm-3.853.858c-.166.811 1.338 4.354 2.149 4.519.812.165 1.438.451 1.604-.36.164-.812-.262-1.975-1.007-2.58-.642-.523-2.582-2.39-2.746-1.579z"/><path fill="#A0041E" d="M32.075 18.869c-.607.715-1.18 1.23-.465 1.835.716.606 1.747.162 2.354-.554.605-.715 2.239-3.42 1.523-4.025-.715-.606-2.807 2.029-3.412 2.744zm-.33 4.88c-.892.295-1.857.801-1.562 1.691.293.891 1.328.991 2.219.698-.889-.295 3.771-1.691 3.477-2.581-.294-.89-3.244-.102-4.134.192zm-1.215 4.532c-.904-.253-1.906-.283-2.158.619-.252.903.282 1.535 1.185 1.787.902.251 4.343.727 4.594-.177.252-.904-2.717-1.979-3.621-2.229zm-4.347 3.188c-.084-.937-.645-1.669-1.577-1.585-.935.085-1.258 1.025-1.173 1.96.084.934-.147 3.562 1.715 4.016.912.221-1.121-3.46 1.035-4.391zM3.148 13.878c-.383-.856.001-1.86.857-2.242.856-.383 1.86.002 2.243.858.381.855 2.651 5.612 1.795 5.994-.855.382-4.513-3.755-4.895-4.61z"/><path fill="#A0041E" d="M3.994 12.034c1.221 2.956 8.341-3.341 8.803-6.281.462-2.939-.308-4.201-.694-4.5-.386-.299.144 1.435-1.187 3.306-1.053 1.482-7.766 5.434-6.922 7.475zm28.858 1.844c.384-.856-.001-1.86-.857-2.242-.857-.383-1.861.002-2.244.858-.381.855-2.65 5.612-1.794 5.994.855.382 4.513-3.755 4.895-4.61z"/><path fill="#A0041E" d="M32.007 12.034c-1.222 2.956-8.341-3.341-8.804-6.281-.461-2.939.309-4.201.694-4.5.386-.299-.144 1.435 1.187 3.306 1.054 1.482 7.766 5.434 6.923 7.475z"/><path fill="#BE1931" d="M6 22c0-2 2-10 12-10s12 8 12 10c-5 3-5.373 7-12 7s-6-4-12-7zm-1.677-9.777c-3.153.543-.358-8.141 1.883-10.099C8.446.166 10.863.207 11.321.374s-1.174 2.595-1.75 4.178c-.293.803-3.072 7.296-5.248 7.671zm27.354 0c3.152.543.358-8.141-1.882-10.099C27.555.166 25.139.207 24.68.374c-.459.167 1.174 2.595 1.75 4.178.293.803 3.071 7.296 5.247 7.671z"/><path fill="#A0041E" d="M17.032 12.136c.335 1.339-.045 1.588-.849 1.789-.804.201-1.727.278-2.061-1.061-.335-1.339.045-2.588.849-2.789.803-.201 1.726.721 2.061 2.061zm4.846.728c-.335 1.34-1.258 1.262-2.061 1.061-.804-.201-1.184-.45-.849-1.789.335-1.34 1.258-2.262 2.062-2.061.803.2 1.183 1.449.848 2.789z"/><circle fill="#292F33" cx="14.5" cy="9.5" r="1.5"/><circle fill="#292F33" cx="21.5" cy="9.5" r="1.5"/><path fill="#DD2E44" d="M9.053 21.529c-.14.236-3.053.732-2.303-.731s2.443.497 2.303.731z"/><path fill="#DD2E44" d="M9.891 20.124c-.218.225-3.188.391-1.922-1.404 1.265-1.793 2.234 1.082 1.922 1.404z"/><path fill="#DD2E44" d="M11.657 18.66c-.378.231-3.471-.501-1.407-1.932 1.872-1.296 1.906 1.626 1.407 1.932z"/><path fill="#DD2E44" d="M14.102 17.427c-1.008.299-3.378-1.302-.881-2.141 2.498-.839 1.889 1.842.881 2.141zm12.754 4.102c.141.235 3.053.731 2.303-.731-.75-1.463-2.443.497-2.303.731z"/><path fill="#DD2E44" d="M26.019 20.124c.218.225 3.188.391 1.922-1.404-1.266-1.793-2.235 1.082-1.922 1.404z"/><path fill="#DD2E44" d="M24.253 18.66c.378.231 3.471-.501 1.406-1.932-1.872-1.296-1.906 1.626-1.406 1.932z"/><path fill="#DD2E44" d="M21.808 17.427c1.008.299(3.378)-1.302.881-2.141-2.499-.839-1.89 1.842-.881 2.141z"/><path fill="#A0041E" d="M26.849 16.25c0 .414-2.189-2.25-8.849-2.25-6.661 0-8.848 2.664-8.848 2.25 0-.414 2.754-3.75 8.848-3.75 6.094 0 8.849 3.336 8.849 3.75z"/><path fill="#DD2E44" d="M10.793 24.433c0-.414 1.782 2.25 7.207 2.25s7.208-2.664 7.208-2.25c0 .414-2.244 3.75-7.208 3.75s-7.207-3.336-7.207-3.75z"/></svg>"##
                }
                "sh" | "bash" | "zsh" | "shell" => {
                    r#"<svg viewBox="0 0 24 24" width="12" height="12" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" style="display: inline-block; vertical-align: -2px; margin-right: 5px;"><polyline points="4 17 10 11 4 5"></polyline><line x1="12" y1="19" x2="20" y2="19"></line></svg>"#
                }
                _ => {
                    r#"<svg viewBox="0 0 384 512" width="9" height="12" fill="currentColor" style="display: inline-block; vertical-align: -2px; margin-right: 5px;"><path d="M0 64C0 28.7 28.7 0 64 0H224V128c0 17.7 14.3 32 32 32H384V448c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V64zm384 64H256V0L384 128z"/></svg>"#
                }
            };
            let close_btn = r#"<svg viewBox="0 0 384 512" width="10" height="10" fill="currentColor" style="display: inline-block; vertical-align: middle; margin-left: 8px; opacity: 0.6;"><path d="M342.6 150.6c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L192 210.7 86.6 105.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3L146.7 256 41.4 361.4c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0L192 301.3 297.4 406.6c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L237.3 256 342.6 150.6z"/></svg>"#;

            write!(
                output,
                "<div class=\"tui-header\"><div class=\"tui-tab active\">{}{}{}</div><div class=\"tui-header-fill\"></div></div>",
                file_icon, fname, close_btn
            )?;
        }

        let syntax = self
            .syntax_set
            .find_syntax_by_token(real_lang)
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

        if let Some(fname) = filename {
            let branch_icon = r#"<svg viewBox="0 0 448 512" width="11" height="12" fill="currentColor" style="display: inline-block; vertical-align: -2px; margin-right: 6px;"><path d="M80 104a24 24 0 1 0 0-48 24 24 0 1 0 0 48zm80-24c0 32.8-19.7 61-48 73.3l0 87.8c18.8-10.9 40.7-17.1 64-17.1l96 0c35.3 0 64-28.7 64-64l0-6.7C307.7 141 288 112.8 288 80c0-44.2 35.8-80 80-80s80 35.8 80 80c0 32.8-19.7 61-48 73.3l0 6.7c0 70.7-57.3 128-128 128l-96 0c-35.3 0-64 28.7-64 64l0 6.7c28.3 12.3 48 40.5 48 73.3c0 44.2-35.8 80-80 80s-80-35.8-80-80c0-32.8 19.7-61 48-73.3l0-6.7 0-198.7C19.7 141 0 112.8 0 80C0 35.8 35.8 0 80 0s80 35.8 80 80zm232 0a24 24 0 1 0 -48 0 24 24 0 1 0 48 0zM80 456a24 24 0 1 0 0-48 24 24 0 1 0 0 48z"/></svg>"#;
            write!(
                output,
                "<div class=\"tui-footer\"><span class=\"tui-sec-a\">NORMAL</span><span class=\"tui-sec-b\">{}{}</span><span class=\"tui-sec-c\">{}</span><span class=\"tui-sec-y\">utf-8</span><span class=\"tui-sec-z\">{}</span></div>",
                branch_icon,
                branch,
                fname,
                real_lang.to_uppercase()
            )?;
        }

        Ok(())
    }

    fn write_pre_tag(
        &self,
        output: &mut dyn Write,
        mut attributes: HashMap<&'static str, Cow<'_, str>>,
    ) -> fmt::Result {
        let bg = self
            .theme
            .settings
            .background
            .unwrap_or(syntect::highlighting::Color {
                r: 45,
                g: 45,
                b: 45,
                a: 255,
            });
        let style = format!("background-color: #{:02x}{:02x}{:02x};", bg.r, bg.g, bg.b);
        attributes.insert("style", Cow::Owned(style));
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
