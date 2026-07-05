#!/usr/bin/env bash
# Release script for opensearch-client-rs workspace.
#
# Usage:
#   ./scripts/release.sh <version>          # full release
#   ./scripts/release.sh <version> --dry-run # rehearsal: no git push, no publish
#
# <version> must be a semantic version string, e.g. "0.3.2" (no leading "v").
#
# What this does:
#   1. Pre-flight checks (clean tree, tests pass, tools present)
#   2. Bumps the version in Cargo.toml (workspace + dependency declarations)
#   3. Updates the docs.rs URL in Cargo.toml
#   4. Promotes [Unreleased] in CHANGELOG.md to the new version
#   5. cargo fmt + cargo clippy + cargo test
#   6. git commit + tag v<version>
#   7. git push + push tag
#   8. GitHub release with changelog body
#   9. cargo publish for each crate in dependency order

set -euo pipefail

# ── helpers ──────────────────────────────────────────────────────────────────

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; CYAN='\033[0;36m'; NC='\033[0m'

info()  { echo -e "${CYAN}[release]${NC} $*"; }
ok()    { echo -e "${GREEN}[release]${NC} $*"; }
warn()  { echo -e "${YELLOW}[release]${NC} $*"; }
die()   { echo -e "${RED}[release] ERROR:${NC} $*" >&2; exit 1; }

run() {
    if [[ "$DRY_RUN" == "true" ]]; then
        echo -e "${YELLOW}[dry-run]${NC} $*"
    else
        "$@"
    fi
}

# ── argument parsing ──────────────────────────────────────────────────────────

