use std::path::{Path, PathBuf};

use ignored_paths::IgnoredPaths;
use rustc_lint::{EarlyContext, Lint, LintContext};
use rustc_span::{FileName, Span};

/// Access to the compiler's source map plus the library's ignored paths.
pub struct Source<'a, 'cx> {
    cx: &'a EarlyContext<'cx>,
    ignored: &'a IgnoredPaths,
    root: Option<PathBuf>,
}

impl<'a, 'cx> Source<'a, 'cx> {
    /// Wrap the early context with the paths the library must not inspect.
    #[must_use]
    pub fn new(cx: &'a EarlyContext<'cx>, ignored: &'a IgnoredPaths) -> Self {
        let root = std::env::var_os("CARGO_MANIFEST_DIR")
            .map(PathBuf::from)
            .and_then(|path| std::path::absolute(path).ok());
        Self { cx, ignored, root }
    }

    /// The real file a span lives in; `None` for synthetic spans.
    pub fn file_of(&self, span: Span) -> Option<PathBuf> {
        match self.cx.sess().source_map().span_to_filename(span) {
            FileName::Real(real) => real.local_path().map(Path::to_path_buf),
            _ => None,
        }
    }

    /// The path relative to the package, so parent directories never match by name.
    #[must_use]
    pub fn relative(&self, file: &Path) -> PathBuf {
        let absolute = std::path::absolute(file).unwrap_or_else(|_| file.to_path_buf());
        self.root
            .as_deref()
            .and_then(|root| absolute.strip_prefix(root).ok())
            .map_or_else(|| absolute.clone(), Path::to_path_buf)
    }

    /// Whether a file is generated or vendored code: it matches one of the `ignored_paths`
    /// patterns, tried on its path relative to the package.
    #[must_use]
    pub fn is_ignored(&self, file: &Path) -> bool {
        self.ignored.matches(&self.relative(file))
    }

    /// Number of lines in the file containing the span.
    #[must_use]
    pub fn line_count(&self, span: Span) -> usize {
        self.cx
            .sess()
            .source_map()
            .lookup_source_file(span.lo())
            .count_lines()
    }

    /// Report a lint at a span.
    pub fn lint(&self, lint: &'static Lint, span: Span, message: String) {
        self.cx.emit_span_lint(
            lint,
            span,
            rustc_errors::DiagDecorator(|diag| {
                diag.primary_message(message);
            }),
        );
    }
}
