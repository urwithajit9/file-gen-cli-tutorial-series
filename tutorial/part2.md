# Building a CLI File Generator in Rust
## Part 2 — Modules, Multi-File Structure, Cargo Workspaces & File Templates

> **Previously in Part 1:** We built a single `main.rs` that accepts `--name`, `--content`, and `--path` and writes a file. It works, but everything lives in one file. As we add features, that gets messy fast.
>
> **This part covers:**
> 1. What Rust modules are and how they work
> 2. Splitting one file into many with a clean structure
> 3. Using a **Cargo workspace** to organize every part of this series
> 4. Auto-detecting file format from extension
> 5. Adding a `--template` flag for explicit control
> 6. Understanding when to use each approach and how both can coexist

---

## Section 1 — Understanding Rust Modules

Before touching any files, let's understand `mod` properly. This is one of the concepts that trips up beginners the most, so we'll go slow.

### What problem does `mod` solve?

In Part 1, `main.rs` had everything: CLI struct, file writing, error handling. That's fine at 40 lines. At 400 lines it becomes hard to navigate. At 4000 lines it becomes impossible.

Modules let you split code into named scopes — and those scopes map directly to files. You decide what's public (usable from outside) and what's private (internal only).

### Module basics — all in one file first

Before we split into files, see how modules work conceptually:

```rust
// src/main.rs

mod writer {
    // Private by default — only code inside this module can use it
    fn build_path(dir: &str, name: &str) -> String {
        format!("{}/{}", dir, name)
    }

    // `pub` makes it visible to the outside world
    pub fn write_file(dir: &str, name: &str, content: &str) {
        let path = build_path(dir, name); // can call private fn from inside
        std::fs::write(&path, content).unwrap();
    }
}

fn main() {
    // Access a public item from another module using ::
    writer::write_file(".", "hello.txt", "hi");

    // This would NOT compile — build_path is private:
    // writer::build_path(".", "hello.txt");
}
```

Key rules:
- Everything inside a `mod` block is **private by default**.
- `pub` makes something accessible from outside the module.
- You access items with `::` — `module_name::item_name`.
- Modules can be nested: `mod a { mod b { pub fn c() {} } }` → `a::b::c()`.

### The `use` keyword with modules

Writing `writer::write_file(...)` every time gets repetitive. `use` creates a local alias:

```rust
use writer::write_file;

fn main() {
    write_file(".", "hello.txt", "hi"); // no prefix needed
}
```

Or bring in the whole module:

```rust
use writer;

fn main() {
    writer::write_file(".", "hello.txt", "hi");
}
```

### Moving a module into its own file

When a module grows large enough, you move it to its own file. The rule is simple:

> A `mod foo;` declaration in a file tells Rust: *"look for the contents of this module in `foo.rs` (or `foo/mod.rs`) next to this file."*

So instead of:

```rust
// main.rs
mod writer {
    pub fn write_file(...) { ... }
}
```

You write:

```rust
// main.rs
mod writer;  // ← semicolon, not a block. Rust finds src/writer.rs automatically.
```

And create:

```rust
// src/writer.rs
pub fn write_file(...) { ... }
```

That's the entire mechanism. The file *is* the module body. No extra wiring needed.

### Module visibility rules in brief

| Syntax | Meaning |
|---|---|
| `fn foo()` | Private — only this module can see it |
| `pub fn foo()` | Public — anyone can see it |
| `pub(crate) fn foo()` | Visible within this crate only, not to external users |
| `pub(super) fn foo()` | Visible to the parent module only |

We'll use `pub` and `pub(crate)` in this project.

---

## Section 2 — Setting Up a Cargo Workspace

### Why a workspace for this series?

Each part of this tutorial builds on the last but is also a standalone, runnable program. You have two options:

| Option | How | Downside |
|---|---|---|
| Separate folders | `fgen-part1/`, `fgen-part2/` | No shared dependencies, can't cross-reference |
| **Cargo workspace** | One root, many member crates | ✅ Best choice here |

A **Cargo workspace** is a single directory containing multiple Rust crates (packages) that share one `Cargo.lock` and one `target/` build cache. This means:
- Dependencies are downloaded and compiled once, shared across all parts.
- You run each part independently: `cargo run -p fgen-part2`
- All source lives in one repo, easy to compare parts side by side.

