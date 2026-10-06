//! File generation for competitive programming problems

use std::path::Path;

use crate::problem::Problem;

/// Default C++ template for solutions
pub const CPP_TEMPLATE: &str = r#"// Problem: {{name}}
// Contest: {{group}}
// URL: {{url}}
// Time Limit: {{time_limit}}
// Memory Limit: {{memory_limit}} MB

#include <bits/stdc++.h>
using namespace std;

#ifdef LOCAL
#define debug(x) cerr << #x << " = " << (x) << endl
#else
#define debug(x)
#endif

using ll = long long;
using pii = pair<int, int>;
using vi = vector<int>;

void solve() {
    // TODO: Implement solution
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    
    int t = 1;
    // cin >> t;  // Uncomment for multiple test cases
    while (t--) {
        solve();
    }
    
    return 0;
}
"#;

/// Default Python template for solutions
pub const PYTHON_TEMPLATE: &str = r#"# Problem: {{name}}
# Contest: {{group}}
# URL: {{url}}
# Time Limit: {{time_limit}}
# Memory Limit: {{memory_limit}} MB

import sys
from collections import defaultdict, deque
from functools import lru_cache
from math import gcd, lcm, sqrt, ceil, floor
from heapq import heappush, heappop
from bisect import bisect_left, bisect_right

input = sys.stdin.readline

def solve():
    # TODO: Implement solution
    pass

def main():
    t = 1
    # t = int(input())  # Uncomment for multiple test cases
    for _ in range(t):
        solve()

if __name__ == "__main__":
    main()
"#;

/// Default Rust template for solutions
pub const RUST_TEMPLATE: &str = r#"// Problem: {{name}}
// Contest: {{group}}
// URL: {{url}}
// Time Limit: {{time_limit}}
// Memory Limit: {{memory_limit}} MB

#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{min, max, Ordering};
#[allow(unused_imports)]
use std::io::{self, Read, Write, BufRead, BufReader};

fn solve() {
    // TODO: Implement solution
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let _ = input;
    solve();
}
"#;

/// Default Java template for solutions
pub const JAVA_TEMPLATE: &str = r#"// Problem: {{name}}
// Contest: {{group}}
// URL: {{url}}
// Time Limit: {{time_limit}}
// Memory Limit: {{memory_limit}} MB

import java.io.*;
import java.util.*;

// The class is intentionally NOT `public`: javac requires a public class to live
// in a file named after it, and this file is named after the problem.
class Main {
    static void solve() throws Exception {
        // TODO: Implement solution
    }

    public static void main(String[] args) throws Exception {
        FastScanner fs = new FastScanner(System.in);
        int t = 1;
        // t = fs.nextInt(); // Uncomment for multiple test cases
        while (t-- > 0) {
            solve();
        }
    }

    static class FastScanner {
        private final BufferedReader br;
        private StringTokenizer st;

        FastScanner(InputStream is) {
            br = new BufferedReader(new InputStreamReader(is));
        }

        String next() throws IOException {
            while (st == null || !st.hasMoreTokens()) {
                st = new StringTokenizer(br.readLine());
            }
            return st.nextToken();
        }

        int nextInt() throws IOException {
            return Integer.parseInt(next());
        }

        long nextLong() throws IOException {
            return Long.parseLong(next());
        }
    }
}
"#;

/// Get the template for a language, falling back to C++ for unknown values.
fn template_for(language: &str) -> &'static str {
    match language {
        "python" | "py" => PYTHON_TEMPLATE,
        "rust" | "rs" => RUST_TEMPLATE,
        "java" => JAVA_TEMPLATE,
        _ => CPP_TEMPLATE,
    }
}

/// Generate solution file content from template
pub fn generate_solution(problem: &Problem, language: &str) -> String {
    template_for(language)
        .replace("{{name}}", &problem.name)
        .replace("{{group}}", &problem.group)
        .replace("{{url}}", &problem.url)
        .replace("{{time_limit}}", &problem.time_limit_str())
        .replace("{{memory_limit}}", &problem.memory_limit.to_string())
}

/// Get file extension for a language
pub fn get_extension(language: &str) -> &str {
    match language {
        "python" | "py" => "py",
        "java" => "java",
        "rust" | "rs" => "rs",
        _ => "cpp", // Default to C++
    }
}

