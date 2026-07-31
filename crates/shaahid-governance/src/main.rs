//! Executable architectural governance for the shaahid workspace.

#![forbid(unsafe_code)]

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

use tianheng::prelude::*;

const CONTRACT_REASON: &str = "shaahid-contract is the isolated adjudication core. At this shape it depends on nothing, and must never depend on another workspace crate or a runtime framework: its adjudication is pure.";
const GOVERNANCE_REASON: &str = "the governance gate must stay independent of the workspace graph it judges: it may depend only on the tianheng governance harness, never on a workspace crate under judgment.";
const CORE_NO_IO_REASON: &str = "the sans-I/O adjudication core performs no I/O: no code in shaahid-contract may call into std::io/fs/net/process; I/O lives in a runtime outside the core. Coverage is partial by nature (I/O entry points cannot be enumerated, and macro-expanded I/O such as println! is invisible to a source scan), so this tooth complements review rather than replacing it.";
const CORE_PURITY_REASON: &str = "the sans-I/O adjudication core reads no ambient clock and stays runtime-agnostic: witnessed state is supplied at the runtime edge, and its public API exposes no async fn.";
const FACADE_REASON: &str = "shaahid is the curated published entrypoint. It may depend only on shaahid-contract, never on a backend, runtime, or external framework.";
const FACADE_REEXPORT_REASON: &str =
    "the shaahid facade must stay a pure re-export entrypoint and hold no logic of its own";
const FACADE_NON_REEXPORT: &str = "non-re-export item in facade library";
const PROSE_REASON: &str =
    "active prose must be present and must not reintroduce stale architecture-defining vocabulary";

/// The facade source tree the re-exports-only scan guards, relative to the workspace root.
const FACADE_SOURCE_DIR: &str = "crates/shaahid/src";

const ACTIVE_PROSE_FILES: &[&str] = &[
    "AGENTS.md",
    "PROJECT.md",
    "README.md",
    "BACKLOG.md",
    "docs/development-flow.md",
    "docs/domain-language.md",
];

#[cfg(test)]
const LAW_PROJECTION_PREAMBLE: &str = "\
# Shaahid Tianheng Law Projection

Generated from `constitution()` in `crates/shaahid-governance/src/main.rs`.
**Do not edit by hand.** Regenerate it with:
`BLESS=1 cargo test -p shaahid-governance law_projection_is_fresh`.
If the law itself is wrong, amend the Constitution through the governed OpenSpec workflow.

This projection covers Tianheng-observable structure only. The custom active-prose and
facade-reexports reactions remain executable in `shaahid-governance`, but are outside
Tianheng's generated projection.
";

// No legacy vocabulary exists to guard against at this shape — Shaahid is new, with no
// prior architecture to regress toward. The hook below is ready: add entries as real
// drift risks emerge (a term that means the core does what it must not). Entries must
// be phrases that never appear in legitimate prose (including non-goals), so they flag
// drift rather than false-positive on a "Shaahid is not a ..." sentence.
const STALE_PHRASES: &[StalePhrase] = &[];

#[derive(Debug, Clone, Copy)]
struct StalePhrase {
    phrase: &'static str,
    reason: &'static str,
}

#[derive(Debug, PartialEq, Eq)]
struct ProseViolation {
    path: String,
    line: usize,
    phrase: &'static str,
    reason: &'static str,
}

