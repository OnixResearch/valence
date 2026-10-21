#!/usr/bin/env python3
"""Retain final isolated receiver captures and their source and output bindings."""
import json,shutil,subprocess,tarfile
from pathlib import Path
BASE=Path('/home/brittonr/mantle-cairn-finish')
SOURCE=BASE/'xrepo/valence-isolated-source-cohort-resume28'
EVIDENCE=SOURCE/'.cairn/changes/align-source-review-auth-cohort/evidence'
BINDINGS=[('valence-isolated-source-cohort-resume28','isolated-cohort-v28'),('valence-isolated-source-cohort-resume29','isolated-cohort-v29'),('valence-isolated-receiver-smoke-resume30','isolated-receiver-v30'),('valence-isolated-native-final-resume31','isolated-native-v31')]
def digest(path):
    return subprocess.check_output(['b3sum','--no-names',str(path)],text=True).strip()
for run_name,destination_name in BINDINGS:
    source=BASE/'runs'/run_name; destination=EVIDENCE/destination_name
    assert source.is_dir() and not destination.exists()
    destination.mkdir()
    for path in source.iterdir():
        if not path.is_file() or path.name=='valence': continue
        assert path.stat().st_size<=16*1024*1024
        shutil.copy2(path,destination/path.name)
    driver=BASE/'bin/jobs'/(run_name+'.py')
    if driver.is_file(): shutil.copy2(driver,destination/'executed-driver.py')
acceptance=json.loads((EVIDENCE/'isolated-cohort-v29/complete.json').read_text())
smoke=json.loads((EVIDENCE/'isolated-receiver-v30/complete.json').read_text())
native=json.loads((EVIDENCE/'isolated-native-v31/complete.json').read_text())
assert all(row['exit']==0 for row in acceptance['records'])
assert native['gate_verdicts']=={'proposal-gate':'PASS','design-gate':'PASS','tasks-gate':'PASS'}
assert not native['archive_preview_blocked']
locks=acceptance['generated_lock_blake3']
for name,expected in locks.items(): assert digest(SOURCE/name)==expected
members=[]
with tarfile.open(BASE/'inputs/valence-isolated-source-cohort-resume28.tar') as archive:
    for member in archive.getmembers():
        if not member.isfile(): continue
        relative=member.name.removeprefix('./')
        if relative.startswith(('.cairn/','cairn/')): continue
        path=SOURCE/relative
        if relative in ['Cargo.lock','flake.lock','docs/cairn-policy-compatibility.md']: continue
        assert path.read_bytes()==archive.extractfile(member).read(),relative
        members.append(dict(path=relative,blake3=digest(path)))
paths=['Cargo.toml','Cargo.lock','flake.nix','flake.lock','nix/artifact-source-pin.nix','cairn-policy/default.ncl','cairn-policy/contracts.ncl','cairn-policy/generated/cairn-policy.json','docs/cairn-policy-compatibility.md']
receipts=[dict(path=destination+'/complete.json',blake3=digest(EVIDENCE/destination/'complete.json')) for _,destination in BINDINGS if (EVIDENCE/destination/'complete.json').exists()]
summary=dict(schema='valence.isolated-main-line-receiver-acceptance.v1',public_main_base='b5e5558e87f57b0615b50e532846d6865e31042b',initial_source_archive_blake3=digest(BASE/'inputs/valence-isolated-source-cohort-resume28.tar'),artifact_revision=acceptance['artifact_revision'],generated_locks=locks,final_input_bindings=[dict(path=name,blake3=digest(SOURCE/name)) for name in paths],unchanged_product_members=members,proof_bindings=receipts,workspace_test_result_lines=[line for row in acceptance['records'] for line in row.get('test_results',[])],affected_package_formatting_passed=True,workspace_clippy_passed=True,full_workspace_formatting_passed=False,unrelated_baseline_format_failure=acceptance['preserved_unrelated_baseline_format_failure'],receiver_cases=smoke['receipts'],native_gate_verdicts=native['gate_verdicts'],archive_preview_blocked=False,test_only=True,real_human_review=False,release_authority=False,archive_executed=False,publication=False)
(EVIDENCE/'isolated-main-line-acceptance.json').write_text(json.dumps(summary,indent=2)+chr(10))
shutil.copy2(BASE/'bin/jobs/valence-isolated-evidence-retain-resume32.py',EVIDENCE/'isolated-main-line-retention-driver.py')
print(json.dumps(dict(retained=True,unchanged_product_members=len(members),evidence=str(EVIDENCE),summary_blake3=digest(EVIDENCE/'isolated-main-line-acceptance.json'))))
