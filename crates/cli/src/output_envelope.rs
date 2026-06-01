//! Typed envelope structs for the JSON output contract.
//!
//! This module is the schema-side source of truth for fallow's top-level JSON
//! envelopes.

use fallow_core::results::AnalysisResults;
use fallow_types::envelope::{
    BaselineDeltas, BaselineMatch, CheckSummary, ElapsedMs, EntryPoints, Meta, RegressionResult,
    SchemaVersion, ToolVersion,
};
use serde::Serialize;

use crate::audit::{AuditAttribution, AuditSummary, AuditVerdict};
use crate::health_types::{HealthGroup, HealthReport, RuntimeCoverageReport};
use crate::output_dupes::DupesReportPayload;
use crate::report::dupes_grouping::DuplicationGroup;

/// `fallow coverage setup --json` envelope.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(title = "fallow coverage setup --json"))]
pub struct CoverageSetupOutput {
    pub schema_version: CoverageSetupSchemaVersion,
    pub framework_detected: CoverageSetupFramework,
    pub package_manager: Option<CoverageSetupPackageManager>,
    pub runtime_targets: Vec<CoverageSetupRuntimeTarget>,
    pub members: Vec<CoverageSetupMember>,
    pub config_written: Option<serde_json::Value>,
    pub commands: Vec<String>,
    pub files_to_edit: Vec<CoverageSetupFileToEdit>,
    pub snippets: Vec<CoverageSetupSnippet>,
    pub dockerfile_snippet: Option<String>,
    pub next_steps: Vec<String>,
    pub warnings: Vec<String>,
    #[serde(rename = "_meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum CoverageSetupSchemaVersion {
    #[serde(rename = "1")]
    V1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum CoverageSetupFramework {
    #[serde(rename = "nextjs")]
    NextJs,
    #[serde(rename = "nestjs")]
    NestJs,
    Nuxt,
    #[serde(rename = "sveltekit")]
    SvelteKit,
    Astro,
    Remix,
    Vite,
    PlainNode,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum CoverageSetupPackageManager {
    Npm,
    Pnpm,
    Yarn,
    Bun,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum CoverageSetupRuntimeTarget {
    Node,
    Browser,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CoverageSetupMember {
    pub name: String,
    pub path: String,
    pub framework_detected: CoverageSetupFramework,
    pub package_manager: Option<CoverageSetupPackageManager>,
    pub runtime_targets: Vec<CoverageSetupRuntimeTarget>,
    pub files_to_edit: Vec<CoverageSetupFileToEdit>,
    pub snippets: Vec<CoverageSetupSnippet>,
    pub dockerfile_snippet: Option<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CoverageSetupFileToEdit {
    pub path: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CoverageSetupSnippet {
    pub label: String,
    pub path: String,
    pub content: String,
}

/// `fallow audit --format json` envelope.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(title = "fallow audit --format json"))]
#[allow(
    dead_code,
    reason = "schema-source-of-truth: audit.rs still builds the wire via serde_json::json!; this struct locks the schema shape via the drift gate. Migration is a follow-up to issue #384 items 3a/3b/3c."
)]
pub struct AuditOutput {
    pub schema_version: SchemaVersion,
    pub version: ToolVersion,
    pub command: AuditCommand,
    pub verdict: AuditVerdict,
    pub changed_files_count: u32,
    pub base_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_sha: Option<String>,
    pub elapsed_ms: ElapsedMs,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_snapshot_skipped: Option<bool>,
    pub summary: AuditSummary,
    pub attribution: AuditAttribution,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dead_code: Option<CheckOutput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duplication: Option<DupesReportPayload>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub complexity: Option<HealthReport>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
#[allow(dead_code, reason = "schema-source-of-truth: see `AuditOutput`.")]
pub enum AuditCommand {
    Audit,
}

/// Bare `fallow --format json` envelope.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "schema",
    schemars(title = "fallow --format json (bare, combined)")
)]
pub struct CombinedOutput {
    pub schema_version: SchemaVersion,
    pub version: ToolVersion,
    pub elapsed_ms: ElapsedMs,
    #[serde(rename = "_meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<CombinedMeta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check: Option<CheckOutput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dupes: Option<DupesReportPayload>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<HealthReport>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CombinedMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check: Option<Meta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dupes: Option<Meta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<Meta>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum CoverageAnalyzeSchemaVersion {
    #[serde(rename = "1")]
    V1,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "schema",
    schemars(title = "fallow coverage analyze --format json")
)]
pub struct CoverageAnalyzeOutput {
    pub schema_version: CoverageAnalyzeSchemaVersion,
    pub version: ToolVersion,
    pub elapsed_ms: ElapsedMs,
    pub runtime_coverage: RuntimeCoverageReport,
    #[serde(rename = "_meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(title = "fallow dupes --format json"))]
pub struct DupesOutput {
    pub schema_version: SchemaVersion,
    pub version: ToolVersion,
    pub elapsed_ms: ElapsedMs,
    #[serde(flatten)]
    pub report: DupesReportPayload,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grouped_by: Option<GroupByMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_issues: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<DuplicationGroup>>,
    /// `_meta` block with metric / rule definitions, emitted when `--explain`
    /// is passed (always present in MCP responses).
    #[serde(rename = "_meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    /// Workspace-discovery diagnostics surfaced during config load
    /// (issue #473). See [`CheckOutput::workspace_diagnostics`] for the full
    /// contract; the same list is repeated on each top-level command's
    /// envelope so single-command consumers see it without having to look at
    /// a separate top-level field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub workspace_diagnostics: Vec<fallow_config::WorkspaceDiagnostic>,
}

/// Envelope emitted by `fallow dead-code --format json` (plus the `check`
/// block inside the combined and audit envelopes).
///
/// The body is the full `AnalysisResults` flattened into the envelope so
/// every issue array (`unused_files`, `unused_exports`, ...) lives at the
/// top level, matching the existing wire shape. `entry_points` lifts the
/// otherwise `#[serde(skip)]`'d `AnalysisResults::entry_point_summary` back
/// into the JSON output. `summary` carries the per-category counts the
/// JSON layer always emits.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(title = "fallow dead-code --format json"))]
pub struct CheckOutput {
    pub schema_version: SchemaVersion,
    pub version: ToolVersion,
    pub elapsed_ms: ElapsedMs,
    pub total_issues: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_points: Option<EntryPoints>,
    pub summary: CheckSummary,
    #[serde(flatten)]
    pub results: AnalysisResults,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub baseline_deltas: Option<BaselineDeltas>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub baseline: Option<BaselineMatch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regression: Option<RegressionResult>,
    #[serde(rename = "_meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub workspace_diagnostics: Vec<fallow_config::WorkspaceDiagnostic>,
}

/// Envelope emitted by `fallow dead-code --group-by ... --format json`.
///
/// Issues are partitioned into resolver buckets (CODEOWNERS team, directory
/// prefix, workspace package, or GitLab CODEOWNERS section) instead of flat
/// arrays. Each bucket carries the same issue-array shape as the ungrouped
/// `CheckOutput` body, plus per-group `key` / `owners` / `total_issues`.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "schema",
    schemars(
        title = "fallow dead-code --group-by <owner|directory|package|section> --format json"
    )
)]
pub struct CheckGroupedOutput {
    pub schema_version: SchemaVersion,
    pub version: ToolVersion,
    pub elapsed_ms: ElapsedMs,
    pub grouped_by: GroupByMode,
    pub total_issues: usize,
    pub groups: Vec<CheckGroupedEntry>,
    #[serde(rename = "_meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

/// Single resolver bucket inside `CheckGroupedOutput`. Carries the group's
/// identifier, optional section owners, and a per-group flattened
/// `AnalysisResults`.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CheckGroupedEntry {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owners: Option<Vec<String>>,
    pub total_issues: usize,
    #[serde(flatten)]
    pub results: AnalysisResults,
}

/// Envelope emitted by `fallow health --format json` (plus the `health` block
/// inside the combined and audit envelopes).
///
/// The body is `HealthReport` flattened into the envelope so every report
/// field (`findings`, `summary`, `vital_signs`, `hotspots`, `actions_meta`,
/// ...) lives at the top level. Grouped runs populate `grouped_by` +
/// `groups` with per-bucket recomputed metrics. The `actions_meta`
/// breadcrumb is modeled on `HealthReport` as an `Option<HealthActionsMeta>`
/// and is set at construction time by the report builder when the active
/// `HealthActionContext` requests suppress-line omission, so the schema
/// documents the field and serde populates it natively.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(title = "fallow health --format json"))]
pub struct HealthOutput {
    pub schema_version: SchemaVersion,
    pub version: ToolVersion,
    pub elapsed_ms: ElapsedMs,
    #[serde(flatten)]
    pub report: HealthReport,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grouped_by: Option<GroupByMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<HealthGroup>>,
    #[serde(rename = "_meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub workspace_diagnostics: Vec<fallow_config::WorkspaceDiagnostic>,
}

/// Envelope emitted by `fallow explain <issue-type> --format json`.
///
/// Standalone rule explanation. This command does not run project analysis
/// and intentionally returns a compact object without `schema_version` /
/// `version` metadata; consumers that need those should call any other
/// fallow JSON-producing command.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "schema",
    schemars(title = "fallow explain <issue-type> --format json")
)]
#[serde(deny_unknown_fields)]
pub struct ExplainOutput {
    pub id: String,
    pub name: String,
    pub summary: String,
    pub rationale: String,
    pub example: String,
    pub how_to_fix: String,
    pub docs: String,
}

