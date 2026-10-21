#!/usr/bin/env python3
"""Regenerate the Valence consumer locks and execute its source and workspace gates."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import time

base = Path('/home/brittonr/mantle-cairn-finish')
source = base / 'xrepo/valence-isolated-source-cohort-resume28'
run = base / 'runs/valence-isolated-source-cohort-resume29'
run.mkdir(exist_ok=False)
records = []
os.environ['CARGO_HOME'] = '/home/brittonr/.cargo'
os.environ['CARGO_NET_GIT_FETCH_WITH_CLI'] = 'true'
os.environ.pop('CARGO_NET_OFFLINE', None)
os.environ['RUSTC_WRAPPER'] = ''
nix = str(Path(shutil.which('nix')).resolve(strict=True))
b3 = str(Path(shutil.which('b3sum')).resolve(strict=True))


def execute(label, command, require_tests=False):
    started = time.monotonic()
    stdout = run / (label + '.stdout')
    stderr = run / (label + '.stderr')
    with stdout.open('xb') as out, stderr.open('xb') as err:
        result = subprocess.run(command, cwd=source, stdout=out, stderr=err)
    row = dict(label=label, command=command, cwd=str(source), exit=result.returncode, elapsed_s=time.monotonic() - started)
    if require_tests:
        text = stdout.read_text(errors='replace') + stderr.read_text(errors='replace')
        row['test_results'] = [line for line in text.splitlines() if line.startswith('test result:')]
        row['executed_behavior_tests'] = any(line.startswith('test result: ok.') and 'ok. 0 passed;' not in line for line in row['test_results'])
    records.append(row)
    (run / 'results.json').write_text(json.dumps(records, indent=2) + '\n')
    print(json.dumps(row), flush=True)
    if result.returncode:
        raise SystemExit(result.returncode)
    if require_tests and not row['executed_behavior_tests']:
        raise SystemExit('workspace behavior tests did not execute')
    return stdout.read_text()

owner = json.loads((base / 'runs/artifact-published-source-verification-resume23-v3/complete.json').read_text())
if owner['revision'] != '5f3962d7471d1547e044f57daeda72f56621fd52' or any(record['exit'] for record in owner['records']):
    raise SystemExit('published owner source acceptance differs')
prior_run = base / 'runs/valence-isolated-source-cohort-resume28'
prior_records = json.loads((prior_run / 'results.json').read_text())
if [r['exit'] for r in prior_records[:4]] != [0, 0, 0, 0] or prior_records[4]['label'] != 'package-formatting' or prior_records[4]['exit'] != 1:
    raise SystemExit('unexpected prior acceptance boundary')
shell = [nix, 'develop', '--no-write-lock-file', '--command']

metadata = json.loads((prior_run / 'consumer-resolved-graph.stdout').read_text())
source_prefix = 'git+ssh://git@github.com/OnixResearch/onix-artifact.git'
packages = [dict(name=package['name'], source=package['source']) for package in metadata['packages'] if (package.get('source') or '').startswith(source_prefix) or package['name'].startswith('artifact-')]
expected_source = source_prefix + '?rev=' + owner['revision'] + '#' + owner['revision']
if sorted(package['name'] for package in packages) != ['artifact-auth-core', 'artifact-auth-ed25519'] or any(package['source'] != expected_source for package in packages):
    raise SystemExit('consumer graph is not the exact two-package authentication cohort: ' + repr(packages))

execute('affected-package-formatting', shell + ['cargo', 'fmt', '-p', 'valence-core', '-p', 'valence-cli', '-p', 'valence-radicle', '-p', 'valence-shared-identity-pilot', '--', '--check'])
execute('workspace-lib-tests', shell + ['cargo', 'test', '--locked', '--offline', '--workspace', '--lib', '--tests'], require_tests=True)
execute('strict-workspace-clippy', shell + ['cargo', 'clippy', '--locked', '--offline', '--workspace', '--all-targets', '--', '-D', 'warnings'])
execute('actual-valence-cli-build', shell + ['cargo', 'build', '--locked', '--offline', '-p', 'valence-cli'])
lock_hashes = {name: execute(name.replace('.', '-') + '-blake3', [b3, '--no-names', str(source / name)]).strip() for name in ['Cargo.lock', 'flake.lock']}
(run / 'complete.json').write_text(json.dumps(dict(source=str(source), prior_run=str(prior_run), prior_records=prior_records, preserved_unrelated_baseline_format_failure='pilots/schema-identity/src/lib.rs: verify_published_cohort line wrapping; original public main source retained', artifact_revision=owner['revision'], consumer_packages=packages, generated_lock_blake3=lock_hashes, records=records, receiver_smoke_completed=False, lifecycle_completed=False, publication=False, real_reviewer_approval=False, release_authority=False), indent=2) + '\n')
