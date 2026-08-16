#!/usr/bin/env python3
"""Validator for Content Engine Part 4 / FINAL (Sections 76-100)."""

import hashlib, json, os, re, sys
try:
    import yaml
except ImportError:
    sys.stderr.write("PyYAML required\n"); sys.exit(2)

REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
TARGET = os.path.join(REPO_ROOT, "context", "09_CONTENT_ENGINE.md")
FROZEN = "/home/user/ce_p4/frozen_p123.md"

PREFIX_BYTES = 1335467
PREFIX_SHA256 = "41696eb799755a7bd8cdac8fe1bc1497314504fe5f72fc608d210cbc3235629f"

SECTION_TITLES = {
    76: "Enterprise Content Operating Model",
    77: "Content Knowledge and Evidence Fabric",
    78: "Global Content Intelligence at Scale",
    79: "Content Identity, Trust and Attestation",
    80: "Agent-Native Content Protocol",
    81: "Multi-Agent Content Federation",
    82: "Cross-Network and Cross-Merchant Content Federation",
    83: "Content Economic Coordination",
    84: "Autonomous Content Operations",
    85: "Human-Agent Responsibility Boundaries",
    86: "Decision, Planning and Reasoning Continuity",
    87: "Content Control Plane",
    88: "Cross-Engine Content Integration",
    89: "Content Analytics and Intelligence Fabric",
    90: "Content KPIs and Economic Measurement",
    91: "Content Optimization at Scale",
    92: "Content Economics and Capital Allocation",
    93: "Content Scenario Simulation and Stress Testing",
    94: "Content Forecasting and Long-Horizon Planning",
    95: "Enterprise Content Governance",
    96: "Content Observability and Auditability",
    97: "Content Testing, Verification and Assurance",
    98: "Content Deployment, Migration and Recovery",
    99: "Content Engine Evolution and Long-Horizon Architecture",
    100: "Content Engine FINAL Completion Contract",
}

results = []
def check(ok, cid, detail):
    results.append((bool(ok), cid, detail))

raw = open(TARGET, "rb").read()
text = raw.decode("utf-8")
frozen = open(FROZEN, "rb").read()
p4 = text[PREFIX_BYTES:]

# 1-3. Basic
check(b"\r" not in raw, "3-lf-only", "LF-only")
check(bool(raw.decode("utf-8", "strict")), "2-utf8", "UTF-8")
check(len(raw) > PREFIX_BYTES, "1-has-content", "has Part 4 content")

# 4-5. Section numbering and naming
headings = {}
for m in re.finditer(r"^## (\d+)\. (.+)$", text, re.M):
    n = int(m.group(1))
    if 76 <= n <= 100:
        headings[n] = m.group(2).strip()
for n in range(76, 101):
    check(n in headings, "4-section-%02d-exists" % n, "## %d heading" % n)
    check(n in headings and headings[n] == SECTION_TITLES[n],
          "5-section-%02d-title" % n, "section %d title" % n)

# 6-7. FINAL boundary
check(not re.search(r"^## 101\. ", p4, re.M), "6-no-section-101", "no Section 101")
check("# CAT Content Engine Bible — Part 5" not in p4, "7-no-part5", "no Part 5")

# 8-10. Diagrams
dids = re.findall(r"^\*\*Diagram ID:\*\* `(CAT-CE-P4-S\d{2,3}-D\d{3})`", p4, re.M)
check(len(dids) >= 75, "8-diagram-count", "%d diagrams (min 75)" % len(dids))
check(len(set(dids)) == len(dids), "10-diagram-unique", "diagram IDs unique")
for sec in range(76, 101):
    start = p4.find("## %d. " % sec)
    end = p4.find("## %d. " % (sec + 1), start) if sec < 100 else len(p4)
    seg = p4[start:end] if start >= 0 else ""
    check(seg.count("**Diagram ID:**") >= 3, "8-diagram-sec-%02d" % sec, "sec %d has 3 diagrams" % sec)
    check(all(f in seg for f in ["**Title:**", "**Purpose:**", "**Audience:**", "**Reading Order:**"]),
          "9-diagram-meta-%02d" % sec, "sec %d diagram metadata" % sec)

# 11-13. JSON
jblocks = re.findall(r"^```json\s*$(.*?)^```\s*$", p4, re.M | re.S)
check(len(jblocks) >= 25, "11-json-count", "%d JSON blocks" % len(jblocks))
jok = 0; rts = []
for b in jblocks:
    try: obj = json.loads(b); jok += 1; rts.append(obj.get("record_type", ""))
    except: pass
check(jok == len(jblocks), "11-json-valid", "JSON %d/%d" % (jok, len(jblocks)))
check(len(set(rts)) == len(rts), "12-json-unique", "record_type unique (%d)" % len(rts))

# 14-16. YAML
yblocks = re.findall(r"^```yaml\s*$(.*?)^```\s*$", p4, re.M | re.S)
check(len(yblocks) >= 25, "14-yaml-count", "%d YAML blocks" % len(yblocks))
yok = 0
for b in yblocks:
    try: yaml.safe_load(b); yok += 1
    except: pass
