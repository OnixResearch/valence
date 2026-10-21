#!/usr/bin/env python3
"""Archive the checked committed producer change and retain post-archive results."""
import json,shutil,subprocess,time
from pathlib import Path
BASE=Path('/home/brittonr/mantle-cairn-finish');SOURCE=BASE/'xrepo/valence-isolated-source-cohort-resume28'
RUN=BASE/'runs/valence-isolated-native-archive-resume34';RUN.mkdir(exist_ok=False)
CLI=Path('/home/brittonr/mantle-cairn-finish/runs/cairn-source-review-native-complete-20261020-v1/cairn');CHANGE='align-source-review-auth-cohort'
head='f93e66192b49f651cdbaa57cbaf2dbbed19e0912'
binding=json.loads((SOURCE/'.cairn/changes/align-source-review-auth-cohort/evidence/isolated-main-line-acceptance.json').read_text())
for entry in binding['final_input_bindings']:
    assert subprocess.check_output(['b3sum','--no-names',str(SOURCE/entry['path'])],text=True).strip()==entry['blake3']
records=[]
def capture(label,args):
    args=list(map(str,args));start=time.monotonic()
    with (RUN/(label+'.stdout')).open('xb') as out,(RUN/(label+'.stderr')).open('xb') as err:
        result=subprocess.run(args,cwd=SOURCE,stdout=out,stderr=err,timeout=600)
    row=dict(label=label,command=args,cwd=str(SOURCE),exit=result.returncode,elapsed_s=time.monotonic()-start)
    records.append(row);(RUN/'results.json').write_text(json.dumps(records,indent=2)+chr(10));print(json.dumps(row),flush=True)
    if result.returncode:raise SystemExit(result.returncode)
    return json.loads((RUN/(label+'.stdout')).read_text())
plan=capture('native-archive-preview',[CLI,'archive',CHANGE,'--root','.'])
assert not plan['blocked'] and plan['reasons']==[]
assert [action['kind'] for action in plan['actions']]==['archive_change']
result=capture('native-archive-execute',[CLI,'archive',CHANGE,'--root','.','--execute'])
assert result['mutated'] and not result['blocked']
assert not (SOURCE/'.cairn/changes'/CHANGE).exists()
archives=[path for root in ['.cairn/archive','cairn/archive'] for path in (SOURCE/root).glob('*-'+CHANGE)]
assert len(archives)==1,archives
archive=archives[0]
assert capture('native-post-archive-validation',[CLI,'validate','--root','.'])['valid']
capture('native-post-archive-status',[CLI,'status','--root','.'])
(RUN/'complete.json').write_text(json.dumps(dict(implementation_revision=head,pre_archive_commit=head,records=records,archive_path=str(archive.relative_to(SOURCE)),archive_executed=True,post_archive_validation_passed=True,release_authority=False,archive_publication=False),indent=2)+chr(10))
evidence=archive/'evidence/native-terminal-v34';evidence.mkdir()
for path in RUN.iterdir():
    if path.is_file():shutil.copy2(path,evidence/path.name)
shutil.copy2(BASE/'bin/jobs/valence-isolated-native-archive-resume34.py',evidence/'executed-driver.py')
print(json.dumps(dict(archive=str(archive),validated=True)))