/// Envelope emitted by `fallow --format codeclimate` and
/// `fallow --format gitlab-codequality`. GitLab Code Quality consumes the
/// same shape. The wire form is a bare JSON array, not an object.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "schema",
    schemars(title = "fallow --format codeclimate / gitlab-codequality")
)]
#[serde(transparent)]
#[allow(
    dead_code,
    reason = "schema-source-of-truth wrapper: runtime emits a `Vec<CodeClimateIssue>` directly via `codeclimate::issues_to_value`; this newtype exists so `schemars` can title and document the bare-array shape for the drift gate."
)]
pub struct CodeClimateOutput(pub Vec<CodeClimateIssue>);

/// Single CodeClimate-compatible issue inside [`CodeClimateOutput`].
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CodeClimateIssue {
    #[serde(rename = "type")]
    pub kind: CodeClimateIssueKind,
    pub check_name: String,
    pub description: String,
    pub categories: Vec<String>,
    pub severity: CodeClimateSeverity,
    pub fingerprint: String,
    pub location: CodeClimateLocation,
}

/// Discriminator value for [`CodeClimateIssue::kind`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum CodeClimateIssueKind {
    /// The only valid CodeClimate type today.
    Issue,
}

/// CodeClimate severity scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum CodeClimateSeverity {
    /// Informational. Reserved for future severity mappings; not produced
    /// by the current runtime path (which only emits Minor / Major /
    /// Critical via `severity_to_codeclimate` and the health / runtime-
    /// coverage match arms).
    #[allow(
        dead_code,
        reason = "schema-source-of-truth: documents the full CodeClimate severity spec; runtime never produces this variant today, but the schema needs it so consumers can validate against either fallow output or a third-party CodeClimate emitter without spec divergence."
    )]
    Info,
    /// Minor finding.
    Minor,
    /// Major finding.
    Major,
    /// Critical finding.
    Critical,
    /// Blocker (highest severity). Reserved for future severity
    /// mappings; not produced by the current runtime path.
    #[allow(
        dead_code,
        reason = "schema-source-of-truth: documents the full CodeClimate severity spec; runtime never produces this variant today, but the schema needs it so consumers can validate against either fallow output or a third-party CodeClimate emitter without spec divergence."
    )]
    Blocker,
}

