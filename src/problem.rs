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
        self.name
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '_' || c == '-' {
                    c
                } else if c == ' ' {
                    '_'
                } else {
                    '_'
                }
            })
            .collect::<String>()
            .replace("__", "_")
            .trim_matches('_')
            .to_string()
    }
    
    /// Generate a filename-safe version of the group name
    pub fn safe_group(&self) -> String {
        self.group
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '_' || c == '-' {
                    c
                } else if c == ' ' {
                    '_'
                } else {
                    '_'
                }
            })
            .collect::<String>()
            .replace("__", "_")
            .trim_matches('_')
            .to_string()
    }
    
    /// Get the time limit in seconds (formatted string)
    pub fn time_limit_str(&self) -> String {
        let seconds = self.time_limit as f64 / 1000.0;
        format!("{:.1}s", seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_problem() {
        let json = r#"{
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
        
        let problem = Problem::from_json(json).unwrap();
        assert_eq!(problem.name, "A. Test Problem");
        assert_eq!(problem.safe_name(), "A_Test_Problem");
    }
}