### Creating the workspace

Start fresh. Create a new root directory (not inside any existing Cargo project):

```bash
mkdir fgen-series
cd fgen-series
```

Create the workspace manifest — this is NOT a Rust package, just a coordinator:

```toml
# fgen-series/Cargo.toml
[workspace]
members = [
    "part01",
    "part02",
]
resolver = "2"
```

Now move Part 1's code in as a workspace member:

```bash
cargo new part01
```

Copy Part 1's `src/main.rs` and `Cargo.toml` `[dependencies]` into `part01/`. Then create Part 2:

```bash
cargo new part02
```

Your directory now looks like:

```
fgen-series/
├── Cargo.toml          ← workspace root (no [package], just [workspace])
├── Cargo.lock          ← single shared lockfile for all members
├── target/             ← single shared build cache
├── part01/
│   ├── Cargo.toml      ← [package] name = "fgen-part01"
│   └── src/
│       └── main.rs
└── part02/
    ├── Cargo.toml      ← [package] name = "fgen-part02"
    └── src/
        └── main.rs
```

### Workspace commands you'll use

```bash
# Run a specific part
cargo run -p fgen-part02 -- --name test.txt

# Build everything
cargo build

# Build only one part
cargo build -p fgen-part01

# Check all parts for errors without building
cargo check

# Run tests in all parts
cargo test

# Run tests in one part only
cargo test -p fgen-part02
```

> **Tip for each future part:** just add a new `cargo new partN` directory and add `"partN"` to the `members` list in the root `Cargo.toml`. That's it — it inherits the shared build cache immediately.

### Sharing a dependency version across members (optional but clean)

You can declare dependency versions once at the workspace level to keep them consistent:

```toml
# fgen-series/Cargo.toml
[workspace]
members = ["part01", "part02"]
resolver = "2"

[workspace.dependencies]
clap = { version = "4", features = ["derive"] }
```

Then in each member's `Cargo.toml`:

```toml
# part02/Cargo.toml
[dependencies]
clap.workspace = true   # inherits version from workspace root
```

This way, when you upgrade `clap`, you change one line in one file.

---

## Section 3 — Part 2's Multi-File Structure

Here is the target structure for `part02/`:

```
part02/
├── Cargo.toml
└── src/
    ├── main.rs         ← entry point: parse args, call into other modules
    ├── cli.rs          ← Cli struct, argument definitions
    ├── template.rs     ← file template logic (content generation)
    └── writer.rs       ← file writing logic
```

We split on **responsibility**:
- `cli.rs` — knows about user input, nothing else.
- `template.rs` — knows how to produce file content from a name or a template choice.
- `writer.rs` — knows how to write bytes to disk.
- `main.rs` — wires them together. No real logic here.

This pattern (thin `main`, logic in modules) is idiomatic Rust for CLI tools.

---

## Section 4 — The Code, File by File

### `part02/Cargo.toml`

```toml
[package]
name = "fgen-part02"
version = "0.2.0"
edition = "2021"

[dependencies]
clap.workspace = true
```

---

### `src/cli.rs` — Argument definitions

```rust
use clap::Parser;
use std::path::PathBuf;

/// The template to apply to a new file.
///
/// Using an enum here gives us two benefits:
/// 1. clap auto-validates the value — passing `--template blah` is a hard error.
/// 2. We can match on it exhaustively — the compiler tells us if we forget a case.
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum TemplateChoice {
    /// Plain text, no special formatting
    Plain,
    /// A minimal JSON object skeleton
    Json,
    /// A CSV file with a sample header row
    Csv,
    /// An .env file with commented example variables
    Env,
}

/// fgen — generate files from the command line
#[derive(Parser, Debug)]
#[command(
    name = "fgen",
    version = "0.2.0",
    about = "Generate files with optional templates"
)]
pub struct Cli {
    /// Name of the file to create (e.g. notes.txt, data.csv)
    #[arg(short, long)]
    pub name: String,

    /// Content to write into the file (overrides template content)
    #[arg(short, long)]
    pub content: Option<String>,

    /// Directory where the file will be saved (defaults to current directory)
    #[arg(short, long, default_value = ".")]
    pub path: PathBuf,

    /// Explicitly choose a file template (overrides extension detection)
    #[arg(short, long, value_enum)]
    pub template: Option<TemplateChoice>,
}
```

