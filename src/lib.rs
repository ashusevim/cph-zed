//! CPH for Zed - Competitive Programming Helper Extension
//!
//! This extension integrates with competitive-companion browser extension
//! to streamline competitive programming workflows in Zed editor.

mod file_generator;
mod problem;

use zed_extension_api::{
    self as zed, SlashCommand, SlashCommandOutput, SlashCommandOutputSection, Worktree,
};

/// The main extension struct
struct CphExtension;

impl zed::Extension for CphExtension {
    fn new() -> Self {
        CphExtension
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
    /// Handle /cph-fetch command - fetch problem from clipboard JSON
    fn handle_fetch_command(
        &self,
        args: Vec<String>,
        worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        // Get the JSON from the first argument (user pastes it)
        let json = args.join(" ");
        
        if json.is_empty() {
            return Err(
                "Usage: /cph-fetch <problem JSON>\n\n\
                 Paste the problem JSON from competitive-companion.\n\n\
                 Tip: Use the companion server for automatic fetching:\n\
                 1. Run: cph-server start\n\
                 2. Click competitive-companion in browser\n\
                 3. Files are created automatically!"
                    .to_string(),
            );
        }

        // Parse the problem
        let problem = problem::Problem::from_json(&json)
            .map_err(|e| format!("Failed to parse problem JSON: {}", e))?;

        // Get workspace root
        let base_path = worktree
            .map(|w| w.root_path())
            .unwrap_or_else(|| ".".to_string());

        // Generate files
        let files = file_generator::ProblemFiles::new(&problem, "cpp", &base_path);

        let mut output_text = format!(
            "📋 Problem: {}\n\
             📁 Group: {}\n\
             🔗 URL: {}\n\
             ⏱️ Time: {}\n\
             💾 Memory: {} MB\n\
             📝 Tests: {}\n\n\
             Created files:\n",
            problem.name,
            problem.group,
            problem.url,
            problem.time_limit_str(),
            problem.memory_limit,
            problem.tests.len()
        );

        // List all files to be created
        for (path, _) in files.all_files() {
            output_text.push_str(&format!("  • {}\n", path));
        }

        output_text.push_str(&format!(
            "\n✨ Ready to solve! Open {} to start coding.",
            files.solution_file
        ));

        Ok(SlashCommandOutput {
            text: output_text.clone(),
            sections: vec![SlashCommandOutputSection {
                range: (0..output_text.len()).into(),
                label: format!("CPH: {}", problem.name),
            }],
        })
    }

    /// Handle /cph-run command - compile and run solution
    fn handle_run_command(
        &self,
        args: Vec<String>,
        _worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        let test_num = args.first().and_then(|s| s.parse::<usize>().ok());

        let text = match test_num {
            Some(n) => format!(
                "🏃 Running test case {}...\n\n\
                 To run tests, use your terminal:\n\
                 ```bash\n\
                 g++ -o solution solution.cpp && ./solution < tests/{}.in\n\
                 ```",
                n, n
            ),
            None => "🏃 Running all test cases...\n\n\
                     To run all tests, use your terminal:\n\
                     ```bash\n\
                     for i in tests/*.in; do\n\
                       echo \"Test: $i\"\n\
                       ./solution < \"$i\"\n\
                     done\n\
                     ```"
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

    /// Handle /cph-test command - run solution and compare outputs
    fn handle_test_command(
        &self,
        _args: Vec<String>,
        _worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        let text = "🧪 Testing solution against all cases...\n\n\
                    To test and compare outputs:\n\
                    ```bash\n\
                    for i in tests/*.in; do\n\
                      out=\"${i%.in}.out\"\n\
                      echo \"Test: $i\"\n\
                      ./solution < \"$i\" > actual.out\n\
                      diff -q actual.out \"$out\" && echo '✅ PASS' || echo '❌ FAIL'\n\
                    done\n\
                    ```"
        .to_string();

        Ok(SlashCommandOutput {
            text: text.clone(),
            sections: vec![SlashCommandOutputSection {
                range: (0..text.len()).into(),
                label: "CPH: Test".to_string(),
            }],
        })
    }

    /// Handle /cph-server command - info about the companion server
    fn handle_server_command(&self, args: Vec<String>) -> Result<SlashCommandOutput, String> {
        let subcommand = args.first().map(|s| s.as_str()).unwrap_or("help");

        let text = match subcommand {
            "start" => {
                "🚀 Starting CPH Server...\n\n\
                 To receive problems automatically from competitive-companion:\n\n\
                 1. Install the companion server (one time):\n\
                    ```bash\n\
                    cargo install cph-server\n\
                    ```\n\n\
                 2. Run the server:\n\
                    ```bash\n\
                    cph-server --port 10045 --dir ./problems\n\
                    ```\n\n\
                 3. Click competitive-companion icon on any problem page\n\n\
                 The server will create files automatically!"
                    .to_string()
            }
            "stop" => "🛑 Stopping CPH Server...\n\n\
                       Press Ctrl+C in the terminal running cph-server."
                .to_string(),
            _ => {
                "📡 CPH Server Commands:\n\n\
                 • /cph-server start - How to start the problem receiver\n\
                 • /cph-server stop  - How to stop the server\n\n\
                 The server listens for problems from competitive-companion\n\
                 and creates solution files automatically."
                    .to_string()
            }
        };

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
