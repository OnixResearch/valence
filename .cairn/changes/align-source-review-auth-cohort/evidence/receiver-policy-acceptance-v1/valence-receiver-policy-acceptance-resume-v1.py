#!/usr/bin/env python3
"""Resume exact unchanged inputs, retaining preliminary observations without reruns."""
from pathlib import Path
p=Path('/home/brittonr/mantle-cairn-finish/bin/jobs/valence-receiver-policy-acceptance-20261020-v1.py')
s=p.read_text()
s=s.replace('RUN.mkdir(exist_ok=False)','assert RUN.is_dir()')
s=s.replace("shutil.copy2(BASE/'targets/valence-source-review-v1/debug/valence', VAL)","assert VAL.is_file()")
s=s.replace("FIX=RUN/'TEST-ONLY-NOT-REAL-REVIEW'; FIX.mkdir()","FIX=RUN/'TEST-ONLY-NOT-REAL-REVIEW'; assert FIX.is_dir()")
s=s.replace('shutil.copy2(ORIGIN/name,FIX/name)','assert (FIX/name).read_bytes()==(ORIGIN/name).read_bytes()')
s=s.replace("shutil.copy2(BASE/'runs/cairn-source-review-operator-smoke-20261020-v1/complete.json',RUN/'producer-complete.json')",'')
s=s.replace("shutil.copy2(BASE/'bin/jobs/finish-cairn-source-review-operator-smoke-v1.py',RUN/'producer-driver.py')",'')
s=s.replace('records=[]',"records=json.loads((RUN/'results.json').read_text())")
s=s.replace("args=list(map(str,args)); start=time.monotonic()", "args=list(map(str,args)); prior=next((r for r in records if r['label']==label),None)\n    if prior:\n        assert prior['command']==args\n        return prior\n    start=time.monotonic()")
s=s.replace("if row['exit']==0: raise RuntimeError('negative fixture admitted '+str(fixture))", "if row['exit']==0:\n        parser=capture('semantic-'+fixture.stem,[CLI,'policy','--policy',RUN/('negative-'+fixture.stem+'.stdout')])\n        assert parser['exit']!=0, 'invalid fixture passed semantic parser' ")
s=s.replace("'exit':row['exit']", "'nickel_exit':row['exit'],'effective_rejection':'nickel' if row['exit'] else 'owner-semantic-parser'")
s=s.replace("'--root','.cairn'", "'--root','.'")
exec(compile(s,str(p), 'exec'))
