"""Run binding checks and expose useful failure details in GitHub Actions."""
import os
from pathlib import Path
import re
import subprocess
import sys


def run(command):
    result = subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                            text=True, encoding='utf-8', errors='replace')
    print(result.stdout, end='')
    if result.returncode:
        if os.environ.get('GITHUB_OUTPUT'):
            lines = result.stdout.splitlines()
            selected = [line for line in lines if re.search(r'error |Error|Exception|FAILED|Assertion|Traceback|File "', line)]
            detail = ' | '.join((selected or lines[-12:])[-12:])[:2500]
            with Path(os.environ['GITHUB_OUTPUT']).open('a', encoding='utf-8') as output:
                output.write('detail=' + detail + '\n')
        raise SystemExit(result.returncode)


if '--dotnet' in sys.argv:
    run(['dotnet', 'build', 'bindings/csharp/Oxide.NET/Oxide.NET.csproj', '--configuration', 'Debug', '-warnaserror:CS1591'])
    for project in sorted(Path('examples/csharp').rglob('*.csproj')):
        run(['dotnet', 'build', str(project), '--configuration', 'Debug'])
    run(['dotnet', 'run', '--project', 'tests/csharp/Oxide.Tests.csproj'])
else:
    run([sys.executable, '-m', 'compileall', '-q', 'bindings/python/oxide', 'examples/python', 'tests'])
    run([sys.executable, '-c', 'import oxide; import oxide.app; import oxide.collision; import oxide.content; import oxide.graphics; import oxide.input; import oxide.math'])
    for path in ['python_math.py', 'python_shadows.py', 'python_audio.py', 'python_monitors.py', 'python_sprites.py']:
        run([sys.executable, str(Path('tests') / path)])