/// Location block inside [`CodeClimateIssue::location`].
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CodeClimateLocation {
    /// File path relative to the analysed root.
    pub path: String,
    /// Wrapper carrying the begin line so the schema lines up with
    /// CodeClimate's spec.
    pub lines: CodeClimateLines,
}

/// `lines.begin` for [`CodeClimateLocation`].
#[derive(Debug, Clone, Copy, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CodeClimateLines {
    /// 1-based start line.
    pub begin: u32,
}

/// Envelope emitted by `fallow --format review-github` / `review-gitlab`.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "schema",
    schemars(title = "fallow --format review-github / review-gitlab")
)]
pub struct ReviewEnvelopeOutput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<ReviewEnvelopeEvent>,
    pub body: String,
    #[serde(default = "ReviewEnvelopeSummary::empty_default")]
    pub summary: ReviewEnvelopeSummary,
    pub comments: Vec<ReviewComment>,
    #[serde(default = "default_marker_regex")]
    pub marker_regex: String,
    #[serde(default = "default_marker_regex_flags")]
    pub marker_regex_flags: String,
    pub meta: ReviewEnvelopeMeta,
}

/// Default for [`ReviewEnvelopeOutput::marker_regex`].
#[must_use]
pub fn default_marker_regex() -> String {
    MARKER_REGEX_V2.to_owned()
}