**What changed from Part 1:**
- `content` is now `Option<String>` — it's truly optional. `None` means "use a template or leave empty."
- `template` is `Option<TemplateChoice>` — an optional enum. `clap::ValueEnum` on the enum makes it work automatically.

---

### `src/template.rs` — Content generation

This is the heart of Part 2. Read through the comments carefully.

```rust
use crate::cli::TemplateChoice;

/// Resolve what content to write into the file.
///
/// Priority order:
///   1. If the user passed `--content "..."`, use that verbatim.
///   2. If the user passed `--template <name>`, use that template.
///   3. Try to detect the template from the file's extension.
///   4. Fall back to an empty string.
///
/// `crate::` means "start from the root of this crate" — it's how you
/// reference items in sibling modules.
pub fn resolve_content(
    name: &str,
    content: Option<String>,
    template: Option<TemplateChoice>,
) -> String {
    // Priority 1: explicit content wins over everything
    if let Some(text) = content {
        return text;
    }

    // Priority 2: explicit template flag
    if let Some(choice) = template {
        return render_template(choice);
    }

    // Priority 3: detect from extension
    if let Some(detected) = detect_from_extension(name) {
        return render_template(detected);
    }

    // Priority 4: empty file
    String::new()
}

/// Detect a TemplateChoice from the file's extension.
///
/// Returns None if no matching template exists for that extension.
/// This is intentional — not every extension maps to a template.
fn detect_from_extension(name: &str) -> Option<TemplateChoice> {
    // rsplit_once splits on the LAST occurrence of '.'
    // "archive.tar.gz" → Some(("archive.tar", "gz"))
    // "README"         → None
    let ext = name.rsplit_once('.')?.1;

    match ext.to_lowercase().as_str() {
        "json" => Some(TemplateChoice::Json),
        "csv"  => Some(TemplateChoice::Csv),
        "env"  => Some(TemplateChoice::Env),
        "txt" | "text" | "md" => Some(TemplateChoice::Plain),
        // Any other extension → no template detected
        _ => None,
    }
}

/// Turn a TemplateChoice into actual file content.
fn render_template(choice: TemplateChoice) -> String {
    match choice {
        TemplateChoice::Plain => {
            String::from("# New file\n")
        }

        TemplateChoice::Json => {
            // A minimal, valid JSON object skeleton
            String::from("{\n  \"\": \"\"\n}\n")
        }

        TemplateChoice::Csv => {
            // A CSV file with example column headers
            String::from("id,name,value\n1,example,0\n")
        }

        TemplateChoice::Env => {
            // .env format: KEY=VALUE, with commented-out examples
            String::from(
                "# Application environment variables\n\
                 # Copy this file to .env and fill in your values\n\
                 \n\
                 APP_NAME=myapp\n\
                 APP_ENV=development\n\
                 # SECRET_KEY=changeme\n"
            )
        }
    }
}

// ─── Unit tests ──────────────────────────────────────────────────────────────
//
// Tests live right next to the code they test in Rust. The `#[cfg(test)]`
// attribute means this block is compiled ONLY when running `cargo test` —
// it does not end up in your release binary.

#[cfg(test)]
mod tests {
    use super::*; // bring everything from the parent module into scope

    #[test]
    fn detect_json_extension() {
        let result = detect_from_extension("config.json");
        assert!(matches!(result, Some(TemplateChoice::Json)));
    }

    #[test]
    fn detect_env_extension() {
        let result = detect_from_extension(".env");
        // ".env" has no text before the dot, so rsplit_once('.') gives ("", "env")
        assert!(matches!(result, Some(TemplateChoice::Env)));
    }

    #[test]
    fn unknown_extension_returns_none() {
        let result = detect_from_extension("archive.zip");
        assert!(result.is_none());
    }

    #[test]
    fn no_extension_returns_none() {
        let result = detect_from_extension("Makefile");
        assert!(result.is_none());
    }

