"""Run Rust tests and expose a concise failure summary in GitHub Actions."""
import os
from pathlib import Path
import re
import subprocess
import sys

result = subprocess.run(
    ["cargo", "check", "--locked", "--all-targets"] if "--check" in sys.argv else
    ["cargo", "test", "--locked", "--lib", "--tests", "--", "--test-threads=1", "--nocapture"],
    stdout=subprocess.PIPE,
    stderr=subprocess.STDOUT,
    text=True,
    encoding="utf-8",
    errors="replace",
)
print(result.stdout, end="")
if result.returncode and os.environ.get("GITHUB_OUTPUT"):
    plain = re.sub(r"\x1b\[[0-9;]*m", "", result.stdout)
    lines = plain.splitlines()
    details = [line for line in lines if any(term in line for term in (
        "error", " -->", "panicked at", "wrong transmission", "receiver must be lit", "opposing camera sort",
        "assertion", "left:", "right:", " != ", "FAILED", "GPU CHECK", "called `Result", "D3D12 validation",
    ))]
    detail = " | ".join((details or lines[-12:])[-12:])[:1800]
    with Path(os.environ["GITHUB_OUTPUT"]).open("a", encoding="utf-8") as output:
        output.write("detail=" + detail + "\n")
sys.exit(result.returncode)
