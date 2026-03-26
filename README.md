# timings

`timings` is a small Rust CLI that behaves like the Linux/macOS `time` command for measuring the execution duration of another command.

It prints the elapsed wall-clock time (`real`) in a human-readable form and forwards `stdin/stdout/stderr` so the wrapped command behaves normally.

## Install / Build

```bash
cargo install timings-rs
```

## Usage

```text
timings <command> [args...]
timings -- <command> [args...]   # optional separator
```

The wrapped program runs as a child process. `timings` measures wall-clock time from right before spawning the child until the child exits.

## Output

The timing is written to `stderr` in the following format:

```text
real\t<duration>
```

Examples of `<duration>` formatting:

* `18.837ms`
* `1m05s`
* `1h1m01s`

## Exit Code

* If the wrapped command runs successfully, `timings` exits with the wrapped command's exit code.
* If the wrapped command cannot be executed:
  * returns `127` for `NotFound`
  * otherwise returns `1`

## Examples

### Basic

```powershell
timings cmd /C "echo hello"
```

Expected output:

```text
hello
real 18.837ms
```

### Measuring an arbitrary command

```powershell
timings powershell -NoProfile -Command "Start-Sleep -Milliseconds 120; 'done'"
```

## Notes

This tool currently reports only `real` (wall-clock time). If you want `user`/`sys` CPU timings too, tell me your preferred output format (GNU `time` vs BSD `time`) and I can extend it.