if [[ $# -lt 1 ]]; then
    die "Usage: $0 <version> [--dry-run]"
fi

NEW_VERSION="$1"
DRY_RUN="false"
for arg in "${@:2}"; do
    [[ "$arg" == "--dry-run" ]] && DRY_RUN="true"
done

# Validate semver format (major.minor.patch, optional pre-release)
if ! echo "$NEW_VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$'; then
    die "Version '$NEW_VERSION' is not a valid semver string (e.g. 0.3.2)"
fi

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TAG="v${NEW_VERSION}"

[[ "$DRY_RUN" == "true" ]] && warn "Dry-run mode — no git push, no publish"
info "Releasing ${TAG} from ${REPO_ROOT}"

cd "$REPO_ROOT"

# ── pre-flight ────────────────────────────────────────────────────────────────

info "Pre-flight checks…"

command -v cargo  &>/dev/null || die "'cargo' not found"
command -v gh     &>/dev/null || die "'gh' not found — install from https://cli.github.com"
command -v git    &>/dev/null || die "'git' not found"
command -v sed    &>/dev/null || die "'sed' not found"

# Ensure logged in to crates.io
if [[ "$DRY_RUN" == "false" ]]; then
    cargo login --help &>/dev/null  # sanity-check cargo is usable
    if ! cargo owner --list opensearch-dsl &>/dev/null 2>&1; then
        warn "Could not verify crates.io ownership — ensure you are logged in with 'cargo login'"
    fi
fi

# Ensure logged in to GitHub
if [[ "$DRY_RUN" == "false" ]]; then
    gh auth status &>/dev/null || die "Not authenticated with GitHub. Run 'gh auth login' first."
fi

# Working tree must be clean
if ! git diff --quiet || ! git diff --cached --quiet; then
    die "Working tree has uncommitted changes. Commit or stash them first."
fi

# Must be on main branch
CURRENT_BRANCH="$(git rev-parse --abbrev-ref HEAD)"
if [[ "$CURRENT_BRANCH" != "main" ]]; then
    warn "Current branch is '${CURRENT_BRANCH}', not 'main'. Proceed? [y/N]"
    read -r answer
    [[ "$answer" =~ ^[Yy]$ ]] || die "Aborted."
fi

# Tag must not already exist
if git tag --list "$TAG" | grep -q "$TAG"; then
    die "Tag $TAG already exists."
fi

# Derive current version from workspace Cargo.toml
CURRENT_VERSION="$(grep -m1 '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')"
info "Current version: ${CURRENT_VERSION}  →  New version: ${NEW_VERSION}"

# ── bump version in Cargo.toml ────────────────────────────────────────────────

info "Bumping version in Cargo.toml…"

# Workspace package version
sed -i "s/^version = \"${CURRENT_VERSION}\"/version = \"${NEW_VERSION}\"/" Cargo.toml

# Workspace dependency declarations
sed -i "s/version = \"${CURRENT_VERSION}\"$/version = \"${NEW_VERSION}\"/" Cargo.toml

# Update docs.rs URL (points to the previous release)
sed -i "s|docs.rs/opensearch-client/[^/]*/|docs.rs/opensearch-client/${NEW_VERSION}/|" Cargo.toml

ok "Cargo.toml updated"

# ── bump CHANGELOG.md ─────────────────────────────────────────────────────────

info "Updating CHANGELOG.md…"
TODAY="$(date +%Y-%m-%d)"
CHANGELOG="CHANGELOG.md"

if ! grep -q '## \[Unreleased\]' "$CHANGELOG"; then
    die "CHANGELOG.md has no '## [Unreleased]' section. Add release notes there first."
fi

# Replace "## [Unreleased] — 0.x.x" (or just "## [Unreleased]") with the new version heading
# and insert a fresh [Unreleased] placeholder above it.
UNRELEASED_PLACEHOLDER="## [Unreleased]\n\n### Added\n\n### Changed\n\n### Fixed\n\n---\n"
sed -i "s|## \[Unreleased\].*|${UNRELEASED_PLACEHOLDER}\n## [${NEW_VERSION}] — ${TODAY}|" "$CHANGELOG"

# Update the comparison links at the bottom
if grep -q '^\[Unreleased\]:' "$CHANGELOG"; then
    REPO_URL="$(grep -m1 '^repository' Cargo.toml | sed 's/repository = "\(.*\)"/\1/')"
    sed -i "s|\[Unreleased\]: .*|\[Unreleased\]: ${REPO_URL}/compare/${TAG}...HEAD|" "$CHANGELOG"
    # Insert a new link for this version below the Unreleased line
    PREV_TAG="v${CURRENT_VERSION}"
    NEW_LINK="[${NEW_VERSION}]: ${REPO_URL}/compare/${PREV_TAG}...${TAG}"
    sed -i "/^\[Unreleased\]:/a ${NEW_LINK}" "$CHANGELOG"
fi

ok "CHANGELOG.md updated"

# ── quality gates ─────────────────────────────────────────────────────────────

info "Running cargo fmt --check…"
cargo fmt --check || die "Formatting check failed. Run 'cargo fmt' and commit."

info "Running cargo clippy…"
cargo clippy -- -D warnings || die "Clippy check failed. Fix warnings before releasing."

info "Running cargo test…"
cargo test || die "Tests failed. Fix them before releasing."

ok "All quality gates passed"

# ── git commit + tag ──────────────────────────────────────────────────────────

info "Committing version bump…"
run git add Cargo.toml Cargo.lock CHANGELOG.md
run git commit -m "chore: release ${TAG}"

info "Creating tag ${TAG}…"
run git tag -a "$TAG" -m "Release ${TAG}"

# ── push to remote ────────────────────────────────────────────────────────────

info "Pushing commits and tag…"
run git push origin "$CURRENT_BRANCH"
run git push origin "$TAG"

# ── GitHub release ────────────────────────────────────────────────────────────

info "Creating GitHub release…"

# Extract the section for this version from CHANGELOG.md as release notes.
# Grab everything between "## [VERSION]" and the next "## [" heading.
RELEASE_NOTES="$(awk "/^## \[${NEW_VERSION}\]/{found=1; next} found && /^## \[/{exit} found{print}" "$CHANGELOG")"

if [[ -z "$RELEASE_NOTES" ]]; then
    warn "Could not extract release notes from CHANGELOG.md; release will have an empty body."
fi

run gh release create "$TAG" \
    --title "Release ${TAG}" \
    --notes "$RELEASE_NOTES"

ok "GitHub release ${TAG} created"

# ── publish to crates.io ──────────────────────────────────────────────────────
#
# Publish in dependency order:
#   1. opensearch-dsl       (no internal deps)
#   2. opensearch-macro     (no internal deps)
#   3. opensearch-testcontainer (no internal deps)
#   4. opensearch-client    (depends on dsl, macro)
#   5. opensearch-cli       (depends on client, dsl)
#
# crates.io needs ~30 s to index each crate before dependents can resolve it.

CRATES=(
    "opensearch-dsl"
    "opensearch-macro"
    "opensearch-testcontainer"
    "opensearch-client"
    "opensearch-cli"
)

PUBLISH_DELAY=40   # seconds between publishes

for crate in "${CRATES[@]}"; do
    info "Publishing ${crate}…"
    if [[ "$DRY_RUN" == "true" ]]; then
        echo -e "${YELLOW}[dry-run]${NC} cargo publish --package ${crate} --no-verify"
    else
        cargo publish --package "$crate" --no-verify
        if [[ "$crate" != "${CRATES[-1]}" ]]; then
            info "Waiting ${PUBLISH_DELAY}s for crates.io to index ${crate}…"
            sleep "$PUBLISH_DELAY"
        fi
    fi
done

ok "All crates published to crates.io"

echo ""
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}  Released ${TAG} successfully!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "  GitHub release: https://github.com/aparo/opensearch-client-rs/releases/tag/${TAG}"
echo "  crates.io:      https://crates.io/crates/opensearch-client/${NEW_VERSION}"
echo ""