fn constitution() -> Constitution {
    Constitution::new("shaahid")
        .boundary(
            CrateBoundary::crate_("shaahid-contract")
                .restrict_dependencies_to(Vec::<&str>::new())
                .because(CONTRACT_REASON),
        )
        .boundary(
            CrateBoundary::crate_("shaahid-contract")
                .restrict_dependencies_to(Vec::<&str>::new())
                .dependency_kind(DependencyKind::Dev)
                .because(CONTRACT_REASON),
        )
        .boundary(
            CrateBoundary::crate_("shaahid-contract")
                .restrict_dependencies_to(Vec::<&str>::new())
                .dependency_kind(DependencyKind::Build)
                .because(CONTRACT_REASON),
        )
        .boundary(
            CrateBoundary::crate_("shaahid-governance")
                .restrict_dependencies_to(["tianheng"])
                .because(GOVERNANCE_REASON),
        )
        .boundary(
            CrateBoundary::crate_("shaahid-governance")
                .restrict_dependencies_to(Vec::<&str>::new())
                .dependency_kind(DependencyKind::Dev)
                .because(GOVERNANCE_REASON),
        )
        .boundary(
            CrateBoundary::crate_("shaahid-governance")
                .restrict_dependencies_to(Vec::<&str>::new())
                .dependency_kind(DependencyKind::Build)
                .because(GOVERNANCE_REASON),
        )
        .boundary(
            CrateBoundary::crate_("shaahid")
                .restrict_dependencies_to(["shaahid-contract"])
                .because(FACADE_REASON),
        )
        .boundary(
            CrateBoundary::crate_("shaahid")
                .restrict_dependencies_to(Vec::<&str>::new())
                .dependency_kind(DependencyKind::Dev)
                .because(FACADE_REASON),
        )
        .boundary(
            CrateBoundary::crate_("shaahid")
                .restrict_dependencies_to(Vec::<&str>::new())
                .dependency_kind(DependencyKind::Build)
                .because(FACADE_REASON),
        )
        .sans_io_pure(
            SansIoPure::in_crate("shaahid-contract")
                .module("crate")
                .reading_clock_via("std::time", ["now"])
                .because(CORE_PURITY_REASON),
        )
        .boundary(
            ModuleBoundary::in_crate("shaahid-contract")
                .module("crate")
                .must_not_call_inline("std::io")
                .because(CORE_NO_IO_REASON),
        )
        .boundary(
            ModuleBoundary::in_crate("shaahid-contract")
                .module("crate")
                .must_not_call_inline("std::fs")
                .because(CORE_NO_IO_REASON),
        )
        .boundary(
            ModuleBoundary::in_crate("shaahid-contract")
                .module("crate")
                .must_not_call_inline("std::net")
                .because(CORE_NO_IO_REASON),
        )
        .boundary(
            ModuleBoundary::in_crate("shaahid-contract")
                .module("crate")
                .must_not_call_inline("std::process")
                .because(CORE_NO_IO_REASON),
        )
}

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();

    if should_check_prose(&args) {
        let manifest = manifest_path_from_args(&args);
        let root = manifest
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."));

        if let Err(violations) = check_active_prose(&root) {
            eprintln!("shaahid prose governance failed: {PROSE_REASON}");
            for violation in violations {
                eprintln!(
                    "{}:{}: `{}` - {}",
                    violation.path, violation.line, violation.phrase, violation.reason
                );
            }
            return ExitCode::from(1);
        }

        if let Err(violations) = check_facade_reexports_only(&root) {
            eprintln!("shaahid facade governance failed: {FACADE_REEXPORT_REASON}");
            for violation in violations {
                eprintln!(
                    "{}:{}: `{}`",
                    violation.path, violation.line, violation.marker
                );
            }
            return ExitCode::from(1);
        }
    }

    tianheng::run(&constitution(), args)
}

fn should_check_prose(args: &[String]) -> bool {
    args.iter().skip(1).any(|arg| arg == "check")
}

fn manifest_path_from_args(args: &[String]) -> PathBuf {
    for index in 0..args.len() {
        if args[index] == "--manifest-path"
            && let Some(path) = args.get(index + 1)
        {
            return PathBuf::from(path);
        }

        if let Some(path) = args[index].strip_prefix("--manifest-path=") {
            return PathBuf::from(path);
        }
    }

    PathBuf::from("Cargo.toml")
}

