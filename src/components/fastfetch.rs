use crate::backend::get_fastfetch_data;
use dioxus::prelude::*;

const NIXOS_ASCII: &str = r#"          ▗▄▄▄       ▗▄▄▄▄    ▄▄▄▖
          ▜███▙       ▜███▙  ▟███▛
           ▜███▙       ▜███▙▟███▛
            ▜███▙       ▜██████▛
     ▟█████████████████▙ ▜████▛     ▟▙
    ▟███████████████████▙ ▜███▙    ▟██▙
           ▄▄▄▄▖           ▜███▙  ▟███▛
          ▟███▛             ▜██▛ ▟███▛
         ▟███▛               ▜▛ ▟███▛
▟███████████▛                  ▟██████████▙
▜██████████▛                  ▟███████████▛
      ▟███▛ ▟▙               ▟███▛
     ▟███▛ ▟██▙             ▟███▛
    ▟███▛  ▜███▙           ▝▀▀▀▀
    ▜██▛    ▜███▙ ▜██████████████████▛
     ▜▛     ▟████▙ ▜████████████████▛
           ▟██████▙         ▜███▙
          ▟███▛▜███▙         ▜███▙
         ▟███▛  ▜███▙         ▜███▙
         ▝▀▀▀    ▀▀▀▀▘         ▀▀▀▘"#;

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
    let data = use_loader(get_fastfetch_data)?();
    rsx! {
        div {
            class: "fastfetch-layout",

            pre {
                class: "fastfetch-logo",
                "{NIXOS_ASCII}"
            }

            div {
                class: "fastfetch-info",

                div {
                    class: "fastfetch-divider",
                }

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
                div {
                    class: "fastfetch-divider",
                }
            }
        }
    }
}
