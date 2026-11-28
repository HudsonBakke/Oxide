"""Compile HLSL and surface errors in the CI job summary step name."""
import os
import subprocess
from pathlib import Path
result = subprocess.run(['pwsh', '-NoProfile', '-File', './scripts/build-shaders.ps1'], stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, encoding='utf-8', errors='replace')
print(result.stdout)
if result.returncode and os.environ.get('GITHUB_OUTPUT'):
    errors = [line.strip() for line in result.stdout.splitlines() if 'error:' in line or 'error ' in line]
    with Path(os.environ['GITHUB_OUTPUT']).open('a', encoding='utf-8') as output:
        output.write('detail=' + ' | '.join(errors)[:3000] + '\n')
raise SystemExit(result.returncode)
