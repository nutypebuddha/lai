# Athena Stress Test — Bug Tickets & Audits
Date: 2026-07-06
Tested against: `athena-x86_64` (build_time 1783398095, v0.1.0) + source zip `athena-src-20260707.zip`
Method: black-box CLI stress testing (`reason`, `chain`, `eval`), no source modifications except one sandboxed test formula (not applied to repo).

---

## TICKET-001 [Bug — High] `bayes_theorem` trusts a hand-supplied marginal likelihood with no compositional check

**Component:** `formulas/atomic/sagittarius_history.toml` — `bayes_theorem`

**Description:**
`bayes_theorem` takes `evidence` (the marginal `P(E)`) as a free input rather than deriving it. Nothing in the formula, gate, or CLI validates that the supplied `evidence` is internally consistent with `likelihood`/`prior`. A caller (human or LLM) who plugs in a wrong marginal gets a confidently-returned, wrong posterior with no error, no confidence penalty, no flag.

**Repro:**
```
# Correct marginal (0.9*0.01 + 0.1*0.99 = 0.108):
./athena eval --formula bayes_theorem --args likelihood=0.9 --args prior=0.01 --args evidence=0.108
→ 0.083333   (correct)

# Wrong marginal (naively using "test accuracy" 0.9 instead of true marginal):
./athena eval --formula bayes_theorem --args likelihood=0.9 --args prior=0.01 --args evidence=0.9
→ 0.010000   (silently wrong — equals the prior, i.e. "no update happened", with zero indication anything is off)
```

**Impact:** This is exactly the class of error Athena exists to catch (an LLM confidently producing a plausible-but-wrong number). Right now the gate only verifies *that formula's* arithmetic, not the semantic validity of a value supplied from outside the formula. Classic base-rate-neglect mistakes will pass MathGate cleanly.

**Suggested fix (already prototyped, not yet merged):**
Add a `marginal_likelihood` formula that derives `evidence` from `likelihood`, `prior`, and `false_positive_rate` via the law of total probability, so `evidence` is no longer a free input:
```toml
[[formula]]
id = "marginal_likelihood"
domain = "sagittarius"
level = 9
inputs = ["likelihood", "prior", "false_positive_rate"]
output = "evidence"
expression = "likelihood * prior + false_positive_rate * (1 - prior)"
description = "Marginal likelihood P(E) = P(E|H)P(H) + P(E|not H)P(not H)"
```
Verified working in sandbox — `reason --have likelihood,prior,false_positive_rate --want posterior --execute` auto-finds and executes the 2-step chain, producing the correct 0.083333 with no way to hand-supply a bad marginal.

**Action needed:** Port this formula block into the real repo's `sagittarius_history.toml`. Then audit other atomic formulas for similarly "free" inputs that are actually derivable elsewhere (see TICKET-004/audit below — this was one instance of a general pattern).

---

## TICKET-002 [Bug — Critical] `reason`/`chain` auto-wiring matches formulas by variable *name* only, with zero unit/semantic checking

**Component:** graph search in `reason` / `chain` (formula composition engine)

**Description:**
The auto-chaining feature (a genuinely good piece of architecture — it lets you go from raw inputs to a target output without knowing the intermediate formula) selects intermediate formulas purely by matching an input name to some other formula's output name. There is no type system, unit check, or semantic tag check preventing two completely unrelated quantities that happen to share a variable name (`r`, `velocity`, `z`, `alpha`, `x`, `growth_rate`, `quantum_yield`, etc.) from being chained together.

**Repro (cross-domain case — Pisces → Taurus):**
```
./athena reason --have n,sum_x,sum_y,sum_xy,sum_x2,sum_y2,G,mass --want potential
→ Found path (2 step(s)): pearson_correlation → gravitational_potential
```
`pearson_correlation` outputs `r` = a dimensionless correlation coefficient bounded in [-1, 1]. `gravitational_potential` consumes `r` expecting a radius in meters. The planner presents this as a valid "Found path" with green checkmarks, with no caveat, before any adjacency or semantic check runs.

