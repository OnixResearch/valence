#!/usr/bin/env python3
"""Exercise the actual source-review CLI with an external test-only OpenSSL signer."""
import copy
import json
import os
from pathlib import Path
import shutil
import subprocess
import time

BASE = Path('/home/brittonr/mantle-cairn-finish')
SOURCE = BASE / 'xrepo/cairn-source-review-main-20261020'
RUN = BASE / 'runs/cairn-source-review-operator-smoke-20261020-v1'
RUN.mkdir(mode=0o700, exist_ok=False)
FIXTURE = RUN / 'TEST-ONLY-NOT-REAL-REVIEW'
FIXTURE.mkdir(mode=0o700)
BINARY = RUN / 'cairn'
shutil.copy2(BASE / 'targets/cairn-source-review-v1/debug/cairn', BINARY)
records = []

def digest(data):
    return subprocess.check_output(['b3sum', '--no-names'], input=data).decode().strip()

def save(name, value):
    path = FIXTURE / name
    path.write_text(json.dumps(value, indent=2) + '\n')
    return path

def capture(label, argv, expect=0, required_error=None):
    command = [str(value) for value in argv]
    started = time.monotonic()
    with (RUN / (label + '.stdout')).open('xb') as out, (RUN / (label + '.stderr')).open('xb') as err:
        result = subprocess.run(command, cwd=SOURCE, stdout=out, stderr=err, timeout=60)
    row = {'label': label, 'command': command, 'exit': result.returncode, 'expected': expect, 'elapsed_s': time.monotonic() - started}
    records.append(row)
    (RUN / 'results.json').write_text(json.dumps(records, indent=2) + '\n')
    print(json.dumps(row), flush=True)
    if result.returncode != expect:
        raise SystemExit('unexpected CLI status: ' + label)
    if required_error and required_error not in (RUN / (label + '.stderr')).read_text():
        raise SystemExit('expected distinct rejection missing: ' + label)
    return (RUN / (label + '.stdout')).read_bytes()

capture('openssl-version', ['openssl', 'version'])
# Publicly reproducible fixture key: never a real reviewer or trust root.
key = FIXTURE / 'PUBLIC-TEST-ONLY-PRIVATE-KEY.der'
key.write_bytes(bytes.fromhex('302e020100300506032b657004220420') + bytes([53]) * 32)
key.chmod(0o600)
public_der = capture('test-only-public-key', ['openssl', 'pkey', '-inform', 'DER', '-in', key, '-pubout', '-outform', 'DER'])
if len(public_der) != 44 or public_der[:12].hex() != '302a300506032b6570032100':
    raise SystemExit('unexpected Ed25519 public-key encoding')
public = public_der[12:]
public_hash = digest(public)
non_claims = sorted('source-review-does-not-prove-' + suffix for suffix in ['source-correctness', 'review-quality', 'build-correctness', 'reproducibility', 'witness-independence', 'deployment-safety', 'release-eligibility'])
hash_text = lambda text: digest(('TEST ONLY ' + text).encode())
subject = {'schema': 'cairn.source-review-subject.v1', 'source_archive_blake3': hash_text('fixture archive, not real source'), 'source_revision': 'test-only-operator-smoke', 'source_tree_blake3': None, 'snapshot_profile': 'mantle-release-source', 'snapshot_profile_version': 1, 'projection': 'source'}
root = {'claim_identity': hash_text('claim'), 'subject_root': hash_text('claim root'), 'revision': 5}
context = {'profile': 'test-only-currentness.v1', 'algorithm': 'blake3', 'digest_hex': hash_text('currentness')}
policy_hash = hash_text('review policy')
policy = {'schema': 'cairn.source-review-verification-policy.v1', 'subject': subject, 'claim_root': root, 'review_policy_blake3': policy_hash, 'authentication_policy': {'schema': 'artifact-auth.policy.v1', 'profile_id': 'cairn.source-review.v1', 'threshold': 1, 'trusted_keys': [{'producer_id': 'test-only-reviewer', 'key_id': 'test-only-key', 'key_identity': {'profile': 'ed25519-public-key.v1', 'algorithm': 'blake3', 'digest_hex': public_hash}, 'allowed_purposes': ['source-review-approval'], 'generation': 1, 'currentness': 'current', 'currentness_ref': context}]}, 'verifier_context': context, 'author_key_blake3': [], 'exclude_authors': True, 'required_approvals': 1, 'non_claims': non_claims}
save('policy.json', policy)