/// Default for [`ReviewEnvelopeOutput::marker_regex_flags`].
#[must_use]
pub fn default_marker_regex_flags() -> String {
    MARKER_REGEX_FLAGS_V2.to_owned()
}

/// Canonical v2 marker-regex literal.
pub const MARKER_REGEX_V2: &str =
    r"^<!-- fallow-fingerprint:v2: ((?:[a-z]+:)?[0-9a-f]{16}) -->\s*$";

/// Canonical v2 marker-regex flags.
pub const MARKER_REGEX_FLAGS_V2: &str = "m";

/// Summary block on [`ReviewEnvelopeOutput`].
#[derive(Debug, Clone, Serialize, Default)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ReviewEnvelopeSummary {
    pub body: String,
    pub fingerprint: String,
}

impl ReviewEnvelopeSummary {
    /// Empty-default factory for [`ReviewEnvelopeOutput::summary`].
    #[must_use]
    #[allow(
        dead_code,
        reason = "referenced via serde default = \"...\" attr; no direct callsite until Deserialize is derived"
    )]
    pub fn empty_default() -> Self {
        Self::default()
    }
}

/// Singleton GitHub review-event marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum ReviewEnvelopeEvent {
    #[serde(rename = "COMMENT")]
    Comment,
}

/// Per-line review comment. Schema is an `anyOf` between GitHub and GitLab
/// shapes; at runtime every entry in a single envelope comes from the same
/// provider because the envelope is built from one provider's branch in
/// `crates/cli/src/report/ci/review.rs::render_review_envelope`.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum ReviewComment {
    GitHub(GitHubReviewComment),
    GitLab(GitLabReviewComment),
}

/// GitHub pull-request review comment.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GitHubReviewComment {
    pub path: String,
    pub line: u32,
    pub side: GitHubReviewSide,
    pub body: String,
    pub fingerprint: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub truncated: bool,
}

/// Singleton side discriminator for [`GitHubReviewComment::side`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum GitHubReviewSide {
    #[serde(rename = "RIGHT")]
    Right,
}

/// GitLab merge-request discussion comment.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GitLabReviewComment {
    pub body: String,
    pub position: GitLabReviewPosition,
    pub fingerprint: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub truncated: bool,
}

/// Helper for `skip_serializing_if = "is_false"` on `truncated` fields above.
/// Serde calls `skip_serializing_if` with `&T`, so the reference signature
/// is dictated by the trait and cannot be changed to pass-by-value. Uses
/// `#[allow]` rather than `#[expect]` per `.claude/rules/code-quality.md`:
/// `trivially_copy_pass_by_ref` is a pedantic lint that fires inconsistently
/// across build configurations (lib vs bin), which would trigger
/// `unfulfilled_lint_expectations` under `#[expect]`.
#[must_use]
#[allow(
    clippy::trivially_copy_pass_by_ref,
    reason = "serde's skip_serializing_if requires fn(&T) -> bool"
)]
pub fn is_false(value: &bool) -> bool {
    !*value
}

/// `position` block inside [`GitLabReviewComment`]. Mirrors the GitLab
/// merge-request discussion-position API.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GitLabReviewPosition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_sha: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_sha: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_sha: Option<String>,
    pub position_type: GitLabReviewPositionType,
    pub old_path: String,
    pub new_path: String,
    pub new_line: u32,
}

