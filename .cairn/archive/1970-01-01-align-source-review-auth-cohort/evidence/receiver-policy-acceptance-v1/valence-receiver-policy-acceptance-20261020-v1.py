#!/usr/bin/env python3
"""Bounded Valence receiver and owner-policy evidence; no source mutations."""
import json, os, shutil, subprocess, time
from pathlib import Path
BASE=Path('/home/brittonr/mantle-cairn-finish')
SOURCE=BASE/'xrepo/valence-artifact-cohort-20261020'
RUN=BASE/'runs/valence-receiver-policy-acceptance-20261020-v1'
RUN.mkdir(exist_ok=False)
CLI=BASE/'runs/cairn-source-review-native-complete-20261020-v1/cairn'
VAL=RUN/'valence'
shutil.copy2(BASE/'targets/valence-source-review-v1/debug/valence', VAL)
FIX=RUN/'TEST-ONLY-NOT-REAL-REVIEW'; FIX.mkdir()
ORIGIN=BASE/'runs/cairn-source-review-operator-smoke-20261020-v1/TEST-ONLY-NOT-REAL-REVIEW'
for name in ['policy.json','approved-attachment.json','needs_revision-attachment.json','tampered-source.json','revoked-policy.json','author-policy.json','approved-request.json','approved-unsigned.bin','approved-signature.bin','needs_revision-request.json','needs_revision-unsigned.bin','needs_revision-signature.bin']:
    shutil.copy2(ORIGIN/name,FIX/name)
shutil.copy2(BASE/'runs/cairn-source-review-operator-smoke-20261020-v1/complete.json',RUN/'producer-complete.json')
shutil.copy2(BASE/'bin/jobs/finish-cairn-source-review-operator-smoke-v1.py',RUN/'producer-driver.py')
records=[]
def digest(path): return subprocess.check_output(['b3sum','--no-names',str(path)],text=True).strip()
def capture(label,args,expected=None):
    args=list(map(str,args)); start=time.monotonic()
    with (RUN/(label+'.stdout')).open('xb') as out,(RUN/(label+'.stderr')).open('xb') as err:
        p=subprocess.run(args,cwd=SOURCE,stdout=out,stderr=err,timeout=300)
    row={'label':label,'command':args,'cwd':str(SOURCE),'exit':p.returncode,'elapsed_s':time.monotonic()-start}
    records.append(row); (RUN/'results.json').write_text(json.dumps(records,indent=2)+'\n'); print(json.dumps(row),flush=True)
    if expected is not None and p.returncode!=expected: raise RuntimeError(label+' unexpected exit '+str(p.returncode))
    return row
capture('nickel-version',['nickel','--version'],0)
for name in ['nickel','nickel-export']:
    capture(name+'-runtime',['bash','-c','command -v '+name],0)
capture('policy-export-check',[CLI,'policy','export','--source','cairn-policy/default.ncl','--output','cairn-policy/generated/cairn-policy.json','--check'],0)
positive=SOURCE/'cairn-policy/fixtures/valid-function-address-policy.ncl'
capture('positive-nickel',['nickel','export','--format','json',positive],0)
negative=[]
for fixture in sorted((SOURCE/'cairn-policy/fixtures').glob('invalid-*.ncl')):
    row=capture('negative-'+fixture.stem,['nickel','export','--format','json',fixture])
    if row['exit']==0: raise RuntimeError('negative fixture admitted '+str(fixture))
    negative.append({'path':str(fixture.relative_to(SOURCE)),'blake3':digest(fixture),'exit':row['exit']})
for label,args in [('policy-parser',['policy','--policy','cairn-policy/generated/cairn-policy.json']),('native-status',['status','--root','.cairn']),('native-change-list',['change','list','--root','.cairn']),('native-workflow',['workflow','status','align-source-review-auth-cohort','--root','.cairn']),('default-root-status',['status','--root','.'])]:
    capture(label,[CLI,*args])
receipts=[]
public=json.loads((RUN/'producer-complete.json').read_text())['test_only_public_key_blake3']
policy=json.loads((FIX/'policy.json').read_text())
for name,attachment,pol,exitcode,issue in [('approved','approved-attachment.json','policy.json',0,None),('needs-revision','needs_revision-attachment.json','policy.json',1,None),('revoked','approved-attachment.json','revoked-policy.json',1,'key_not_current'),('substituted-source','tampered-source.json','policy.json',1,'source_subject_mismatch'),('excluded-author','approved-attachment.json','author-policy.json',1,'author_key_excluded')]:
    receipt=name+'-receipt.json'
    capture('receiver-'+name,[VAL,'source-review','verify','--artifact-root',FIX,'--attachment',attachment,'--policy',pol,'--output-root',RUN,'--receipt',receipt],exitcode)
    r=json.loads((RUN/receipt).read_text())
    assert r['attachment_artifact_blake3']==digest(FIX/attachment)
    assert r['policy_artifact_blake3']==digest(FIX/pol)
    assert r['subject']==policy['subject'] and r['claim_root']==policy['claim_root']
    assert r['non_claims']==policy['non_claims'] and r['verification_role']=='manual_review'
    if name=='approved':
        assert r['outcome']=='accepted_manual_review' and r['approved_key_blake3']==[public] and not r['needs_revision_key_blake3']
    else:
        assert r['outcome'] in ['invalid','insufficient'] and not r['approved_key_blake3']
        if name=='needs-revision': assert r['needs_revision_key_blake3']==[public]
        if issue: assert any(issue in value for value in r['issues']),r['issues']
    receipts.append({'case':name,'path':receipt,'file_blake3':digest(RUN/receipt),'receipt_identity':r['receipt_blake3'],'outcome':r['outcome'],'issues':r['issues'],'subject_identity':r['subject_identity'],'claim_root_identity':r['claim_root_identity'],'attachment_identity':r['attachment_identity']})
inputs=[CLI,VAL,SOURCE/'Cargo.toml',SOURCE/'Cargo.lock',SOURCE/'flake.nix',SOURCE/'flake.lock',SOURCE/'cairn-policy/default.ncl',SOURCE/'cairn-policy/contracts.ncl',SOURCE/'cairn-policy/generated/cairn-policy.json',*sorted(FIX.iterdir())]
summary={'schema':'valence.receiver-policy-acceptance.v1','artifact_revision':'5f3962d7471d1547e044f57daeda72f56621fd52','cairn_revision':'1cacafebbff7b2716a953961a1ebf52bafb352e7','records':records,'negative_fixtures':negative,'inputs':[{'path':str(p),'blake3':digest(p)} for p in inputs],'receipts':receipts,'test_only_public_key_blake3':public,'genuine_manual_review':False,'property_proof':False,'witness_authority':False,'release_authority':False,'publication':False,'archive_executed':False}
(RUN/'complete.json').write_text(json.dumps(summary,indent=2)+'\n')
