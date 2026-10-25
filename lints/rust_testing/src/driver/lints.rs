//! Lint declarations.

dylint_linting::declare_lint! {
    /// ### What it does
    /// Rejects hand-written `#[test]` functions.
    ///
    /// ### Why is this bad?
    /// Tests use `#[rstest]` so that setup comes from fixtures and variants from cases.
    /// Run with `--all-targets`: test code is only compiled for test targets.
    pub TESTING_PLAIN_TEST,
    Warn,
    "plain #[test] instead of #[rstest]"
}

dylint_linting::declare_lint! {
    /// ### What it does
    /// Rejects helper functions in test code which are not tests or `#[fixture]`s.
    pub TESTING_HELPER_FN,
    Warn,
    "test helper function instead of a #[fixture]"
}

dylint_linting::declare_lint! {
    /// ### What it does
    /// Rejects `let` bindings which construct test data inside an rstest test body.
    pub TESTING_INLINE_ARRANGE,
    Warn,
    "test data built inline instead of through a #[fixture]"
}