check(yok == len(yblocks), "14-yaml-valid", "YAML %d/%d" % (yok, len(yblocks)))

# 17-19. Rules
rules = set(re.findall(r"CAT-CE-CONST-\d{3}", p4))
expected_rules = set("CAT-CE-CONST-%03d" % n for n in range(151, 201))
check(expected_rules.issubset(rules), "17-rules-complete", "rules 151-200 present")

# 20-22. Tests
tests = set(re.findall(r"CAT-CE-AT-S\d{2,3}-\d{3}", p4))
expected_tests = set("CAT-CE-AT-S%02d-%03d" % (s, t) for s in range(76, 101) for t in range(1, 4))
check(expected_tests.issubset(tests), "20-tests-complete", "tests S76-S100 present")

# 23-25. Anchors
anchors = set(re.findall(r"CAT-CE-MEM-S\d{2,3}-001", p4))
expected_anchors = set("CAT-CE-MEM-S%02d-001" % s for s in range(76, 101))
check(expected_anchors.issubset(anchors), "23-anchors-complete", "anchors S76-S100 present")

# 26. Cross-reference resolution
for ref in set(re.findall(r"`(context/[^`]+?)`", p4)):
    p = os.path.join(REPO_ROOT, ref)
    if os.path.exists(p):
        check(True, "26-ref-%s" % ref.replace("/", "-"), "ref %s exists" % ref)
    else:
        ctx = p4[max(0, p4.find("`%s`" % ref) - 200): p4.find("`%s`" % ref) + 200]
        check("Planned" in ctx, "26-ref-%s" % ref.replace("/", "-"), "ref %s marked Planned" % ref)

# 27-42. Coverage
coverage = {
    "27-architecture": ["architecture", "control plane", "enterprise"],
    "28-runtime": ["runtime", "pipeline"],
    "29-ai-contract": ["AI", "intelligence", "derived"],
    "30-agent": ["agent", "agent-native"],
    "31-knowledge": ["knowledge", "graph"],
    "32-memory": ["Memory", "memory"],
    "33-decision": ["Decision", "decision"],
    "34-planning": ["Planning", "planning"],
    "35-reasoning": ["Reasoning", "reasoning"],
    "36-treasury": ["Treasury", "monetary"],
    "37-affiliate": ["Affiliate", "affiliate"],
    "38-security": ["security", "least privilege"],
    "39-governance": ["governance", "policy"],
    "40-observability": ["observability", "audit"],
    "41-scenario": ["scenario", "stress"],
    "42-forecast": ["forecast", "forecasting"],
    "43-optimization": ["optimization"],
    "44-economics": ["economics", "cost"],
    "45-multiagent": ["multi-agent", "federation"],
}
for cid, kws in coverage.items():
    for kw in kws:
        check(kw.lower() in p4.lower(), "%s-%s" % (cid, kw.replace(" ", "-")), "keyword %s" % kw)

# Repository paths marked Planned
for m in re.finditer(r"`(core/content/[^`]+)`", p4):
    ctx = p4[max(0, m.start() - 200): m.end() + 100]
    check("Planned" in ctx, "46-repo-%s" % m.group(1).replace("/", "-"), "path %s marked Planned" % m.group(1))

# 47. Filler / omission
for term in ["etc.", "and so on", "to be defined", "TBD", "placeholder", "will be added later", "similar", "and more"]:
    check(term.lower() not in p4.lower(), "47-filler-%s" % term.replace(" ", "-"), "filler absent")
clean = p4.replace("SNN-NNN", "").replace("SNN-DNNN", "").replace("SNN-001", "")
for term in ["NNN", "XXX", "TODO", "FIXME", "DUMMY"]:
    check(term not in clean, "48-omission-%s" % term, "omission absent")

# 49. Receipt
if "#### Measured Content" in p4:
    r = p4[p4.rfind("#### Measured Content"):]
    check("Sections | 25" in r, "49-receipt-sections", "receipt shows 25 sections")

# 50. Digest
_body = re.sub(rb"^`[0-9a-f]{64}`\n", b"", raw, flags=re.MULTILINE)
if not _body.endswith(b"\n"):
    _body += b"\n"
digest_excluded = hashlib.sha256(_body).hexdigest()
matches = list(re.finditer(r"`([0-9a-f]{64})`", p4))
if matches:
    recorded = matches[-1].group(1)
    check(recorded == digest_excluded, "50-digest", "digest reproducible")
else:
    check(False, "50-digest", "no digest line")

# 51-52. Append-only and final boundary
check(raw[:PREFIX_BYTES] == frozen, "51-append-only", "prefix byte-identical")
check(not re.search(r"^## 101\. ", p4, re.M), "52-final-boundary", "no Section 101")

passed = sum(1 for ok, _, _ in results if ok)
failed = sum(1 for ok, _, _ in results if not ok)
for ok, cid, detail in results:
    if not ok:
        print("FAIL %s - %s" % (cid, detail))
print("=" * 72)
print("%d of %d PASSED (%d failed)" % (passed, len(results), failed))
sys.exit(0 if failed == 0 else 1)