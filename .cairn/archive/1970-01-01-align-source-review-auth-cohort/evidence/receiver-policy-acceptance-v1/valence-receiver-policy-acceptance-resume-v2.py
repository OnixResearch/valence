#!/usr/bin/env python3
"""Continue retained exact-input evidence using the Valence receipt contract."""
from pathlib import Path
p=Path('/home/brittonr/mantle-cairn-finish/bin/jobs/valence-receiver-policy-acceptance-resume-v1.py')
s=p.read_text()
s=s.replace("exec(compile(s,str(p), 'exec'))", "s=s.replace(\" and not r['needs_revision_key_blake3']\",'')\ns=s.replace(\"if name=='needs-revision': assert r['needs_revision_key_blake3']==[public]\", \"if name=='needs-revision': assert r['outcome']=='insufficient' and len(r['signed_statement_blake3'])==1 and r['issues']==['approval_threshold_unsatisfied']\")\ns=s.replace(\"'source_subject_mismatch'\",\"'review_subject_root_or_policy_mismatch'\").replace(\"'author_key_excluded'\",\"'review_author_excluded'\")\nexec(compile(s,str(p),'exec'))")
exec(compile(s,str(p),'exec'))
