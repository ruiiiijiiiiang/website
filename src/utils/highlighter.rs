use comrak::adapters::SyntaxHighlighterAdapter;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{self, Write};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::html::{IncludeBackground, styled_line_to_highlighted_html};
use syntect::parsing::{SyntaxDefinition, SyntaxSet};
use syntect::util::LinesWithEndings;

use dioxus::core::{TemplateAttribute, TemplateNode};
use dioxus_free_icons::IconShape;
use dioxus_free_icons::icons::fa_brands_icons::FaRust;
use dioxus_free_icons::icons::fa_solid_icons::{FaCodeBranch, FaFile, FaLock, FaTerminal, FaXmark};

fn render_template_node(node: &TemplateNode) -> String {
    match node {
        TemplateNode::Element {
            tag,
            attrs,
            children,
            ..
        } => {
            let mut attr_str = String::new();
            for attr in *attrs {
                if let TemplateAttribute::Static { name, value, .. } = attr {
                    attr_str.push_str(&format!(" {}=\"{}\"", name, value));
                }
            }
            let mut children_str = String::new();
            for child in *children {
                children_str.push_str(&render_template_node(child));
            }
            format!("<{}{}>{}</{}>", tag, attr_str, children_str, tag)
        }
        TemplateNode::Text { text } => text.to_string(),
        TemplateNode::Dynamic { .. } => String::new(),
    }
}

fn get_icon_svg_inner<I: IconShape>(icon: I) -> String {
    let mut inner_html = String::new();
    if let Ok(vnode) = icon.child_elements() {
        for root in vnode.template.roots {
            inner_html.push_str(&render_template_node(root));
        }
    }
    inner_html
}

fn render_icon_svg<I: IconShape>(icon: I, attrs: &str) -> String {
    let view_box = icon.view_box().to_string();
    let xmlns = icon.xmlns().to_string();
    let inner = get_icon_svg_inner(icon);
    format!(
        r#"<svg viewBox="{}" xmlns="{}" fill="currentColor" {}>{}</svg>"#,
        view_box, xmlns, attrs, inner
    )
}

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
                    r#"<svg viewBox="0 0 24 24" width="12" height="12" fill="currentColor" style="display: inline-block; vertical-align: -2px; margin-right: 5px;"><path d="M7.352 1.592l-1.364.002L5.32 2.75l1.557 2.713-3.137-.008-1.32 2.34H14.11l-1.353-2.332-3.192-.006-2.214-3.865zm6.175 0l-2.687.025 5.846 10.127 1.341-2.34-1.59-2.765 2.24-3.85-.683-1.182h-1.336l-1.57 2.705-1.56-2.72zm6.887 4.195l-5.846 10.125 2.696-.008 1.601-2.76 4.453.016.682-1.183-.666-1.157-3.13-.008L21.778 8.1l-1.365-2.313zM9.432 8.086l-2.696.008-1.601 2.76-4.453-.016L0 12.02l.666 1.157 3.13.008-1.575 2.71 1.365 2.315L9.432 8.086zM7.33 12.25l-.006.01-.002-.004-1.342 2.34 1.59 2.765-2.24 3.85.684 1.182H7.35l.004-.006h.001l1.567-2.698 1.558 2.72 2.688-.026-.004-.006h.01L7.33 12.25zm2.55 3.93l1.354 2.332 3.192.006 2.215 3.865 1.363-.002.668-1.156-1.557-2.713 3.137.008 1.32-2.34H9.881Z"/></svg>"#.to_string()
                }
                "age" => {
                    render_icon_svg(FaLock, r#"width="11" height="12" style="display: inline-block; vertical-align: -2px; margin-right: 5px;""#)
                }
                "rust" | "rs" => {
                    render_icon_svg(FaRust, r#"width="13" height="13" style="display: inline-block; vertical-align: -2px; margin-right: 5px;""#)
                }
                "sh" | "bash" | "zsh" | "shell" => {
                    render_icon_svg(FaTerminal, r#"width="12" height="12" style="display: inline-block; vertical-align: -2px; margin-right: 5px;""#)
                }
                _ => {
                    render_icon_svg(FaFile, r#"width="9" height="12" style="display: inline-block; vertical-align: -2px; margin-right: 5px;""#)
                }
            };
            let close_btn = render_icon_svg(
                FaXmark,
                r#"width="10" height="10" style="display: inline-block; vertical-align: middle; margin-left: 8px; opacity: 0.6;""#,
            );

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

        let total_lines = LinesWithEndings::from(code).count();
        let mut digits = 0;
        let mut temp = total_lines;
        while temp > 0 {
            digits += 1;
            temp /= 10;
        }
        let width = std::cmp::max(3, digits);

        for (i, line) in LinesWithEndings::from(code).enumerate() {
            let line_num = i + 1;
            let ranges = highlighter
                .highlight_line(line, &self.syntax_set)
                .map_err(|_| fmt::Error)?;

            let html = styled_line_to_highlighted_html(&ranges, IncludeBackground::No)
                .map_err(|_| fmt::Error)?;

            write!(
                output,
                r#"<span class="tui-line-no">{:>width$} </span>{}"#,
                line_num,
                html,
                width = width
            )?;
        }

        if let Some(fname) = filename {
            let branch_icon = render_icon_svg(
                FaCodeBranch,
                r#"width="11" height="12" style="display: inline-block; vertical-align: -2px; margin-right: 6px;""#,
            );
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