/// Singleton position-type discriminator for [`GitLabReviewPosition`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum GitLabReviewPositionType {
    Text,
}

/// `meta` block inside [`ReviewEnvelopeOutput`].
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ReviewEnvelopeMeta {
    pub schema: ReviewEnvelopeSchema,
    pub provider: ReviewProvider,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_conclusion: Option<ReviewCheckConclusion>,
}

/// Schema-version discriminator for the review envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum ReviewEnvelopeSchema {
    /// First release of the review envelope format. Historical only; no v1
    /// emit path remains on the current code. Retained on the enum so a
    /// future Deserialize derive can still parse v1 captures (e.g. from
    /// committed snapshots predating the issue #528 migration) without
    /// erroring on an unknown variant.
    #[serde(rename = "fallow-review-envelope/v1")]
    #[allow(
        dead_code,
        reason = "kept for forward-compat with v1 historical inputs once Deserialize is derived"
    )]
    V1,
    /// Issue #528 evolution. Adds (1) the [`ReviewEnvelopeOutput::summary`]
    /// block, (2) [`ReviewEnvelopeOutput::marker_regex`], (3) same-line
    /// `(path, line)` merging in `comments[]` with a
    /// `merged:<16-char hash>` primary fingerprint over sorted constituent
    /// fingerprints (identity shifts whenever the set of constituents
    /// changes, so the bundled skip-if-fingerprint-exists wrappers
    /// correctly re-post on content change), (4) UTF-8-safe body
    /// truncation at the GitLab/GitHub note-size floor (65,536 bytes)
    /// with paired `truncated: bool` + `<!-- fallow-truncated -->`
    /// signals, (5) `:v2:`-namespaced marker shape
    /// (`<!-- fallow-fingerprint:v2: <fingerprint> -->`) preventing v1
    /// marker collision and user-paste spoofing, and (6) diff-aware
    /// `position.old_path` for renamed files on GitLab.
    #[serde(rename = "fallow-review-envelope/v2")]
    V2,
}

/// Review-envelope provider tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum ReviewProvider {
    /// GitHub pull-request review envelope.
    Github,
    /// GitLab merge-request discussion envelope.
    Gitlab,
}

/// `meta.check_conclusion` for the GitHub review envelope. Maps to the
/// GitHub Checks API conclusion field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum ReviewCheckConclusion {
    /// No findings.
    Success,
    /// Findings but none gated as failure.
    Neutral,
    /// At least one finding gated as failure.
    Failure,
}

/// Envelope emitted by `fallow ci reconcile-review --format json`. Used by
/// CI integrations to drive comment carry-over and stale-comment cleanup
/// across PR / MR revisions.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "schema",
    schemars(title = "fallow ci reconcile-review --format json")
)]
pub struct ReviewReconcileOutput {
    pub schema: ReviewReconcileSchema,
    pub provider: ReviewProvider,
    pub target: Option<String>,
    pub dry_run: bool,
    pub comments: u32,
    pub current_fingerprints: u32,
    pub existing_fingerprints: u32,
    pub new_fingerprints: u32,
    pub stale_fingerprints: u32,
    pub new: Vec<String>,
    pub stale: Vec<String>,
    pub provider_warning: Option<String>,
    pub resolution_comments_posted: u32,
    pub threads_resolved: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_hint: Option<String>,
    pub apply_errors: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub failed_fingerprints: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unapplied_fingerprints: Vec<String>,
}

/// Schema-version discriminator for the review reconcile envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum ReviewReconcileSchema {
    /// First release of the review reconcile format.
    #[serde(rename = "fallow-review-reconcile/v1")]
    V1,
}

