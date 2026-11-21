# Nuclear Engineering (nuclear-engineering-ai999x)

[![tests](https://github.com/AI999X/nuclear-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/nuclear-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/nuclear-engineering-ai999x)](https://pypi.org/project/nuclear-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/nuclear-engineering-ai999x)

Nuclear Engineering covers radioactive decay and decay chains, binding energy, reactor physics basics, radiation shielding and attenuation, and dosimetry: 84 calculations in 10 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Nuclear Engineering desktop app: Activity and specific activity of a mass of a radionuclide, with its worked example and live scene](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/hero.webp)](https://www.ai999x.com/models/nuclear-engineering-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-nuclear-engineering-1.6.5-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-nuclear-engineering-1.6.5-linux-x86_64.tar.gz) |
| **Command line** `nuclear-engineering` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/nuclear-engineering-ai999x/#capabilities) |
| **Python** 3.10+ | `pip install nuclear-engineering-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## See it

From the Nuclear Engineering desktop app: every scene and chart is drawn from the computed answer.

[![Nuclear Engineering desktop app: a slider moves an input and the answer, the scene and the charts follow](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/clip-loop.webp)](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/clip.mp4)

*[Watch the full clip (MP4)](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/clip.mp4)*

| | |
|---|---|
| [![Activity and specific activity of a mass of a radionuclide: the live scene in the Nuclear Engineering desktop app](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/scene-1.webp)](https://www.ai999x.com/models/nuclear-engineering-ai999x/#cap-nuc_specific_activity)<br>Activity and specific activity of a mass of a radionuclide | [![Dose rate from a shielded point gamma source with buildup: the live scene in the Nuclear Engineering desktop app](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/scene-2.webp)](https://www.ai999x.com/models/nuclear-engineering-ai999x/#cap-nuc_point_source_dose_rate)<br>Dose rate from a shielded point gamma source with buildup |
| [![Critical radius of a bare spherical reactor (one group): the live scene in the Nuclear Engineering desktop app](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/scene-3.webp)](https://www.ai999x.com/models/nuclear-engineering-ai999x/#cap-nuc_critical_sphere)<br>Critical radius of a bare spherical reactor (one group) | [![Uranium enrichment: feed, tails and separative work (SWU): the live scene in the Nuclear Engineering desktop app](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/scene-4.webp)](https://www.ai999x.com/models/nuclear-engineering-ai999x/#cap-nuc_enrichment_swu)<br>Uranium enrichment: feed, tails and separative work (SWU) |
| [![Compton edge and backscatter peak of a gamma line: the live scene in the Nuclear Engineering desktop app](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/scene-5.webp)](https://www.ai999x.com/models/nuclear-engineering-ai999x/#cap-nuc_compton_edge)<br>Compton edge and backscatter peak of a gamma line | [![Committed effective dose from a dose coefficient: the live scene in the Nuclear Engineering desktop app](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/scene-6.webp)](https://www.ai999x.com/models/nuclear-engineering-ai999x/#cap-nuc_committed_dose_coefficient)<br>Committed effective dose from a dose coefficient |
| [![Average number of collisions to slow a neutron down: the live scene in the Nuclear Engineering desktop app](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/scene-7.webp)](https://www.ai999x.com/models/nuclear-engineering-ai999x/#cap-nuc_collisions_to_thermalize)<br>Average number of collisions to slow a neutron down | [![Integral and differential control-rod worth (sine-squared weighting): the live scene in the Nuclear Engineering desktop app](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/scene-8.webp)](https://www.ai999x.com/models/nuclear-engineering-ai999x/#cap-nuc_control_rod_worth)<br>Integral and differential control-rod worth (sine-squared weighting) |

Charts:

|   |   |   |   |
|---|---|---|---|
| [![Activity and specific activity of a mass of a radionuclide: a chart in the Nuclear Engineering desktop app](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/chart-1.webp)](https://www.ai999x.com/models/nuclear-engineering-ai999x/#cap-nuc_specific_activity)<br>Activity and specific activity of a mass of a radionuclide | [![Dose rate from a shielded point gamma source with buildup: a chart in the Nuclear Engineering desktop app](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/chart-2.webp)](https://www.ai999x.com/models/nuclear-engineering-ai999x/#cap-nuc_point_source_dose_rate)<br>Dose rate from a shielded point gamma source with buildup | [![Critical radius of a bare spherical reactor (one group): a chart in the Nuclear Engineering desktop app](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/chart-3.webp)](https://www.ai999x.com/models/nuclear-engineering-ai999x/#cap-nuc_critical_sphere)<br>Critical radius of a bare spherical reactor (one group) | [![Uranium enrichment: feed, tails and separative work (SWU): a chart in the Nuclear Engineering desktop app](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/chart-4.webp)](https://www.ai999x.com/models/nuclear-engineering-ai999x/#cap-nuc_enrichment_swu)<br>Uranium enrichment: feed, tails and separative work (SWU) |

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

## Command line: 15 examples

Each command below was run with `nuclear-engineering` of this release; the lines under it are the answer it printed (the full answer goes on with the method, every input in SI units and the conditions it assumes).

```text
$ nuclear-engineering solve nuc_activation_with_burnup N_T0=1e+22 "sigma_T=3.718e-27 m^2" "sigma_P=2e-28 m^2" "phi=1e+14 1/(cm^2*s)" "T_half=166346024.4 s" "t=31557600 s"
A_P = 4.303591319e+12 1/s
f_T = 0.8892906498
t_max = 247907988.5 s
A_max = 1.41140119e+13 1/s

$ nuclear-engineering solve nuc_compton_edge "E=662 keV"
T_max = 0.4776504094 MeV
E_bs = 0.1843495906 MeV

$ nuclear-engineering solve nuc_committed_dose_coefficient "I=1000000 (1/s)" "e=2.2e-08 (J/kg)/(1/s)"
E = 0.022 J/kg
E_mSv = 22
I_Bq = 1000000 1/s

$ nuclear-engineering solve nuc_enrichment_swu x_p=0.045 x_f=0.00711 x_w=0.0025 "P=1 kg"
F = 9.219088937 kg
W = 8.219088937 kg
SWU = 6.871112989 kg
F_per_P = 9.219088937
SWU_per_P = 6.871112989

$ nuclear-engineering solve nuc_logarithmic_energy_decrement A=12
alpha = 0.7159763314
xi = 0.1577689898
mu_bar = 0.05555555556

$ nuclear-engineering solve nuc_control_rod_worth rho_tot=0.01 "H=3.66 m" "x=1.83 m"
rho = 0.005
frac = 0.5
drho_dx = 0.005464480874 1/m

$ nuclear-engineering solve nuc_inverse_multiplication "x=[0, 20, 40, 60, 80]" "C=[100, 125, 167, 250, 500]"
x_crit = 99.9760479
slope = -0.01
x_crit_last2 = 100
inv_M_last = 0.2

$ nuclear-engineering solve nuc_half_value_layer "mu=0.656 1/cm"
HVL = 0.010566268 m
TVL = 0.03510038252 m
mfp = 0.01524390244 m
HVL_cm = 1.0566268
TVL_cm = 3.510038252

$ nuclear-engineering solve nuc_accumulated_dose_decaying_source "H0=0.001 (J/kg)/h" "T_half=6586.2 s" "t=2 h"
D = 0.001402260154 J/kg
D_mSv = 1.402260154
D_inf = 0.002639410577 J/kg
H_mean = 1.947583547e-07 J/(kg*s)

$ nuclear-engineering solve nuc_channel_coolant_temperature "q0=43 kW/m" "H=3.66 m" "H_e=4 m" "m_dot=0.3 kg/s" "c_p=5500 J/(kg*K)" "T_in=565 K" "z=0 m"
T = 597.8860714 K
T_out = 630.7721428 K
Q = 108524.0356 W

$ nuclear-engineering solve nuc_specific_activity nuclide=Co-60 "m=1 g"
A = 4.18689443e+13 1/s
A_Ci = 1131.593089
a = 4.18689443e+16 1/(kg*s)
a_Ci_per_g = 1131.593089
N = 1.004798494e+22

$ nuclear-engineering solve nuc_count_time_for_precision "Rg=100 (1/s)" "Rb=25 (1/s)" r=0.01
T = 400 s
tg = 266.6666667 s
tb = 133.3333333 s

$ nuclear-engineering solve nuc_committed_dose_retention "I=1000000 (1/s)" "f=[0.3]" "Tb=[80]" t_unit=d "T_p=692988.48 s" "E=0.18 MeV" w_R=1 "m=20 g" "tau=1577880000 s"
U = 2.726007069e+11
H = 0.3930790347 J/kg
H_mSv = 393.0790347

$ nuclear-engineering solve nuc_collisions_to_thermalize A=12 "E0=2 MeV" "Ef=0.025 eV"
n = 115.3429277
xi = 0.1577689898

$ nuclear-engineering solve nuc_power_after_step_one_group rho=0.001 beta=0.0065 "lam=0.08 1/s" "Lambda=0.0001 s" "t=10 s"
P = 1.365788055
A1 = 1.181194942
omega = 0.01452050027 1/s
```

## Rust library: 15 examples

The crate `nuclear-engineering-ai999x` (in `rust/` of the GitHub repository) answers a JSON request with `nuclear_engineering_ai999x::solve_json`. Each call below was compiled and run on this release; the comment under it is what it printed. The whole program is `rust/nuclear-engineering/examples/readme.rs`: `cd rust && cargo run --release --example readme -p nuclear-engineering-ai999x`.

```rust
use serde_json::{json, Value};

fn show(request: Value) {
    let answer: Value = serde_json::from_str(&nuclear_engineering_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

show(json!({"key": "nuc_activation_with_burnup", "inputs": {"N_T0": "1e+22", "sigma_T": "3.718e-27 m^2", "sigma_P": "2e-28 m^2", "phi": "1e+14 1/(cm^2*s)", "T_half": "166346024.4 s", "t": "31557600 s"}}));
// SOLVED
// A_P = 4303591319410.0 1/s
// f_T = 0.88929064975
// t_max = 247907988.467 s
// A_max = 14114011896900.0 1/s

show(json!({"key": "nuc_compton_edge", "inputs": {"E": "662 keV"}}));
// SOLVED
// T_max = 0.477650409375 MeV
// E_bs = 0.184349590625 MeV

show(json!({"key": "nuc_committed_dose_coefficient", "inputs": {"I": "1000000 (1/s)", "e": "2.2e-08 (J/kg)/(1/s)"}}));
// SOLVED
// E = 0.022 J/kg
// E_mSv = 22.0
// I_Bq = 1000000.0 1/s

show(json!({"key": "nuc_enrichment_swu", "inputs": {"x_p": "0.045", "x_f": "0.00711", "x_w": "0.0025", "P": "1 kg"}}));
// SOLVED
// F = 9.21908893709 kg
// W = 8.21908893709 kg
// SWU = 6.87111298932 kg
// F_per_P = 9.21908893709
// SWU_per_P = 6.87111298932

show(json!({"key": "nuc_logarithmic_energy_decrement", "inputs": {"A": "12"}}));
// SOLVED
// alpha = 0.715976331361
// xi = 0.157768989823
// mu_bar = 0.0555555555556

show(json!({"key": "nuc_control_rod_worth", "inputs": {"rho_tot": "0.01", "H": "3.66 m", "x": "1.83 m"}}));
// SOLVED
// rho = 0.005
// frac = 0.5
// drho_dx = 0.00546448087432 1/m

show(json!({"key": "nuc_inverse_multiplication", "inputs": {"x": [0, 20, 40, 60, 80], "C": [100, 125, 167, 250, 500]}}));
// SOLVED
// x_crit = 99.9760479042
// slope = -0.01
// x_crit_last2 = 100.0
// inv_M_last = 0.2

show(json!({"key": "nuc_half_value_layer", "inputs": {"mu": "0.656 1/cm"}}));
// SOLVED
// HVL = 0.0105662679963 m
// TVL = 0.0351003825152 m
// mfp = 0.015243902439 m
// HVL_cm = 1.05662679963
// TVL_cm = 3.51003825152

show(json!({"key": "nuc_accumulated_dose_decaying_source", "inputs": {"H0": "0.001 (J/kg)/h", "T_half": "6586.2 s", "t": "2 h"}}));
// SOLVED
// D = 0.00140226015359 J/kg
// D_mSv = 1.40226015359
// D_inf = 0.00263941057731 J/kg
// H_mean = 1.94758354665e-7 J/(kg*s)

show(json!({"key": "nuc_channel_coolant_temperature", "inputs": {"q0": "43 kW/m", "H": "3.66 m", "H_e": "4 m", "m_dot": "0.3 kg/s", "c_p": "5500 J/(kg*K)", "T_in": "565 K", "z": "0 m"}}));
// SOLVED
// T = 597.886071405 K
// T_out = 630.77214281 K
// Q = 108524.035637 W

show(json!({"key": "nuc_specific_activity", "inputs": {"nuclide": "Co-60", "m": "1 g"}}));
// SOLVED
// A = 41868944303400.0 1/s
// A_Ci = 1131.59308928
// a = 4.18689443034e+16 1/(kg*s)
// a_Ci_per_g = 1131.59308928
// N = 1.00479849416e+22

show(json!({"key": "nuc_count_time_for_precision", "inputs": {"Rg": "100 (1/s)", "Rb": "25 (1/s)", "r": "0.01"}}));
// SOLVED
// T = 400.0 s
// tg = 266.666666667 s
// tb = 133.333333333 s

show(json!({"key": "nuc_committed_dose_retention", "inputs": {"I": "1000000 (1/s)", "f": [0.3], "Tb": [80], "t_unit": "d", "T_p": "692988.48 s", "E": "0.18 MeV", "w_R": "1", "m": "20 g", "tau": "1577880000 s"}}));
// SOLVED
// U = 272600706922.0
// H = 0.393079034738 J/kg
// H_mSv = 393.079034738

show(json!({"key": "nuc_collisions_to_thermalize", "inputs": {"A": "12", "E0": "2 MeV", "Ef": "0.025 eV"}}));
// SOLVED
// n = 115.342927739
// xi = 0.157768989823

show(json!({"key": "nuc_power_after_step_one_group", "inputs": {"rho": "0.001", "beta": "0.0065", "lam": "0.08 1/s", "Lambda": "0.0001 s", "t": "10 s"}}));
// SOLVED
// P = 1.36578805499
// A1 = 1.18119494222
// omega = 0.0145205002736 1/s
```

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/nuclear-engineering-ai999x/blob/main/docs/nuclear_engineering_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/nuclear-engineering-ai999x/blob/main/docs/nuclear_engineering_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/nuclear-engineering-ai999x/blob/main/docs/nuclear_engineering_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/nuclear-engineering-ai999x/blob/main/docs/nuclear_engineering_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/nuclear-engineering-ai999x/blob/main/docs/nuclear_engineering_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/nuclear-engineering-ai999x/blob/main/docs/nuclear_engineering_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/nuclear-engineering-ai999x/blob/main/docs/nuclear_engineering_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/nuclear-engineering-ai999x/blob/main/docs/nuclear_engineering_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/nuclear-engineering-ai999x/blob/main/docs/nuclear_engineering_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Nuclear Engineering on ai999x.com](https://www.ai999x.com/models/nuclear-engineering-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/nuclear-engineering-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.6`; the [GitHub release](https://github.com/AI999X/nuclear-engineering-ai999x/releases/tag/v1.6.6) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `nuclear-engineering` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/nuclear-engineering`) and its data (`rust/models/nuclear_engineering/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p nuclear-engineering-ai999x
./target/release/nuclear-engineering --help
cargo test -p nuclear-engineering-ai999x
```