fn check_active_prose(root: &Path) -> Result<(), Vec<ProseViolation>> {
    let mut violations = Vec::new();

    for relative in ACTIVE_PROSE_FILES {
        let path = root.join(relative);
        let Ok(content) = fs::read_to_string(&path) else {
            // A canonical governed file that cannot be read must fail the gate, not be
            // silently skipped — otherwise a governed doc that vanishes grants a free
            // pass. Fail loudly, naming the file.
            violations.push(ProseViolation {
                path: String::from(*relative),
                line: 0,
                phrase: "<unreadable>",
                reason: "a governed active-prose file must be present and readable",
            });
            continue;
        };

        violations.extend(check_prose_content(relative, &content));
    }

    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

fn check_prose_content(path: &str, content: &str) -> Vec<ProseViolation> {
    let mut violations = Vec::new();

    for (index, line) in content.lines().enumerate() {
        for rule in STALE_PHRASES {
            if line.contains(rule.phrase) {
                violations.push(ProseViolation {
                    path: path.to_owned(),
                    line: index + 1,
                    phrase: rule.phrase,
                    reason: rule.reason,
                });
            }
        }
    }

    violations
}

#[derive(Debug, PartialEq, Eq)]
struct SourceViolation {
    path: String,
    line: usize,
    marker: &'static str,
}

fn check_facade_reexports_only(root: &Path) -> Result<(), Vec<SourceViolation>> {
    let mut violations = Vec::new();
    let files = collect_rs_files(&root.join(FACADE_SOURCE_DIR));

    // No facade source found at all (missing or empty source tree) is a vacuous
    // pass — mirror the coverage check's non-vacuous guard and fail. Keyed on files
    // *found*, not files *read*, so a present-but-unreadable file reports as
    // unreadable below rather than as an empty tree.
    if files.is_empty() {
        violations.push(SourceViolation {
            path: FACADE_SOURCE_DIR.to_owned(),
            line: 0,
            marker: "no facade source files found",
        });
    }

    for file in files {
        let relative = file
            .strip_prefix(root)
            .unwrap_or(&file)
            .to_string_lossy()
            .into_owned();
        let Ok(content) = fs::read_to_string(&file) else {
            // An unreadable facade source file must fail the gate, not be skipped —
            // a file the scan cannot read cannot be certified re-exports-only.
            violations.push(SourceViolation {
                path: relative,
                line: 0,
                marker: "unreadable facade source",
            });
            continue;
        };
        violations.extend(check_facade_content(&relative, &content));
    }

    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

/// A brace-depth-aware line scan: at brace depth zero, the facade library may hold
/// only re-exports, `use` imports, attributes, and comments. Any other item
/// (a `fn`, `struct`, `impl`, `const`, ...) is logic the facade must not carry. It
/// is deliberately a line scan, not a parser: `shaahid-governance` may depend only on
/// `tianheng`, so it cannot pull in `syn`. A logic item co-located on a `pub use`
/// line (`pub use X; pub const Y = 1;`) escapes this line heuristic, but the DoD
/// `cargo fmt --all --check` gate splits it onto its own line, where this scan then
/// catches it.
fn check_facade_content(path: &str, content: &str) -> Vec<SourceViolation> {
    let mut violations = Vec::new();
    let mut depth: i32 = 0;

    for (index, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        let is_comment = trimmed.starts_with("//");

        // A line inside a multi-line `pub use { ... }` block is a re-export
        // continuation; only judge lines that start a fresh item at depth zero.
        if depth == 0
            && !trimmed.is_empty()
            && !is_comment
            && !trimmed.starts_with('#')
            && !trimmed.starts_with("pub use ")
            && !trimmed.starts_with("use ")
        {
            violations.push(SourceViolation {
                path: path.to_owned(),
                line: index + 1,
                marker: FACADE_NON_REEXPORT,
            });
        }

        // Track brace depth off code lines only, so a brace inside a doc comment
        // does not desynchronize the scan.
        if !is_comment {
            depth += line.matches('{').count() as i32;
            depth -= line.matches('}').count() as i32;
            if depth < 0 {
                depth = 0;
            }
        }
    }

    violations
}

fn collect_rs_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    let Ok(entries) = fs::read_dir(dir) else {
        return files;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            files.extend(collect_rs_files(&path));
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }

    files
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEPENDENCY_RULE: &str = "tianheng.rule/guibiao/restrict-dependencies-to";
    const DEPENDENCY_FACT: &str = "tianheng.fact/guibiao/dependency";
    const INLINE_RULE: &str = "tianheng.rule/guibiao/confine-inline-symbol-path";
    const INLINE_FACT: &str = "tianheng.fact/guibiao/inline-path";
    const ASYNC_RULE: &str = "tianheng.rule/hunyi/async-exposure";
    const ASYNC_FACT: &str = "tianheng.fact/hunyi/async-exposure";

    #[test]
    fn current_workspace_satisfies_constitution() {
        governance_test().assert_clean();
    }

    #[test]
    fn every_workspace_crate_is_covered() {
        governance_test().assert_all_workspace_members_covered();
    }

    #[test]
    fn law_projection_is_fresh() {
        governance_test().assert_projection_fresh_with_preamble(
            "AGENTS.shaahid-law.md",
            LAW_PROJECTION_PREAMBLE,
        );
    }

    #[test]
    fn unapproved_core_dependency_is_rejected() {
        let workspace = TempWorkspace::new("shaahid-governance-forbidden-dependency");
        workspace.write_package("tokio", "");
        workspace.write_package(
            "shaahid-contract",
            r#"
[dependencies]
tokio = { path = "../tokio" }
"#,
        );
        workspace.write_package("shaahid-governance", "");
        workspace.write_facade();
        workspace.write_root_manifest_members(&[
            "shaahid",
            "shaahid-contract",
            "shaahid-governance",
            "tokio",
        ]);

        let outcome = check_constitution(&constitution(), &workspace.path.join("Cargo.toml"));

        let Outcome::Violations(report) = outcome else {
            panic!("expected an unapproved dependency violation, got {outcome:?}");
        };
        assert_violation(
            &report,
            "shaahid-contract",
            DEPENDENCY_RULE,
            DEPENDENCY_FACT,
            "dependency-edge",
            &[("kind", "normal"), ("package", "tokio")],
        );
    }

    #[test]
    fn dependency_table_tightening_reacts_with_structured_identities() {
        let workspace = TempWorkspace::new("shaahid-governance-dependency-table-tightening");
        workspace.write_package("tokio", "");
        workspace.write_package("guibiao", "");
        workspace.write_package(
            "shaahid-contract",
            r#"
[dev-dependencies]
tokio = { path = "../tokio" }

[build-dependencies]
tokio = { path = "../tokio" }
"#,
        );
        workspace.write_package(
            "shaahid-governance",
            r#"
[dependencies]
guibiao = { path = "../guibiao" }
"#,
        );
        workspace.write_facade();
        workspace.write_root_manifest_members(&[
            "shaahid",
            "shaahid-contract",
            "shaahid-governance",
            "tokio",
            "guibiao",
        ]);

        let outcome = check_constitution(&constitution(), &workspace.path.join("Cargo.toml"));
        let Outcome::Violations(report) = outcome else {
            panic!("expected dependency-table violations, got {outcome:?}");
        };
        assert_eq!(
            report.violations.len(),
            3,
            "the tightening fixture should isolate exactly its three declared dependency facts"
        );
        assert_violation(
            &report,
            "shaahid-contract",
            DEPENDENCY_RULE,
            DEPENDENCY_FACT,
            "dependency-edge",
            &[("kind", "dev"), ("package", "tokio")],
        );
        assert_violation(
            &report,
            "shaahid-contract",
            DEPENDENCY_RULE,
            DEPENDENCY_FACT,
            "dependency-edge",
            &[("kind", "build"), ("package", "tokio")],
        );
        assert_violation(
            &report,
            "shaahid-governance",
            DEPENDENCY_RULE,
            DEPENDENCY_FACT,
            "dependency-edge",
            &[("kind", "normal"), ("package", "guibiao")],
        );

        let manifest = workspace.path.join("Cargo.toml").display().to_string();
        assert_eq!(
            tianheng::run(
                &constitution(),
                ["shaahid-governance", "check", "--manifest-path", &manifest],
            ),
            ExitCode::from(1),
            "an enforced candidate violation must map to the runner's exit-1 contract"
        );
    }

    #[test]
    fn core_io_call_is_rejected() {
        // Prove the sans-I/O tooth bites, not just the async one: a std::fs call in the
        // core must fire the no-I/O ModuleBoundary. In this single-crate fixture no
        // other boundary can fire, so any violation is this tooth.
        let workspace = TempWorkspace::new("shaahid-governance-core-io-leak");
        workspace.write_package_with_source(
            "shaahid-contract",
            "",
            "pub fn leak() -> bool {\n    std::fs::metadata(\"x\").is_ok()\n}\n",
        );
        workspace.write_package("shaahid-governance", "");
        workspace.write_facade();
        workspace.write_root_manifest_members(&[
            "shaahid",
            "shaahid-contract",
            "shaahid-governance",
        ]);

        let outcome = check_constitution(&constitution(), &workspace.path.join("Cargo.toml"));

        let Outcome::Violations(report) = outcome else {
            panic!("expected a no-I/O violation, got {outcome:?}");
        };
        assert_violation(
            &report,
            "std::fs",
            INLINE_RULE,
            INLINE_FACT,
            "path-in-module",
            &[("module", "crate"), ("path", "std::fs::metadata")],
        );
    }

    #[test]
    fn core_ambient_clock_call_is_rejected() {
        // Prove the ambient-clock tooth bites (the `.ending_with("now")` matcher), a
        // distinct mechanism from the plain no-I/O paths.
        let workspace = TempWorkspace::new("shaahid-governance-core-clock-leak");
        workspace.write_package_with_source(
            "shaahid-contract",
            "",
            "pub fn leak() -> std::time::SystemTime {\n    std::time::SystemTime::now()\n}\n",
        );
        workspace.write_package("shaahid-governance", "");
        workspace.write_facade();
        workspace.write_root_manifest_members(&[
            "shaahid",
            "shaahid-contract",
            "shaahid-governance",
        ]);

        let outcome = check_constitution(&constitution(), &workspace.path.join("Cargo.toml"));

        let Outcome::Violations(report) = outcome else {
            panic!("expected an ambient-clock violation, got {outcome:?}");
        };
        assert_violation(
            &report,
            "std::time",
            INLINE_RULE,
            INLINE_FACT,
            "path-in-module",
            &[("module", "crate"), ("path", "std::time::SystemTime::now")],
        );
    }

    #[test]
    fn current_active_prose_satisfies_governance() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");

        assert_eq!(check_active_prose(&root), Ok(()));
    }

    #[test]
    fn current_facade_is_reexports_only() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");

        assert_eq!(check_facade_reexports_only(&root), Ok(()));
    }

    #[test]
    fn facade_reexports_and_comments_are_allowed() {
        let content = "\
//! Facade docs.
#![forbid(unsafe_code)]

pub use shaahid_contract::{Attestation, Deed};
pub use shaahid_contract::{
    Fingerprint,
    Outcome,
};
";
        assert!(check_facade_content("lib.rs", content).is_empty());
    }

    #[test]
    fn facade_logic_item_is_rejected() {
        assert_eq!(
            check_facade_content("lib.rs", "pub fn helper() {}\n"),
            vec![SourceViolation {
                path: "lib.rs".to_owned(),
                line: 1,
                marker: FACADE_NON_REEXPORT,
            }]
        );
        // A struct declaration inside the facade is logic, not a re-export.
        assert_eq!(
            check_facade_content("lib.rs", "struct Sneaky;\n"),
            vec![SourceViolation {
                path: "lib.rs".to_owned(),
                line: 1,
                marker: FACADE_NON_REEXPORT,
            }]
        );
    }

    #[test]
    fn empty_facade_source_tree_fails_loudly() {
        // A root with no facade source tree scans zero files; the non-vacuous guard
        // must convert that into a failure rather than an empty (clean) pass.
        let workspace = TempWorkspace::new("shaahid-governance-empty-facade");

        let Err(violations) = check_facade_reexports_only(&workspace.path) else {
            panic!("a root with no facade source must fail the gate");
        };
        assert!(
            violations
                .iter()
                .any(|violation| violation.marker == "no facade source files found"),
            "expected a no-facade-source violation: {violations:?}"
        );
    }

    #[test]
    fn unapproved_facade_dependency_is_rejected() {
        // The facade boundary must bite: a `shaahid` that depends on anything other
        // than `shaahid-contract` fails the gate.
        let workspace = TempWorkspace::new("shaahid-governance-facade-dependency");
        workspace.write_package("tokio", "");
        workspace.write_package("shaahid-contract", "");
        workspace.write_package("shaahid-governance", "");
        workspace.write_package(
            "shaahid",
            r#"
[dependencies]
shaahid-contract = { path = "../shaahid-contract" }
tokio = { path = "../tokio" }
"#,
        );
        workspace.write_root_manifest_members(&[
            "shaahid",
            "shaahid-contract",
            "shaahid-governance",
            "tokio",
        ]);

        let outcome = check_constitution(&constitution(), &workspace.path.join("Cargo.toml"));

        let Outcome::Violations(report) = outcome else {
            panic!("expected an unapproved facade dependency violation, got {outcome:?}");
        };
        assert_violation(
            &report,
            "shaahid",
            DEPENDENCY_RULE,
            DEPENDENCY_FACT,
            "dependency-edge",
            &[("kind", "normal"), ("package", "tokio")],
        );
    }

    #[test]
    fn missing_active_prose_file_fails_loudly() {
        // A root with none of the canonical governed prose files must fail the gate,
        // not pass vacuously by skipping every unreadable file.
        let workspace = TempWorkspace::new("shaahid-governance-missing-prose");

        let Err(violations) = check_active_prose(&workspace.path) else {
            panic!("a root missing every governed prose file must fail the gate");
        };
        assert!(
            violations
                .iter()
                .any(|violation| violation.phrase == "<unreadable>"),
            "expected an unreadable-file violation naming a governed file: {violations:?}"
        );
    }

    #[test]
    fn core_async_exposure_reaction_fires() {
        let outcome = semantic_reaction_outcome(
            "shaahid-governance-core-async-leak",
            "pub async fn leak() {}\n",
        );

        let Outcome::Violations(report) = outcome else {
            panic!("expected a core async-exposure violation, got {outcome:?}");
        };
        assert_violation(
            &report,
            "crate",
            ASYNC_RULE,
            ASYNC_FACT,
            "async-free-function",
            &[
                ("module", "crate"),
                ("name", "leak"),
                ("owner", "crate"),
                ("owner_kind", "module"),
            ],
        );
    }

    #[test]
    fn core_async_in_submodule_is_rejected() {
        // Prove `.including_submodules()` actually recurses: an async fn nested in a
        // submodule must fire too, not just one at the crate root.
        let outcome = semantic_reaction_outcome(
            "shaahid-governance-core-async-submodule-leak",
            "pub mod inner {\n    pub async fn leak() {}\n}\n",
        );

        let Outcome::Violations(report) = outcome else {
            panic!("expected a submodule async-exposure violation, got {outcome:?}");
        };
        assert_violation(
            &report,
            "crate",
            ASYNC_RULE,
            ASYNC_FACT,
            "async-free-function",
            &[
                ("module", "crate::inner"),
                ("name", "leak"),
                ("owner", "crate::inner"),
                ("owner_kind", "module"),
            ],
        );
    }

    #[test]
    fn semantic_reactions_stay_clean_without_a_leak() {
        // Precision: the same crate shape without the leak must be clean, so the firing
        // test proves a reacting boundary, not one that always fires.
        let outcome =
            semantic_reaction_outcome("shaahid-governance-semantic-clean", "pub fn witness() {}\n");

        assert_eq!(
            outcome,
            Outcome::Clean,
            "a core with no async exposure must raise no semantic violation"
        );
    }

    fn governance_test() -> GovernanceTest {
        GovernanceTest::for_constitution(constitution())
            .with_manifest_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
    }

    fn assert_violation(
        report: &Report,
        target: &str,
        rule_type: &str,
        fact_type: &str,
        fact_shape: &str,
        fact_fields: &[(&str, &str)],
    ) {
        assert!(
            report.violations.iter().any(|violation| {
                violation.target() == target
                    && violation.rule_key().rule_type() == rule_type
                    && violation.fact().fact_type() == fact_type
                    && violation.fact().shape() == fact_shape
                    && violation.fact().fields().collect::<Vec<_>>() == fact_fields
            }),
            "expected structured violation target={target:?}, rule={rule_type:?}, \
             fact={fact_type:?}/{fact_shape:?} fields={fact_fields:?}; report: {report:?}"
        );
    }

    /// Build a minimal governed workspace and run the unified Constitution against it.
    /// A firing fixture differs from a clean one only in the contract source.
    fn semantic_reaction_outcome(name: &str, contract_source: &str) -> Outcome {
        let workspace = TempWorkspace::new(name);
        workspace.write_package_with_source("shaahid-contract", "", contract_source);
        workspace.write_package("shaahid-governance", "");
        workspace.write_facade();
        workspace.write_root_manifest_members(&[
            "shaahid",
            "shaahid-contract",
            "shaahid-governance",
        ]);

        check_constitution(&constitution(), &workspace.path.join("Cargo.toml"))
    }

    struct TempWorkspace {
        path: PathBuf,
    }

    impl TempWorkspace {
        fn new(name: &str) -> Self {
            let path = std::env::temp_dir().join(format!("{name}-{}", std::process::id()));
            if path.exists() {
                fs::remove_dir_all(&path).expect("stale temporary workspace should be removable");
            }
            fs::create_dir_all(&path).expect("temporary workspace should be creatable");
            Self { path }
        }

        fn write_root_manifest_members(&self, members: &[&str]) {
            let entries = members
                .iter()
                .map(|member| format!("    \"{member}\","))
                .collect::<Vec<_>>()
                .join("\n");
            fs::write(
                self.path.join("Cargo.toml"),
                format!(
                    r#"
[workspace]
resolver = "2"
members = [
{entries}
]
"#
                ),
            )
            .expect("workspace manifest should be writable");
        }

        fn write_package(&self, name: &str, dependencies: &str) {
            self.write_package_with_source(name, dependencies, "");
        }

        /// Write a `shaahid` facade package depending only on `shaahid-contract`, so the
        /// facade `CrateBoundary` has a real target in a fixture workspace.
        fn write_facade(&self) {
            self.write_package(
                "shaahid",
                "[dependencies]\nshaahid-contract = { path = \"../shaahid-contract\" }\n",
            );
        }

        fn write_package_with_source(&self, name: &str, dependencies: &str, source: &str) {
            let package = self.path.join(name);
            fs::create_dir_all(package.join("src")).expect("package source dir should be writable");
            fs::write(
                package.join("Cargo.toml"),
                format!(
                    r#"
[package]
name = "{name}"
version = "0.1.0"
edition = "2024"
{dependencies}
"#
                ),
            )
            .expect("package manifest should be writable");
            fs::write(package.join("src/lib.rs"), source)
                .expect("package source should be writable");
        }
    }

    impl Drop for TempWorkspace {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}
