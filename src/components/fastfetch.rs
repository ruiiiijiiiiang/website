use crate::backend::get_fastfetch_data;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_brands_icons::FaRust;
use dioxus_free_icons::icons::fa_solid_icons::FaFolder;

const NIXOS_ASCII_LARGE: &str = r#"          $1▗▄▄▄       $2▗▄▄▄▄    ▄▄▄▖
          $1▜███▙       $2▜███▙  ▟███▛
           $1▜███▙       $2▜███▙▟███▛
            $1▜███▙       $2▜██████▛
     $1▟█████████████████▙ $2▜████▛     $3▟▙
    $1▟███████████████████▙ $2▜███▙    $3▟██▙
           $6▄▄▄▄▖           $2▜███▙  $3▟███▛
          $6▟███▛             $2▜██▛ $3▟███▛
         $6▟███▛               $2▜▛ $3▟███▛
$6▟███████████▛                  $3▟██████████▙
$6▜██████████▛                  $3▟███████████▛
      $6▟███▛ $5▟▙               $3▟███▛
     $6▟███▛ $5▟██▙             $3▟███▛
    $6▟███▛  $5▜███▙           $3▝▀▀▀▀
    $6▜██▛    $5▜███▙ $4▜██████████████████▛
     $6▜▛     $5▟████▙ $4▜████████████████▛
           $5▟██████▙         $4▜███▙
          $5▟███▛▜███▙         $4▜███▙
         $5▟███▛  ▜███▙         $4▜███▙
         $5▝▀▀▀    ▀▀▀▀▘         $4▀▀▀▘"#;

const NIXOS_ASCII_SMALL: &str = r#"            $2___   __
     $1/¯\    $2\  \ /  ;
     $1\  \    $2\  v  /
  $1/¯¯¯   ¯¯¯¯\\   $2/  $3/\
 $6’————————————·$1\  $3\ /  ;
      $6/¯¯;      $1\ $3//  /_
$6_____/  /        $3‘/     \
$6\      /,        $3/  /¯¯¯¯
 $6¯¯/  // \      $3/__/
  $5.  / \  \$4·————————————.
   $5\/  /   $4\\_____   ___/
      $5/  ,  \     $4\  \
      $5\_/ \__\     $4\_/"#;

fn render_colored_logo(template: &str) -> Element {
    let mut lines = Vec::new();

    for line in template.lines() {
        let mut segments = Vec::new();
        let mut current_text = String::new();
        let mut current_class = "";

        let mut chars = line.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '$'
                && let Some(&digit_char) = chars.peek()
                && let Some(digit) = digit_char.to_digit(10)
                && (1..=6).contains(&digit)
            {
                if !current_text.is_empty() {
                    segments.push(rsx! {
                        span { class: "{current_class}", "{current_text}" }
                    });
                    current_text.clear();
                }
                current_class = match digit {
                    1 | 3 | 5 => "fastfetch-logo-nixos-c1",
                    2 | 4 | 6 => "fastfetch-logo-nixos-c2",
                    _ => "",
                };
                chars.next(); // Consume the digit
                continue;
            }
            current_text.push(c);
        }

        if !current_text.is_empty() {
            segments.push(rsx! {
                span { class: "{current_class}", "{current_text}" }
            });
        }

        lines.push(rsx! {
            span {
                for seg in segments {
                    {seg}
                }
                "\n"
            }
        });
    }

    rsx! {
        for line in lines {
            {line}
        }
    }
}

fn draw_braille_bar(pct: u8) -> String {
    let mut bar = String::new();
    let total_width = 15;

    let scale = 100.0 / total_width as f32;
    let val = (pct as f32 / scale).min(total_width as f32);
    let full = val.floor() as usize;
    let fraction = val - full as f32;

    for _ in 0..full {
        bar.push('⣿');
    }

    if full < total_width {
        let level = (fraction * 8.0).round() as usize;
        match level {
            0 => bar.push('⠤'),
            1 => bar.push('⡀'),
            2 => bar.push('⡄'),
            3 => bar.push('⡆'),
            4 => bar.push('⡇'),
            5 => bar.push('⣇'),
            6 => bar.push('⣧'),
            7 => bar.push('⣷'),
            _ => bar.push('⣿'),
        }

        let remaining = total_width - full - 1;
        for _ in 0..remaining {
            bar.push('⠤');
        }
    }

    bar
}

