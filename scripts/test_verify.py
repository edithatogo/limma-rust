"""Contract checks for the gate's failure handling; no numerical fixtures duplicated."""

import hashlib
import io
import json
import os
import subprocess
import sys
import tarfile
import tempfile
import unittest
import zipfile
from contextlib import redirect_stdout
from pathlib import Path
from unittest.mock import patch

import bootstrap
import verify
from bootstrap import extract_binary, verified
from verify import run_step


class VerificationFailureTests(unittest.TestCase):
    def test_candidate_identity_includes_untracked_deleted_and_linked_files(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            subprocess.run(["git", "init", "--quiet", str(root)], check=True)
            tracked = root / "tracked.rs"
            tracked.write_text("original", encoding="utf-8")
            subprocess.run(["git", "-C", str(root), "add", "tracked.rs"], check=True)
            pending = root / "pending.rs"
            pending.write_text("pending", encoding="utf-8")
            with patch.object(verify, "ROOT", root):
                files = verify.source_files()
                self.assertEqual(set(files), {tracked, pending})
                first = verify.source_identity(files)
                tracked.unlink()
                self.assertNotEqual(first, verify.source_identity(verify.source_files()))
                original_is_symlink = Path.is_symlink
                with (
                    patch.object(
                        Path,
                        "is_symlink",
                        lambda path: path == pending or original_is_symlink(path),
                    ),
                    patch.object(verify.os, "readlink", return_value="first-target") as target,
                ):
                    first = verify.source_identity(files)
                    target.return_value = "other-target"
                    self.assertNotEqual(first, verify.source_identity(files))

    def test_document_gate_checks_formats_and_local_targets(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            manifest = root / "example.json"
            config = root / "example.toml"
            document = root / "README.md"
            code = root / "example.rs"
            code.write_text("// example", encoding="utf-8")
            manifest.write_text("{}", encoding="utf-8")
            config.write_text('key = "value"', encoding="utf-8")
            document.write_text(
                "[json](example.json) [web](https://example.org) [anchor](#top)", encoding="utf-8"
            )
            files = [manifest, config, document, code, root / "missing"]
            with (
                patch.object(verify, "ROOT", root),
                patch.object(verify, "source_files", return_value=files),
            ):
                verify.check_documents(files)
                with patch.object(sys, "argv", ["verify.py", "--documents-only"]):
                    self.assertEqual(verify.main(), 0)
                document.write_text("[absent](absent.txt)", encoding="utf-8")
                with self.assertRaises(ValueError):
                    verify.check_documents(files)
                document.write_text("", encoding="utf-8")
                manifest.write_text("{", encoding="utf-8")
                with self.assertRaises(ValueError):
                    verify.check_documents(files)

    def test_bootstrap_pins_cargo_tools_and_native_only_does_not_install_rust(self):
        for native_only in (False, True):
            with (
                self.subTest(native_only=native_only),
                patch.object(
                    sys, "argv", ["bootstrap.py"] + (["--native-only"] if native_only else [])
                ),
                patch.object(bootstrap.subprocess, "run") as execute,
                patch.object(bootstrap, "install_native") as native,
                redirect_stdout(io.StringIO()),
            ):
                bootstrap.main()
                native.assert_called_once()
                if native_only:
                    execute.assert_not_called()
                else:
                    commands = [call.args[0] for call in execute.call_args_list]
                    self.assertEqual(len(commands), 3)
                    for tool, version in bootstrap.CARGO_TOOLS.items():
                        self.assertIn(
                            [
                                "cargo",
                                f"+{bootstrap.RUST}",
                                "install",
                                tool,
                                "--locked",
                                "--version",
                                version,
                            ],
                            commands,
                        )

    def test_command_failure_and_missing_tool_do_not_pass(self):
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory)
            failed = run_step(
                "failed", [sys.executable, "-c", "raise SystemExit(7)"], output, os.environ.copy()
            )
            passed = run_step("passed", [sys.executable, "-c", "pass"], output, os.environ.copy())
            missing = run_step(
                "missing", [str(output / "missing-command")], output, os.environ.copy()
            )
            self.assertEqual(failed["exit_code"], 7)
            self.assertEqual(failed["status"], "fail")
            self.assertEqual(passed["status"], "pass")
            self.assertEqual(missing["status"], "fail")

    def test_wrong_tool_version_is_failure_even_with_zero_exit(self):
        with tempfile.TemporaryDirectory() as directory:
            for version in ("2.0", "1.0.1", "1.0-rc1", "1.0+local"):
                result = run_step(
                    "version",
                    [sys.executable, "-c", f"print('tool {version}')"],
                    Path(directory),
                    os.environ.copy(),
                    "tool 1.0",
                )
                self.assertEqual(result["status"], "fail")

    def test_tool_integrity_and_archive_boundary(self):
        payload = b"test binary"
        self.assertEqual(verified(payload, hashlib.sha256(payload).hexdigest()), payload)
        with self.assertRaises(ValueError):
            verified(payload + b"tampered", hashlib.sha256(payload).hexdigest())
        archive = io.BytesIO()
        with tarfile.open(fileobj=archive, mode="w:gz") as tar:
            for name in ("tool", "../unrelated"):
                member = tarfile.TarInfo(name)
                member.size = len(payload)
                tar.addfile(member, io.BytesIO(payload))
        self.assertEqual(extract_binary(archive.getvalue(), "tool", False), payload)
        with self.assertRaises(KeyError):
            extract_binary(archive.getvalue(), "missing", False)

    def test_full_gate_retains_failure_and_rejects_source_changes(self):
        for mode in ("pass", "failed-step", "source-changed", "missing-coverage"):
            with self.subTest(mode=mode), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                source = root / "input.rs"
                source.write_text("original", encoding="utf-8")
                output = root / "artifacts/receipt"

                def fake_step(
                    name, argv, destination, env, expected_version=None, mode=mode, source=source
                ):
                    if name == "coverage":
                        if mode != "missing-coverage":
                            (destination / "lcov.info").write_text(
                                "SF:input.rs\nDA:1,1\nend_of_record\n", encoding="utf-8"
                            )
                        if mode == "source-changed":
                            source.write_text("changed", encoding="utf-8")
                    if name == "python-coverage" and mode != "missing-coverage":
                        (destination / "python-coverage.xml").write_text(
                            "<coverage />", encoding="utf-8"
                        )
                    status = "fail" if mode == "failed-step" and name == "format" else "pass"
                    return {"name": name, "status": status, "exit_code": int(status == "fail")}

                with (
                    patch.object(verify, "ROOT", root),
                    patch.object(verify, "source_files", return_value=[source]),
                    patch.object(verify, "run_step", side_effect=fake_step),
                    patch.object(verify.subprocess, "check_output", return_value="test-source\n"),
                    patch.object(sys, "argv", ["verify.py", "--output", str(output)]),
                ):
                    status = verify.main()
                receipt = json.loads((output / "receipt.json").read_text(encoding="utf-8"))
                self.assertEqual(status, 0 if mode == "pass" else 1)
                self.assertEqual(receipt["status"], "pass" if mode == "pass" else "fail")

    def test_tool_installation_rejects_wrong_hash_and_link_member(self):
        with self.assertRaises(ValueError):
            verified(b"unexpected", "0" * 64)
        payload = io.BytesIO()
        with tarfile.open(fileobj=payload, mode="w:gz") as archive:
            member = tarfile.TarInfo("gitleaks")
            member.type = tarfile.SYMTYPE
            member.linkname = "outside"
            archive.addfile(member)
        with self.assertRaises(ValueError):
            extract_binary(payload.getvalue(), "gitleaks", False)

    @staticmethod
    def native_fixture(system, machine, suffix):
        resources = {}
        tools = {}
        for name, (version, repository, _) in bootstrap.NATIVE_TOOLS.items():
            architecture = (
                "arm64" if machine == "arm64" else ("x64" if name == "gitleaks" else "amd64")
            )
            filename = f"{name}_{version}_{system}_{architecture}.{suffix}"
            executable = name + (".exe" if suffix == "zip" else "")
            payload = io.BytesIO()
            contents = f"fixture binary: {name}".encode()
            if suffix == "zip":
                with zipfile.ZipFile(payload, "w") as archive:
                    archive.writestr(executable, contents)
            else:
                with tarfile.open(fileobj=payload, mode="w:gz") as archive:
                    member = tarfile.TarInfo(executable)
                    member.size = len(contents)
                    archive.addfile(member, io.BytesIO(contents))
            binary_archive = payload.getvalue()
            manifest = f"{hashlib.sha256(binary_archive).hexdigest()}  {filename}\n".encode()
            base = f"https://github.com/{repository}/releases/download/v{version}"
            resources[f"{base}/{name}_{version}_checksums.txt"] = manifest
            resources[f"{base}/{filename}"] = binary_archive
            tools[name] = (version, repository, hashlib.sha256(manifest).hexdigest())
        return resources, tools

    def test_native_installation_uses_verified_platform_binaries(self):
        for system, machine, suffix in (
            ("linux", "x86_64", "tar.gz"),
            ("darwin", "arm64", "tar.gz"),
            ("windows", "AMD64", "zip"),
        ):
            with self.subTest(system=system), tempfile.TemporaryDirectory() as folder:
                resources, tools = self.native_fixture(system, machine, suffix)
                with (
                    patch.object(bootstrap.platform, "system", return_value=system),
                    patch.object(bootstrap.platform, "machine", return_value=machine),
                    patch.object(bootstrap, "NATIVE_TOOLS", tools),
                    patch.object(
                        bootstrap,
                        "urlopen",
                        side_effect=lambda url, resources=resources, **kwargs: io.BytesIO(
                            resources[url]
                        ),
                    ),
                    redirect_stdout(io.StringIO()),
                ):
                    bootstrap.install_native(Path(folder))
                for name in tools:
                    target = Path(folder) / (name + (".exe" if suffix == "zip" else ""))
                    self.assertEqual(target.read_bytes(), f"fixture binary: {name}".encode())

    def test_native_installation_rejects_tampering_and_unsupported_platforms(self):
        for kind in ("checksums.txt", "tar.gz"):
            with self.subTest(kind=kind), tempfile.TemporaryDirectory() as folder:
                resources, tools = self.native_fixture("linux", "x86_64", "tar.gz")
                altered = next(url for url in resources if url.endswith(kind))
                resources[altered] = b"tampered"
                with (
                    patch.object(bootstrap.platform, "system", return_value="Linux"),
                    patch.object(bootstrap.platform, "machine", return_value="x86_64"),
                    patch.object(bootstrap, "NATIVE_TOOLS", tools),
                    patch.object(
                        bootstrap,
                        "urlopen",
                        side_effect=lambda url, resources=resources, **kwargs: io.BytesIO(
                            resources[url]
                        ),
                    ),
                ):
                    with self.assertRaises(ValueError):
                        bootstrap.install_native(Path(folder))
                self.assertEqual(list(Path(folder).iterdir()), [])
        with (
            patch.object(bootstrap.platform, "system", return_value="unsupported"),
            patch.object(bootstrap, "urlopen") as download,
        ):
            with self.assertRaises(ValueError):
                bootstrap.install_native(Path("unused"))
            download.assert_not_called()

    def test_bootstrap_tool_failure_stops_native_installation(self):
        with (
            patch.object(sys, "argv", ["bootstrap.py"]),
            patch.object(
                bootstrap.subprocess,
                "run",
                side_effect=subprocess.CalledProcessError(1, ["rustup"]),
            ),
            patch.object(bootstrap, "install_native") as native,
        ):
            with self.assertRaises(subprocess.CalledProcessError):
                bootstrap.main()
            native.assert_not_called()


if __name__ == "__main__":
    unittest.main()
