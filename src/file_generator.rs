//! File generation for competitive programming problems

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

/// Generate solution file content from template
pub fn generate_solution(problem: &Problem, language: &str) -> String {
    let template = match language {
        "python" | "py" => PYTHON_TEMPLATE,
        _ => CPP_TEMPLATE, // Default to C++
    };
    
    template
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
    /// Base directory path (e.g., "problems/Contest_Name/A_Problem")
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
            .map(|(i, test)| {
                (format!("{}/{}.in", tests_dir, i + 1), test.input.clone())
            })
            .collect();
            
        let output_files: Vec<(String, String)> = problem
            .tests
            .iter()
            .enumerate()
            .map(|(i, test)| {
                (format!("{}/{}.out", tests_dir, i + 1), test.output.clone())
            })
            .collect();
        
        // Metadata
        let metadata_file = format!("{}/problem.json", base_dir);
        let metadata_content = serde_json::to_string_pretty(problem)
            .unwrap_or_else(|_| "{}".to_string());
        
        ProblemFiles {
            base_dir,
            solution_file,
            solution_content,
            input_files,
            output_files,
            metadata_file,
            metadata_content,
        }
    }
    
    /// Get all files as (path, content) pairs
    pub fn all_files(&self) -> Vec<(&String, &String)> {
        let mut files = vec![
            (&self.solution_file, &self.solution_content),
            (&self.metadata_file, &self.metadata_content),
        ];
        
        for (path, content) in &self.input_files {
            files.push((path, content));
        }
        
        for (path, content) in &self.output_files {
            files.push((path, content));
        }
        
        files
    }
}