def roundtrip(disposition, expected_verify):
    claim_name = disposition + '-claim.json'
    claim = {'schema': 'cairn.source-review-claim.v1', 'subject': subject, 'claim_root': root, 'review_policy_blake3': policy_hash, 'disposition': disposition, 'non_claims': non_claims}
    save(claim_name, claim)
    args = [BINARY, 'source-review', 'signing-request', '--root', FIXTURE, '--claim', claim_name, '--policy', 'policy.json', '--producer', 'test-only-reviewer', '--key-id', 'test-only-key', '--generation', '1', '--public-key-hex', public.hex()]
    request_bytes = capture(disposition + '-request', args)
    request = json.loads(request_bytes)
    (FIXTURE / (disposition + '-request.json')).write_bytes(request_bytes)
    raw = capture(disposition + '-unsigned', [*args, '--raw'])
    if raw != bytes.fromhex(request['canonical_statement_hex']) or digest(raw) != request['statement_blake3']:
        raise SystemExit('raw operator bytes differ from the independently hashed request')
    raw_path = FIXTURE / (disposition + '-unsigned.bin')
    raw_path.write_bytes(raw)
    signature = FIXTURE / (disposition + '-signature.bin')
    capture(disposition + '-external-sign', ['openssl', 'pkeyutl', '-sign', '-inkey', key, '-keyform', 'DER', '-rawin', '-in', raw_path, '-out', signature])
    attachment = capture(disposition + '-assemble', [BINARY, 'source-review', 'assemble', '--root', FIXTURE, '--request', disposition + '-request.json', '--signature-hex', signature.read_bytes().hex(), '--policy', 'policy.json'])
    (FIXTURE / (disposition + '-attachment.json')).write_bytes(attachment)
    report = json.loads(capture(disposition + '-verify', [BINARY, 'source-review', 'verify', '--root', FIXTURE, '--attachment', disposition + '-attachment.json', '--policy', 'policy.json'], expect=expected_verify))
    if report['attachment_artifact_blake3'] != digest(attachment) or report['report']['non_claims'] != non_claims:
        raise SystemExit('attachment identity or non-claim boundary changed')
    return attachment, report

approved, approved_report = roundtrip('approved', 0)
if approved_report['report']['outcome'] != 'accepted_manual_review' or approved_report['report']['approved_key_blake3'] != [public_hash] or approved_report['report']['needs_revision_key_blake3']:
    raise SystemExit('exact test-only approval attribution mismatch')
rejected, rejected_report = roundtrip('needs_revision', 1)
if rejected_report['report']['outcome'] != 'insufficient' or rejected_report['report']['approved_key_blake3'] or rejected_report['report']['needs_revision_key_blake3'] != [public_hash]:
    raise SystemExit('authenticated rejection counted as approval')

tampered = json.loads(approved)
tampered['reviews'][0]['claim']['subject']['source_archive_blake3'] = 'f' * 64
save('tampered-source.json', tampered)
capture('reject-substituted-source', [BINARY, 'source-review', 'verify', '--root', FIXTURE, '--attachment', 'tampered-source.json', '--policy', 'policy.json'], expect=1, required_error='source_subject_mismatch')
revoked = copy.deepcopy(policy)
revoked['authentication_policy']['trusted_keys'][0]['currentness'] = 'revoked'
save('revoked-policy.json', revoked)
capture('reject-revoked-key', [BINARY, 'source-review', 'verify', '--root', FIXTURE, '--attachment', 'approved-attachment.json', '--policy', 'revoked-policy.json'], expect=1, required_error='key_not_current')
author = copy.deepcopy(policy)
author['author_key_blake3'] = [public_hash]
save('author-policy.json', author)
capture('reject-excluded-author', [BINARY, 'source-review', 'verify', '--root', FIXTURE, '--attachment', 'approved-attachment.json', '--policy', 'author-policy.json'], expect=1, required_error='author_key_excluded')
(RUN / 'complete.json').write_text(json.dumps({'records': records, 'binary': str(BINARY), 'binary_blake3': digest(BINARY.read_bytes()), 'test_only_public_key_blake3': public_hash, 'external_signer': 'OpenSSL Ed25519 over exact CLI raw bytes', 'authenticated_rejection_is_not_approval': True, 'source_substitution_and_revoked_or_excluded_key_rejected': True, 'genuine_manual_review_approval': False, 'build_witness_authority': False, 'release_authority': False}, indent=2) + '\n')