/// Resolver mode label for grouped envelopes (dead-code, dupes, health).
///
/// `owner` groups by CODEOWNERS team, `directory` groups by top-level
/// directory prefix, `package` groups by workspace package name, `section`
/// groups by GitLab CODEOWNERS `[Section]` header name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum GroupByMode {
    Owner,
    Directory,
    Package,
    Section,
}
/// Envelope emitted by `fallow list --boundaries --format json`. Surfaces
/// the architecture boundary zones, rules, and (issue #373) the user's
/// pre-expansion `autoDiscover` logical groups so consumers can render
/// grouping intent that `expand_auto_discover` would otherwise flatten out
/// of `zones[]`.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "schema",
    schemars(title = "fallow list --boundaries --format json")
)]
#[allow(
    dead_code,
    reason = "schema-source-of-truth: list.rs still builds the wire via serde_json::json!; this struct and its sub-types lock the schema shape via the drift gate. Migration is a follow-up to issue #384 items 3a/3b/3c."
)]
pub struct ListBoundariesOutput {
    pub boundaries: BoundariesListing,
}

/// `boundaries` block carried by [`ListBoundariesOutput`].
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[allow(
    dead_code,
    reason = "schema-source-of-truth: see `ListBoundariesOutput`."
)]
pub struct BoundariesListing {
    pub configured: bool,
    pub zone_count: usize,
    pub zones: Vec<BoundariesListZone>,
    pub rule_count: usize,
    pub rules: Vec<BoundariesListRule>,
    pub logical_group_count: usize,
    pub logical_groups: Vec<BoundariesListLogicalGroup>,
}

/// A boundary zone after preset and `autoDiscover` expansion. Each entry
/// classifies files into a single zone via glob patterns.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[allow(
    dead_code,
    reason = "schema-source-of-truth: see `ListBoundariesOutput`."
)]
pub struct BoundariesListZone {
    pub name: String,
    pub patterns: Vec<String>,
    pub file_count: usize,
}

/// A boundary import rule, expanded to operate on concrete child zone
/// names after `autoDiscover` flattening. The user's pre-expansion rule
/// (keyed on the logical parent name, if any) is preserved on the
/// corresponding [`BoundariesListLogicalGroup::authored_rule`].
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[allow(
    dead_code,
    reason = "schema-source-of-truth: see `ListBoundariesOutput`."
)]
pub struct BoundariesListRule {
    pub from: String,
    pub allow: Vec<String>,
}

/// A pre-expansion `autoDiscover` logical group surfaced for observability
/// (issue #373). Captured during `expand_auto_discover` so consumers can
/// see the user-authored parent name and grouping intent after expansion
/// would otherwise flatten it out of [`BoundariesListing::zones`].
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[allow(
    dead_code,
    reason = "schema-source-of-truth: see `ListBoundariesOutput`."
)]
pub struct BoundariesListLogicalGroup {
    pub name: String,
    pub children: Vec<String>,
    pub auto_discover: Vec<String>,
    pub status: fallow_config::LogicalGroupStatus,
    pub source_zone_index: usize,
    pub file_count: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authored_rule: Option<fallow_config::AuthoredRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_zone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merged_from: Option<Vec<usize>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_zone_root: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub child_source_indices: Vec<usize>,
}

