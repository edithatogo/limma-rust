#!/usr/bin/env python3
"""Run the same strict native gates locally and in CI, preserving actual evidence."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import platform
import re
import shutil
import subprocess
import sys
import tempfile
import time
from datetime import UTC, datetime
from pathlib import Path

from bootstrap import CARGO_TOOLS, NATIVE_TOOLS, RUST

ROOT = Path(__file__).resolve().parents[1]
TOOLS = {
    "python": ([sys.executable, "--version"], "Python 3.15.2"),
    "uv": (["uv", "--version"], "uv 0.13.9"),
    "rustc": (["rustc", f"+{RUST}", "--version"], f"rustc {RUST} "),
    "cargo": (["cargo", f"+{RUST}", "--version"], f"cargo {RUST} "),
    "coverage": (
        ["cargo", "llvm-cov", "--version"],
        f"cargo-llvm-cov {CARGO_TOOLS['cargo-llvm-cov']}",
    ),
    "audit": (["cargo-audit", "--version"], f"cargo-audit {CARGO_TOOLS['cargo-audit']}"),
    "gitleaks": (["gitleaks", "version"], NATIVE_TOOLS["gitleaks"][0]),
    "actionlint": (["actionlint", "--version"], NATIVE_TOOLS["actionlint"][0]),
}


def source_files() -> list[Path]:
    """Include pending work; never certify only HEAD while testing a dirty tree."""
    result = subprocess.run(
        ["git", "ls-files", "--cached", "--others", "--exclude-standard", "-z"],
        cwd=ROOT,
        capture_output=True,
        check=True,
    )
    return sorted({ROOT / os.fsdecode(name) for name in result.stdout.split(b"\0") if name})


def source_identity(files: list[Path]) -> dict[str, str]:
    identities: dict[str, str] = {}
    for path in files:
        if path.is_symlink():
            content = b"symlink\0" + os.fsencode(os.readlink(path))
        elif path.is_file():
            content = path.read_bytes()
        else:
            content = b"directory-or-missing\0"
        identities[str(path.relative_to(ROOT))] = hashlib.sha256(content).hexdigest()
    return identities


def run_step(
    name: str,
    argv: list[str],
    output: Path,
    env: dict[str, str],
    expected_version: str | None = None,
) -> dict[str, object]:
    started = time.monotonic()
    log = output / f"{name}.log"
    try:
        with log.open("w", encoding="utf-8") as handle:
            result = subprocess.run(
                argv,
                cwd=ROOT,
                env=env,
                stdout=handle,
                stderr=subprocess.STDOUT,
                check=False,
                timeout=2400,
            )
        exit_code = result.returncode
        if (
            expected_version is not None
            and re.search(
                rf"(?<![\w.+-]){re.escape(expected_version.strip())}(?![\w.+-])",
                log.read_text(encoding="utf-8"),
            )
            is None
        ):
            exit_code = exit_code or 1
    except (OSError, subprocess.TimeoutExpired) as error:
        with log.open("a", encoding="utf-8") as handle:
            handle.write(f"{type(error).__name__}: required command could not complete\n")
        exit_code = 1
    return {
        "name": name,
        "argv": argv,
        "exit_code": exit_code,
        "status": "pass" if exit_code == 0 else "fail",
        "seconds": round(time.monotonic() - started, 3),
        "log": log.name,
    }


def check_documents(files: list[Path]) -> None:
    """Check local JSON and Markdown links without introducing a docs framework."""
    import tomllib

    for path in files:
        if not path.is_file():
            continue
        if path.suffix == ".json":
            json.loads(path.read_text(encoding="utf-8"))
        elif path.suffix == ".toml":
            tomllib.loads(path.read_text(encoding="utf-8"))
        elif path.suffix == ".md":
            for link in re.findall(
                r"(?<!!)\[[^\]]+\]\(([^\s)]+)\)", path.read_text(encoding="utf-8")
            ):
                target = link.split("#", 1)[0]
                if target and not re.match(r"[a-zA-Z][a-zA-Z0-9+.-]*:", target):
                    if not (path.parent / target).exists():
                        raise ValueError(f"Broken local link: {path.relative_to(ROOT)} -> {target}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, help="new directory for logs and receipt")
    parser.add_argument(
        "--base",
        default=os.environ.get("VERIFY_BASE", "origin/main"),
        help="Git base for the 90 percent changed-line coverage gate",
    )
    parser.add_argument("--documents-only", action="store_true", help=argparse.SUPPRESS)
    args = parser.parse_args()
    files = source_files()
    if args.documents_only:
        check_documents(files)
        return 0

    output = args.output or ROOT / "artifacts/verification" / datetime.now(UTC).strftime(
        "%Y%m%dT%H%M%S%fZ"
    )
    output = output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    env = os.environ.copy()
    env["PATH"] = str(ROOT / "artifacts/tools/bin") + os.pathsep + env.get("PATH", "")
    if env.get("CARGO_HOME"):
        env["PATH"] = str(Path(env["CARGO_HOME"]) / "bin") + os.pathsep + env["PATH"]
    # A developer's private alternate reference corpus must not replace the committed suite.
    env.pop("MD_GOLDEN_CORPUS_DIR", None)
    env.pop("LIMMA_FRESH_REFERENCE_DIR", None)
    env.pop("GITLEAKS_CONFIG", None)
    env.pop("GITLEAKS_CONFIG_TOML", None)
    env.update(CARGO_TERM_COLOR="never", RUSTDOCFLAGS="-D warnings")
    env["COVERAGE_FILE"] = str(output / ".coverage")
    source_hashes = source_identity(files)
    receipt: dict[str, object] = {
        "schema_version": 1,
        "started_at": datetime.now(UTC).isoformat(),
        "source_commit": subprocess.check_output(
            ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True
        ).strip(),
        "source_sha256": source_hashes,
        "coverage_base": subprocess.check_output(
            ["git", "rev-parse", "--verify", "--end-of-options", f"{args.base}^{{commit}}"],
            cwd=ROOT,
            text=True,
        ).strip(),
        "platform": platform.platform(),
        "python": platform.python_version(),
        "evidence_class": "code_checks_only",
        "github_run_id": env.get("GITHUB_RUN_ID"),
        "runner_environment": env.get("RUNNER_ENVIRONMENT"),
        "device_acceptance": "not_established_by_this_command",
        "fresh_R_execution": "not_run",
        "checks": [],
        "status": "running",
    }
    checks: list[dict[str, object]] = []
    receipt["checks"] = checks

    def record(name: str, argv: list[str], expected: str | None = None) -> None:
        checks.append(run_step(name, argv, output, env, expected))
        (output / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n", encoding="utf-8")
        print(f"{name}: {checks[-1]['status']}", flush=True)

    for name, (argv, version) in TOOLS.items():
        record(f"version-{name}", argv, version)

    cargo = ["cargo", f"+{RUST}"]
    commands = [
        ("whitespace", ["git", "diff", "--check", "HEAD"]),
        ("documents", [sys.executable, str(Path(__file__).resolve()), "--documents-only"]),
        ("python-lock", ["uv", "lock", "--check"]),
        ("python-format", ["uv", "run", "--locked", "ruff", "format", "--check", "."]),
        ("python-lint", ["uv", "run", "--locked", "ruff", "check", "."]),
        ("python-types", ["uv", "run", "--locked", "pyright"]),
        (
            "harness-tests",
            [
                "uv",
                "run",
                "--locked",
                "coverage",
                "run",
                "-m",
                "unittest",
                "discover",
                "-s",
                "scripts",
                "-p",
                "test_verify.py",
                "-v",
            ],
        ),
        (
            "python-coverage",
            ["uv", "run", "--locked", "coverage", "xml", "-o", str(output / "python-coverage.xml")],
        ),
        (
            "python-advisories",
            ["uv", "run", "--locked", "pip-audit", "--strict", "--format", "json"],
        ),
        ("workflows", ["actionlint"]),
        ("format", cargo + ["fmt", "--all", "--", "--check"]),
        (
            "clippy",
            cargo
            + [
                "clippy",
                "--locked",
                "--workspace",
                "--all-targets",
                "--all-features",
                "--",
                "-D",
                "warnings",
            ],
        ),
        ("debug-tests", cargo + ["test", "--locked", "--workspace", "--all-features"]),
        (
            "release-tests",
            cargo + ["test", "--locked", "--workspace", "--all-features", "--release"],
        ),
        ("rustdoc", cargo + ["doc", "--locked", "--workspace", "--all-features", "--no-deps"]),
        ("dependency-metadata", cargo + ["metadata", "--locked", "--format-version", "1"]),
        ("dependency-audit", ["cargo", "audit", "--deny", "warnings", "--json"]),
        (
            "coverage",
            cargo
            + [
                "llvm-cov",
                "--locked",
                "--workspace",
                "--all-features",
                "--lcov",
                "--output-path",
                str(output / "lcov.info"),
            ],
        ),
        (
            "rust-coverage-xml",
            cargo
            + [
                "llvm-cov",
                "report",
                "--cobertura",
                "--output-path",
                str(output / "rust-coverage.xml"),
            ],
        ),
        (
            "patch-coverage",
            [
                "uv",
                "run",
                "--locked",
                "diff-cover",
                str(output / "python-coverage.xml"),
                str(output / "rust-coverage.xml"),
                "--compare-branch",
                str(receipt["coverage_base"]),
                "--fail-under",
                "90",
                "--include-untracked",
                "--branch-coverage",
            ],
        ),
    ]
    for name, argv in commands:
        record(name, argv)
    # Scan only the versioned candidate and visible pending files, not build outputs or private caches.
    with tempfile.TemporaryDirectory(prefix="limma-source-scan-") as temporary:
        snapshot = Path(temporary)
        for path in files:
            if path.is_symlink():
                raise ValueError(
                    f"Source symlink requires separate scan qualification: {path.relative_to(ROOT)}"
                )
            if path.is_file():
                destination = snapshot / path.relative_to(ROOT)
                destination.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(path, destination)
        config = output / "secret-scan.toml"
        config.write_text("[extend]\nuseDefault = true\n", encoding="utf-8")
        ignored = output / "empty-gitleaksignore"
        ignored.touch()
        record(
            "source-secrets",
            [
                "gitleaks",
                "dir",
                str(snapshot),
                "--redact",
                "--no-banner",
                "--exit-code",
                "1",
                "--config",
                str(config),
                "--gitleaks-ignore-path",
                str(ignored),
                "--ignore-gitleaks-allow",
            ],
        )
    coverage = output / "lcov.info"
    if not coverage.is_file() or "DA:" not in coverage.read_text(encoding="utf-8"):
        checks.append({"name": "coverage-data", "status": "fail", "exit_code": 1})
    python_coverage = output / "python-coverage.xml"
    if not python_coverage.is_file() or not python_coverage.stat().st_size:
        checks.append({"name": "python-coverage-data", "status": "fail", "exit_code": 1})
    if source_identity(source_files()) != source_hashes:
        checks.append({"name": "source-stability", "status": "fail", "exit_code": 1})
    receipt["status"] = "pass" if all(check["status"] == "pass" for check in checks) else "fail"
    receipt["finished_at"] = datetime.now(UTC).isoformat()
    (output / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n", encoding="utf-8")
    print(f"Strict verification: {receipt['status']}; receipt: {output / 'receipt.json'}")
    return 0 if receipt["status"] == "pass" else 1


if __name__ == "__main__":
    raise SystemExit(main())