/// Generate the directory structure for a problem
pub struct ProblemFiles {
    /// Destination root that `base_dir` is relative to (the extension work dir).
    pub root: String,
    /// Base directory path (e.g., "<root>/problems/Contest_Name/A_Problem")
    pub base_dir: String,
    /// Solution file path
    pub solution_file: String,
    /// Solution file content
    pub solution_content: String,
    /// Test input files
    pub input_files: Vec<(String, String)>,
    /// Test output files
    pub output_files: Vec<(String, String)>,
    /// Problem metadata JSON
    pub metadata_file: String,
    pub metadata_content: String,
}

impl ProblemFiles {
    /// Generate all files for a problem
    pub fn new(problem: &Problem, language: &str, base_path: &str) -> Self {
        let safe_group = problem.safe_group();
        let safe_name = problem.safe_name();
        let extension = get_extension(language);

        let base_dir = format!("{}/problems/{}/{}", base_path, safe_group, safe_name);
        let tests_dir = format!("{}/tests", base_dir);

        let solution_file = format!("{}/{}.{}", base_dir, safe_name, extension);
        let solution_content = generate_solution(problem, language);

        // Generate test files
        let input_files: Vec<(String, String)> = problem
            .tests
            .iter()
            .enumerate()
            .map(|(i, test)| (format!("{}/{}.in", tests_dir, i + 1), test.input.clone()))
            .collect();

        let output_files: Vec<(String, String)> = problem
            .tests
            .iter()
            .enumerate()
            .map(|(i, test)| (format!("{}/{}.out", tests_dir, i + 1), test.output.clone()))
            .collect();

        // Metadata
        let metadata_file = format!("{}/problem.json", base_dir);
        let metadata_content =
            serde_json::to_string_pretty(problem).unwrap_or_else(|_| "{}".to_string());

        ProblemFiles {
            root: base_path.to_string(),
            base_dir,
            solution_file,
            solution_content,
            input_files,
            output_files,
            metadata_file,
            metadata_content,
        }
    }

    /// Relative path of the problem directory inside the destination root,
    /// e.g. `problems/Codeforces_-_Round_123/A_Example_Problem`.
    pub fn relative_dir(&self) -> String {
        // `base_dir` is `<root>/<relative>`; strip the root prefix back off so the
        // caller can show a path relative to wherever they copy the tree to.
        self.base_dir
            .strip_prefix(&format!("{}/", self.root))
            .unwrap_or(&self.base_dir)
            .to_string()
    }

    /// Write every generated file to disk under `self.root`.
    ///
    /// Returns the list of files that were written, in creation order.
    ///
    /// An existing solution file is never clobbered: re-running `/cph-fetch` on a
    /// problem you have already started must not destroy your work. Test and
    /// metadata files are always refreshed, since they come from the judge.
    pub fn write(&self) -> std::io::Result<Vec<String>> {
        let tests_dir = Path::new(&self.base_dir).join("tests");
        std::fs::create_dir_all(&tests_dir)?;

        let mut written = Vec::new();

        // Don't overwrite work in progress.
        let solution_existed = Path::new(&self.solution_file).exists();
        if !solution_existed {
            std::fs::write(&self.solution_file, &self.solution_content)?;
            written.push(self.solution_file.clone());
        }

        std::fs::write(&self.metadata_file, &self.metadata_content)?;
        written.push(self.metadata_file.clone());

        for (path, content) in &self.input_files {
            std::fs::write(path, content)?;
            written.push(path.clone());
        }

        for (path, content) in &self.output_files {
            std::fs::write(path, content)?;
            written.push(path.clone());
        }

        Ok(written)
    }

    /// Whether the solution file already exists on disk.
    pub fn solution_exists(&self) -> bool {
        Path::new(&self.solution_file).exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"{
        "name": "A. Example Problem",
        "group": "Codeforces - Educational Round 123",
        "url": "https://codeforces.com/contest/123/problem/A",
        "interactive": false,
        "memoryLimit": 256,
        "timeLimit": 2000,
        "tests": [
            {"input": "1 2\n", "output": "3\n"},
            {"input": "4 5\n", "output": "9\n"}
        ],
        "testType": "single",
        "input": {"type": "stdin"},
        "output": {"type": "stdout"},
        "languages": {"java": {"mainClass": "Main", "taskClass": "AExampleProblem"}},
        "batch": {"id": "123", "size": 2}
    }"#;

    fn parse() -> Problem {
        Problem::from_json(SAMPLE).expect("sample JSON should parse")
    }