    #[test]
    fn explicit_content_wins_over_template() {
        let content = resolve_content(
            "data.json",
            Some("custom content".to_string()),
            Some(TemplateChoice::Json),
        );
        assert_eq!(content, "custom content");
    }

    #[test]
    fn explicit_template_wins_over_extension() {
        // File is named .env but user forced --template csv
        let content = resolve_content(
            ".env",
            None,
            Some(TemplateChoice::Csv),
        );
        assert!(content.contains("id,name,value"));
    }

    #[test]
    fn extension_detection_fallback() {
        // No content, no explicit template → detect from .csv extension
        let content = resolve_content("report.csv", None, None);
        assert!(content.contains("id,name,value"));
    }

    #[test]
    fn empty_for_unknown_extension() {
        let content = resolve_content("archive.zip", None, None);
        assert_eq!(content, "");
    }
}
```

---

### `src/writer.rs` — File writing

```rust
use std::fs;
use std::io;
use std::path::Path;

/// Write `content` to `file_path`.
///
/// Returns an `io::Error` if the write fails — we let main.rs decide
/// how to handle and display the error. This keeps writer.rs focused
/// on one job: writing bytes to disk.
pub fn write_file(file_path: &Path, content: &str) -> Result<(), io::Error> {
    fs::write(file_path, content)
}
```

> **Why return `Result` instead of printing the error here?** Because the writer shouldn't decide how errors are presented. Maybe you want a plain message, maybe a coloured terminal output, maybe a JSON log. Returning the error keeps the decision with the caller.

---

### `src/main.rs` — Wiring it all together

```rust
// Declare the modules. Rust will look for:
//   mod cli;      → src/cli.rs
//   mod template; → src/template.rs
//   mod writer;   → src/writer.rs
mod cli;
mod template;
mod writer;

use clap::Parser;
use cli::Cli;

fn main() {
    let args = Cli::parse();

    // Build the full output path
    let file_path = args.path.join(&args.name);

    // Resolve what content to write, using the priority chain in template.rs
    let content = template::resolve_content(
        &args.name,
        args.content,
        args.template,
    );

    // Write and handle the result
    match writer::write_file(&file_path, &content) {
        Ok(_) => {
            println!("✅  Created: {}", file_path.display());

            // Show the user which template was used (or that it was custom/empty)
            if content.is_empty() {
                println!("    (empty file)");
            } else {
                // Show just the first line as a preview
                let preview = content.lines().next().unwrap_or("");
                println!("    Preview: {}", preview);
            }
        }
        Err(e) => {
            eprintln!("❌  Failed: {}", e);
            std::process::exit(1);
        }
    }
}
```

---

## Section 5 — Why Two Mechanisms for Templates?

You asked for both auto-detection and an explicit flag. Here's the full reasoning behind why you'd want both:

### Auto-detection from extension

**When it's useful:** The user just wants to get something created quickly without thinking about it.

```bash
fgen --name config.json
# Produces: {"": ""} — a valid JSON skeleton
```

The filename already encodes the intent. Making the user type `--template json` would be redundant noise. This is the "do what I mean" mode.

**The limitation:** Extensions can lie. A file called `data.json` might actually be intended as raw text for a tutorial. Auto-detection can't know.

### Explicit `--template` flag

**When it's useful:**
- The user wants a template that doesn't match the extension: create a `.txt` file but with the JSON skeleton to paste into.
- The user is creating a file with an unusual extension that has no template (e.g. `.conf`, `.ini`, `.hcl`).
- The user wants to be explicit and self-documenting in a script or Makefile.

```bash
# Create a .txt scratch file pre-filled with a JSON skeleton
fgen --name scratch.txt --template json

