//! CPH for Zed - Competitive Programming Helper Extension
//!
//! This extension integrates with competitive-companion browser extension
//! to streamline competitive programming workflows in Zed editor.

mod file_generator;
mod problem;

use zed_extension_api::{
    self as zed, SlashCommand, SlashCommandArgumentCompletion, SlashCommandOutput,
    SlashCommandOutputSection, Worktree,
};

const USAGE: &str = "\
Usage: /cph-fetch [language] <problem JSON>

Paste the JSON that competitive-companion sends you.

  language: cpp (default), python, rust, java

Example:
  /cph-fetch python {\"name\": \"A. Two Sum\", ...}

Note: Zed's extension sandbox cannot write into your worktree, so the files are
written to the extension work directory and you get a `cp -r` command to move
them.";

/// Languages accepted by `/cph-fetch`.
const LANGUAGES: [&str; 4] = ["cpp", "python", "rust", "java"];

/// Split a leading language token off the argument list.
///
/// `/cph-fetch python {...}` and `/cph-fetch {...}` are both accepted. An
/// unrecognized first token is treated as part of the JSON, since the payload
/// itself may legitimately start with an array or brace.
fn split_language_arg(args: &[String]) -> (String, String) {
    match args.first() {
        Some(first) if LANGUAGES.contains(&first.as_str()) => {
            let rest = args[1..].join(" ");
            (first.clone(), rest)
        }
        _ => ("cpp".to_string(), args.join(" ")),
    }
}

/// The main extension struct
struct CphExtension;

impl zed::Extension for CphExtension {
    fn new() -> Self {
        CphExtension
    }

    fn complete_slash_command_argument(
        &self,
        command: SlashCommand,
        args: Vec<String>,
    ) -> Result<Vec<SlashCommandArgumentCompletion>, String> {
        // Only `/cph-fetch` takes a language, and only as its first argument.
        if command.name.as_str() != "cph-fetch" || !args.is_empty() {
            return Ok(Vec::new());
        }

        Ok(LANGUAGES
            .iter()
            .map(|lang| SlashCommandArgumentCompletion {
                label: lang.to_string(),
                new_text: lang.to_string(),
                run_command: false,
            })
            .collect())
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        match command.name.as_str() {
            "cph-fetch" => self.handle_fetch_command(args, worktree),
            "cph-run" => self.handle_run_command(args, worktree),
            "cph-test" => self.handle_test_command(args, worktree),
            "cph-server" => self.handle_server_command(args),
            unknown => Err(format!("Unknown command: {}", unknown)),
        }
    }
}

impl CphExtension {
    /// Handle /cph-fetch command - fetch problem from pasted JSON
    ///
    /// Zed's WASM sandbox only preopens the extension's own work directory, so the
    /// generated tree is written there rather than into the worktree. The output
    /// includes a copy command the user runs themselves to move it into place.
    fn handle_fetch_command(
        &self,
        args: Vec<String>,
        _worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        let (language, json) = split_language_arg(&args);

        if json.trim().is_empty() {
            return Err(USAGE.to_string());
        }

        let problem = problem::Problem::from_json(&json)
            .map_err(|e| format!("Failed to parse problem JSON: {e}"))?;

        // `PWD` is set by Zed to the extension's work dir, which is the only
        // writable location in the sandbox.
        let root = std::env::var("PWD").unwrap_or_else(|_| ".".to_string());

        let files = file_generator::ProblemFiles::new(&problem, &language, &root);
        let already_solved = files.solution_exists();
        let written = files
            .write()
            .map_err(|e| format!("Failed to write files: {e}"))?;

        let mut text = format!(
            "📋 Problem: {}\n\
             📁 Group: {}\n\
             🔗 URL: {}\n\
             ⏱️ Time: {}\n\
             💾 Memory: {} MB\n\
             📝 Tests: {}\n\
             💻 Language: {}\n\n",
            problem.name,
            problem.group,
            problem.url,
            problem.time_limit_str(),
            problem.memory_limit,
            problem.tests.len(),
            language,
        );

        if already_solved {
            text.push_str(&format!(
                "⚠️  {} already exists and was left untouched.\n\n",
                files.solution_file
            ));
        }

        text.push_str(&format!("✅ Wrote {} files:\n", written.len()));
        for path in &written {
            text.push_str(&format!("  • {path}\n"));
        }

        text.push_str(&format!(
            "\n📂 To move this into your project, run:\n\n\
             ```bash\n\
             cp -r {}/{} ./\n\
             ```\n\n\
             Zed extensions cannot write into your worktree directly, so this copy \
             step is required.",
            root,
            files.relative_dir(),
        ));

        Ok(SlashCommandOutput {
            text: text.clone(),
            sections: vec![SlashCommandOutputSection {
                range: (0..text.len()).into(),
                label: format!("CPH: {}", problem.name),
            }],
        })
    }

    /// Handle /cph-run command - print the command to run a test case
    fn handle_run_command(
        &self,
        args: Vec<String>,
        _worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        let test_num = args.first().and_then(|s| s.parse::<usize>().ok());

        let text = match test_num {
            Some(n) => format!(
                "🏃 Test case {n}\n\n\
                 Run this from the problem directory. The solution file is named \
                 after the problem, so `*.cpp` picks it up.\n\n\
                 ```bash\n\
                 g++ -std=c++20 -O2 -o solution *.cpp && ./solution < tests/{n}.in\n\
                 ```\n\n\
                 Compare against the expected output:\n\
                 ```bash\n\
                 ./solution < tests/{n}.in | diff - tests/{n}.out && echo PASS || echo FAIL\n\
                 ```\n\n\
                 _C++ assumed. For another language, compile the file your \
                 `/cph-fetch` generated._"
            ),
            None => "🏃 All test cases\n\n\
                     Run this from the problem directory.\n\n\
                     ```bash\n\
                     g++ -std=c++20 -O2 -o solution *.cpp\n\
                     for i in tests/*.in; do\n\
                       echo \"── $i\"\n\
                       ./solution < \"$i\"\n\
                     done\n\
                     ```\n\n\
                     _C++ assumed. For another language, compile the file your \
                     `/cph-fetch` generated._"
                .to_string(),
        };

        Ok(SlashCommandOutput {
            text: text.clone(),
            sections: vec![SlashCommandOutputSection {
                range: (0..text.len()).into(),
                label: "CPH: Run".to_string(),
            }],
        })
    }

    /// Handle /cph-test command - print the command to test against all cases
    fn handle_test_command(
        &self,
        _args: Vec<String>,
        _worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        let text = "🧪 Testing against all cases\n\n\
                    Run this from the problem directory.\n\n\
                    ```bash\n\
                    g++ -std=c++20 -O2 -o solution *.cpp\n\
                    pass=0; fail=0\n\
                    for i in tests/*.in; do\n\
                      expected=\"${i%.in}.out\"\n\
                      if ./solution < \"$i\" | diff -q - \"$expected\" > /dev/null; then\n\
                        echo \"✅ PASS  $i\"; pass=$((pass+1))\n\
                      else\n\
                        echo \"❌ FAIL  $i\"; fail=$((fail+1))\n\
                      fi\n\
                    done\n\
                    echo \"---\"; echo \"passed: $pass  failed: $fail\"\n\
                    ```\n\n\
                    _C++ assumed. For another language, compile the file your \
                    `/cph-fetch` generated._"
            .to_string();

        Ok(SlashCommandOutput {
            text: text.clone(),
            sections: vec![SlashCommandOutputSection {
                range: (0..text.len()).into(),
                label: "CPH: Test".to_string(),
            }],
        })
    }

    /// Handle /cph-server command - explains why automatic fetching needs a
    /// native server, and what the real CPH project does.
    fn handle_server_command(&self, _args: Vec<String>) -> Result<SlashCommandOutput, String> {
        let text = "📡 Automatic fetching is not available in this extension.\n\n\
                    The original CPH (github.com/agrawal-d/cph) is a VS Code extension. \
                    It runs an HTTP listener on port 27121 inside the VS Code extension \
                    host and writes files straight into your workspace.\n\n\
                    A Zed extension cannot do either half of that:\n\n\
                    1. Its WASM sandbox has no socket API, so it cannot listen for \
                    competitive-companion's POST.\n\
                    2. Only the extension's own work directory is writable, so it \
                    cannot write into your project even if it had the data.\n\n\
                    Use /cph-fetch and paste the JSON instead. It writes the files to \
                    the extension work directory and prints a `cp -r` command to move \
                    them into your project.\n\n\
                    Making this fully automatic would require a separate native server \
                    binary that listens on a port and writes to disk — a new project, \
                    not a Zed extension."
            .to_string();

        Ok(SlashCommandOutput {
            text: text.clone(),
            sections: vec![SlashCommandOutputSection {
                range: (0..text.len()).into(),
                label: "CPH: Server".to_string(),
            }],
        })
    }
}

zed::register_extension!(CphExtension);

#[cfg(test)]
mod tests {
    use super::*;

    fn args(parts: &[&str]) -> Vec<String> {
        parts.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn language_arg_is_split_off_when_known() {
        let (language, json) = split_language_arg(&args(&["python", r#"{"a":1}"#]));
        assert_eq!(language, "python");
        assert_eq!(json, r#"{"a":1}"#);
    }

    #[test]
    fn language_defaults_to_cpp() {
        let (language, json) = split_language_arg(&args(&[r#"{"name":"A"}"#]));
        assert_eq!(language, "cpp");
        assert_eq!(json, r#"{"name":"A"}"#);
    }

    #[test]
    fn unknown_first_token_is_treated_as_json() {
        // The payload itself can start with a brace or bracket, so an
        // unrecognized token must not be eaten as a language.
        let (language, json) = split_language_arg(&args(&[r#"{"name":"A"}"#]));
        assert_eq!(language, "cpp");
        assert!(json.starts_with('{'));
    }

    #[test]
    fn multi_token_json_is_rejoined() {
        let (language, json) = split_language_arg(&args(&["rust", "{", r#""name":"A""#, "}"]));
        assert_eq!(language, "rust");
        assert_eq!(json, r#"{ "name":"A" }"#);
    }

    #[test]
    fn empty_args_yield_empty_json() {
        let (language, json) = split_language_arg(&[]);
        assert_eq!(language, "cpp");
        assert!(json.trim().is_empty());
    }

    #[test]
    fn usage_is_shown_for_empty_input() {
        let err = CphExtension.handle_fetch_command(vec![], None).unwrap_err();
        assert!(err.contains("/cph-fetch"));
    }

    #[test]
    fn no_command_invents_an_installable_binary() {
        // `cph-server` was documented as `cargo install cph-server`, but no such
        // crate exists on crates.io. Nothing may point users at it again.
        let server = CphExtension.handle_server_command(vec![]).unwrap();
        for cmd in ["/cph-fetch", "/cph-run", "/cph-test"] {
            let out = match cmd {
                "/cph-fetch" => CphExtension.handle_fetch_command(vec![], None).unwrap_err(),
                "/cph-run" => CphExtension.handle_run_command(vec![], None).unwrap().text,
                _ => CphExtension.handle_test_command(vec![], None).unwrap().text,
            };
            assert!(
                !out.contains("cargo install cph-server"),
                "{cmd} still tells users to install the nonexistent cph-server crate"
            );
        }
        assert!(!server.text.contains("cargo install cph-server"));
    }

    #[test]
    fn malformed_json_is_reported() {
        let err = CphExtension
            .handle_fetch_command(args(&["{not json"]), None)
            .unwrap_err();
        assert!(err.contains("Failed to parse problem JSON"), "got: {err}");
    }

    #[test]
    fn run_command_targets_the_requested_test() {
        let out = CphExtension.handle_run_command(args(&["2"]), None).unwrap();
        assert!(out.text.contains("tests/2.in"), "got: {}", out.text);
        assert!(out.text.contains("tests/2.out"), "got: {}", out.text);
    }

    #[test]
    fn run_command_without_a_number_covers_all_tests() {
        let out = CphExtension.handle_run_command(vec![], None).unwrap();
        assert!(out.text.contains("tests/*.in"), "got: {}", out.text);
    }

    #[test]
    fn test_command_reports_pass_and_fail_counts() {
        let out = CphExtension.handle_test_command(vec![], None).unwrap();
        assert!(out.text.contains("passed:"), "got: {}", out.text);
        assert!(out.text.contains("failed:"), "got: {}", out.text);
    }

    #[test]
    fn unknown_command_is_an_error() {
        use zed_extension_api::Extension as _;

        let command = SlashCommand {
            name: "cph-nope".to_string(),
            description: String::new(),
            requires_argument: false,
            tooltip_text: String::new(),
        };
        let err = CphExtension
            .run_slash_command(command, vec![], None)
            .unwrap_err();
        assert!(err.contains("Unknown command"), "got: {err}");
    }
}
