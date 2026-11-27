"""Enforce the repository rule that every test lives under tests/."""
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
violations = []
for path in (ROOT / "src").rglob("*"):
    if not path.is_file():
        continue
    text = path.read_text(encoding="utf-8")
    if re.search(r"#\s*\[[^\]]*\btest\b|\bmod\s+\w*tests\b|\bwarp_for_test\b", text):
        violations.append(str(path.relative_to(ROOT)))
for directory in ("src", "examples", "scripts"):
    for path in (ROOT / directory).rglob("*"):
        if path.is_file() and path.suffix in (".rs", ".py", ".cs"):
            name = path.stem.lower()
            if name.startswith(("test_", "tests_")) or name.endswith(("_test", "_tests")):
                violations.append(str(path.relative_to(ROOT)))
if violations:
    raise SystemExit("Move tests to tests/: " + ", ".join(sorted(set(violations))))
print("Test layout OK: all test code is under tests/.")
