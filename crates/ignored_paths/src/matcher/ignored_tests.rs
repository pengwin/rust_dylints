use std::path::Path;

use rstest::{fixture, rstest};

use super::ignored::{IgnoredPaths, default_patterns};

/// The default patterns, compiled.
#[fixture]
fn defaults() -> Result<IgnoredPaths, globset::Error> {
    IgnoredPaths::new(&default_patterns())
}

#[rstest]
#[case::generated_directory("src/generated/tables.rs")]
#[case::nested_generated_directory("src/proto/generated/deep/api.rs")]
#[case::generated_at_the_root("generated/api.rs")]
#[case::generated_suffix("src/api_generated.rs")]
#[case::generated_dot_suffix("src/api.generated.rs")]
#[case::just_generated("src/generated.rs")]
#[case::build_output("target/debug/build/out/gen.rs")]
fn generated_code_is_skipped_by_default(
    defaults: Result<IgnoredPaths, globset::Error>,
    #[case] path: &str,
) -> Result<(), globset::Error> {
    assert!(defaults?.matches(Path::new(path)));
    Ok(())
}

#[rstest]
#[case::ordinary("src/lib.rs")]
#[case::nested("src/feature/plugin.rs")]
#[case::similar_directory("src/generators/tables.rs")]
#[case::word_inside_the_name("src/generated_values.rs")]
fn ordinary_code_is_inspected(
    defaults: Result<IgnoredPaths, globset::Error>,
    #[case] path: &str,
) -> Result<(), globset::Error> {
    assert!(!defaults?.matches(Path::new(path)));
    Ok(())
}

/// Turns a pattern into a matcher.
type Compile = Box<dyn Fn(&str) -> Result<IgnoredPaths, globset::Error>>;

/// Compiles a pattern into a matcher.
#[fixture]
fn compile() -> Compile {
    Box::new(|pattern| IgnoredPaths::new(&[pattern.to_owned()]))
}

#[rstest]
#[case::star_stays_in_a_directory("**/generated/*.rs", "src/generated/a.rs", true)]
#[case::star_does_not_cross_directories("**/generated/*.rs", "src/generated/deep/a.rs", false)]
#[case::double_star_does_cross("**/generated/**", "src/generated/deep/a.rs", true)]
#[case::anchored("src/vendor/**", "src/vendor/a.rs", true)]
#[case::anchored_elsewhere("src/vendor/**", "crates/src/vendor/a.rs", false)]
fn patterns_follow_gitignore_like_rules(
    compile: Compile,
    #[case] pattern: &str,
    #[case] path: &str,
    #[case] expected: bool,
) -> Result<(), globset::Error> {
    assert_eq!(compile(pattern)?.matches(Path::new(path)), expected);
    Ok(())
}

#[rstest]
fn an_invalid_pattern_is_reported(compile: Compile) {
    assert!(compile("src/[").is_err());
}
