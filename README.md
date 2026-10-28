# nuclear-engineering-ai999x — offline, verified nuclear engineering solver with step-by-step answers

[![tests](https://github.com/AI999X/nuclear-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/nuclear-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/nuclear-engineering-ai999x)](https://pypi.org/project/nuclear-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/nuclear-engineering-ai999x)

**[Install](https://pypi.org/project/nuclear-engineering-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/nuclear-engineering-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/nuclear-engineering-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.3

Field: **Engineering** · Tasks: Question Answering, Token Classification, Text Classification, Sentence Similarity, Feature Extraction

nuclear-engineering-ai999x is an offline, deterministic Python solver that answers nuclear engineering questions with verified results from 83 certified capabilities, gives 554 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**nuclear-engineering-ai999x** is a compact, deterministic nuclear engineering engine for Python. It reads a question in plain English (or a typed call), picks a certified nuclear engineering capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified nuclear engineering capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 554 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** radioactive decay chains and activity, radiation shielding and dosimetry, detectors and counting, neutron physics, reactor criticality and kinetics, fission-product poisons, fuel depletion and enrichment, and fuel-rod heat transfer.

## Model overview

| Property | Value |
|---|---|
| Package | `nuclear-engineering-ai999x` 1.0.3 (import `nuclear_engineering_ai999x`) |
| Type | Deterministic neuro-symbolic nuclear engineering engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 83 certified procedures |
| Knowledge pack | version 1.0.1 |
| Golden answers | 554 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install nuclear-engineering-ai999x
```

```python
import nuclear_engineering_ai999x as nuclear_engineering

nuclear_engineering.ask("Specific activity: nuclide = Co-60, m = 1 g").show()
# -> SOLVED — A = 4.18689e+13, A_Ci = 1131.59, a = 4.18689e+16, a_Ci_per_g = 1131.59  (nuc_specific_activity)
nuclear_engineering.ask("Point source dose rate: Gamma = 0.351 uSv*m^2/(MBq*h), A = 10 Ci, r = 2 m, mu = 0.0656 1/cm, x = 10 cm, buildup = linear").show()
# -> SOLVED — H_dot = 7.75014e-06, H_uSv_h = 27900.5, B = 1.656, T = 0.859336  (nuc_point_source_dose_rate)
nuclear_engineering.ask("Critical sphere: D = 0.9 cm, Sigma_a = 0.15 1/cm, nuSigma_f = 0.18 1/cm, extrapolation = 2.13D").show()
# -> SOLVED — R = 0.152891, R_ext = 0.172072, V = 0.0149705  (nuc_critical_sphere)
nuclear_engineering.ask("Xenon after shutdown: Sigma_f = 0.1 1/cm, phi = 3e13 1/(cm^2*s), gamma_I = 0.0639, gamma_X = 0.00237, sigma_X = 2.65e6 b, t = 10 h").show()
# -> SOLVED — X = 3.72338e+21, ratio = 1.88343, t_peak = 30167, peak_ratio = 1.90155  (nuc_xenon_after_shutdown)
nuclear_engineering.solve("nuc_enrichment_swu", x_p=0.045, x_f=0.00711, x_w=0.0025, P="1 kg").show()
# -> SOLVED — F = 9.21909, W = 8.21909, SWU = 6.87111, F_per_P = 9.21909  (nuc_enrichment_swu)
nuclear_engineering.ask("Who created you?").message
# -> nuclear-engineering-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

Command line: `nuclear-engineering-ai999x "<question>"`, `nuclear-engineering-ai999x --capabilities`. The outputs above were produced by running this exact code on nuclear-engineering-ai999x 1.0.3.

## Find the right capability, and draw it

`nuclear_engineering.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "nuclear-engineering-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import nuclear_engineering_ai999x as nuclear_engineering

nuclear_engineering.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (math-foundations-ai999x, 0.63); Roots of a cubic equation (math-foundations-ai999x, 0.59)
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Worked examples

Each question was put to `nuclear-engineering-ai999x`; the answer is the one it returned, with the steps it took.

**Activity and specific activity of a mass of a radionuclide** · _Decay and activity_

> Specific activity: nuclide = Co-60, m = 1 g

→ **A = 4.18689443e+13 1/s, A_Ci = 1131.593089, a = 4.18689443e+16 1/(kg*s), a_Ci_per_g = 1131.593089, N = 1.004798494e+22**

<details><summary>Steps</summary>

- Activity and specific activity of a mass of a radionuclide: nuclide = Co-60, m = 1 g → A = 4.18689443e+13 1/s, A_Ci = 1131.593089 1/s, a = 4.18689443e+16 1/(kg*s), a_Ci_per_g = 1131.593089 1/s, N = 1.004798494e+22 1/s

</details>

**Dose rate from a shielded point gamma source with buildup** · _Shielding and dose_

> Point source dose rate: Gamma = 0.351 uSv*m^2/(MBq*h), A = 10 Ci, r = 2 m, mu = 0.0656 1/cm, x = 10 cm, buildup = linear

→ **H_dot = 7.750139161e-06 J/(kg*s), H_uSv_h = 27900.50098, B = 1.656, T = 0.8593362895**

<details><summary>Steps</summary>

- Dose rate from a shielded point gamma source with buildup: Gamma = 3.51e-13 (J/kg)*m^2/((1/s)*h), A = 3.7e+11 (1/s), r = 2 m, mu = 0.0656 1/cm, x = 10 cm, buildup = linear → H_dot = 7.750139161e-06 J/(kg*s), H_uSv_h = 27900.50098 J/(kg*s), B = 1.656 J/(kg*s), T = 0.8593362895 J/(kg*s)

</details>

**Critical radius of a bare spherical reactor (one group)** · _Reactor physics_

> Critical sphere: D = 0.9 cm, Sigma_a = 0.15 1/cm, nuSigma_f = 0.18 1/cm, extrapolation = 2.13D

→ **R = 0.1528913163 m, R_ext = 0.1720721163 m, V = 0.01497052629 m^3**

<details><summary>Steps</summary>

- Critical radius of a bare spherical reactor (one group): D = 0.9 cm, Sigma_a = 0.15 1/cm, nuSigma_f = 0.18 1/cm, extrapolation = 2.13D → R = 0.1528913163 m, R_ext = 0.1720721163 m, V = 0.01497052629 m^3

</details>

**Xenon-135 after shutdown: concentration and peak** · _Reactor physics_

> Xenon after shutdown: Sigma_f = 0.1 1/cm, phi = 3e13 1/(cm^2*s), gamma_I = 0.0639, gamma_X = 0.00237, sigma_X = 2.65e6 b, t = 10 h

→ **X = 3.723382215e+21 1/m^3, ratio = 1.883429887, t_peak = 30167.01507 s, peak_ratio = 1.901549197, I = 2.277591284e+21 1/m^3**

<details><summary>Steps</summary>

- Xenon-135 after shutdown: concentration and peak: Sigma_f = 0.1 1/cm, phi = 3e+13 1/(cm^2*s), gamma_I = 0.0639, gamma_X = 0.00237, sigma_X = 2.65e-22 m^2, t = 10 h → X = 3.723382215e+21 1/m^3, ratio = 1.883429887 1/m^3, t_peak = 30167.01507 s, peak_ratio = 1.901549197 1/m^3, I = 2.277591284e+21 1/m^3

</details>

**Uranium enrichment: feed, tails and separative work (SWU)** · _Fuel cycle_

`m.solve("nuc_enrichment_swu", x_p=0.045, x_f=0.00711, x_w=0.0025, P="1 kg")`

→ **F = 9.219088937 kg, W = 8.219088937 kg, SWU = 6.871112989 kg, F_per_P = 9.219088937, SWU_per_P = 6.871112989**

<details><summary>Steps</summary>

- Uranium enrichment: feed, tails and separative work (SWU): x_p = 0.045, x_f = 0.00711, x_w = 0.0025, P = 1 kg → F = 9.219088937 kg, W = 8.219088937 kg, SWU = 6.871112989 kg, F_per_P = 9.219088937 kg, SWU_per_P = 6.871112989 kg

</details>

**Compton edge and backscatter peak of a gamma line** · _Detectors and counting_

> Compton edge: E = 662 keV

→ **T_max = 0.4776504094 MeV, E_bs = 0.1843495906 MeV**

<details><summary>Steps</summary>

- Compton edge and backscatter peak of a gamma line: E = 662 keV → T_max = 0.4776504094 MeV, E_bs = 0.1843495906 MeV

</details>

## Figures

Every point was computed by `nuclear-engineering-ai999x`.

**A three-member decay chain**

![A three-member decay chain — computed by nuclear-engineering-ai999x](https://huggingface.co/AI999X/nuclear-engineering-ai999x/resolve/main/figures/nuc-three-member-chain-diagram.svg)

_A three-member decay chain (half-lives 100 s → 10 s → 1 s) with the activities the package computes at t = 50 s from a pure parent of 1000 Bq: parent 707.107 Bq, daughter 750.952 Bq, granddaughter 755.03 Bq, total 2213.09 Bq._

**Decay-chain activities at five times**

![Decay-chain activities at five times — computed by nuclear-engineering-ai999x](https://huggingface.co/AI999X/nuclear-engineering-ai999x/resolve/main/figures/nuc-three-member-chain-bars.svg)

_Activities of the three members (parent / daughter / granddaughter) at five times, each from the package — the daughters overtake the parent and then decay with it in transient equilibrium: t = 10 s: 933.033 / 481.148 / 430.001 Bq; t = 50 s: 707.107 / 750.952 / 755.03 Bq; t = 100 s: 500 / 554.47 / 559.962 Bq; t = 200 s: 250 / 277.777 / 280.582 Bq; t = 400 s: 62.5 / 69.4444 / 70.1459 Bq._

**The xenon transient after shutdown**

![The xenon transient after shutdown — computed by nuclear-engineering-ai999x](https://huggingface.co/AI999X/nuclear-engineering-ai999x/resolve/main/figures/nuc-xenon-after-shutdown-curve.svg)

_Xenon-135 after a shutdown from a flux of 3.0000e+17 n/(m² s) (Σ_f = 10 1/m), relative to its equilibrium, at 144 times computed by the package: iodine decay builds xenon up before it decays away; marked: peak 1.90155 times equilibrium at t = 8.37973 h._

**Generator ingrowth: parent and daughter activity**

![Generator ingrowth: parent and daughter activity — computed by nuclear-engineering-ai999x](https://huggingface.co/AI999X/nuclear-engineering-ai999x/resolve/main/figures/nuc-generator-daughter-activity-curve.svg)

_A Mo-99/Tc-99m generator: parent half-life 65.94 h, daughter 6.015 h, branching 0.8773, parent 37 GBq at t = 0; activities at 120 hourly times from the package; marked: the daughter's maximum 25.5251 GBq at t = 22.8646 h._

**Attenuation measurements and their exponential fit**

![Attenuation measurements and their exponential fit — computed by nuclear-engineering-ai999x](https://huggingface.co/AI999X/nuclear-engineering-ai999x/resolve/main/figures/nuc-attenuation-coefficient-fit-fit.svg)

_Narrow-beam attenuation on a logarithmic intensity axis, where the exponential law is a straight line: the five measured intensities and the fit by the package, μ = 65.3966 1/m with I₀ = 1000.02, drawn as I₀ times the transmission it computes at 51 thicknesses; half-value layer 1.05991 cm, tenth-value layer 3.52096 cm._

**Reactor power after a reactivity step: a table**

![Reactor power after a reactivity step: a table — computed by nuclear-engineering-ai999x](https://huggingface.co/AI999X/nuclear-engineering-ai999x/resolve/main/figures/nuc-power-after-step-six-group-heatmap.svg)

_Relative reactor power P/P₀ after a step of reactivity (six delayed-neutron groups, β = 0.006502, Λ = 5.0000e-05 s): rows are the reactivities ρ = 0.0005, 0.001, 0.002, 0.003 (top to bottom), columns the times t = 0.1, 1, 5, 10, 30 s, every cell computed by the package (shaded by size); ρ = 0.0005: stable period 136.947 s; ρ = 0.001: stable period 54.964 s; ρ = 0.002: stable period 17.355 s; ρ = 0.003: stable period 7.1042 s; at ρ = 0.003 the power is 172.212 times its start after 30 s._

## Capabilities

83 certified procedures. A few of them:

- **Procedures:** Activation with target depletion and product burn-up; Activity and specific activity of a mass of a radionuclide; Activity of a radionuclide and its progeny after a decay time; Approach to criticality: inverse multiplication (1/M) extrapolation; Average logarithmic energy decrement and mean scattering cosine; Average number of collisions to slow a neutron down; Committed effective dose from a dose coefficient; Committed equivalent dose to an organ from an intake (retention compartments); Compton edge and backscatter peak of a gamma line; Coolant temperature along a channel with a chopped-cosine power shape; Counting time for a target precision of the net count rate; Counts recorded from a decaying source (decay during counting); Critical radius of a bare cylindrical reactor of given height (one group); Critical radius of a bare spherical reactor (one group); Critical thickness of a bare slab reactor (one group); Critical thickness of a reflected slab reactor and reflector savings (one group); Critical third side of a bare rectangular reactor (one group); Currie critical level, detection limit and minimum detectable activity; Dead-time correction, non-paralyzable detector; Dead-time correction, paralyzable detector; Decay heat after shutdown (Way-Wigner); Decay heat energy released between two times after shutdown; Diffusion length from an exponential pile measurement; Dose accumulated near a decaying source …

`nuclear_engineering_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified nuclear engineering capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (radioactive decay chains and activity, radiation shielding and dosimetry, detectors and counting, neutron physics, reactor criticality and kinetics, fission-product poisons, fuel depletion and enrichment, and fuel-rod heat transfer); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{nuclear_engineering_ai999x_1_0_3,
  title  = {nuclear-engineering-ai999x: an offline, verified nuclear engineering solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.3},
  url    = {https://huggingface.co/AI999X/nuclear-engineering-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/nuclear-engineering-ai999x/discussions or visit https://ai999x.com.

## Support the project

nuclear-engineering-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/nuclear-engineering-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/nuclear-engineering-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/nuclear-engineering-ai999x/tree/main/onnx).