/// Typed root of every fallow `--format json` envelope shape that
/// serializes as a JSON object. The schema derived from this enum drives
/// the document-root `oneOf` in `docs/output-schema.json`, replacing the
/// previously hand-maintained block.
///
/// `#[serde(untagged)]` preserves wire compatibility: consumers see exactly
/// the same top-level keys today (`schema_version`, `version`, plus the
/// per-envelope shape). The schema's `oneOf` lets agents narrow by trying
/// variants in order; field sets differ enough that the first matching
/// variant is the correct one in practice. Note that [`HealthOutput`] and
/// [`DupesOutput`] flatten their inner body (`HealthReport` /
/// `DuplicationReport`) into top-level fields, so the actual
/// discriminators are nested-body keys such as `health_score` (health) and
/// `clone_groups` (dupes), NOT `report` or `groups`.
///
/// Variant order is **most-specific first**. Schemars 1 preserves
/// declaration order in the emitted `oneOf`, and validators that enforce
/// strict `oneOf` (and any future migration that adds `Deserialize`) will
/// try branches top-to-bottom. The required-field sets shrink as we move
/// down the list, with [`CombinedOutput`] last because its three required
/// fields (`schema_version`, `version`, `elapsed_ms`) are a strict subset
/// of every other variant's required set; placing it earlier would let a
/// `CheckOutput` payload silently match `CombinedOutput` first.
///
/// One envelope is intentionally NOT in this enum:
/// - `CodeClimateOutput` serializes as a bare JSON array
///   (`#[serde(transparent)]`) per the Code Climate / GitLab Code Quality
///   spec; `#[serde(tag = ...)]` cannot internally tag a non-object
///   variant and wrapping the array would break the spec. The root schema
///   carries it as a sibling `oneOf` branch alongside `FallowOutput`.
///
/// A future major release plans to switch this to
/// `#[serde(tag = "kind")]` for true O(1) discriminability on AI / agent
/// consumers, paired with a one-cycle `--legacy-envelope` opt-out flag.
/// Tracked under issue #384.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "schema",
    schemars(title = "fallow --format json (typed root)")
)]
#[serde(untagged)]
#[allow(
    dead_code,
    reason = "consumed at schema-emit time only; runtime code uses the per-variant envelope structs directly"
)]
pub enum FallowOutput {
    /// `fallow audit --format json`. Required `command: "audit"` singleton
    /// plus `verdict` and `summary`.
    Audit(AuditOutput),
    /// `fallow explain <issue-type> --format json`. Required `id`, `name`,
    /// `rationale`, `example`, `how_to_fix`, `docs`; no `schema_version`.
    Explain(ExplainOutput),
    /// `fallow --format review-github` / `--format review-gitlab`. Required
    /// `body`, `comments`, `meta`; no `schema_version`.
    ReviewEnvelope(ReviewEnvelopeOutput),
    /// `fallow ci reconcile-review --format json`. Required `schema`
    /// singleton plus `provider`, `comments`, and the various
    /// `*_fingerprints` arrays.
    ReviewReconcile(ReviewReconcileOutput),
    /// `fallow coverage setup --json`. Required `schema_version` singleton
    /// plus `framework_detected`, `members`, `commands`, `snippets`.
    CoverageSetup(CoverageSetupOutput),
    /// `fallow coverage analyze --format json`. Required
    /// `schema_version: "1"` singleton plus `version`, `elapsed_ms`,
    /// `runtime_coverage`. The `runtime_coverage` discriminator field is
    /// uniquely present here; ordered before broader variants so untagged
    /// narrowing matches `CoverageAnalyzeOutput` first.
    CoverageAnalyze(CoverageAnalyzeOutput),
    /// `fallow list --boundaries --format json`. Required `boundaries`
    /// sub-object; no `schema_version`.
    ListBoundaries(ListBoundariesOutput),
    /// `fallow health --format json`. Required `report: HealthReport`.
    Health(HealthOutput),
    /// `fallow dupes --format json`. Required `report: DupesReportPayload`
    /// (typed wrapper payload carrying `clone_groups[]: CloneGroupFinding`
    /// and `clone_families[]: CloneFamilyFinding`).
    Dupes(DupesOutput),
    /// `fallow check --format json --group-by <mode>`. Required `grouped_by`
    /// plus a `groups` array; ordered before [`Self::Check`] because the
    /// `grouped_by` discriminator field is uniquely present here.
    CheckGrouped(CheckGroupedOutput),
    /// `fallow impact --format json`. Required `enabled`, `record_count`,
    /// `containment_count`, `recent_containment`; no `schema_version`,
    /// `command`, `total_issues`, or `report`. Ordered before the broader
    /// variants because its `record_count` + `containment_count` discriminator
    /// pair is uniquely present here.
    Impact(crate::impact::ImpactReport),
    /// `fallow check --format json` / `fallow dead-code --format json`.
    /// Required `total_issues` plus `summary: CheckSummary`.
    Check(CheckOutput),
    /// Bare `fallow --format json` (combined dead-code + dupes + health).
    /// LAST because its required-field set (`schema_version`, `version`,
    /// `elapsed_ms`) is a strict subset of every other variant's required
    /// set; placing it earlier would let untagged narrowing match a
    /// `CheckOutput` payload against `CombinedOutput` first.
    Combined(CombinedOutput),
}