#[component]
pub fn FastfetchCard() -> Element {
    let data_resource = use_resource(get_fastfetch_data);

    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        let mut data_resource = data_resource;
        data_resource.restart();
    });

    let data_res = data_resource.read();
    let data = match &*data_res {
        Some(Ok(data)) => data,
        _ => return rsx! { div { "Loading system statistics..." } },
    };

    rsx! {
        div {
            class: "fastfetch-container",
            div {
                class: "fastfetch-prompt",
                span { class: "prompt-segment prompt-dir",
                    Icon {
                        icon: FaFolder,
                        width: 14,
                        height: 14,
                        class: "prompt-icon",
                    }
                    "website"
                }
                span { class: "prompt-separator", " on" }
                span { class: "prompt-segment prompt-git", "  main" }
                span { class: "prompt-separator", " via" }
                span { class: "prompt-segment prompt-rust",
                    Icon {
                        icon: FaRust,
                        width: 14,
                        height: 14,
                        class: "prompt-icon prompt-rust-icon",
                    }
                    "v1.96.1"
                }
                br {}
                span { class: "prompt-char", "❯ " }
                span { class: "prompt-cmd", "fastfetch" }
                span { class: "terminal-cursor" }
            }
            div {
                class: "fastfetch-layout",

            pre {
                class: "fastfetch-logo fastfetch-logo-large",
                {render_colored_logo(NIXOS_ASCII_LARGE)}
            }
            pre {
                class: "fastfetch-logo fastfetch-logo-small",
                {render_colored_logo(NIXOS_ASCII_SMALL)}
            }

            div {
                class: "fastfetch-info",

                div {
                    class: "fastfetch-info-grid",

                    div { class: "fastfetch-key", "OS" }
                    div { class: "fastfetch-val", "{data.os_name}" }

                    div { class: "fastfetch-key", "Kernel" }
                    div { class: "fastfetch-val", "{data.kernel}" }

                    div { class: "fastfetch-key", "Uptime" }
                    div { class: "fastfetch-val", "{data.uptime}" }

                    div { class: "fastfetch-key", "Packages" }
                    div { class: "fastfetch-val", "{data.packages} (nix)" }

                    div { class: "fastfetch-key", "Age" }
                    div { class: "fastfetch-val", "{data.os_age}" }

                    div { class: "fastfetch-key", "CPU" }
                    div { class: "fastfetch-val", "{data.cpu_model}" }

                    div { class: "fastfetch-key", "CPU Load" }
                    div {
                        class: "fastfetch-val",
                        style: "display: flex; align-items: center; gap: 0.5rem;",
                        span { class: "fastfetch-bar", "{draw_braille_bar(data.cpu_load)}" }
                        span { "{data.cpu_load}%" }
                    }

                    div { class: "fastfetch-key", "RAM" }
                    div {
                        class: "fastfetch-val",
                        style: "display: flex; align-items: center; gap: 0.5rem;",
                        span { class: "fastfetch-bar", "{draw_braille_bar(data.ram_pct)}" }
                        span { "{data.ram_pct}%" }
                    }

                    div { class: "fastfetch-key", "Disk" }
                    div {
                        class: "fastfetch-val",
                        style: "display: flex; align-items: center; gap: 0.5rem;",
                        span { class: "fastfetch-bar", "{draw_braille_bar(data.disk_pct)}" }
                        span { "{data.disk_pct}%" }
                    }

                    div { class: "fastfetch-key", "Fetched At" }
                    div { class: "fastfetch-val", "{data.fetched_at}" }
                }
            }
        }
        }
    }
}
