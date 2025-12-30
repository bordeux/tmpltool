/// Global context for template rendering
///
/// This context provides information about the template execution environment,
/// such as the base directory for resolving relative file paths.
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Global context shared across all template functions
///
/// This struct is thread-safe and can be cloned cheaply (uses Arc internally)
#[derive(Clone, Debug)]
pub struct TemplateContext {
    /// Base directory for resolving relative file paths
    /// - If template is from a file: directory containing the template file
    /// - If template is from stdin: current working directory
    base_dir: Arc<PathBuf>,

    /// Trust mode: if true, disables filesystem security restrictions
    trust_mode: bool,
}

impl TemplateContext {
    /// Create a new context with the given base directory and trust mode
    pub fn new(base_dir: PathBuf, trust_mode: bool) -> Self {
        Self {
            base_dir: Arc::new(base_dir),
            trust_mode,
        }
    }

    /// Create context from a template file path
    ///
    /// The base directory will be the directory containing the template file
    pub fn from_template_file(template_path: &str, trust_mode: bool) -> std::io::Result<Self> {
        let path = Path::new(template_path);
        let base_dir = path
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));

        // Canonicalize to get absolute path
        let canonical = std::fs::canonicalize(&base_dir)?;
        Ok(Self::new(canonical, trust_mode))
    }

    /// Create context for stdin (uses current working directory)
    pub fn from_stdin(trust_mode: bool) -> std::io::Result<Self> {
        let cwd = std::env::current_dir()?;
        Ok(Self::new(cwd, trust_mode))
    }

    /// Get the base directory for file operations
    pub fn base_dir(&self) -> &Path {
        &self.base_dir
    }

    /// Check if trust mode is enabled
    pub fn is_trust_mode(&self) -> bool {
        self.trust_mode
    }

    /// Resolve a relative path against the base directory
    ///
    /// # Arguments
    ///
    /// * `path` - The path to resolve (can be relative or absolute)
    ///
    /// # Returns
    ///
    /// The resolved absolute path
    pub fn resolve_path(&self, path: &str) -> PathBuf {
        let path_obj = Path::new(path);

        // If path is absolute, return as-is
        if path_obj.is_absolute() {
            path_obj.to_path_buf()
        } else {
            // Resolve relative to base directory
            self.base_dir.join(path)
        }
    }
}
