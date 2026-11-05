# nuclear-engineering-ai999x — offline, verified nuclear engineering solver with step-by-step answers

[![tests](https://github.com/AI999X/nuclear-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/nuclear-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/nuclear-engineering-ai999x)](https://pypi.org/project/nuclear-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/nuclear-engineering-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/nuclear-engineering-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/nuclear-engineering-ai999x/)** · **[Guides](https://huggingface.co/AI999X/nuclear-engineering-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/nuclear-engineering-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.3

Field: **Engineering** · Tasks: Question Answering, Token Classification, Text Classification, Sentence Similarity, Feature Extraction

nuclear-engineering-ai999x is an offline, deterministic Python solver that answers nuclear engineering questions with verified results from 83 certified capabilities, gives 554 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**nuclear-engineering-ai999x** is a compact, deterministic nuclear engineering engine for Python. It reads a question in plain English (or a typed call), picks a certified nuclear engineering capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** nuclear-engineering-ai999x comes as the AI999X desktop app and as the command-line program `nuclear-engineering` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Nuclear Engineering desktop app** 1.2.0 | Windows, Linux — this model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-nuclear-engineering-1.2.0-windows-x86_64.zip) · [Linux (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-nuclear-engineering-1.2.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.2.0 | Windows, Linux — every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-1.2.0-windows-x86_64.zip) · [Linux (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-1.2.0-linux-x86_64.tar.gz) |
| **Command line** (`nuclear-engineering`) | Windows, Linux — one program per model, offline, no Python | [Windows (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-windows-x86_64.zip) · [Linux (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-linux-x86_64.tar.gz) |
| **In your browser** | any device — nothing to install | [Compute on the model page](https://www.ai999x.com/models/nuclear-engineering-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install nuclear-engineering-ai999x` |

Checksums: [SHA256SUMS](https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS) · every version: [https://www.ai999x.com/downloads/desktop/latest/](https://www.ai999x.com/downloads/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Rust versions

Nuclear Engineering also comes as native programs compiled from Rust, version 1.2.0 of the AI999X desktop app and command line: they run offline on your computer and need no Python. Each file is in this repository's `rust/` folder, on the [GitHub release v1.0.3](https://github.com/AI999X/nuclear-engineering-ai999x/releases/tag/v1.0.3) and on the website.

| Program | System | File | Size | Download | Use |
|---|---|---|---|---|---|
| Desktop app (Nuclear Engineering) | Windows x86-64 | `ai999x-nuclear-engineering-1.2.0-windows-x86_64.zip` | 7.5 MB | [Hub](https://huggingface.co/AI999X/nuclear-engineering-ai999x/resolve/main/rust/ai999x-nuclear-engineering-1.2.0-windows-x86_64.zip) · [GitHub](https://github.com/AI999X/nuclear-engineering-ai999x/releases/download/v1.0.3/ai999x-nuclear-engineering-1.2.0-windows-x86_64.zip) · [website](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-nuclear-engineering-1.2.0-windows-x86_64.zip) | unzip, then double-click `ai999x-nuclear-engineering.exe` (no installer) |
| Desktop app (Nuclear Engineering) | Linux x86-64 | `ai999x-nuclear-engineering-1.2.0-linux-x86_64.tar.gz` | 8.4 MB | [Hub](https://huggingface.co/AI999X/nuclear-engineering-ai999x/resolve/main/rust/ai999x-nuclear-engineering-1.2.0-linux-x86_64.tar.gz) · [GitHub](https://github.com/AI999X/nuclear-engineering-ai999x/releases/download/v1.0.3/ai999x-nuclear-engineering-1.2.0-linux-x86_64.tar.gz) · [website](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-nuclear-engineering-1.2.0-linux-x86_64.tar.gz) | extract, then run `./ai999x-nuclear-engineering` (X11 or Wayland desktop) |
| Command line (`nuclear-engineering`) | Windows x86-64 | `ai999x-nuclear-engineering-cli-1.2.0-windows-x86_64.zip` | 3.1 MB | [Hub](https://huggingface.co/AI999X/nuclear-engineering-ai999x/resolve/main/rust/ai999x-nuclear-engineering-cli-1.2.0-windows-x86_64.zip) · [GitHub](https://github.com/AI999X/nuclear-engineering-ai999x/releases/download/v1.0.3/ai999x-nuclear-engineering-cli-1.2.0-windows-x86_64.zip) · [website (all models)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-windows-x86_64.zip) | unzip, then `nuclear-engineering.exe ask "<question>"` or `nuclear-engineering.exe solve <capability> name=value` |
| Command line (`nuclear-engineering`) | Linux x86-64 | `ai999x-nuclear-engineering-cli-1.2.0-linux-x86_64.tar.gz` | 3.1 MB | [Hub](https://huggingface.co/AI999X/nuclear-engineering-ai999x/resolve/main/rust/ai999x-nuclear-engineering-cli-1.2.0-linux-x86_64.tar.gz) · [GitHub](https://github.com/AI999X/nuclear-engineering-ai999x/releases/download/v1.0.3/ai999x-nuclear-engineering-cli-1.2.0-linux-x86_64.tar.gz) · [website (all models)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-linux-x86_64.tar.gz) | extract, then `./nuclear-engineering ask "<question>"` or `./nuclear-engineering solve <capability> name=value` |

The command-line archive holds `nuclear-engineering` with a README of its commands and examples, the licence and the notice. Checksums: [`rust/SHA256SUMS`](https://huggingface.co/AI999X/nuclear-engineering-ai999x/resolve/main/rust/SHA256SUMS) (`sha256sum -c SHA256SUMS`).

## Quick start

### Desktop app

Download **AI999X Nuclear Engineering** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Nuclear Engineering among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-nuclear-engineering-1.2.0-linux-x86_64.tar.gz
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-nuclear-engineering-1.2.0-linux-x86_64.tar.gz
./ai999x-nuclear-engineering-1.2.0-linux-x86_64/ai999x-nuclear-engineering
```

### Command line

Linux:

```bash
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-linux-x86_64.tar.gz
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.2.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.2.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
nuclear-engineering --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.2.0-windows-x86_64.zip"
Invoke-WebRequest "https://www.ai999x.com/downloads/desktop/1.2.0/$F" -OutFile $F
Invoke-WebRequest "https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.2.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
nuclear-engineering --version
```

Every command below was run as shown; the text under it is what the program printed.

**Ask in words** — Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```console
$ nuclear-engineering ask "Specific activity: nuclide = Co-60, m = 1 g"
Read as: Activity and specific activity of a mass of a radionuclide
A = 4.18689443e+13 1/s
A_Ci = 1131.593089
a = 4.18689443e+16 1/(kg*s)
a_Ci_per_g = 1131.593089
N = 1.004798494e+22
  by: Activity and specific activity of a mass of a radionuclide  (N = m N_A / M, A = lambda N with lambda = ln 2 / T_half)
  m = 0.001 kg
  nuclide = Co-60
  assumes: the nuclide is in the table (ICRP-107 half-lives, radioactivedecay 0.6.1); m > 0
```

**What a capability needs** — The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ nuclear-engineering info nuc_specific_activity
Activity and specific activity of a mass of a radionuclide
  N = m N_A / M, A = lambda N with lambda = ln 2 / T_half
  in   nuclide  the radionuclide, written like Co-60, Cs-137, Tc-99m, U-235 (choice) [1]
  in   m        mass of the radionuclide (the pure nuclide, not the compound) [kg]
  out  A        activity (Bq) [1/s]
  out  A_Ci     activity in curies [1]
  out  a        specific activity (Bq/kg) [1/(kg*s)]
  out  a_Ci_per_g specific activity in Ci/g [1]
  out  N        number of atoms [1]
```

**Solve** — Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ nuclear-engineering solve nuc_specific_activity nuclide=Co-60 "m=1 g"
A = 4.18689443e+13 1/s
A_Ci = 1131.593089
a = 4.18689443e+16 1/(kg*s)
a_Ci_per_g = 1131.593089
N = 1.004798494e+22
  by: Activity and specific activity of a mass of a radionuclide  (N = m N_A / M, A = lambda N with lambda = ln 2 / T_half)
  m = 0.001 kg
  nuclide = Co-60
  assumes: the nuclide is in the table (ICRP-107 half-lives, radioactivedecay 0.6.1); m > 0
```

**Draw the figures** — The live figures for your own inputs, written as SVG files into the folder given with --out (here the current folder); open them in any browser. Without --out the SVG text comes back as JSON.

```console
$ nuclear-engineering figures nuc_specific_activity nuclide=Co-60 "m=1 g" --out .
[
  {
    "slug": "nuc-specific-activity-live-1",
    "title": "Activity (Bq) as mass of the radionuclide (the pure nuclide, not... varies",
    "caption": "Activity (Bq) as m runs from 0.25 to 1.75 g, every other input as you entered it (41 solutions). Marked: your m = 1 g gives A = 4.187e+13. Your inputs: nuclide = Co-60, m = 1 g.",
    "file": "./nuc-specific-activity-live-1.svg"
  },
  {
    "slug": "nuc-specific-activity-live-2",
    "title": "The computed values side by side",
    "caption": "Every answer of one unit for your inputs: A_Ci (activity in curies) = 1132; a_Ci_per_g (specific activity in Ci/g) = 1132; N (number of atoms) = 1.005e+22. Your inputs: nuclide = Co-60, m = 1 g.",
    "file": "./nuc-specific-activity-live-2.svg"
  }
]
```

**Many questions at once** — One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | nuclear-engineering batch

`cases.jsonl`:

```json
{"key": "nuc_specific_activity", "inputs": {"nuclide": "Co-60", "m": "1 g"}}
{"key": "nuc_point_source_dose_rate", "inputs": {"Gamma": "3.51e-13 (J/kg)*m^2/((1/s)*h)", "A": "3.7e+11 (1/s)", "r": "2 m", "mu": "0.0656 1/cm", "x": "10 cm", "buildup": "linear"}}
```

```console
$ nuclear-engineering batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":41868944303400.0,"unit":"1/s","solved_for":"A","si":[["m",0.001]],"values":[["A",41868944303400.0,"1/s"],["A_Ci",1131.59308928,""],["a",4.18689443034e+16,"1/(kg*s)"],["a_Ci_per_g",1131.59308928,""],["N",1.00479849416e+22,""]],"target":"A","phrase":"activity (Bq)","assumptions":["the nuclide is in the table (ICRP-107 half-lives, radioactivedecay 0.6.1)","m > 0"],"name":"Activity and specific activity of a mass of a radionuclide","equation":"N = m N_A / M, A = lambda N with lambda = ln 2 / T_half"}
{"outcome":"SOLVED","message":"","value":7.75013916132e-6,"unit":"J/(kg*s)","solved_for":"H_dot","si":[["Gamma",9.750000000000001e-17],["A",370000000000.0],["r",2.0],["mu",6.5600000000000005],["x",0.1]],"values":[["H_dot",7.75013916132e-6,"J/(kg*s)"],["H_uSv_h",27900.5009807,""],["B",1.656,""],["T",0.859336289543,""]],"target":"H_dot","phrase":"dose rate (same quantity as Gamma: Sv/s, Gy/s)","assumptions":["point isotropic source","shield between source and point (x < r)","Taylor parameters only with the Taylor form"],"name":"Dose rate from a shielded point gamma source with buildup","equation":"H = Gamma A B(mu x) e^(-mu x) / r^2; B = 1, 1 + mu x, or A1 e^(-alpha1 mu x) + (1 - A1) e^(-alpha2 mu x)"}
```

### In your browser

Open [Nuclear Engineering on ai999x.com](https://www.ai999x.com/models/nuclear-engineering-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `nuclear_engineering_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install nuclear-engineering-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Activation with target depletion and product burn-up

The product activity when a high flux both depletes the target nuclide and burns up the product by its own absorption, the fraction of target left, and the time and value of the maximum product activity.

Command line:

```console
$ nuclear-engineering solve nuc_activation_with_burnup N_T0=1e+22 "sigma_T=3.718e-27 m^2" "sigma_P=2e-28 m^2" "phi=1e+14 1/(cm^2*s)" "T_half=166346024.4 s" "t=31557600 s"
A_P = 4.303591319e+12 1/s
f_T = 0.8892906498
t_max = 247907988.5 s
A_max = 1.41140119e+13 1/s
  by: Activation with target depletion and product burn-up  (dN_T/dt = -sigma_T phi N_T, dN_P/dt = sigma_T phi N_T - (lambda + sigma_P phi) N_P: N_P = N_T0 sigma_T phi (e^(-a t) - e^(-b t))/(b - a) (40 digits))
  N_T0 = 1e+22
  sigma_T = 3.718e-27 m^2
  sigma_P = 2e-28 m^2
  phi = 1e+18 1/(m^2*s)
  T_half = 166346024.4 s
  t = 31557600 s
  assumes: constant flux; removal rates sigma_T phi and lambda + sigma_P phi differ
```

Python:

```python
import nuclear_engineering_ai999x as m

a = m.solve("nuc_activation_with_burnup", N_T0="1e+22", sigma_T="3.718e-27 m^2", sigma_P="2e-28 m^2", phi="1e+14 1/(cm^2*s)", T_half="166346024.4 s", t="31557600 s")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/nuclear-engineering-ai999x/#cap-nuc_activation_with_burnup) and press **Compute in your browser**.

### Activity and specific activity of a mass of a radionuclide

The activity of a stated mass of a named radionuclide, its specific activity (activity per unit mass) and the number of atoms, from its tabulated half-life and atomic mass.

Command line:

```console
$ nuclear-engineering solve nuc_specific_activity nuclide=Co-60 "m=1 g"
A = 4.18689443e+13 1/s
A_Ci = 1131.593089
a = 4.18689443e+16 1/(kg*s)
a_Ci_per_g = 1131.593089
N = 1.004798494e+22
  by: Activity and specific activity of a mass of a radionuclide  (N = m N_A / M, A = lambda N with lambda = ln 2 / T_half)
  m = 0.001 kg
  nuclide = Co-60
  assumes: the nuclide is in the table (ICRP-107 half-lives, radioactivedecay 0.6.1); m > 0
```

Python:

```python
import nuclear_engineering_ai999x as m

a = m.solve("nuc_specific_activity", nuclide="Co-60", m="1 g")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/nuclear-engineering-ai999x/#cap-nuc_specific_activity) and press **Compute in your browser**.

_What this means:_ This calculation finds the total decay rate of a pure sample. Starting with a mass of 1 g, the number of atoms follows from the atomic mass and is then multiplied by the decay constant derived from the half-life. The result is an activity of 4.18689e+13 1/s and a specific activity of 4.18689e+16 1/(kg*s).

### Activity of a radionuclide and its progeny after a decay time

Starting from a pure parent of stated activity, the activities of the parent, of its first daughter and of the whole chain (all radioactive progeny) after a time t, with the real branching fractions (Sr-90/Y-90, Mo-99/Tc-99m, Cs-137/Ba-137m, Ge-68/Ga-68, ...).

Command line:

```console
$ nuclear-engineering solve nuc_chain_activity nuclide=Mo-99 "A0=3.7e+10 (1/s)" "t=86400 s"
A_parent = 2.874992172e+10 1/s
A_daughter = 2.550612488e+10 1/s
A_total = 5.425604681e+10 1/s
ratio = 0.8871719766
  by: Activity of a radionuclide and its progeny after a decay time  (Bateman solution of the whole chain (every branch), evaluated with 40-digit arithmetic)
  A0 = 3.7e+10 1/s
  t = 86400 s
  nuclide = Mo-99
  assumes: pure parent at t = 0; t >= 0; branching fractions and half-lives of radioactivedecay 0.6.1
```

Python:

```python
import nuclear_engineering_ai999x as m

a = m.solve("nuc_chain_activity", nuclide="Mo-99", A0="3.7e+10 (1/s)", t="86400 s")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/nuclear-engineering-ai999x/#cap-nuc_chain_activity) and press **Compute in your browser**.

_What this means:_ This calculation determines the activities of a radionuclide and its progeny after a specific decay time. Starting with an initial parent activity of 3.7e+10 (1/s), the model uses the Bateman solution to compute the remaining parent activity, which is now 2.87499e+10 1/s, and the grown-in activity of the first daughter, now 2.55061e+10 1/s. The total activity for all radioactive members in the chain is reported as 5.4256e+10 1/s, while the ratio of daughter to parent activity is given by 0.887172. These values reflect the natural decay kinetics without external removal or production.

### Irradiation time to reach a fraction of saturation activity

How long a target must be irradiated (constant flux) for the induced activity to reach a stated fraction of its saturation value, in time and in product half-lives.

Command line:

```console
$ nuclear-engineering solve nuc_irradiation_time_for_saturation "T_half=15 h" f=0.9
t_irr = 179384.1171 s
n_half = 3.321928095
  by: Irradiation time to reach a fraction of saturation activity  (1 - e^(-lambda t) = f -> t = -ln(1 - f)/lambda)
  T_half = 54000 s
  f = 0.9
  assumes: constant flux; 0 < f < 1
```

Python:

```python
import nuclear_engineering_ai999x as m

a = m.solve("nuc_irradiation_time_for_saturation", T_half="15 h", f="0.9")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/nuclear-engineering-ai999x/#cap-nuc_irradiation_time_for_saturation) and press **Compute in your browser**.

_What this means:_ This calculation determines the precise duration a target must be exposed to a constant neutron flux to achieve a specific fraction of its maximum possible induced activity. Given a product half-life of 15 h and a desired saturation fraction of 0.9, the required irradiation time is computed as 179384 s. This result indicates that the process takes 3.32193 half-lives, providing engineers with a clear schedule for production planning while accounting for the exponential approach to saturation limits.

## Documentation

- **Rust user guide** — the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/nuclear-engineering-ai999x/blob/main/docs/nuclear_engineering_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/nuclear-engineering-ai999x/blob/main/docs/nuclear_engineering_ai999x_rust_guide.md)
- **Python guide** — every capability with a Python example: [PDF](https://huggingface.co/AI999X/nuclear-engineering-ai999x/blob/main/docs/nuclear_engineering_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/nuclear-engineering-ai999x/blob/main/docs/nuclear_engineering_ai999x_capabilities.md)
- **Desktop app & command line** — downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page** — compute in your browser, every capability explained: https://www.ai999x.com/models/nuclear-engineering-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified nuclear engineering capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 554 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `nuclear-engineering` command line for Windows and Linux, and WebAssembly on the model page — no Python, no network.
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

## Python quick start

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

The Python package also installs a Python command, `nuclear-engineering-ai999x "<question>"`; for the native program use `nuclear-engineering` (above). The outputs above were produced by running this exact code on nuclear-engineering-ai999x 1.0.3.

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

## Python worked examples, with their steps

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