**Partial mitigation that already exists:** at `--execute` time there is a domain-adjacency gate that happened to catch this specific case:
```
Chain execution failed: composition error: no edge between Pisces and Taurus
```
This is good, but it:
1. Only triggers at execution, not at planning — `reason` without `--execute` will confidently show a bogus path as "found," which is misleading on its own (a human or an LLM agent using `reason` output as a citation would have no reason to doubt it).
2. Only checks domain *adjacency*, not semantic/unit compatibility — it would pass fine if the colliding formulas happened to be in the same or an adjacent domain. See TICKET-003 for a confirmed case where this happens.

**Suggested fix:** Either (a) add a `unit`/`quantity_type` field to formula inputs/outputs in the TOML schema and check it during planning, not just domain adjacency at execution; or (b) at minimum, print a warning in `reason`'s non-executed path output when a proposed edge crosses domains, so a human/LLM doesn't mistake "found" for "valid."

---

## TICKET-003 [Bug — Critical] Same-domain collisions bypass the adjacency gate entirely — confirmed silent wrong execution

**Component:** `formulas/atomic/leo_bio.toml` — `photosynthetic_efficiency` → `vitamin_d_synthesis`

**Description:** Direct consequence of TICKET-002, but this one **actually executes** because both formulas are tagged `domain = "leo"`, so the domain-adjacency gate (same domain trivially "passes") doesn't block it. `photosynthetic_efficiency` computes a plant's photosynthetic quantum yield (O₂ evolved per photon absorbed, theoretical ceiling ~0.125 per the formula's own evidence field). `vitamin_d_synthesis` expects `quantum_yield` to mean the previtamin-D₃ photoconversion yield in human skin — a completely different photochemical system, different organism, different wavelength regime, different magnitude range.

**Repro:**
```
./athena reason --have photons_absorbed,o2_evolved,uvb_irradiance,skin_area,exposure_time \
  --want previtamin_d3 --execute \
  --args photons_absorbed=1000 --args o2_evolved=100 \
  --args uvb_irradiance=5 --args skin_area=0.2 --args exposure_time=15

→ photosynthetic_efficiency(...) = quantum_yield = 0.400000
→ vitamin_d_synthesis(...) = previtamin_d3 = 6.000000
→ Status: ✓ SUCCESS (confidence: 0.90)
```
Two additional red flags stack on top of the core bug: (a) the derived `quantum_yield = 0.4` already exceeds the photosynthesis formula's own stated theoretical maximum of 0.125 with no bounds check, and (b) the result is reported at **confidence 0.90** — high confidence — despite being biological nonsense.

**Impact:** This is a live, reproducible case of Athena's own architecture producing a fabricated-looking but "verified" number, in the same category of error the tool exists to catch when it comes from an LLM.

**Suggested fix:**
1. Short term: add explicit `quantity_type` disambiguation so `photosynthetic_efficiency`'s output is `quantum_yield_photosynthesis` or similarly namespaced, not the bare `quantum_yield` that collides with the skin formula's input.
2. Systemic: same-domain does not imply same-quantity. The adjacency gate needs to become a semantic-compatibility gate (see TICKET-002's suggested fix), because "same zodiac domain" is doing no useful work here as a safety check.

---

## TICKET-004 [Bug — Medium] `eval`/`chain` accept and silently return `inf`/`NaN` as valid results

**Component:** formula evaluator (affects any formula with a `log`, `sqrt`, or division in its expression — confirmed on `radiocarbon_dating`)

**Description:** No domain/range validation is applied to inputs or to the resulting output before it's reported as a completed "Bankai" result.

**Repro:**
```
./athena eval --formula radiocarbon_dating --args remaining_fraction=0 --args half_life=5730
→ Bankai: radiocarbon_dating → inf

./athena eval --formula radiocarbon_dating --args remaining_fraction=-0.5 --args half_life=5730
→ Bankai: radiocarbon_dating → NaN
```
Both are physically impossible inputs (a sample can't have 0% or negative "remaining fraction"), and both should be rejected before evaluation — instead they're evaluated, produce `inf`/`NaN`, and are printed with the same "Bankai: formula → value" framing as a normal successful result, with no warning.

**Suggested fix:** Add a post-evaluation check in the shared eval path: if the result is `inf`, `-inf`, or `NaN`, return an evaluation error instead of a value (or at minimum flag it prominently and zero out confidence). Optionally add per-input domain constraints to the TOML schema (e.g. `remaining_fraction: (0, 1]`) for formulas where the physical domain is known, checked before the expression ever runs.

---

## AUDIT-001 [Audit task] Systematic variable-name collision sweep across all formulas

**What was done:** Wrote a quick parser over every `.toml` in `formulas/` (274 formula blocks across atomic/bridging/nonmath/vortex) that flags any formula input whose name matches another formula's output name where that other formula isn't the intended producer. This surfaces both legitimate multi-step chains (good — e.g. `redshift → hubble_law`, verified correct: 428.27 Mpc for z=0.1, H0=70) and dangerous collisions (bad — TICKET-002/003).

**Findings:** 45 candidate collisions total. Two were manually verified as confirmed bugs (TICKET-002, TICKET-003) and one as a confirmed-good legitimate chain (`redshift`/`hubble_law`). The remaining ~42 are unreviewed and worth triaging by hand — notable-looking suspects from the raw list:

| Formula needing input | Colliding producer(s) | Why suspicious |
|---|---|---|
| `hamiltonian`, `bernoulli_equation`, `reynolds_number` need `velocity` | `michaelis_menten` (enzyme reaction velocity, not kinematic velocity) | different physical quantity, same name |
| `membrane_potential` needs `z` | `z_score` (statistics) | ionic valence vs. statistical z-score |
| `lotka_volterra`, `exponential_smoothing` need `alpha` | `cronbach_alpha` (survey reliability coefficient) | unrelated domains reusing a generic Greek-letter name |
| `exponential_growth`, `logistic_growth` need `growth_rate` | `computational_complexity` | Big-O growth vs. population growth rate |
| `gravitational_field`, `electric_field`, `electric_potential`, `magnetic_vector_potential` need `r` | `pearson_correlation` | same class of bug as TICKET-002, unverified whether domain-adjacent |
| `ideal_gas_law` needs `pressure` | `hydrostatic_pressure` | plausibly legitimate (both physical pressure) — needs a unit check, not necessarily a bug |
| `first_law_thermo` needs `work` | `work_energy`, `force_to_work` | plausibly legitimate — same quantity, worth confirming units match |

**Recommended follow-up:** Someone (or an agent session) should go through the remaining ~42 by hand the same way TICKET-002/003 were confirmed: pull both formula definitions, check whether the physical quantity is genuinely the same thing, and either (a) leave it as an intentional chain, (b) rename one side to disambiguate, or (c) add the `quantity_type` field from TICKET-002's suggested fix so this whole class stops being a manual audit and becomes a compile-time-style check.

**Script used** (adapt as needed, not committed to repo):
```python
import re, glob
from collections import defaultdict
formulas = {}
for f in glob.glob('formulas/**/*.toml', recursive=True):
    content = open(f, encoding='utf-8').read()
    for b in content.split('[[formula]]')[1:]:
        idm = re.search(r'id\s*=\s*"([^"]+)"', b)
        inm = re.search(r'inputs\s*=\s*\[(.*?)\]', b, re.S)
        outm = re.search(r'output\s*=\s*"([^"]+)"', b)
        if not idm: continue
        formulas[idm.group(1)] = {
            'inputs': re.findall(r'"([^"]+)"', inm.group(1)) if inm else [],
            'output': outm.group(1) if outm else None,
        }
producers = defaultdict(list)
for fid, v in formulas.items():
    if v['output']: producers[v['output']].append(fid)
for fid, v in formulas.items():
    for inp in v['inputs']:
        others = [p for p in producers.get(inp, []) if p != fid]
        if others:
            print(f'{fid} needs "{inp}" <- producible by {others}')
```

---

## TICKET-005 [Minor / UX] CLI arg parsing inconsistencies

Lower priority, noted in passing during testing:
- `--args` requires bare `key=value` pairs; passing JSON (`'{"x": 1}'`) fails with a raw clap error rather than a helpful message.
- `entity-get` requires `--id <ID>` rather than accepting a positional argument, inconsistent with how `search`/`eval --formula` read.

Not urgent, but worth a pass for consistency's sake.

---

## Priority summary
1. **TICKET-003** (confirmed live wrong-execution, high confidence shown) — fix first
2. **TICKET-002** (the systemic cause of #3, and of an unknown number of the other 44 audit candidates)
3. **TICKET-001** (known compositional gap, fix already prototyped and verified — just needs porting)
4. **TICKET-004** (silent inf/NaN)
5. **AUDIT-001** (triage the remaining ~42 collision candidates)
6. **TICKET-005** (CLI polish)
