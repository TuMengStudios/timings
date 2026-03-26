use std::env;
use std::io;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};





fn main() {
    let mut args = env::args_os();
    let _program = args.next();

    let Some(mut cmd) = args.next() else {
        eprintln!("{}", usage());
        exit_with_code(2);
    };

    // Support "timings -- <command> [args...]"
    if cmd == "--" {
        cmd = match args.next() {
            Some(c) => c,
            None => {
                eprintln!("{}", usage());
                exit_with_code(2);
            }
        };
    }

    let cmd_str = cmd.to_string_lossy().into_owned();
    let cmd_args: Vec<_> = args.collect();

    let start = Instant::now();
    let status = run_command(&cmd, &cmd_args);
    let elapsed = start.elapsed();

    // Match `time` behavior: print timing to stderr so stdout stays clean.
    eprintln!("real\t{}", format_duration_human(elapsed));

    match status {
        Ok(Some(code)) => exit_with_code(code),
        Ok(None) => exit_with_code(1),
        Err(e) => {
            eprintln!("timings: failed to run `{cmd_str}`: {e}");
            // If it's a "not found" style error, return 127.
            if e.kind() == io::ErrorKind::NotFound {
                exit_with_code(127);
            }
            exit_with_code(1);
        }
    }
}

fn run_command(cmd: &std::ffi::OsStr, args: &[std::ffi::OsString]) -> io::Result<Option<i32>> {
    let mut command = Command::new(cmd);
    command
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let status = command.status()?;
    Ok(status.code())
}

fn format_duration_human(d: Duration) -> String {
    let secs = d.as_secs();
    let nanos = d.subsec_nanos();

    // < 1s: show ms/us with precision but keep it compact.
    if secs == 0 {
        if nanos >= 1_000_000 {
            // milliseconds, include fractional ms based on remaining nanos.
            let ms = nanos / 1_000_000;
            let rem_nanos = nanos % 1_000_000;
            // Convert remaining nanos to fractional ms with 3 decimals (ms precision).
            // 1ms = 1_000_000ns, and 3 decimals means 1us granularity.
            let frac_ms = rem_nanos / 1_000; // 0..999
            // If frac_ms is 0 we just show integer ms.
            if frac_ms == 0 {
                return format!("{ms}ms");
            }
            return format!("{ms}.{frac_ms:03}ms");
        }

        // < 1ms: show microseconds with fractional part in "0.XXXus".
        let micros = nanos / 1_000;
        let rem_nanos = nanos % 1_000;
        if rem_nanos == 0 {
            return format!("{micros}us");
        }
        // Render remaining nanoseconds as 0..999 to get 3 decimal places of us.
        let frac = rem_nanos; // 0..999
        return format!("{micros}.{:03}us", frac);
    }

    // >= 1s: show h/m/s with millisecond precision.
    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;
    let millis = nanos / 1_000_000;

    if hours == 0 && minutes == 0 {
        // Keep it as "SS.mmm s" (not "SSs.mmm") for the common case.
        if millis > 0 {
            return format!("{seconds}.{millis:03}s");
        }
        return format!("{seconds}s");
    }

    let mut out = String::new();
    if hours > 0 {
        out.push_str(&format!("{hours}h"));
        out.push_str(&format!("{minutes}m"));
        out.push_str(&format!("{seconds:02}s"));
    } else {
        out.push_str(&format!("{minutes}m"));
        out.push_str(&format!("{seconds:02}s"));
    }

    if millis > 0 {
        // For h/m/s, append fractional part to seconds.
        out.push_str(&format!(".{:03}", millis));
    }

    out
}

fn usage() -> &'static str {
    "Usage: timings <command> [args...]\n\
Example: timings curl https://example.com\n\
Output: prints execution time (real) in human-readable form"
}

fn exit_with_code(code: i32) -> ! {
    // i32 -> process exit code (lowest 8 bits used by OS)
    std::process::exit(code);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_duration_seconds() {
        let d = Duration::from_secs(12);
        assert_eq!(format_duration_human(d), "12s");
    }

    #[test]
    fn format_duration_seconds_with_ms() {
        let d = Duration::from_millis(12_345);
        assert_eq!(format_duration_human(d), "12.345s");
    }

    #[test]
    fn format_duration_minutes() {
        let d = Duration::from_secs(65);
        assert_eq!(format_duration_human(d), "1m05s");
    }

    #[test]
    fn format_duration_hours() {
        let d = Duration::from_secs(3661);
        assert_eq!(format_duration_human(d), "1h1m01s");
    }
}
