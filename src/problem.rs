//! Problem data model matching competitive-companion JSON format

use serde::{Deserialize, Serialize};

/// Represents a single test case with input and expected output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub input: String,
    pub output: String,
}

/// Input configuration for the problem
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum InputConfig {
    Stdin,
    File {
        #[serde(rename = "fileName")]
        file_name: String,
    },
    Regex {
        pattern: String,
    },
}

/// Output configuration for the problem
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum OutputConfig {
    Stdout,
    File {
        #[serde(rename = "fileName")]
        file_name: String,
    },
}

/// Java-specific language configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JavaConfig {
    #[serde(rename = "mainClass")]
    pub main_class: String,
    #[serde(rename = "taskClass")]
    pub task_class: String,
}

/// Language-specific configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    pub java: JavaConfig,
}

/// Batch information for contest problems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchInfo {
    pub id: String,
    pub size: u32,
}

/// Test type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TestType {
    Single,
    MultiNumber,
}

/// Main problem structure received from competitive-companion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Problem {
    /// Full name of the problem
    pub name: String,

    /// Group/contest name
    pub group: String,

    /// URL to the problem
    pub url: String,

    /// Whether this is an interactive problem
    #[serde(default)]
    pub interactive: bool,

    /// Memory limit in MB
    #[serde(rename = "memoryLimit")]
    pub memory_limit: u32,

    /// Time limit in milliseconds
    #[serde(rename = "timeLimit")]
    pub time_limit: u32,

    /// Test cases
    pub tests: Vec<TestCase>,

    /// Type of test (single or multi-number)
    #[serde(rename = "testType")]
    pub test_type: TestType,

    /// Input configuration
    pub input: InputConfig,

    /// Output configuration
    pub output: OutputConfig,

    /// Language-specific configurations
    pub languages: LanguageConfig,

    /// Batch information
    pub batch: BatchInfo,
}

impl Problem {
    /// Parse a problem from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Generate a filename-safe version of the problem name
    pub fn safe_name(&self) -> String {
        sanitize_for_path(&self.name)
    }

    /// Generate a filename-safe version of the group name
    pub fn safe_group(&self) -> String {
        sanitize_for_path(&self.group)
    }

    /// Get the time limit in seconds (formatted string)
    pub fn time_limit_str(&self) -> String {
        let seconds = self.time_limit as f64 / 1000.0;
        format!("{seconds:.1}s")
    }
}

/// Reduce a contest or problem name to a single safe path segment.
///
/// Everything outside `[A-Za-z0-9_-]` collapses to `_`, runs of `_` are squeezed
/// into one, and leading/trailing separators are trimmed. The result is guaranteed
/// non-empty, so it can never collapse into a `.` or `..` path component.
fn sanitize_for_path(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    for c in name.chars() {
        let keep = c.is_ascii_alphanumeric() || c == '_' || c == '-';
        if keep {
            out.push(c);
        } else if !out.ends_with('_') {
            out.push('_');
        }
    }

    let trimmed = out.trim_matches('_');
    if trimmed.is_empty() {
        "unnamed".to_string()
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"{
        "name": "A. Test Problem",
        "group": "Codeforces - Round 123",
        "url": "https://codeforces.com/contest/123/problem/A",
        "interactive": false,
        "memoryLimit": 256,
        "timeLimit": 1000,
        "tests": [{"input": "1\n", "output": "2\n"}],
        "testType": "single",
        "input": {"type": "stdin"},
        "output": {"type": "stdout"},
        "languages": {"java": {"mainClass": "Main", "taskClass": "ATestProblem"}},
        "batch": {"id": "123", "size": 1}
    }"#;

    #[test]
    fn test_parse_problem() {
        let problem = Problem::from_json(SAMPLE).unwrap();
        assert_eq!(problem.name, "A. Test Problem");
        assert_eq!(problem.safe_name(), "A_Test_Problem");
        assert_eq!(problem.safe_group(), "Codeforces_-_Round_123");
    }

    #[test]
    fn time_limit_is_formatted_in_seconds() {
        let problem = Problem::from_json(SAMPLE).unwrap();
        assert_eq!(problem.time_limit_str(), "1.0s");
    }

    #[test]
    fn sanitize_collapses_separators_and_trims() {
        assert_eq!(sanitize_for_path("A. Test Problem"), "A_Test_Problem");
        assert_eq!(
            sanitize_for_path("Codeforces - Round 123"),
            "Codeforces_-_Round_123"
        );
        assert_eq!(sanitize_for_path("a  b"), "a_b");
        assert_eq!(sanitize_for_path("a...b"), "a_b");
        // Only `_` is trimmed from the edges; `-` is a legal filename character.
        assert_eq!(sanitize_for_path("__trim__"), "trim");
        assert_eq!(sanitize_for_path("--keep--"), "--keep--");
    }

    #[test]
    fn sanitize_keeps_underscores_and_dashes() {
        assert_eq!(sanitize_for_path("snake_case-name"), "snake_case-name");
        assert_eq!(sanitize_for_path("v1.2.3"), "v1_2_3");
    }

    #[test]
    fn sanitize_strips_path_traversal() {
        // The old implementation mapped '.' to '_' but relied on trim_matches to
        // clean up, so these must never survive as real path components.
        assert_eq!(sanitize_for_path(".."), "unnamed");
        assert_eq!(sanitize_for_path("/etc/passwd"), "etc_passwd");
        assert_eq!(sanitize_for_path("../../etc"), "etc");
        assert_eq!(sanitize_for_path(""), "unnamed");
        assert_eq!(sanitize_for_path("///"), "unnamed");
    }

    #[test]
    fn sanitized_names_are_always_a_single_segment() {
        for raw in ["..", ".", "/", "", "a/b/c", "///etc//passwd"] {
            let safe = sanitize_for_path(raw);
            assert!(!safe.is_empty(), "{raw:?} produced an empty name");
            assert!(!safe.contains('/'), "{raw:?} produced {safe:?}");
            assert!(!safe.contains('\\'), "{raw:?} produced {safe:?}");
            assert_ne!(safe, "..", "{raw:?} escaped the directory");
            assert_ne!(safe, ".", "{raw:?} escaped the directory");
        }
    }
}
