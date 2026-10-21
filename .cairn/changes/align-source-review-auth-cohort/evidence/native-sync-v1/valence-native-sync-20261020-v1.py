#!/usr/bin/env python3
"""Execute inspected eight-requirement native sync; never execute archive."""
import json, subprocess, time
from pathlib import Path
BASE=Path('/home/brittonr/mantle-cairn-finish'); SOURCE=BASE/'xrepo/valence-artifact-cohort-20261020'
RUN=BASE/'runs/valence-native-sync-20261020-v1'; RUN.mkdir(exist_ok=False)
CLI=BASE/'runs/cairn-source-review-native-complete-20261020-v1/cairn'; CHANGE='align-source-review-auth-cohort'
SMOKE=BASE/'runs/valence-receiver-policy-acceptance-20261020-v1/complete.json'; assert SMOKE.is_file()
PLAN=BASE/'runs/valence-native-readiness-20261020-v1/sync-plan.stdout'; plan=json.loads(PLAN.read_text())
expected={'.cairn/specs/artifact-auth-adoption/spec.md','.cairn/specs/source-review-acceptance/spec.md'}
assert plan['blocked'] is False and plan['mutated'] is False and plan['dry_run'] is True
assert {d['destination'] for d in plan['merge_preflight']['documents']}==expected
assert sum(len(d['operations']) for d in plan['merge_preflight']['documents'])==8
assert all(not d['blocked'] and not d['diagnostics'] for d in plan['merge_preflight']['documents'])
def hashes(paths):
    paths=sorted(paths)
    if not paths:return {}
    values=subprocess.check_output(['b3sum','--no-names',*map(str,paths)],text=True).splitlines()
    return {str(p.relative_to(SOURCE)):d for p,d in zip(paths,values)}
def inventory():
    paths=[]
    for root in ['crates','.cairn','cairn','cairn-policy']:
        paths.extend(p for p in (SOURCE/root).rglob('*') if p.is_file() and not p.is_symlink())
    paths.extend(SOURCE/p for p in ['Cargo.toml','Cargo.lock','flake.nix','flake.lock'])
    return hashes(paths)
for doc in plan['merge_preflight']['documents']:
    assert hashes([SOURCE/doc['source']])[doc['source']]==doc['delta_hash']
before=inventory(); (RUN/'before-inventory.json').write_text(json.dumps(before,indent=2)+'\n')
records=[]
def capture(label,args):
    command=[str(CLI),*args]; start=time.monotonic()
    with (RUN/(label+'.stdout')).open('xb') as out,(RUN/(label+'.stderr')).open('xb') as err:
        p=subprocess.run(command,cwd=SOURCE,stdout=out,stderr=err,timeout=300)
    row={'label':label,'command':command,'cwd':str(SOURCE),'exit':p.returncode,'elapsed_s':time.monotonic()-start}; records.append(row)
    print(json.dumps(row),flush=True)
    assert p.returncode==0
    return json.loads((RUN/(label+'.stdout')).read_text())
result=capture('sync-execute',['sync',CHANGE,'--root','.','--execute'])
assert result['mutated'] is True and result['blocked'] is False and result['dry_run'] is False
post=inventory(); changed=sorted(k for k in set(before)|set(post) if before.get(k)!=post.get(k))
assert set(changed)==expected,changed
(RUN/'after-inventory.json').write_text(json.dumps(post,indent=2)+'\n')
validation=capture('post-sync-validation',['validate','--root','.'])
archive=capture('post-sync-archive-plan',['archive',CHANGE,'--root','.'])
assert archive['dry_run'] is True and archive['mutated'] is False
(RUN/'complete.json').write_text(json.dumps({'records':records,'inspected_plan':str(PLAN),'inspected_plan_hash':plan['plan_hash'],'changed_paths':changed,'sync_executed':True,'archive_executed':False,'archive_preview_blocked':archive['blocked'],'archive_preview_reasons':archive['reasons'],'publication':False,'release_authority':False},indent=2)+'\n')