# Create a .env-style file with an unusual extension
fgen --name secrets.conf --template env
```

### How both coexist — the priority chain

The key design decision is the priority order in `resolve_content`:

```
explicit --content  >  explicit --template  >  extension detection  >  empty
```

This ordering is intentional:
- User's literal text always wins — it's the most explicit signal.
- An explicit template flag overrides guessing.
- Extension detection is a convenience, not a command.
- Empty is always the safe fallback.

This means the tool is **predictable**: the user can always override any automatic behaviour by being more explicit.

---

## Section 6 — Running Everything

### Run Part 2

```bash
# From the workspace root:
cargo run -p fgen-part02 -- --name config.json
```

Output:
```
✅  Created: ./config.json
    Preview: {
```

```bash
# Auto-detect .env
cargo run -p fgen-part02 -- --name .env
```

```
✅  Created: ./.env
    Preview: # Application environment variables
```

```bash
# Explicit template overrides extension
cargo run -p fgen-part02 -- --name mydata.txt --template csv
```

```
✅  Created: ./mydata.txt
    Preview: id,name,value
```

```bash
# Custom content overrides everything
cargo run -p fgen-part02 -- --name config.json --content '{"port": 3000}'
```

```
✅  Created: ./config.json
    Preview: {"port": 3000}
```

```bash
# See all available templates listed in help
cargo run -p fgen-part02 -- --help
```

### Run the tests

```bash
# All tests in part02
cargo test -p fgen-part02

# With output printed (helpful when debugging)
cargo test -p fgen-part02 -- --nocapture

# Run a specific test by name
cargo test -p fgen-part02 detect_json_extension
```

Expected output:
```
running 8 tests
test template::tests::detect_json_extension ... ok
test template::tests::detect_env_extension ... ok
test template::tests::unknown_extension_returns_none ... ok
test template::tests::no_extension_returns_none ... ok
test template::tests::explicit_content_wins_over_template ... ok
test template::tests::explicit_template_wins_over_extension ... ok
test template::tests::extension_detection_fallback ... ok
test template::tests::empty_for_unknown_extension ... ok

test result: ok. 8 passed; 0 failed; 0 finished; 0 measured
```

---

## Complete File Reference

Here is every file for Part 2, all in one place:

### `fgen-series/Cargo.toml` (workspace root)
```toml
[workspace]
members = ["part01", "part02"]
resolver = "2"

[workspace.dependencies]
clap = { version = "4", features = ["derive"] }
```

### `part02/Cargo.toml`
```toml
[package]
name = "fgen-part02"
version = "0.2.0"
edition = "2021"

[dependencies]
clap.workspace = true
```

### `part02/src/main.rs`
*(see Section 4 above)*

### `part02/src/cli.rs`
*(see Section 4 above)*

### `part02/src/template.rs`
*(see Section 4 above)*

### `part02/src/writer.rs`
*(see Section 4 above)*

---

## What We Built

| Concept | Where |
|---|---|
| `mod` declarations | `main.rs` |
| Module files | `cli.rs`, `template.rs`, `writer.rs` |
| `pub` visibility | All modules expose only what `main.rs` needs |
| Cargo workspace | Root `Cargo.toml` coordinates `part01` + `part02` |
| Shared dependencies | `[workspace.dependencies]` in root |
| `clap::ValueEnum` on enum | `cli.rs` → auto-validates `--template` values |
| `Option<T>` arguments | `--content` and `--template` are truly optional |
| Priority chain | `resolve_content` in `template.rs` |
| Unit tests | `#[cfg(test)] mod tests` inside `template.rs` |

---

## 🧪 Your Challenge

**Level 1 — Easy**
Add a `Toml` variant to `TemplateChoice` that generates:
```toml
[section]
key = "value"
```
Wire it up in `render_template` and add extension detection for `.toml`.

**Level 2 — Medium**
Add a `--list-templates` flag that prints all available templates and their example output, then exits — without requiring `--name`. You'll need to handle the case where `--name` is absent.
*Hint: look at `clap`'s `conflicts_with` attribute and `std::process::exit`.*

**Level 3 — Hard**
Write a test that creates a real file in a temporary directory and verifies its contents on disk.
*Hint: look at `std::env::temp_dir()` and `std::fs::read_to_string()`.*

---

## What's Next

In **Part 3**, we'll add **subcommands** — turning `fgen` from a single-action tool into a multi-command tool (`fgen create`, `fgen list`, `fgen delete`). We'll introduce proper error types using Rust's `enum` for errors instead of raw `io::Error`, and add coloured terminal output.

---

*Part of the **Building a CLI File Generator in Rust** series.*
*Part 1: Basic CLI → **Part 2: Modules & Templates** → Part 3: Subcommands & Error Types*