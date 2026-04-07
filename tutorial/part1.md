# Building a CLI File Generator in Rust
## Part 1 — Your First CLI Tool with `clap`

> **Series overview:** In this series, we'll build `fgen` — a CLI tool that generates files (JSON, CSV, `.env`, TXT, and more) with default or custom content. We'll start simple and refactor as we go, learning Rust concepts naturally along the way.
>
> **This part covers:** Setting up the project, understanding `clap`, parsing CLI arguments, writing a file, handling errors, and then solving three progressive challenges — each building toward a final merged version.

---

## Prerequisites

Before we begin, make sure you have Rust installed. If not, install it via [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify your installation:

```bash
rustc --version   # e.g. rustc 1.77.0
cargo --version   # e.g. cargo 1.77.0
```

> **What is `cargo`?** Cargo is Rust's build tool and package manager — think of it like `npm` for Node.js or `pip` for Python. You'll use it for everything: creating projects, adding dependencies, building, and running your code.

---

## Step 1 — Create the Project

```bash
cargo new fgen
cd fgen
```

Cargo creates this structure for you:

```
fgen/
├── Cargo.toml      ← project manifest (name, version, dependencies)
└── src/
    └── main.rs     ← your program entry point
```

Open `Cargo.toml`. It looks like this:

```toml
[package]
name = "fgen"
version = "0.1.0"
edition = "2021"

[dependencies]
```

The `[dependencies]` section is where we'll add external libraries (called **crates** in Rust).

---

## Step 2 — Add `clap` as a Dependency

[`clap`](https://docs.rs/clap) is the most popular CLI argument parsing library in the Rust ecosystem. We'll use its **derive** feature, which lets us define our CLI arguments as a plain Rust struct — no boilerplate needed.

### The fast way — `cargo add`

Instead of editing `Cargo.toml` by hand, you can use the `cargo add` command directly from the terminal:

```bash
cargo add clap -F derive
```

Breaking this down:
- `cargo add clap` — tells Cargo to find and add the latest stable `clap` crate.
- `-F derive` — enables the `derive` feature (`-F` is short for `--features`).

After running it, your `Cargo.toml` will update automatically (version may differ):

```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
```

> **What does `features = ["derive"]` mean?** Crates can ship optional features to keep compile times low. The `derive` feature unlocks the `#[derive(Parser)]` macro we'll use shortly — it automatically generates argument-parsing code from our struct.

**Other useful `cargo add` patterns you'll encounter in this series:**

```bash
# Pin to a specific major version
cargo add clap@4 -F derive

# Add multiple features at once
cargo add clap -F derive -F env

# Add as a dev-only dependency (for test helpers, not in release binary)
cargo add some-crate --dev
```

Now download and compile the dependency:

```bash
cargo build
```

You'll see Cargo fetch and compile `clap` and its dependencies. This only happens once; subsequent builds are fast.

---

## Step 3 — Write the Tool

Now open `src/main.rs` and replace everything with the following. Read through it — every line is explained:

```rust
use clap::Parser;
use std::fs;
use std::path::PathBuf;

/// fgen — a simple file generator CLI
///
/// The triple-slash comments on the struct and fields become the --help text
/// automatically. Try running: cargo run -- --help
#[derive(Parser, Debug)]
#[command(
    name = "fgen",
    version = "0.1.0",
    about = "Generate files with custom content from the command line"
)]
struct Cli {
    /// Name of the file to create (e.g. notes.txt, config.json)
    #[arg(short, long)]
    name: String,

    /// Content to write into the file
    #[arg(short, long, default_value = "")]
    content: String,

    /// Directory where the file will be saved (defaults to current directory)
    #[arg(short, long, default_value = ".")]
    path: PathBuf,
}

fn main() {
    // Parse the CLI arguments into our `Cli` struct.
    // If the user passes invalid args or --help, clap handles it automatically.
    let args = Cli::parse();

    // Build the full file path by joining the directory and filename.
    // e.g. path="./output" + name="notes.txt" → "./output/notes.txt"
    let file_path = args.path.join(&args.name);

    // Attempt to write the file. `fs::write` creates the file if it doesn't
    // exist, or overwrites it if it does.
    match fs::write(&file_path, &args.content) {
        Ok(_) => {
            println!("✅ File created: {}", file_path.display());
        }
        Err(e) => {
            eprintln!("❌ Failed to create file: {}", e);
            std::process::exit(1);
        }
    }
}
```

---

## Step 4 — Understand the Code

Let's walk through the key concepts one by one.

### 4.1 — `use` statements

```rust
use clap::Parser;
use std::fs;
use std::path::PathBuf;
```

`use` brings items into scope — similar to `import` in Python or JavaScript. Here:
- `clap::Parser` is the trait (interface) that gives our struct the `.parse()` method.
- `std::fs` is Rust's standard library module for filesystem operations.
- `std::path::PathBuf` is an owned, mutable file path type — think of it as a smarter `String` that understands file paths cross-platform.

> **Common beginner mistake:** Using a type like `Path` without importing it causes `error[E0433]: failed to resolve: use of undeclared type Path`. Rust's compiler always tells you exactly how to fix this — look for the `help: consider importing this struct` line, which gives you the exact `use` statement to add.

When you need multiple items from the same module, you can group them in one line:

```rust
use std::path::{Path, PathBuf};  // instead of two separate `use` lines
```

### 4.2 — Deriving `Parser`

```rust
#[derive(Parser, Debug)]
struct Cli { ... }
```

`#[derive(...)]` is a **procedural macro** — it writes code for you at compile time. `Parser` generates all the argument parsing logic; `Debug` lets you print the struct with `{:?}` for debugging.

### 4.3 — Defining arguments with `#[arg(...)]`

```rust
#[arg(short, long)]
name: String,
```

- `short` → enables `-n`
- `long` → enables `--name`
- The field type (`String`, `PathBuf`) determines how the value is parsed.
- `default_value` makes an argument optional — if the user doesn't pass it, the default is used.

### 4.4 — Boolean flags and a critical gotcha

```rust
#[arg(short, long)]
overwrite: bool,
```

A `bool` field in `clap` is a **presence flag**. The flag's presence means `true`; its absence means `false`. You do **not** pass a value after it:

```bash
# ✅ Correct — flag presence = true
cargo run -- --name hello.txt --overwrite

# ❌ Wrong — clap sees "True" as an unexpected positional argument
cargo run -- --name hello.txt --overwrite True
#   error: unexpected argument 'True' found
```

This trips up almost everyone the first time. If you see `unexpected argument 'True'`, this is why. Run `cargo run --bin level_1 -- --help` and look at the Options section — boolean flags have no `<VALUE>` bracket next to them, confirming they're toggles not value-taking flags.

If you genuinely want users to type `--overwrite true`, you can use `value_parser = clap::value_parser!(bool)` in the `#[arg]` attribute — but the toggle style is the standard Rust convention.

### 4.5 — `PathBuf::join`

```rust
let file_path = args.path.join(&args.name);
```

`join` appends a filename to a path safely. It handles slashes correctly on both Unix and Windows — you never need to manually concatenate strings for paths.

### 4.6 — `match` and error handling

```rust
match fs::write(&file_path, &args.content) {
    Ok(_) => { println!(...) }
    Err(e) => { eprintln!(...); std::process::exit(1); }
}
```

In Rust, functions that can fail return a `Result<T, E>` type — it's either `Ok(value)` or `Err(error)`. There are no exceptions. The `match` expression forces you to handle both cases.

- `println!` writes to **stdout**.
- `eprintln!` writes to **stderr** — the right place for errors.
- `std::process::exit(1)` exits with a non-zero code, signalling failure to the shell.

---

## Step 5 — Build and Run

### Build the project

```bash
cargo build
```

The compiled binary lands at `./target/debug/fgen`.

### See the auto-generated help

```bash
cargo run -- --help
```

Output:
```
Generate files with custom content from the command line

Usage: fgen [OPTIONS] --name <NAME>

Options:
  -n, --name <NAME>       Name of the file to create (e.g. notes.txt, config.json)
  -c, --content <CONTENT> Content to write into the file [default: ]
  -p, --path <PATH>       Directory where the file will be saved [default: .]
  -h, --help              Print help
  -V, --version           Print version
```

> Notice how all the `///` doc comments became help text — zero extra work.

### Create a simple text file

```bash
cargo run -- --name hello.txt --content "Hello, world!"
# ✅ File created: ./hello.txt

cat hello.txt
# Hello, world!
```

### Use short flags

```bash
cargo run -- -n notes.txt -c "My first note"
```

### Save to a custom directory

```bash
mkdir output
cargo run -- --name config.json --content '{"debug": true}' --path ./output
# ✅ File created: output/config.json
```

### Create an empty file

```bash
cargo run -- --name empty.txt
# Since `content` has default_value = "", this works fine.
```

### Test the error path — write to a non-existent directory

```bash
cargo run -- --name test.txt --path ./does-not-exist
# ❌ Failed to create file: No such file or directory (os error 2)
```

---

## Step 6 — Install it Globally

Once you're happy with the tool, install it to your system PATH:

```bash
cargo install --path .
fgen --name todo.txt --content "- Buy milk"

# To uninstall later:
cargo uninstall fgen
```

---

## 🛠️ The Cargo Toolbelt

Before the challenges, here are four Cargo commands you'll use constantly throughout this series. Learn them now.

### `cargo build` — The Constructor
Compiles your code and checks for errors without running it. Use when you want to verify your code compiles without executing any logic.

### `cargo fmt` — The Stylist
Automatically reformats your code to follow official Rust style guidelines — spacing, indentation, line length. Run it before sharing or recording.

```bash
cargo fmt
```

> **Pro tip:** VS Code, Zed, and RustRover can all be configured to run `cargo fmt` on every save. Set this up now — it keeps code readable without thinking about it.

### `cargo clippy` — The Mentor
Clippy is Rust's linter. It catches code that compiles and runs but is unidiomatic or inefficient. For example, it'll suggest `.is_empty()` over `.len() == 0`, or flag unnecessary `.clone()` calls. Run it before committing code.

```bash
cargo clippy
```

### `cargo test` — The Safety Net
Runs every function annotated with `#[test]`. We'll write real tests from Part 2 onward, but know this command exists.

```bash
cargo test
```

---

## Extending the Tool

The following three challenges each add one feature to the base tool independently. After understanding each solution, we'll merge all three into one final version.

### 🧪 Your Challenge

Try extending the tool on your own before Part 2. Here are three levels:

**Level 1 — Easy**
Add a `--overwrite` flag (boolean). If the target file already exists and `--overwrite` is not passed, print a warning and exit without creating the file.
*Hint: look at `std::path::Path::exists()` and `#[arg(short, long)]` on a `bool` field.*

**Level 2 — Medium**
Add a `--uppercase` flag that transforms all content to uppercase before writing.
*Hint: `String` has a `.to_uppercase()` method.*

**Level 3 — Hard**
Create the output directory automatically if it doesn't exist, instead of returning an error.
*Hint: look at `std::fs::create_dir_all()`.*

### Organizing Challenge Code with `src/bin/`

Rather than editing and re-editing `main.rs`, we'll use Rust's built-in support for multiple binaries. Any `.rs` file placed inside `src/bin/` automatically becomes its own independently runnable binary — no extra configuration needed.

Create the directory:

```bash
mkdir src/bin
```

Your project structure:

```
fgen/
├── Cargo.toml
└── src/
    ├── main.rs          ← the original Part 1 tool (untouched)
    └── bin/
        ├── level_1.rs   ← challenge 1: --overwrite flag
        ├── level_2.rs   ← challenge 2: --uppercase flag
        ├── level_3.rs   ← challenge 3: auto-create directories
        └── final.rs     ← all three features merged
```

**Running a specific binary:**

```bash
cargo run --bin level_1 -- --name hello.txt --content "hi"
cargo run --bin level_2 -- --name hello.txt --content "hi"
cargo run --bin final   -- --name hello.txt --content "hi"
```

**The multi-binary problem and `default-run`:** Once you have files in `src/bin/`, `cargo run` alone becomes ambiguous — Cargo doesn't know which binary you mean and will error. Fix this by adding `default-run` to `Cargo.toml`:

```toml
[package]
name = "fgen"
version = "0.1.0"
edition = "2021"
default-run = "fgen"    # ← cargo run with no --bin uses src/main.rs

[dependencies]
clap = { version = "4.5", features = ["derive"] }
```

Now `cargo run -- --name hello.txt` always targets `main.rs`, and `cargo run --bin level_1 -- ...` explicitly picks a challenge file.

---

### Challenge 1 — The `--overwrite` Flag

**Goal:** If the target file already exists and `--overwrite` was not passed, print a warning and exit without touching the file.

**New concepts:** `Path::exists()`, guard clauses, `bool` flags.

**`src/bin/level_1.rs`:**

```rust
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "fgen",
    version = "0.1.0",
    about = "Generate files with custom content from the command line"
)]
struct Cli {
    /// Name of the file to create (e.g. notes.txt, config.json)
    #[arg(short, long)]
    name: String,

    /// Content to write into the file
    #[arg(short, long, default_value = "")]
    content: String,

    /// Directory where the file will be saved (defaults to current directory)
    #[arg(short, long, default_value = ".")]
    path: PathBuf,

    /// Overwrite the file if it already exists
    #[arg(short, long)]
    overwrite: bool, // Default is false — won't overwrite unless flag is passed
}

fn main() {
    let args = Cli::parse();
    let file_path = args.path.join(&args.name);

    // Guard clause: check for existence BEFORE attempting the write.
    // `file_path.exists()` returns true if a file or directory already
    // exists at that path. The `&&` short-circuits: if the file doesn't
    // exist, the second condition is never evaluated.
    if file_path.exists() && !args.overwrite {
        eprintln!(
            "❌ File already exists: {}. Use --overwrite to replace it.",
            file_path.display()
        );
        std::process::exit(1);
    }

    match fs::write(&file_path, &args.content) {
        Ok(_) => println!("✅ File created: {}", file_path.display()),
        Err(e) => {
            eprintln!("❌ Failed to create file: {}", e);
            std::process::exit(1);
        }
    }
}
```

**Test it:**

```bash
cargo run --bin level_1 -- --name hello.txt --content "Hello"
# ✅ File created: ./hello.txt

# Try again — file now exists
cargo run --bin level_1 -- --name hello.txt --content "New content"
# ❌ File already exists: ./hello.txt. Use --overwrite to replace it.

# Explicitly allow overwrite
cargo run --bin level_1 -- --name hello.txt --content "New content" --overwrite
# ✅ File created: ./hello.txt

cat hello.txt
# New content
```

---

### Challenge 2 — The `--uppercase` Flag

**Goal:** When `--uppercase` is passed, transform the content to uppercase before writing.

**New concepts:** `if` as an expression, `String::to_uppercase()`.

**`src/bin/level_2.rs`:**

```rust
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "fgen",
    version = "0.1.0",
    about = "Generate files with custom content from the command line"
)]
struct Cli {
    /// Name of the file to create (e.g. notes.txt, config.json)
    #[arg(short, long)]
    name: String,

    /// Content to write into the file
    #[arg(short, long, default_value = "")]
    content: String,

    /// Directory where the file will be saved (defaults to current directory)
    #[arg(short, long, default_value = ".")]
    path: PathBuf,

    /// Convert content to uppercase before writing
    #[arg(short, long)]
    uppercase: bool, // Default is false — no transformation unless flag is passed
}

fn main() {
    let args = Cli::parse();
    let file_path = args.path.join(&args.name);

    // In Rust, `if` is an expression — it produces a value directly.
    // This lets us write conditional logic as a single binding rather
    // than declaring a `mut` variable and mutating it in two branches.
    let content_to_write = if args.uppercase {
        args.content.to_uppercase() // allocates and returns a new String
    } else {
        args.content                // moves the original String — no copy
    };

    match fs::write(&file_path, &content_to_write) {
        Ok(_) => println!("✅ File created: {}", file_path.display()),
        Err(e) => {
            eprintln!("❌ Failed to create file: {}", e);
            std::process::exit(1);
        }
    }
}
```

**What's new — `if` as an expression:**

In many languages, `if` is a statement that controls flow. In Rust, `if` is an *expression* that produces a value. This means you can assign it directly to a `let` binding. Both branches must return the same type — here, both return a `String`. This pattern is idiomatic Rust: prefer expressions over mutation.

**Test it:**

```bash
cargo run --bin level_2 -- --name shout.txt --content "hello world" --uppercase
cat shout.txt
# HELLO WORLD

cargo run --bin level_2 -- --name normal.txt --content "hello world"
cat normal.txt
# hello world
```

---

### Challenge 3 — Auto-Create Directories

**Goal:** If the output directory doesn't exist, create it automatically rather than returning an error.

**New concepts:** `Path::parent()`, `Option<T>`, `unwrap_or_else`, `fs::create_dir_all`.

**`src/bin/level_3.rs`:**

```rust
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};  // grouped import — two types, one `use` line

#[derive(Parser, Debug)]
#[command(
    name = "fgen",
    version = "0.1.0",
    about = "Generate files with custom content from the command line"
)]
struct Cli {
    /// Name of the file to create (e.g. notes.txt, config.json)
    #[arg(short, long)]
    name: String,

    /// Content to write into the file
    #[arg(short, long, default_value = "")]
    content: String,

    /// Directory where the file will be saved (defaults to current directory)
    #[arg(short, long, default_value = ".")]
    path: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let file_path = args.path.join(&args.name);

    // `.parent()` returns `Option<&Path>`:
    //   Some(dir) → there is a parent directory component in the path
    //   None      → the path is a bare filename like "hello.txt" with no dir
    //
    // `unwrap_or_else` extracts the value inside Some, or calls the closure
    // to produce a fallback when None. We fall back to "." (current dir).
    let output_dir = file_path.parent().unwrap_or_else(|| Path::new("."));

    // `create_dir_all` is like `mkdir -p` — creates the full directory tree
    // and succeeds silently if it already exists. We only call it when needed.
    if !output_dir.exists() {
        fs::create_dir_all(output_dir).expect("❌ Failed to create directories");
    }

    match fs::write(&file_path, &args.content) {
        Ok(_) => println!("✅ File created: {}", file_path.display()),
        Err(e) => {
            eprintln!("❌ Failed to create file: {}", e);
            std::process::exit(1);
        }
    }
}
```

**What's new — `Option<T>`:**

`Path::parent()` returns `Option<&Path>`, not a plain path. `Option` is Rust's way of expressing "this value might not exist" — there is no `null`. It has two variants: `Some(value)` when a value is present, and `None` when it isn't. `unwrap_or_else` is one of many methods on `Option` that lets you safely extract the value or provide a fallback. We'll see `Option` everywhere in Rust — this is your first real encounter with it.

**Test it:**

```bash
# Directory doesn't exist — level_3 creates it automatically
cargo run --bin level_3 -- --name notes.txt --content "auto-dir!" --path ./deep/nested/path
# ✅ File created: deep/nested/path/notes.txt

ls deep/nested/path/
# notes.txt
```

Compare to `main.rs` on the same command:
```
❌ Failed to create file: No such file or directory (os error 2)
```

---

### Final — All Three Features Merged

**`src/bin/final.rs`:**

```rust
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};

/// fgen — a simple file generator CLI
#[derive(Parser, Debug)]
#[command(
    name = "fgen",
    version = "0.1.0",
    about = "Generate files with custom content from the command line"
)]
struct Cli {
    /// Name of the file to create (e.g. notes.txt, config.json)
    #[arg(short, long)]
    name: String,

    /// Content to write into the file
    #[arg(short, long, default_value = "")]
    content: String,

    /// Directory where the file will be saved (defaults to current directory)
    #[arg(short, long, default_value = ".")]
    path: PathBuf,

    /// Overwrite the file if it already exists
    #[arg(short, long)]
    overwrite: bool,

    /// Convert content to uppercase before writing
    #[arg(short, long)]
    uppercase: bool,
}

fn main() {
    let args = Cli::parse();
    let file_path = args.path.join(&args.name);

    // Step 1: Ensure the output directory exists.
    // This runs before the overwrite check — if the directory doesn't exist
    // yet, the file can't exist either, so the guard below would be meaningless.
    let output_dir = file_path.parent().unwrap_or_else(|| Path::new("."));
    if !output_dir.exists() {
        fs::create_dir_all(output_dir).expect("❌ Failed to create directories");
    }

    // Step 2: Guard against accidental overwrites.
    if file_path.exists() && !args.overwrite {
        eprintln!(
            "❌ File already exists: {}. Use --overwrite to replace it.",
            file_path.display()
        );
        std::process::exit(1);
    }

    // Step 3: Optionally transform content.
    let content_to_write = if args.uppercase {
        args.content.to_uppercase()
    } else {
        args.content
    };

    // Step 4: Write the file.
    match fs::write(&file_path, &content_to_write) {
        Ok(_) => println!("✅ File created: {}", file_path.display()),
        Err(e) => {
            eprintln!("❌ Failed to create file: {}", e);
            std::process::exit(1);
        }
    }
}
```

**The order matters.** The four steps are sequenced deliberately:

1. **Create directory first** — if we checked for the file *before* creating the directory, the `.exists()` check would be meaningless (the file obviously can't exist in a directory that doesn't exist yet).
2. **Overwrite guard second** — only meaningful once the directory is confirmed to exist.
3. **Transform content third** — pure data processing, no I/O side effects.
4. **Write last** — only after all preconditions pass.

This "validate before mutating" pattern is a good habit. Check everything that can fail, bail out early if needed, then do the actual work at the end.

**Test the merged version:**

```bash
# All three features together
cargo run --bin final -- \
  --name hello.txt \
  --content "hello world" \
  --path ./new/dir \
  --uppercase
# ✅ File created: new/dir/hello.txt

cat new/dir/hello.txt
# HELLO WORLD

# Attempt to overwrite without the flag
cargo run --bin final -- --name hello.txt --path ./new/dir --content "different"
# ❌ File already exists: new/dir/hello.txt. Use --overwrite to replace it.

# Overwrite explicitly
cargo run --bin final -- --name hello.txt --path ./new/dir --content "different" --overwrite
# ✅ File created: new/dir/hello.txt
```

---

## Complete File Reference

### Final `Cargo.toml`

```toml
[package]
name = "fgen"
version = "0.1.0"
edition = "2021"
default-run = "fgen"

[dependencies]
clap = { version = "4.5", features = ["derive"] }
```

### Final project structure

```
fgen/
├── Cargo.toml
└── src/
    ├── main.rs          ← original Part 1 (name, content, path)
    └── bin/
        ├── level_1.rs   ← adds --overwrite
        ├── level_2.rs   ← adds --uppercase
        ├── level_3.rs   ← adds auto directory creation
        └── final.rs     ← all features merged, correctly ordered
```

---

## What We Built

| Feature | Binary | Key concept |
|---|---|---|
| Basic file creation | `main.rs` | `fs::write`, `PathBuf::join`, `match Result` |
| Overwrite guard | `level_1.rs` | `Path::exists()`, `bool` flag, guard clause |
| Uppercase content | `level_2.rs` | `String::to_uppercase()`, `if` as expression |
| Auto-create dirs | `level_3.rs` | `fs::create_dir_all`, `Path::parent()`, `Option<T>` |
| All features | `final.rs` | All of the above, steps ordered correctly |

---

## What's Next

In **Part 2**, we'll take `final.rs` and refactor it into a proper multi-file Rust project using modules. We'll understand `mod` from first principles, split the code across `cli.rs`, `template.rs`, and `writer.rs`, and restructure everything as a **Cargo workspace** so each part of this series lives as its own crate with shared dependencies. We'll also add file template support — JSON skeleton, `.env` format, CSV headers — triggered either by the file's extension or an explicit `--template` flag.

---

*Part of the **Building a CLI File Generator in Rust** series.*
*Part 1 → Part 2: Modules, Workspaces & File Templates → Part 3: Subcommands & Error Types*