    #[test]
    fn write_creates_solution_metadata_and_tests() {
        let dir = tempfile::tempdir().unwrap();
        let files = ProblemFiles::new(&parse(), "cpp", dir.path().to_str().unwrap());

        let written = files.write().expect("write should succeed");

        // solution + metadata + 2 .in + 2 .out
        assert_eq!(written.len(), 6);

        let base = Path::new(&files.base_dir);
        assert!(base.join("A_Example_Problem.cpp").exists());
        assert!(base.join("problem.json").exists());
        assert!(base.join("tests").join("1.in").exists());
        assert!(base.join("tests").join("1.out").exists());
        assert!(base.join("tests").join("2.in").exists());
        assert!(base.join("tests").join("2.out").exists());

        // Test contents must round-trip exactly, including the trailing newline.
        assert_eq!(
            std::fs::read_to_string(base.join("tests").join("1.in")).unwrap(),
            "1 2\n"
        );
        assert_eq!(
            std::fs::read_to_string(base.join("tests").join("2.out")).unwrap(),
            "9\n"
        );
    }

    #[test]
    fn write_does_not_clobber_existing_solution() {
        let dir = tempfile::tempdir().unwrap();
        let files = ProblemFiles::new(&parse(), "cpp", dir.path().to_str().unwrap());

        files.write().unwrap();
        let solution = Path::new(&files.solution_file);
        std::fs::write(solution, "// my in-progress solution\n").unwrap();

        // Re-fetch the same problem: the solution must survive.
        let written = files.write().unwrap();

        assert_eq!(
            std::fs::read_to_string(solution).unwrap(),
            "// my in-progress solution\n",
            "re-fetching must not overwrite an existing solution"
        );
        assert!(!written.contains(&files.solution_file));
        // Everything else is still refreshed.
        assert!(written.contains(&files.metadata_file));
    }

    #[test]
    fn write_uses_language_specific_extension() {
        let dir = tempfile::tempdir().unwrap();

        for (language, expected) in [
            ("cpp", "A_Example_Problem.cpp"),
            ("python", "A_Example_Problem.py"),
            ("rust", "A_Example_Problem.rs"),
            ("java", "A_Example_Problem.java"),
        ] {
            let files = ProblemFiles::new(&parse(), language, dir.path().to_str().unwrap());
            files.write().unwrap();
            assert!(
                Path::new(&files.base_dir).join(expected).exists(),
                "{language} should produce {expected}"
            );
        }
    }

    #[test]
    fn relative_dir_strips_the_destination_root() {
        let dir = tempfile::tempdir().unwrap();
        let files = ProblemFiles::new(&parse(), "cpp", dir.path().to_str().unwrap());

        assert_eq!(
            files.relative_dir(),
            "problems/Codeforces_-_Educational_Round_123/A_Example_Problem"
        );
    }

    #[test]
    fn template_language_matches_file_extension() {
        // Regression: `get_extension` returned "rs"/"java" while the template
        // lookup fell back to C++, writing C++ source into a .rs/.java file.
        let problem = parse();

        assert!(generate_solution(&problem, "rust").contains("fn main()"));
        assert!(generate_solution(&problem, "java").contains("class Main"));
        assert!(generate_solution(&problem, "python").contains("def main():"));
        assert!(generate_solution(&problem, "cpp").contains("int main()"));

        // An unknown language still yields the C++ template and the .cpp extension.
        assert!(generate_solution(&problem, "brainfuck").contains("int main()"));
        assert_eq!(get_extension("brainfuck"), "cpp");
    }

    #[test]
    fn java_class_is_not_public() {
        // javac rejects a `public class Main` that does not live in Main.java,
        // but the solution file is named after the problem.
        let problem = parse();
        let java = generate_solution(&problem, "java");
        let declaration = java
            .lines()
            .map(str::trim)
            .find(|line| line.starts_with("class ") || line.starts_with("public class"));
        assert_eq!(
            declaration,
            Some("class Main {"),
            "a public class would fail to compile in a problem-named file"
        );
    }

    #[test]
    fn rust_template_compiles_without_warnings() {
        // The unused-import allow keeps a fresh template warning-free.
        let problem = parse();
        let rust = generate_solution(&problem, "rust");
        assert!(rust.contains("#[allow(unused_imports)]"));
    }

    #[test]
    fn solution_template_is_substituted() {
        let problem = parse();
        let content = generate_solution(&problem, "cpp");

        assert!(content.contains("A. Example Problem"));
        assert!(content.contains("Codeforces - Educational Round 123"));
        assert!(content.contains("2.0s"));
        assert!(content.contains("256 MB"));
        assert!(!content.contains("{{"), "no placeholders should remain");
    }
}
