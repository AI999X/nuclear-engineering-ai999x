# Nuclear Engineering (nuclear-engineering-ai999x)

[![tests](https://github.com/AI999X/nuclear-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/nuclear-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/nuclear-engineering-ai999x)](https://pypi.org/project/nuclear-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/nuclear-engineering-ai999x)

Nuclear Engineering covers radioactive decay and decay chains, binding energy, reactor physics basics, radiation shielding and attenuation, and dosimetry: 84 calculations in 10 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Nuclear Engineering desktop app: Decay heat after shutdown (Way-Wigner), with its worked example and live scene](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/hero.webp)](https://www.ai999x.com/models/nuclear-engineering-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-linux-x86_64.tar.gz) |
| **Command line** `nuclear-engineering` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/nuclear-engineering-ai999x/#try) |
| **Python** 3.10+ | `pip install nuclear-engineering-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import nuclear_engineering_ai999x as nuclear_engineering

a = nuclear_engineering.ask("Compton edge: E = 662 keV")
print(a.outcome, a.values)
# SOLVED {'T_max': 0.477650409375, 'E_bs': 0.184349590625}

a = nuclear_engineering.ask(
    "Committed dose from a dose coefficient: I = 1 MBq, e = 2.2e-8 "
    "Sv/Bq"
)
print(a.outcome, a.values)
# SOLVED {'E': 0.022, 'E_mSv': 22.0, 'I_Bq': 1000000.0}

a = nuclear_engineering.ask(
    "Control rod worth: rho_tot = 1000 pcm, H = 3.66 m, x = 1.83 m"
)
print(a.outcome, a.values)
# SOLVED {'rho': 0.005, 'frac': 0.5, 'drho_dx': 0.00546448087432}

a = nuclear_engineering.ask(
    "Channel coolant temperature: q0 = 43 kW/m, H = 3.66 m, H_e = 4 m, "
    "m_dot = 0.3 kg/s, c_p = 5500 J/(kg*K), T_in = 565 K, z = 0 m"
)
print(a.outcome, a.values)
# SOLVED {'T': 597.886071405, 'T_out': 630.77214281, 'Q': 108524.035637}

a = nuclear_engineering.ask(
    "Counting time for precision: Rg = 100 cps, Rb = 25 cps, r = 1%"
)
print(a.outcome, a.values)
# SOLVED {'T': 400.0, 'tg': 266.666666667, 'tb': 133.333333333}

a = nuclear_engineering.ask(
    "Collisions to thermalize: A = 12.0, E0 = 2 MeV, Ef = 0.025 eV"
)
print(a.outcome, a.values)
# SOLVED {'n': 115.342927739, 'xi': 0.157768989823}

a = nuclear_engineering.ask(
    "Counts during decay: A0 = 100 kBq, T_half = 6586.2 s, tc = 1 h, "
    "eff = 20%, y = 96.7%"
)
print(a.outcome, a.values)
# SOLVED {'N': 57953848.461, 'A_mean': 83238.3207816, 'F': 1.20136974246}

a = nuclear_engineering.ask(
    "Effective dose: H = [10.0, 5.0, 2.0], w = [0.12, 0.04, 0.01]"
)
print(a.outcome, a.values)
# SOLVED {'E_mSv': 1.42, 'E': 0.00142, 'w_sum': 0.17}

a = nuclear_engineering.ask(
    "Exponential pile: z = [20.0, 40.0, 60.0, 80.0, 100.0], phi = "
    "[10000.0, 6200.0, 3850.0, 2390.0, 1480.0], a = 2 m, b = 2 m"
)
print(a.outcome, a.values)
# SOLVED {'gamma': 2.38717096827, 'L': 1.14423439231, 'L2': 1.30927234454}

a = nuclear_engineering.ask(
    "Irradiation time for saturation: T_half = 15 h, f = 90%"
)
print(a.outcome, a.values)
# SOLVED {'t_irr': 179384.117124, 'n_half': 3.32192809489}

a = nuclear_engineering.ask("Currie detection limits: B = 100.0")
print(a.outcome, a.values)
# SOLVED {'L_C': 23.2617430735, 'L_D': 49.2290296012}

a = nuclear_engineering.ask(
    "Effective half-life: T_p = 8.0207 d, T_b = 80 d"
)
print(a.outcome, a.values)
# SOLVED {'T_eff': 629841.371405, 'lambda_eff': 1.10051071909e-06}

a = nuclear_engineering.ask(
    "Critical slab: D = 0.9 cm, Sigma_a = 0.15 1/cm, nuSigma_f = 0.18 "
    "1/cm, extrapolation = 2.13D"
)
print(a.outcome, a.values)
# SOLVED {'a': 0.133710516286, 'a_ext': 0.172072116286}

a = nuclear_engineering.ask(
    "Non-paralyzable dead time: m = 50000 cps, tau = 5 us"
)
print(a.outcome, a.values)
# SOLVED {'n': 66666.6666667, 'loss': 0.25, 'corr': 1.33333333333}

a = nuclear_engineering.ask(
    "Reflected slab critical: D_c = 0.9 cm, Sigma_ac = 0.15 1/cm, "
    "nuSigma_fc = 0.18 1/cm, D_r = 0.16 cm, Sigma_ar = 0.0197 1/cm, "
    "T_r = 20 cm, extrapolation = 2.13D"
)
print(a.outcome, a.values)
# SOLVED {'a': 0.0360662339368, 'delta': 0.0680029411748}
```

## Screenshots

[![The Nuclear Engineering desktop app: Decay heat after shutdown (Way-Wigner), with its worked example and live scene](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/hero-dark.webp)](https://www.ai999x.com/models/nuclear-engineering-ai999x/)

[![Solid angle and geometric efficiency of a disk detector (source on axis): the live scene in the Nuclear Engineering desktop app](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/see-1.webp)](https://www.ai999x.com/models/nuclear-engineering-ai999x/)

*Solid angle and geometric efficiency of a disk detector (source on axis)*

[![Macroscopic cross section, mean free path and reaction rate of a material: the live scene in the Nuclear Engineering desktop app](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/see-2.webp)](https://www.ai999x.com/models/nuclear-engineering-ai999x/)

*Macroscopic cross section, mean free path and reaction rate of a material*

[![Irradiation time to reach a fraction of saturation activity: the live scene in the Nuclear Engineering desktop app](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/see-3.webp)](https://www.ai999x.com/models/nuclear-engineering-ai999x/)

*Irradiation time to reach a fraction of saturation activity*

Every calculation, with its formula, a worked example and its figures: **[Nuclear Engineering on ai999x.com](https://www.ai999x.com/models/nuclear-engineering-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/nuclear-engineering-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.4`; the [GitHub release](https://github.com/AI999X/nuclear-engineering-ai999x/releases/tag/v1.6.4) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `nuclear-engineering` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/nuclear-engineering`) and its data (`rust/models/nuclear_engineering/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p nuclear-engineering-ai999x
./target/release/nuclear-engineering --help
cargo test -p nuclear-engineering-ai999x
```
