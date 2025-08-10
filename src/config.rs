use std::collections::HashSet;
use std::path::Path;

// default exclude folders
const DEFAULT_EXCLUDES: &[&str] = &[
    // Version control
    ".git/",
    ".svn/",
    ".hg/",
    // IDE/editor metadata
    ".idea/",
    ".vscode/",
    // JavaScript/Node
    "node_modules/",
    "bower_components/",
    ".next/",
    ".nuxt/",
    ".expo/",
    // Python
    "__pycache__/",
    ".venv/",
    "venv/",
    "env/",
    ".tox/",
    ".mypy_cache/",
    ".pytest_cache/",
    "*.egg-info/",
    ".cache/",
    ".coverage/",
    ".coverage_html/",
    // PHP
    "vendor/",
    // Java / Kotlin / Android
    "build/",
    "dist/",
    "target/",
    ".gradle/",
    "out/",
    "android/build/",
    ".cocoapods/",
    "Pods/",
    ".ios/",
    ".android/",
    ".xcworkspace/",
    ".xcassets/",
    // Ruby
    ".bundle/",
    // Rust
    "target/",
    // Elixir / Erlang
    "_build/",
    "deps/",
    // Terraform
    ".terraform/",
    // CI/CD Systems
    ".circleci/",
    ".github/",
    ".gitlab/",
    // Temp / Logs / Cache
    "tmp/",
    "temp/",
    "logs/",
    "log/",
    ".sass-cache/",
    "coverage/",
    "coverage_html/",
    "cache/",
    ".jekyll-cache/",
];

pub struct Config {
    pub exclude_dir: HashSet<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            exclude_dir: DEFAULT_EXCLUDES
                .iter()
                .map(|s| s.trim_end_matches("/").to_string())
                .collect(),
        }
    }

    /// add dir to exclude_dir
    pub fn add_exclude(&mut self, dir: &str) {
        let trimed = dir.trim_end_matches("/").to_string();
        self.exclude_dir.insert(trimed);
    }

    /// check if to exclude.
    pub fn dir_is_excluded(&self, dir: &Path) -> bool {
        dir.components().any(|component| {
            component
                .as_os_str()
                .to_str()
                .map_or(false, |name| self.exclude_dir.contains(name))
        })
    }
}
