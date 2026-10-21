#!/usr/bin/env python3
"""Final updated-task gates and archive preview only, never terminal mutation."""
import json, subprocess, time
from pathlib import Path
BASE=Path('/home/brittonr/mantle-cairn-finish'); SOURCE=BASE/'xrepo/valence-isolated-source-cohort-resume28'
RUN=BASE/'runs/valence-isolated-native-final-resume31'; RUN.mkdir(exist_ok=False)
CLI=BASE/'runs/cairn-source-review-native-complete-20261020-v1/cairn'; CHANGE='align-source-review-auth-cohort'
records=[]; outputs={}
for label,args in [('validate',['validate','--root','.']),('proposal-gate',['gate','proposal',CHANGE,'--root','.']),('design-gate',['gate','design',CHANGE,'--root','.']),('tasks-gate',['gate','tasks',CHANGE,'--root','.']),('native-status',['status','--root','.']),('native-workflow',['workflow','status',CHANGE,'--root','.']),('sync-plan',['sync',CHANGE,'--root','.']),('archive-plan',['archive',CHANGE,'--root','.']),('readiness',['readiness',CHANGE,'--root','.','--format','json'])]:
    command=[str(CLI),*args]; start=time.monotonic()
    with (RUN/(label+'.stdout')).open('xb') as out,(RUN/(label+'.stderr')).open('xb') as err:
        p=subprocess.run(command,cwd=SOURCE,stdout=out,stderr=err,timeout=300)
    row={'label':label,'command':command,'cwd':str(SOURCE),'exit':p.returncode,'elapsed_s':time.monotonic()-start}; records.append(row)
    print(json.dumps(row),flush=True); assert p.returncode==0
    outputs[label]=json.loads((RUN/(label+'.stdout')).read_text())
    assert outputs[label].get('valid',True) is True
for label in ['proposal-gate','design-gate','tasks-gate']: assert outputs[label]['verdict']=='PASS'
assert outputs['tasks-gate']['task_ordering']['valid'] is True
assert all(n['state']=='done' for n in outputs['tasks-gate']['task_ordering']['nodes'])
assert outputs['archive-plan']['blocked'] is False and outputs['archive-plan']['dry_run'] is True and outputs['archive-plan']['mutated'] is False
assert outputs['sync-plan']['blocked'] is False and all(not d['changed'] for d in outputs['sync-plan']['merge_preflight']['documents'])
(RUN/'complete.json').write_text(json.dumps({'records':records,'gate_verdicts':{k:outputs[k]['verdict'] for k in ['proposal-gate','design-gate','tasks-gate']},'all_five_tasks_done':True,'sync_idempotent':True,'archive_preview_blocked':False,'archive_executed':False,'publication':False,'release_authority':False},indent=2)+'\n')
