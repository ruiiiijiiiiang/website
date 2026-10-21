use dioxus::prelude::*;

use crate::models::FastfetchData;

#[server]
pub async fn get_fastfetch_data() -> Result<FastfetchData, ServerFnError> {
    use std::fs;
    use std::process::Command;

    let cpuinfo = fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
    let (cpu_model, cores) = parse_cpu_info(&cpuinfo);
    let loadavg = fs::read_to_string("/proc/loadavg").unwrap_or_default();
    let cpu_load = parse_cpu_load(&loadavg, cores);
    let uptime = parse_uptime(&fs::read_to_string("/proc/uptime").unwrap_or_default());
    let ram_pct = parse_memory_usage(&fs::read_to_string("/proc/meminfo").unwrap_or_default());

    let disk_pct = Command::new("df")
        .args(["-h", "/"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|stdout| parse_disk_usage(&stdout))
        .unwrap_or(0);

    let os_name = parse_os_name(&fs::read_to_string("/etc/os-release").unwrap_or_default());

    let shell = std::env::var("SHELL").unwrap_or_else(|_| "Unknown".to_string());

    let os_age_days = fs::metadata("/etc/NIXOS")
        .or_else(|_| fs::metadata("/etc/hostname"))
        .or_else(|_| fs::metadata("/lost+found"))
        .and_then(|m| m.modified().or_else(|_| m.created()))
        .ok()
        .map(|time| {
            std::time::SystemTime::now()
                .duration_since(time)
                .map(|d| d.as_secs() / 86400)
                .unwrap_or(120)
        })
        .unwrap_or(120);
    let os_age = format!("{} days", os_age_days);

    let sys_packages = fs::read_dir("/run/current-system/sw/bin")
        .or_else(|_| fs::read_dir("/usr/bin"))
        .map(|entries| entries.count())
        .ok();

    let packages = match sys_packages {
        Some(count) if count > 0 => count.to_string(),
        _ => "Unknown".to_string(),
    };

    let fetched_at = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    Ok(FastfetchData {
        cpu_model,
        cpu_load,
        ram_pct,
        disk_pct,
        uptime,
        os_name,
        shell,
        os_age,
        packages,
        fetched_at,
    })
}

#[cfg(feature = "server")]
fn parse_cpu_info(info: &str) -> (String, usize) {
    let mut model = None;
    let mut cores = 0;
    for line in info.lines() {
        if let Some((key, value)) = line.split_once(':') {
            match key.trim() {
                "model name" if model.is_none() && !value.trim().is_empty() => {
                    model = Some(value.trim().to_string());
                }
                "processor" => cores += 1,
                _ => {}
            }
        }
    }
    (
        model.unwrap_or_else(|| "Unknown CPU".to_string()),
        cores.max(1),
    )
}

#[cfg(feature = "server")]
fn first_nonnegative_number(input: &str) -> f64 {
    input
        .split_whitespace()
        .next()
        .and_then(|value| value.parse::<f64>().ok())
        .filter(|value| value.is_finite() && *value >= 0.0)
        .unwrap_or(0.0)
}

#[cfg(feature = "server")]
fn parse_cpu_load(loadavg: &str, cores: usize) -> u8 {
    ((first_nonnegative_number(loadavg) / cores.max(1) as f64 * 100.0)
        .round()
        .min(100.0)) as u8
}

#[cfg(feature = "server")]
fn parse_uptime(input: &str) -> String {
    let seconds = first_nonnegative_number(input) as u64;
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    if days > 0 {
        format!("{days}d, {hours}h, {minutes}m")
    } else {
        format!("{hours}h, {minutes}m")
    }
}

#[cfg(feature = "server")]
fn parse_memory_usage(meminfo: &str) -> u8 {
    let field = |name| {
        meminfo.lines().find_map(|line| {
            let (key, value) = line.split_once(':')?;
            if key.trim() != name {
                return None;
            }
            value.split_whitespace().next()?.parse::<u64>().ok()
        })
    };
    let total = field("MemTotal").unwrap_or(0);
    if total == 0 {
        return 0;
    }
    let available = field("MemAvailable").unwrap_or(0);
    let used = total.saturating_sub(available);
    (u128::from(used) * 100 / u128::from(total)) as u8
}

#[cfg(feature = "server")]
fn parse_disk_usage(stdout: &str) -> u8 {
    stdout
        .lines()
        .nth(1)
        .and_then(|line| line.split_whitespace().nth(4))
        .and_then(|pct| pct.trim_end_matches('%').parse::<u8>().ok())
        .unwrap_or(0)
}

#[cfg(feature = "server")]
fn parse_os_name(content: &str) -> String {
    content
        .lines()
        .find_map(|line| line.strip_prefix("PRETTY_NAME="))
        .map(|name| name.trim_matches('"').to_string())
        .unwrap_or_else(|| "NixOS".to_string())
}

#[cfg(all(test, feature = "server"))]
mod tests {
    use super::*;

    #[test]
    fn cpu_info_reads_model_and_counts_processors() {
        let info = "processor\t: 0\nmodel name\t: Example CPU\n\nprocessor\t: 1\nmodel name\t: Example CPU\n";
        assert_eq!(parse_cpu_info(info), ("Example CPU".to_string(), 2));
    }

    #[test]
    fn cpu_info_handles_whitespace_and_preserves_model_colons() {
        let info = " processor : 0\n model name :  Example: CPU  \nprocessor_count: 8\n";
        assert_eq!(parse_cpu_info(info), ("Example: CPU".to_string(), 1));
    }

    #[test]
    fn cpu_info_has_fallbacks_for_missing_or_malformed_fields() {
        for info in [
            "",
            "unrecognized input",
            "model name : \nprocessor_count: 8",
        ] {
            assert_eq!(parse_cpu_info(info), ("Unknown CPU".to_string(), 1));
        }
        assert_eq!(
            parse_cpu_info("processor: 0\nprocessor: 1\n"),
            ("Unknown CPU".to_string(), 2)
        );
    }

    #[test]
    fn cpu_load_normalizes_rounds_and_caps() {
        assert_eq!(parse_cpu_load("1.00 2.00 3.00 1/100 42", 4), 25);
        assert_eq!(parse_cpu_load("1.02 0.00 0.00", 4), 26);
        assert_eq!(parse_cpu_load("8.00 0.00 0.00", 4), 100);
        assert_eq!(parse_cpu_load("0.50 0.00 0.00", 0), 50);
    }

    #[test]
    fn invalid_load_values_fall_back_to_zero() {
        for input in ["", "invalid", "-1.0", "NaN", "inf", "-inf"] {
            assert_eq!(parse_cpu_load(input, 4), 0, "{input:?}");
        }
    }

    #[test]
    fn uptime_formats_minute_hour_and_day_boundaries() {
        for (input, expected) in [
            ("0 12345", "0h, 0m"),
            ("59.99 0", "0h, 0m"),
            ("60 0", "0h, 1m"),
            ("3599 0", "0h, 59m"),
            ("3600 0", "1h, 0m"),
            ("86399 0", "23h, 59m"),
            ("86400 0", "1d, 0h, 0m"),
            ("90061.9 0", "1d, 1h, 1m"),
        ] {
            assert_eq!(parse_uptime(input), expected, "{input:?}");
        }
    }

    #[test]
    fn invalid_uptime_values_fall_back_to_zero() {
        for input in ["", "invalid", "-1", "NaN", "inf"] {
            assert_eq!(parse_uptime(input), "0h, 0m", "{input:?}");
        }
    }

    #[test]
    fn memory_usage_uses_available_memory_and_truncates_percentage() {
        assert_eq!(
            parse_memory_usage("MemFree: 10 kB\nMemAvailable: 250 kB\nMemTotal: 1000 kB\n"),
            75
        );
        assert_eq!(
            parse_memory_usage("MemTotal: 3 kB\nMemAvailable: 1 kB\n"),
            66
        );
        assert_eq!(
            parse_memory_usage("MemTotal: 1000 kB\nMemAvailable: 0 kB\n"),
            100
        );
    }

    #[test]
    fn memory_usage_handles_missing_invalid_and_zero_fields() {
        for input in [
            "",
            "MemAvailable: 250 kB",
            "MemTotal: invalid\nMemAvailable: 250 kB",
            "MemTotal: 0 kB\nMemAvailable: 0 kB",
            "MemTotal: 0 kB\nMemAvailable: 250 kB",
        ] {
            assert_eq!(parse_memory_usage(input), 0, "{input:?}");
        }
        assert_eq!(parse_memory_usage("MemTotal: 1000 kB"), 100);
        assert_eq!(
            parse_memory_usage("MemTotal: 1000 kB\nMemAvailable: invalid"),
            100
        );
    }

    #[test]
    fn memory_usage_cannot_underflow_or_overflow() {
        assert_eq!(
            parse_memory_usage("MemTotal: 100 kB\nMemAvailable: 200 kB"),
            0
        );
        assert_eq!(
            parse_memory_usage(&format!("MemTotal: {} kB\nMemAvailable: 0 kB", u64::MAX)),
            100
        );
    }

    #[test]
    fn disk_usage_reads_percentage_from_root_filesystem_row() {
        let output = "Filesystem Size Used Avail Use% Mounted on\n/dev/vda1 100G 42G 58G 42% /\n/dev/vda2 10G 9G 1G 90% /data\n";
        assert_eq!(parse_disk_usage(output), 42);
    }

    #[test]
    fn disk_usage_handles_missing_columns_and_invalid_percentages() {
        for output in [
            "",
            "Filesystem Size Used Avail Use% Mounted on\n",
            "header\n/dev/vda1 100G\n",
            "header\n/dev/vda1 100G 42G 58G unknown /\n",
            "header\n/dev/vda1 100G 42G 58G 256% /\n",
        ] {
            assert_eq!(parse_disk_usage(output), 0, "{output:?}");
        }
    }

    #[test]
    fn os_name_accepts_quoted_and_unquoted_values() {
        assert_eq!(
            parse_os_name("NAME=NixOS\nPRETTY_NAME=\"NixOS 26.05\"\n"),
            "NixOS 26.05"
        );
        assert_eq!(parse_os_name("PRETTY_NAME=Example OS\n"), "Example OS");
    }

    #[test]
    fn os_name_preserves_equals_in_values() {
        assert_eq!(parse_os_name("PRETTY_NAME=\"Example=OS\"\n"), "Example=OS");
        assert_eq!(parse_os_name("PRETTY_NAME=Example=OS\n"), "Example=OS");
    }

    #[test]
    fn os_name_falls_back_when_pretty_name_is_missing() {
        for input in [
            "",
            "NAME=Example OS\n",
            "PRETTY_NAME\n",
            "OTHER_PRETTY_NAME=Example\n",
        ] {
            assert_eq!(parse_os_name(input), "NixOS");
        }
    }
}
