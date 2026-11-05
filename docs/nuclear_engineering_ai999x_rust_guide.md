# nuclear-engineering: the Nuclear Engineering program — user guide

**nuclear-engineering-ai999x 1.1.0**, Rust build: command line, desktop app and browser. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Nuclear Engineering covers radioactive decay and decay chains, binding energy, reactor physics basics, radiation shielding and attenuation, and dosimetry. This guide is about `nuclear-engineering`, the model as a self-contained program for Windows and Linux: 83 capabilities in 10 topics, answered on your own computer with no network, no Python and nothing else to install. The same compiled model runs in the AI999X desktop app and in the browser on the model's page (https://www.ai999x.com/models/nuclear-engineering-ai999x/); the Python package (`pip install nuclear-engineering-ai999x`) has its own guide. Every command output printed here is what the program printed.

## Contents

- 1. Get it: desktop app, command line, browser, Python
- 2. Quick start
- 3. Writing inputs
- 4. Outcomes and exit codes
- 5. Scripts and agents: batch and JSON
- 6. Figures
- 7. In a web page or Node.js: WebAssembly
- 8. Every capability

## 1. Get it

Version 1.4.0 of the AI999X desktop app and of the command-line bundle. Checksums for every file: https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS; the newest version is always under https://downloads.ai999x.com/desktop/latest/. Full instructions: https://www.ai999x.com/desktop-cli/

| what | system | download |
|---|---|---|
| Desktop app: Nuclear Engineering | Windows x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-nuclear-engineering-1.4.0-windows-x86_64.zip |
| Desktop app: Nuclear Engineering | Linux x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-nuclear-engineering-1.4.0-linux-x86_64.tar.gz |
| Desktop app: all models | Windows x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-windows-x86_64.zip |
| Desktop app: all models | Linux x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-linux-x86_64.tar.gz |
| Command line | Windows x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-windows-x86_64.zip |
| Command line | Linux x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz |

### Desktop app

Nuclear Engineering in its own app: search, questions in words, input forms, live figures and explanations. Windows: Download ai999x-nuclear-engineering-1.4.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-nuclear-engineering.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-nuclear-engineering-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-nuclear-engineering-1.4.0-linux-x86_64.tar.gz
./ai999x-nuclear-engineering-1.4.0-linux-x86_64/ai999x-nuclear-engineering
```

It opens on this model: type a question or pick a capability, fill in the form and press Compute. The AI999X desktop app with every model works the same way (choose the model first). Windows: Download ai999x-1.4.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-1.4.0-linux-x86_64.tar.gz
./ai999x-1.4.0-linux-x86_64/ai999x
```

### Command line — Linux

The bundle holds one program per model; `nuclear-engineering` is this one. The checksum line should print OK.

```shell
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.4.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.4.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
nuclear-engineering --version
```

### Command line — Windows (PowerShell)

The fourth line should print True.

```powershell
$F = "ai999x-cli-1.4.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.4.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.4.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
nuclear-engineering --version
```

### In your browser

Open https://www.ai999x.com/models/nuclear-engineering-ai999x/, choose a capability and press Compute in your browser: the same program runs on your device as WebAssembly; nothing is sent anywhere.

### Python

```shell
pip install nuclear-engineering-ai999x
```

## 2. Quick start

### 2.1 Check the install

Prints the model, its version and that this is the Rust build.

```shell
$ nuclear-engineering --version
nuclear-engineering-ai999x 1.0.3 (Rust build, ax-engine 0.1.0)
```

### 2.2 Ask in words

Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```shell
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

### 2.3 Find a capability

Search the capabilities by what you want to know; the first column is the id to pass to info and solve.

```shell
$ nuclear-engineering search "activity and specific activity of a mass of a radionuclide"
nuc_specific_activity                Activity and specific activity of a mass of a radionuclide  (decay and activity / specific activity)
nuc_mass_for_activity                Mass and number of atoms of a radionuclide with a given activity  (decay and activity / specific activity)
nuc_chain_activity                   Activity of a radionuclide and its progeny after a decay time  (decay and activity / decay chains of real nuclides)
nuc_generator_daughter_activity      Radionuclide generator: daughter activity after elution (with branching)  (decay and activity / generators)
nuc_activation_activity              Neutron activation: activity after irradiation and decay  (decay and activity / neutron activation)
nuc_daughter_max_activity_time       Time of maximum daughter activity  (decay and activity / decay chains and equilibrium)
nuc_irradiation_time_for_saturation  Irradiation time to reach a fraction of saturation activity  (decay and activity / neutron activation)
nuc_equilibrium_ratio                Transient and secular equilibrium: activity ratio and approach  (decay and activity / decay chains and equilibrium)
… 12 more
```

### 2.4 List the capabilities

Every capability the program answers: its id (what you pass to solve) and its name.

```shell
$ nuclear-engineering list
nuc_accumulated_dose_decaying_source     Dose accumulated near a decaying source
nuc_activation_activity                  Neutron activation: activity after irradiation and decay
nuc_activation_with_burnup               Activation with target depletion and product burn-up
nuc_attenuation_coefficient_fit          Linear attenuation coefficient from narrow-beam transmission measurements
nuc_bare_reactor_keff                    k-effective and geometric buckling of a bare reactor (one group)
nuc_centreline_temperature_variable_k    Fuel centreline temperature with temperature-dependent conductivity
nuc_chain_activity                       Activity of a radionuclide and its progeny after a decay time
nuc_channel_coolant_temperature          Coolant temperature along a channel with a chopped-cosine power shape
… 75 more
```

### 2.5 What a capability needs

The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```shell
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

### 2.6 Solve

Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```shell
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

### 2.7 Machine-readable answer

--json prints the same answer as one line of JSON (values in SI): for scripts and other programs.

```shell
$ nuclear-engineering solve nuc_specific_activity nuclide=Co-60 "m=1 g" --json
{"outcome":"SOLVED","message":"","value":41868944303400.0,"unit":"1/s","solved_for":"A","si":[["m",0.001]],"values":[["A",41868944303400.0,"1/s"],["A_Ci",1131.59308928,""],["a",4.18689443034e+16,"1/(kg*s)"],["a_Ci_per_g",1131.59308928,""],["N",1.00479849416e+22,""]],"target":"A","phrase":"activity (Bq)","assumptions":["the nuclide is in the table (ICRP-107 half-lives, radioactivedecay 0.6.1)","m > 0"],"name":"Activity and specific activity of a mass of a radionuclide","equation":"N = m N_A / M, A = lambda N with lambda = ln 2 / T_half"}
```

### 2.8 When an input is missing

No number is guessed: the answer names what is missing, shows a complete call, and the program exits with status 1.

```shell
$ nuclear-engineering solve nuc_specific_activity nuclide=Co-60
NEEDS_INPUT: nuc_specific_activity needs m (mass of the radionuclide (the pure nuclide, not the compound)). Example: solve('nuc_specific_activity', nuclide='Co-60', m='0.001 kg').
```

### 2.9 Draw the figures

The live figures for your own inputs, written as SVG files into the folder given with --out (here the current folder); open them in any browser. Without --out the SVG text comes back as JSON.

```shell
$ nuclear-engineering figures nuc_specific_activity nuclide=Co-60 "m=1 g" --out .
[
  {
    "slug": "nuc-specific-activity-live-1",
    "title": "Activity (Bq) as mass of the radionuclide (the pure nuclide, not... varies",
    "caption": "Activity (Bq) as m runs from 0.25 to 1.75 g, every other input as you entered it (41 solutions). Marked: your m = 1 g gives A = 4.187e+13. Your inputs: nuclide = Co-60, m = 1 g.",
    "file": ".\\nuc-specific-activity-live-1.svg"
  },
  {
    "slug": "nuc-specific-activity-live-2",
    "title": "The computed values side by side",
    "caption": "Every answer of one unit for your inputs: A_Ci (activity in curies) = 1132; a_Ci_per_g (specific activity in Ci/g) = 1132; N (number of atoms) = 1.005e+22. Your inputs: nuclide = Co-60, m = 1 g.",
    "file": ".\\nuc-specific-activity-live-2.svg"
  }
]
```

### 2.10 Many questions at once

One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | nuclear-engineering batch

`cases.jsonl`:

```json
{"key": "nuc_specific_activity", "inputs": {"nuclide": "Co-60", "m": "1 g"}}
{"key": "nuc_point_source_dose_rate", "inputs": {"Gamma": "3.51e-13 (J/kg)*m^2/((1/s)*h)", "A": "3.7e+11 (1/s)", "r": "2 m", "mu": "0.0656 1/cm", "x": "10 cm", "buildup": "linear"}}
```

```shell
$ nuclear-engineering batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":41868944303400.0,"unit":"1/s","solved_for":"A","si":[["m",0.001]],"values":[["A",41868944303400.0,"1/s"],["A_Ci",1131.59308928,""],["a",4.18689443034e+16,"1/(kg*s)"],["a_Ci_per_g",1131.59308928,""],["N",1.00479849416e+22,""]],"target":"A","phrase":"activity (Bq)","assumptions":["the nuclide is in the table (ICRP-107 half-lives, radioactivedecay 0.6.1)","m > 0"],"name":"Activity and specific activity of a mass of a radionuclide","equation":"N = m N_A / M, A = lambda N with lambda = ln 2 / T_half"}
{"outcome":"SOLVED","message":"","value":7.75013916132e-6,"unit":"J/(kg*s)","solved_for":"H_dot","si":[["Gamma",9.750000000000001e-17],["A",370000000000.0],["r",2.0],["mu",6.5600000000000005],["x",0.1]],"values":[["H_dot",7.75013916132e-6,"J/(kg*s)"],["H_uSv_h",27900.5009807,""],["B",1.656,""],["T",0.859336289543,""]],"target":"H_dot","phrase":"dose rate (same quantity as Gamma: Sv/s, Gy/s)","assumptions":["point isotropic source","shield between source and point (x < r)","Taylor parameters only with the Taylor form"],"name":"Dose rate from a shielded point gamma source with buildup","equation":"H = Gamma A B(mu x) e^(-mu x) / r^2; B = 1, 1 + mu x, or A1 e^(-alpha1 mu x) + (1 - A1) e^(-alpha2 mu x)"}
```

## 3. Writing inputs

Inputs are written `name=value`, one per argument; `nuclear-engineering info <capability>` lists the names. Quote an argument that holds a space or brackets (bash, zsh and PowerShell all accept double quotes). The answer always states each input as it was read, converted to SI, so a misread unit is visible at once.

| kind | example | how it is read |
|---|---|---|
| a number with a unit | `"R=0.128 nm"` | any unit of the same kind; converted to SI |
| a number in SI | `R=1.28e-10` | no unit means the SI unit shown by info |
| a choice | `structure=FCC` | one of the values info lists |
| a list or vector | `"v=[1, 2, 3]" or "v=1, 2, 3"` | brackets keep it a list |
| a matrix | `"A=[[1, 2], [3, 4]]"` | one list per row |
| true / false | `inclusive=true` | for yes/no options |

An equation can be solved for any of its quantities: give the others and add `--target <symbol>`. Without `--target`, the program solves for the quantity the capability is named after.

## 4. Outcomes and exit codes

The program never guesses a number: when it cannot answer, it says why and exits with status 1, so a script can tell an answer from a refusal without reading the text.

| outcome | meaning | exit status |
|---|---|---|
| SOLVED | the answer: every output with its unit, the method, the inputs in SI, the conditions assumed | 0 |
| NEEDS_INPUT | an input is missing; the message names it and shows a complete call | 1 |
| INVALID_INPUT | a value or unit of the wrong kind, an unknown input name, or a value outside the conditions | 1 |
| AMBIGUOUS | the inputs allow two different answers; say which one is meant | 1 |
| OUT_OF_SCOPE | the model has no capability for this, or the result would not be meaningful | 1 |
| (usage) | a command the program does not know: it prints its usage | 2 |

## 5. Scripts and agents: batch and JSON

`nuclear-engineering solve ... --json` prints one answer as one line of JSON. `nuclear-engineering batch` reads one request per line on standard input and writes one JSON answer per line, in order, starting the program once for any number of requests. A request is `{"key": "<capability>", "inputs": {"name": "value", ...}, "target": null}`. In PowerShell, pipe the file in: `Get-Content cases.jsonl | nuclear-engineering batch`. `nuclear-engineering catalog` prints every capability with its inputs as JSON, to build forms or tool definitions.

| answer field | meaning |
|---|---|
| outcome | one of the outcomes above |
| message | why there is no answer (empty when SOLVED) |
| value, unit | the main result in SI units |
| values | every output: [symbol, value, unit] (procedures) |
| target | the quantity solved for |
| si | each numeric input converted to SI: [symbol, value] |
| assumptions | the conditions the answer relies on |
| name, equation | the capability and its equation or method |

```python
import json, subprocess

reqs = [{"key": "<capability>", "inputs": {"name": "value"}}]
out = subprocess.run(["nuclear-engineering", "batch"], input="\n".join(map(json.dumps, reqs)),
                     capture_output=True, text=True, check=True).stdout
answers = [json.loads(line) for line in out.splitlines()]
```

## 6. Figures

`nuclear-engineering figures <capability> name=value ... --out DIR` draws the live figures for those inputs and writes each as an SVG file into DIR (an existing folder); without `--out` the SVG text comes back as JSON. The desktop app and the model page draw the same figures as soon as you press Compute.

```shell
$ nuclear-engineering figures nuc_specific_activity nuclide=Co-60 "m=1 g" --out .
[
  {
    "slug": "nuc-specific-activity-live-1",
    "title": "Activity (Bq) as mass of the radionuclide (the pure nuclide, not... varies",
    "caption": "Activity (Bq) as m runs from 0.25 to 1.75 g, every other input as you entered it (41 solutions). Marked: your m = 1 g gives A = 4.187e+13. Your inputs: nuclide = Co-60, m = 1 g.",
    "file": ".\\nuc-specific-activity-live-1.svg"
  },
  {
    "slug": "nuc-specific-activity-live-2",
    "title": "The computed values side by side",
    "caption": "Every answer of one unit for your inputs: A_Ci (activity in curies) = 1132; a_Ci_per_g (specific activity in Ci/g) = 1132; N (number of atoms) = 1.005e+22. Your inputs: nuclide = Co-60, m = 1 g.",
    "file": ".\\nuc-specific-activity-live-2.svg"
  }
]
```

## 7. In a web page or Node.js: WebAssembly

The same model is a WebAssembly module, `nuclear_engineering_ai999x.wasm` (https://www.ai999x.com/compute/rust/nuclear_engineering_ai999x.wasm), with a small loader, `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js). It runs in a browser, a Web Worker or Node.js, with no server.

```html
<script src="ax-wasm.js"></script>
<script>
  AXWasm.load("nuclear_engineering_ai999x.wasm").then(function (m) {
    var a = m.solve({key: "<capability>", inputs: {name: "value"}});
    console.log(a.outcome, a.values || a.value);
    console.log(m.catalog().capabilities.length + " capabilities");
  });
</script>
```

## 8. Every capability

By topic. For each: what it does, what it needs and gives, when it applies, and a command with what the program printed for it. `nuclear-engineering info <id>` shows the same inputs and outputs in the terminal.

### Decay and activity (10)

<a id="nuc_activation_with_burnup"></a>

#### Activation with target depletion and product burn-up — `nuc_activation_with_burnup`

The product activity when a high flux both depletes the target nuclide and burns up the product by its own absorption, the fraction of target left, and the time and value of the maximum product activity.

$$
N_P = \frac{N_{T0}\sigma_T\phi}{\Lambda_P-\Lambda_T}\left(e^{-\Lambda_T t}-e^{-\Lambda_P t}\right),\ \Lambda_T=\sigma_T\phi,\ \Lambda_P=\lambda+\sigma_P\phi
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `N_T0` | in | initial number of target atoms | — |
| `sigma_T` | in | target capture cross section (barns) | m^2 |
| `sigma_P` | in | product absorption cross section (barns) | m^2 |
| `phi` | in | neutron flux | 1/(m^2*s) |
| `T_half` | in | half-life of the product | s |
| `t` | in | irradiation time | s |
| `A_P` | out | product activity at t | 1/s |
| `f_T` | out | fraction of target atoms left | — |
| `t_max` | out | irradiation time of maximum product activity | s |
| `A_max` | out | maximum product activity | 1/s |

**Applies when:** constant flux; removal rates sigma_T phi and lambda + sigma_P phi differ.

```shell
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

In the worked example, N_T0 = 1e+22, sigma_T = 3.718e-27 m^2, sigma_P = 2e-28 m^2, phi = 1e+14 1/(cm^2*s), T_half = 166346024.4 s, t = 31557600 s go through the equation above and give A_P = 4.303591318e+12 1/s, f_T = 0.8892906498, t_max = 247907988.5 s, A_max = 1.41140119e+13 1/s. It applies only when constant flux; removal rates sigma_T phi and lambda + sigma_P phi differ: outside that it says what is wrong instead of returning a number.

<a id="nuc_specific_activity"></a>

#### Activity and specific activity of a mass of a radionuclide — `nuc_specific_activity`

The activity of a stated mass of a named radionuclide, its specific activity (activity per unit mass) and the number of atoms, from its tabulated half-life and atomic mass.

$$
A = \lambda N = \frac{\ln 2}{T_{1/2}}\,\frac{m\,N_A}{M}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `nuclide` | in | the radionuclide, written like Co-60, Cs-137, Tc-99m, U-235 | — |
| `m` | in | mass of the radionuclide (the pure nuclide, not the compound) | kg |
| `A` | out | activity (Bq) | 1/s |
| `A_Ci` | out | activity in curies | — |
| `a` | out | specific activity (Bq/kg) | 1/(kg*s) |
| `a_Ci_per_g` | out | specific activity in Ci/g | — |
| `N` | out | number of atoms | — |

**Applies when:** the nuclide is in the table (ICRP-107 half-lives, radioactivedecay 0.6.1); m > 0.

```shell
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

**What the worked example means:** This calculation finds the total decay rate of a pure sample. Starting with a mass of 1 g, the number of atoms follows from the atomic mass and is then multiplied by the decay constant derived from the half-life. The result is an activity of 4.18689e+13 1/s and a specific activity of 4.18689e+16 1/(kg*s).

<a id="nuc_chain_activity"></a>

#### Activity of a radionuclide and its progeny after a decay time — `nuc_chain_activity`

Starting from a pure parent of stated activity, the activities of the parent, of its first daughter and of the whole chain (all radioactive progeny) after a time t, with the real branching fractions (Sr-90/Y-90, Mo-99/Tc-99m, Cs-137/Ba-137m, Ge-68/Ga-68, ...).

$$
N_i(t) = \sum_j c_{ij}\,e^{-\lambda_j t},\quad c_{ij} = \frac{\sum_p b_{pi}\lambda_p c_{pj}}{\lambda_i - \lambda_j}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `nuclide` | in | the parent nuclide (Sr-90, Cs-137, Mo-99, Ge-68, W-188, Ru-106, Ce-144, Ba-140, Te-132, Sn-113, Sr-82, Zr-95, Sb-125, Pb-212, Te-131m, I-133, Pd-103, Ce-134, Ba-131, Hg-195m, Sr-91) | — |
| `A0` | in | activity of the pure parent at t = 0 | 1/s |
| `t` | in | decay time | s |
| `A_parent` | out | activity of the parent at t | 1/s |
| `A_daughter` | out | activity of the first daughter at t (Y-90, Ba-137m, Tc-99m, Ga-68, Re-188, Rh-106, Pr-144, La-140, I-132, In-113m, Rb-82, Nb-95, Te-125m, Bi-212, I-131, Xe-133, Rh-103m, La-134, Cs-131, Hg-195, Y-91m) | 1/s |
| `A_total` | out | total activity of the parent and all radioactive progeny | 1/s |
| `ratio` | out | daughter-to-parent activity ratio | — |

**Applies when:** pure parent at t = 0; t >= 0; branching fractions and half-lives of radioactivedecay 0.6.1.

```shell
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

**What the worked example means:** This calculation determines the activities of a radionuclide and its progeny after a specific decay time. Starting with an initial parent activity of 3.7e+10 (1/s), the model uses the Bateman solution to compute the remaining parent activity, which is now 2.87499e+10 1/s, and the grown-in activity of the first daughter, now 2.55061e+10 1/s. The total activity for all radioactive members in the chain is reported as 5.4256e+10 1/s, while the ratio of daughter to parent activity is given by 0.887172. These values reflect the natural decay kinetics without external removal or production.

<a id="nuc_irradiation_time_for_saturation"></a>

#### Irradiation time to reach a fraction of saturation activity — `nuc_irradiation_time_for_saturation`

How long a target must be irradiated (constant flux) for the induced activity to reach a stated fraction of its saturation value, in time and in product half-lives.

$$
t_{irr} = -\frac{T_{1/2}}{\ln 2}\ln(1-f)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T_half` | in | half-life of the product | s |
| `f` | in | fraction of the saturation activity (0 < f < 1) | — |
| `t_irr` | out | irradiation time | s |
| `n_half` | out | irradiation time in half-lives | — |

**Applies when:** constant flux; 0 < f < 1.

```shell
$ nuclear-engineering solve nuc_irradiation_time_for_saturation "T_half=15 h" f=0.9
t_irr = 179384.1171 s
n_half = 3.321928095
  by: Irradiation time to reach a fraction of saturation activity  (1 - e^(-lambda t) = f -> t = -ln(1 - f)/lambda)
  T_half = 54000 s
  f = 0.9
  assumes: constant flux; 0 < f < 1
```

**What the worked example means:** This calculation determines the precise duration a target must be exposed to a constant neutron flux to achieve a specific fraction of its maximum possible induced activity. Given a product half-life of 15 h and a desired saturation fraction of 0.9, the required irradiation time is computed as 179384 s. This result indicates that the process takes 3.32193 half-lives, providing engineers with a clear schedule for production planning while accounting for the exponential approach to saturation limits.

<a id="nuc_mass_for_activity"></a>

#### Mass and number of atoms of a radionuclide with a given activity — `nuc_mass_for_activity`

The mass and the number of atoms of a named radionuclide that has a stated activity (how much material a source of that strength contains).

$$
m = \frac{A\,T_{1/2}}{\ln 2}\,\frac{M}{N_A}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `nuclide` | in | the radionuclide, written like Co-60, Cs-137, Tc-99m, U-235 | — |
| `A` | in | activity (Bq, or Ci) | 1/s |
| `m` | out | mass of the nuclide | kg |
| `m_ug` | out | mass in micrograms | — |
| `N` | out | number of atoms | — |

**Applies when:** the nuclide is in the table; A > 0.

```shell
$ nuclear-engineering solve nuc_mass_for_activity nuclide=Tc-99m "A=370000000 (1/s)"
m = 1.898400765e-12 kg
m_ug = 0.001898400765
N = 1.155884381e+13
  by: Mass and number of atoms of a radionuclide with a given activity  (N = A T_half / ln 2, m = N M / N_A)
  A = 370000000 1/s
  nuclide = Tc-99m
  assumes: the nuclide is in the table; A > 0
```

**What the worked example means:** This calculation determines the physical mass and total number of atoms required to produce a specific radioactivity level for the given radionuclide. Using the activity input and the nuclide's half-life shows that the source contains 1.8984e-12 kg of material, which corresponds to 1.15588e+13 radioactive atoms. This allows practitioners to understand how much actual substance is present in a source defined by its decay rate rather than its weight.

<a id="nuc_activation_activity"></a>

#### Neutron activation: activity after irradiation and decay — `nuc_activation_activity`

The activity induced in a target by a neutron flux: at the end of irradiation, after a decay time, at saturation, and the saturation fraction reached (target depletion neglected).

$$
A = N\sigma\phi\left(1-e^{-\lambda t_{irr}}\right)e^{-\lambda t_d}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass of the target element | kg |
| `M` | in | molar mass of the target element | kg/mol |
| `theta` | in | isotopic abundance of the target nuclide | — |
| `sigma` | in | activation cross section (barns) | m^2 |
| `phi` | in | neutron flux | 1/(m^2*s) |
| `T_half` | in | half-life of the product | s |
| `t_irr` | in | irradiation time | s |
| `t_d` | in | decay time after irradiation (0 when omitted) (optional) | s |
| `A_eoi` | out | activity at the end of irradiation | 1/s |
| `A_d` | out | activity after the decay time | 1/s |
| `A_sat` | out | saturation activity N sigma phi | 1/s |
| `S` | out | saturation fraction 1 - e^(-lambda t_irr) | — |

**Applies when:** constant flux; target depletion and product burn-up neglected; 0 < theta <= 1.

```shell
$ nuclear-engineering solve nuc_activation_activity "m=1 g" "M=58.933195 g/mol" theta=1 "sigma=3.718e-27 m^2" "phi=1e+13 1/(cm^2*s)" "T_half=166346024.4 s" "t_irr=2592000 s"
A_eoi = 4.081362062e+10 1/s
A_d = 4.081362062e+10 1/s
A_sat = 3.79927125e+12 1/s
S = 0.01074248663
  by: Neutron activation: activity after irradiation and decay  (A(t_irr) = N sigma phi (1 - e^(-lambda t_irr)), N = m N_A theta / M; then A e^(-lambda t_d))
  m = 0.001 kg
  M = 0.058933195 kg/mol
  theta = 1
  sigma = 3.718e-27 m^2
  phi = 1e+17 1/(m^2*s)
  T_half = 166346024.4 s
  t_irr = 2592000 s
  assumes: constant flux; target depletion and product burn-up neglected; 0 < theta <= 1
```

**What the worked example means:** The calculated activity after irradiation and decay is 4.08136e+10 1/s, reaching a saturation level of 3.79927e+12 1/s.

<a id="nuc_generator_daughter_activity"></a>

#### Radionuclide generator: daughter activity after elution (with branching) — `nuc_generator_daughter_activity`

The daughter activity available in a generator (Mo-99/Tc-99m, Ge-68/Ga-68, Sr-82/Rb-82 ...) a time t after the last complete elution, with the branching fraction to the daughter, plus when the daughter activity peaks and its peak value.

$$
A_D(t) = b\,A_{P0}\frac{\lambda_D}{\lambda_D-\lambda_P}\left(e^{-\lambda_P t}-e^{-\lambda_D t}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A_P0` | in | parent activity at the elution | 1/s |
| `T_P` | in | half-life of the parent | s |
| `T_D` | in | half-life of the daughter | s |
| `b` | in | fraction of parent decays that produce the daughter (0 < b <= 1, e.g. 0.8773 for Mo-99 -> Tc-99m) | — |
| `t` | in | time since the elution | s |
| `A_D` | out | daughter activity at t | 1/s |
| `A_P` | out | parent activity at t | 1/s |
| `ratio` | out | A_D / A_P | — |
| `t_max` | out | time of maximum daughter activity | s |
| `A_D_max` | out | maximum daughter activity | 1/s |

**Applies when:** daughter fully removed at the elution; 0 < b <= 1.

```shell
$ nuclear-engineering solve nuc_generator_daughter_activity "A_P0=3.7e+10 (1/s)" "T_P=65.94 h" "T_D=6.015 h" b=0.8773 "t=24 h"
A_D = 2.550612488e+10 1/s
A_P = 2.874992172e+10 1/s
ratio = 0.8871719766
t_max = 82312.61471 s
A_D_max = 2.552513612e+10 1/s
  by: Radionuclide generator: daughter activity after elution (with branching)  (A_D(t) = b A_P0 lambda_D/(lambda_D - lambda_P) (e^(-lambda_P t) - e^(-lambda_D t)) (40 digits); t_max = ln(lambda_D/lambda_P)/(lambda_D - lambda_P))
  A_P0 = 3.7e+10 1/s
  T_P = 237384 s
  T_D = 21654 s
  b = 0.8773
  t = 86400 s
  assumes: daughter fully removed at the elution; 0 < b <= 1
```

**What the worked example means:** This calculation determines the activity of a radioactive daughter nuclide as it grows in from its parent following an elution event. Starting with a parent activity of 3.7e+10 (1/s) and assuming the daughter was completely removed at time zero, the model tracks how the daughter accumulates over a period of 24 h. The result shows that the daughter activity is currently 2.55061e+10 1/s, which represents a ratio of 0.887172 relative to the remaining parent activity of 2.87499e+10 1/s. This quantitative relationship helps predict when the generator will yield optimal amounts of the desired isotope.

<a id="nuc_three_member_chain"></a>

#### Three-member decay chain with initial activities (Bateman) — `nuc_three_member_chain`

Activities of the three radioactive members of a chain 1 -> 2 -> 3 after a time t, from their half-lives and the initial activity of every member (the daughters need not start at zero).

$$
N_3 = N_{30}e^{-\lambda_3 t} + \frac{\lambda_2 N_{20}}{\lambda_3-\lambda_2}(e^{-\lambda_2 t}-e^{-\lambda_3 t}) + \lambda_1\lambda_2 N_{10}\sum_{i}\frac{e^{-\lambda_i t}}{\prod_{j\ne i}(\lambda_j-\lambda_i)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T1` | in | half-life of member 1 (the parent) | s |
| `T2` | in | half-life of member 2 | s |
| `T3` | in | half-life of member 3 | s |
| `A10` | in | initial activity of member 1 | 1/s |
| `A20` | in | initial activity of member 2 (0 if none) | 1/s |
| `A30` | in | initial activity of member 3 (0 if none) | 1/s |
| `t` | in | elapsed time | s |
| `A1` | out | activity of member 1 | 1/s |
| `A2` | out | activity of member 2 | 1/s |
| `A3` | out | activity of member 3 | 1/s |
| `A_total` | out | total activity | 1/s |

**Applies when:** every decay of 1 gives 2 and of 2 gives 3 (no branching); the half-lives differ; t >= 0.

```shell
$ nuclear-engineering solve nuc_three_member_chain "T1=100 s" "T2=10 s" "T3=1 s" "A10=1000 1/s" "A20=0 1/s" "A30=0 1/s" "t=50 s"
A1 = 707.1067812 1/s
A2 = 750.9519791 1/s
A3 = 755.0300574 1/s
A_total = 2213.088818 1/s
  by: Three-member decay chain with initial activities (Bateman)  (Bateman solution with the initial numbers N_i0 = A_i0 / lambda_i, in 40-digit arithmetic)
  T1 = 100 s
  T2 = 10 s
  T3 = 1 s
  A10 = 1000 1/s
  A20 = 0 1/s
  A30 = 0 1/s
  t = 50 s
  assumes: every decay of 1 gives 2 and of 2 gives 3 (no branching); the half-lives differ; t >= 0
```

**What the worked example means:** This calculation determines the radioactive activities of a three-member decay chain at time 50 s. It uses the Bateman equations, converting initial activities 1000 1/s, 0 1/s, and 0 1/s into initial atom counts using their respective half-lives 100 s, 10 s, and 1 s. The resulting activities are 707.107 1/s for the parent, 750.952 1/s for the intermediate, and 755.03 1/s for the final daughter. This method accounts for non-zero starting conditions in the daughters, providing a precise snapshot of the nuclear inventory at the specified elapsed time.

<a id="nuc_daughter_max_activity_time"></a>

#### Time of maximum daughter activity — `nuc_daughter_max_activity_time`

When the activity of a radioactive daughter, growing from a pure parent, reaches its maximum, and how large it is then (it equals the parent's activity at that moment).

$$
t_{max} = \frac{\ln(\lambda_D/\lambda_P)}{\lambda_D - \lambda_P}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T_P` | in | half-life of the parent | s |
| `T_D` | in | half-life of the daughter | s |
| `A_P0` | in | initial parent activity (optional: gives the maximum daughter activity) (optional) | 1/s |
| `t_max` | out | time of maximum daughter activity | s |
| `ratio_max` | out | maximum daughter activity / initial parent activity | — |
| `t_max_half_lives` | out | t_max in daughter half-lives | — |
| `A_D_max` | out | maximum daughter activity (when A_P0 is given) | 1/s |

**Applies when:** pure parent at t = 0; every parent decay gives the daughter.

```shell
$ nuclear-engineering solve nuc_daughter_max_activity_time "T_P=65.94 h" "T_D=6.015 h"
t_max = 82312.61471 s
ratio_max = 0.7863542047
t_max_half_lives = 3.801266034
  by: Time of maximum daughter activity  (dA_D/dt = 0: t_max = ln(lambda_D/lambda_P)/(lambda_D - lambda_P) (1/lambda when equal); A_D(t_max) = A_P(t_max))
  T_P = 237384 s
  T_D = 21654 s
  assumes: pure parent at t = 0; every parent decay gives the daughter
```

**What the worked example means:** The daughter activity peaks at 82312.6 s, which equals 3.80127 daughter half-lives, reaching a ratio of 0.786354 relative to the initial parent.

<a id="nuc_equilibrium_ratio"></a>

#### Transient and secular equilibrium: activity ratio and approach — `nuc_equilibrium_ratio`

The daughter-to-parent activity and atom ratios once equilibrium is reached (transient; close to 1 in secular equilibrium) and the time after which the activity ratio is within 1 % of it, starting from pure parent.

$$
\frac{A_D}{A_P} = \frac{\lambda_D}{\lambda_D-\lambda_P}\left(1 - e^{-(\lambda_D-\lambda_P)t}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T_P` | in | half-life of the parent | s |
| `T_D` | in | half-life of the daughter (shorter than the parent's) | s |
| `ratio_A` | out | equilibrium activity ratio A_D/A_P = T_P/(T_P - T_D) | — |
| `ratio_N` | out | equilibrium atom ratio N_D/N_P | — |
| `t_99` | out | time to reach 99 % of the equilibrium activity ratio | s |

**Applies when:** T_P > T_D; pure parent at t = 0; every parent decay gives the daughter.

```shell
$ nuclear-engineering solve nuc_equilibrium_ratio "T_P=237384 s" "T_D=21654 s"
ratio_A = 1.100375469
ratio_N = 0.1003754693
t_99 = 158306.6854 s
  by: Transient and secular equilibrium: activity ratio and approach  (A_D/A_P -> lambda_D/(lambda_D - lambda_P); the ratio approaches it as 1 - exp(-(lambda_D - lambda_P) t))
  T_P = 237384 s
  T_D = 21654 s
  assumes: T_P > T_D; pure parent at t = 0; every parent decay gives the daughter
```

**What the worked example means:** This calculation determines the steady-state relationship between parent and daughter activities using the half-lives 237384 s for the parent and 21654 s for the daughter. The resulting equilibrium activity ratio is 1.10038, while the corresponding atom ratio is 0.100375. The time required for the system to approach within one percent of this equilibrium, starting from pure parent material, is calculated as 158307 s. These values indicate how the decay chain behaves once transient or secular equilibrium is established under the given conditions.

### Detectors and counting (15)

<a id="nuc_compton_edge"></a>

#### Compton edge and backscatter peak of a gamma line — `nuc_compton_edge`

Where a gamma line's Compton edge (maximum electron energy) and backscatter peak (energy of a photon scattered through 180 degrees) appear in a detector's spectrum.

$$
T_{max} = \frac{2E^2}{m_ec^2 + 2E},\quad E_{bs} = \frac{E}{1+2E/m_ec^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `E` | in | gamma-ray energy | J |
| `T_max` | out | Compton edge: maximum electron recoil energy (MeV) | MeV |
| `E_bs` | out | backscatter peak: energy of the photon scattered through 180 degrees (MeV) | MeV |

**Applies when:** free electron at rest.

```shell
$ nuclear-engineering solve nuc_compton_edge "E=662 keV"
T_max = 0.4776504094 MeV
E_bs = 0.1843495906 MeV
  by: Compton edge and backscatter peak of a gamma line  (E_bs = E/(1 + 2E/(m_e c^2)), T_max = E - E_bs)
  E = 1.060640932e-13 J
  assumes: free electron at rest
```

**What the worked example means:** This calculation determines the positions of two key features in a gamma-ray detector spectrum. Given an incident gamma-ray energy of 662 keV, it computes the maximum energy transferred to an electron, known as the Compton edge (0.47765 MeV), and the energy of a photon scattered directly backward, known as the backscatter peak (0.18435 MeV). These values help identify where specific scattering events appear relative to the full-energy peak.

<a id="nuc_count_time_for_precision"></a>

#### Counting time for a target precision of the net count rate — `nuc_count_time_for_precision`

The total counting time (optimally split between sample and background) needed for the net count rate to have a stated relative standard deviation, and the two counting times.

$$
T = \frac{(\sqrt{R_g}+\sqrt{R_b})^2}{(R_g-R_b)^2\,r^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Rg` | in | gross count rate | 1/s |
| `Rb` | in | background count rate | 1/s |
| `r` | in | target relative standard deviation of the net count rate (e.g. 1%) | — |
| `T` | out | total counting time | s |
| `tg` | out | gross counting time | s |
| `tb` | out | background counting time | s |

**Applies when:** Poisson statistics; optimal split tg/tb = sqrt(Rg/Rb); Rg > Rb.

```shell
$ nuclear-engineering solve nuc_count_time_for_precision "Rg=100 (1/s)" "Rb=25 (1/s)" r=0.01
T = 400 s
tg = 266.6666667 s
tb = 133.3333333 s
  by: Counting time for a target precision of the net count rate  (T = (sqrt(Rg) + sqrt(Rb))^2 / ((Rg - Rb)^2 r^2))
  Rg = 100 1/s
  Rb = 25 1/s
  r = 0.01
  assumes: Poisson statistics; optimal split tg/tb = sqrt(Rg/Rb); Rg > Rb
```

**What the worked example means:** This calculation determines the optimal total measurement duration needed to estimate the net count rate with a specified relative standard deviation. Given a gross count rate of 100 (1/s) and a background count rate of 25 (1/s), the algorithm computes that a total time of 400 s is necessary. This period is split into 266.667 s for measuring the sample plus background and 133.333 s for measuring only the background, ensuring the most efficient use of time under Poisson statistics assumptions.

<a id="nuc_counts_during_decay"></a>

#### Counts recorded from a decaying source (decay during counting) — `nuc_counts_during_decay`

The expected number of counts recorded from a source that decays during the count, the mean activity during the count and the decay-correction factor that converts the mean activity to the activity at the start of counting.

$$
N = \varepsilon\,y\,A_0e^{-\lambda t_d}\frac{1-e^{-\lambda t_c}}{\lambda}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A0` | in | activity at the reference time | 1/s |
| `T_half` | in | half-life | s |
| `tc` | in | counting time | s |
| `eff` | in | detection efficiency (fraction of emitted radiations counted) | — |
| `y` | in | emission probability per decay (1 when every decay is counted) | — |
| `td` | in | delay from the reference time to the start of counting (0 when omitted) (optional) | s |
| `N` | out | expected counts | — |
| `A_mean` | out | mean activity during the count | 1/s |
| `F` | out | decay-correction factor: activity at the start of counting / mean activity | — |

**Applies when:** 0 < eff <= 1; 0 < y <= 1; no dead-time losses.

```shell
$ nuclear-engineering solve nuc_counts_during_decay "A0=100000 (1/s)" "T_half=6586.2 s" "tc=1 h" eff=0.2 y=0.967
N = 57953848.46
A_mean = 83238.32078 1/s
F = 1.201369742
  by: Counts recorded from a decaying source (decay during counting)  (N = eff y A0 e^(-lambda td) (1 - e^(-lambda tc))/lambda; F = lambda tc/(1 - e^(-lambda tc)))
  A0 = 100000 1/s
  T_half = 6586.2 s
  tc = 3600 s
  eff = 0.2
  y = 0.967
  assumes: 0 < eff <= 1; 0 < y <= 1; no dead-time losses
```

**What the worked example means:** This calculation determines the total detected events from a radioactive source that decays while being measured. It uses the initial activity, half-life, counting duration, and detection efficiency to compute the expected counts, which are 5.79538e+07. The average activity over the measurement window is 83238.3 1/s, and the factor needed to adjust this average back to the starting activity is 1.20137. This helps practitioners correct for activity loss during long measurements.

<a id="nuc_currie_detection_limits"></a>

#### Currie critical level, detection limit and minimum detectable activity — `nuc_currie_detection_limits`

Currie's (1968) decision threshold L_C = k sqrt(2B) and detection limit L_D = k^2 + 2 L_C (net counts, paired blank, alpha = beta = 0.05, k = 1.645) from the background counts B, and with the counting time, efficiency and emission yield the minimum detectable activity L_D/(eff y t).

$$
L_C = k\sqrt{2B},\quad L_D = k^2 + 2k\sqrt{2B},\quad MDA = \frac{L_D}{\varepsilon\,y\,t}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `B` | in | background counts in the counting time | — |
| `t` | in | counting time (optional, for the MDA) (optional) | s |
| `eff` | in | counting efficiency (optional, with t) (optional) | — |
| `y` | in | emission yield per decay (default 1) (optional) | — |
| `L_C` | out | critical level (net counts) | — |
| `L_D` | out | detection limit (net counts) | — |
| `MDA` | out | minimum detectable activity (Bq) | 1/s |

**Applies when:** paired blank (same counting time); alpha = beta = 0.05; Gaussian approximation of the counts.

```shell
$ nuclear-engineering solve nuc_currie_detection_limits B=100
L_C = 23.26174307
L_D = 49.2290296
  by: Currie critical level, detection limit and minimum detectable activity  (L_C = k sqrt(2B), L_D = k^2 + 2 k sqrt(2B), k = z(0.95) = 1.645; MDA = L_D/(eff y t))
  B = 100
  assumes: paired blank (same counting time); alpha = beta = 0.05; Gaussian approximation of the counts
```

In the worked example, B = 100 go through the equation above and give L_C = 23.26174307, L_D = 49.2290296. It applies only when paired blank (same counting time); alpha = beta = 0.05; Gaussian approximation of the counts: outside that it says what is wrong instead of returning a number.

<a id="nuc_dead_time_nonparalyzable"></a>

#### Dead-time correction, non-paralyzable detector — `nuc_dead_time_nonparalyzable`

The true event rate from the observed count rate of a non-paralyzable (non-extendable) detector with a stated dead time, the fraction of events lost and the correction factor.

$$
n = \frac{m}{1 - m\tau}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | observed count rate (cps, cpm) | 1/s |
| `tau` | in | dead time (e.g. us) | s |
| `n` | out | true event rate | 1/s |
| `loss` | out | fraction of events lost | — |
| `corr` | out | correction factor n/m | — |

**Applies when:** non-paralyzable (non-extendable) dead time; m tau < 1.

```shell
$ nuclear-engineering solve nuc_dead_time_nonparalyzable "m=50000 (1/s)" "tau=5 us"
n = 66666.66667 1/s
loss = 0.25
corr = 1.333333333
  by: Dead-time correction, non-paralyzable detector  (n = m / (1 - m tau))
  m = 50000 1/s
  tau = 5e-06 s
  assumes: non-paralyzable (non-extendable) dead time; m tau < 1
```

**What the worked example means:** This calculation determines the true event rate from an observed count rate by correcting for dead time in a non-paralyzable detector. Using an observed rate of 50000 (1/s) and a dead time of 5 us, the resulting true rate is 66666.7 1/s. The process reveals that 0.25 of events were missed, requiring a multiplication by the correction factor 1.33333 to recover the actual activity.

<a id="nuc_dead_time_paralyzable"></a>

#### Dead-time correction, paralyzable detector — `nuc_dead_time_paralyzable`

The true event rate from the observed count rate of a paralyzable (extendable) detector, on the stated branch of the rate curve (below or above the maximum), with the fraction lost and the largest observable count rate.

$$
m = n\,e^{-n\tau}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | observed count rate | 1/s |
| `tau` | in | dead time | s |
| `branch` | in | branch of the rate curve: low (true rate below 1/tau) or high (above it) | — |
| `n` | out | true event rate | 1/s |
| `loss` | out | fraction of events lost | — |
| `m_max` | out | largest observable count rate 1/(e tau) | 1/s |

**Applies when:** paralyzable (extendable) dead time; m tau <= 1/e; the branch is stated (two true rates give the same m).

```shell
$ nuclear-engineering solve nuc_dead_time_paralyzable "m=50000 (1/s)" "tau=5 us" branch=low
n = 71480.59124 1/s
loss = 0.3005094231
m_max = 73575.88823 1/s
  by: Dead-time correction, paralyzable detector  (m = n e^(-n tau) solved for n on the stated branch (bisection in x = n tau))
  m = 50000 1/s
  tau = 5e-06 s
  branch = low
  assumes: paralyzable (extendable) dead time; m tau <= 1/e; the branch is stated (two true rates give the same m)
```

**What the worked example means:** This calculation determines the true event rate from an observed count rate for a paralyzable detector. By solving the relationship between observed and true rates on the specified branch, it yields a true rate of 71480.6 1/s. The result indicates that 0.300509 of events were lost due to dead time extension, with the system's theoretical peak sensitivity capped at 73575.9 1/s.

<a id="nuc_count_rate_point_source"></a>

#### Expected count rate from a point source on a detector's axis — `nuc_count_rate_point_source`

The count rate a detector of stated intrinsic efficiency records from a point source on its axis, from the activity, the emission probability per decay and the geometric efficiency of the detector face.

$$
R = A\,y\,\varepsilon_{int}\,\frac{1}{2}\left(1-\frac{d}{\sqrt{d^2+a^2}}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A` | in | source activity | 1/s |
| `y` | in | emission probability of the counted radiation per decay | — |
| `d` | in | source-to-face distance | m |
| `a` | in | radius of the detector face | m |
| `eps_int` | in | intrinsic efficiency (counts per radiation reaching the face) | — |
| `R` | out | expected count rate | 1/s |
| `g` | out | geometric efficiency | — |
| `R_cpm` | out | expected count rate in counts per minute | — |

**Applies when:** point source on the axis; no dead time, no attenuation between source and detector.

```shell
$ nuclear-engineering solve nuc_count_rate_point_source "A=37000 (1/s)" y=0.851 "d=10 cm" "a=2.54 cm" eps_int=0.6
R = 290.7185774 1/s
g = 0.01538828603
R_cpm = 17443.11464
  by: Expected count rate from a point source on a detector's axis  (R = A y g eps_int, g = (1 - d/sqrt(d^2 + a^2))/2)
  A = 37000 1/s
  y = 0.851
  d = 0.1 m
  a = 0.0254 m
  eps_int = 0.6
  assumes: point source on the axis; no dead time, no attenuation between source and detector
```

**What the worked example means:** This calculation determines the expected count rate for a detector measuring a point source positioned on its central axis. By combining the source activity, emission probability, geometric efficiency of the detector face, and intrinsic efficiency, the model predicts a count rate of 290.719 1/s. This result indicates how many events the detector should register per second under ideal conditions where no attenuation or dead time losses occur.

<a id="nuc_decay_curve_fit"></a>

#### Half-life from a measured decay curve (weighted log-linear fit) — `nuc_decay_curve_fit`

The decay constant, half-life, initial count and the standard error of the half-life from counts measured at several times (equal counting intervals), by a Poisson-weighted least-squares fit of ln C against t.

$$
\min\sum_i C_i\left(\ln C_i - \ln C_0 + \lambda t_i\right)^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `t` | in | measurement times, in the unit t_unit | — |
| `C` | in | counts recorded at those times (equal counting intervals) | — |
| `t_unit` | in | unit of the times in the list: s, min, h, d or y | — |
| `lambda_` | out | fitted decay constant | 1/s |
| `T_half` | out | fitted half-life | s |
| `C0` | out | fitted count at t = 0 | — |
| `sigma_T` | out | standard error of the half-life (Poisson weights) | s |

**Applies when:** a single radionuclide; counts positive and decreasing overall; weights w_i = C_i (Var ln C = 1/C).

```shell
$ nuclear-engineering solve nuc_decay_curve_fit "t=[0, 1, 2, 3, 4]" "C=[10000, 7100, 5000, 3540, 2500]" t_unit=h
lambda_ = 9.628452267e-05 1/s
T_half = 7198.947051 s
C0 = 10012.57801
sigma_T = 94.40471674 s
  by: Half-life from a measured decay curve (weighted log-linear fit)  (weighted least squares of ln C_i = ln C0 - lambda t_i with weights C_i; sigma_lambda^2 = S/(S Stt - St^2))
  t_unit = h
  assumes: a single radionuclide; counts positive and decreasing overall; weights w_i = C_i (Var ln C = 1/C)
```

**What the worked example means:** This calculation determines the decay constant and half-life for a single radionuclide by performing a weighted least-squares fit on the natural logarithm of measured counts plotted against time. The weights used are proportional to the count rates themselves, which reflects Poisson statistics where the variance of the log-counts is inversely related to the count number. The result yields the fitted initial count 10012.6, the decay constant 9.62845e-05 1/s, and the half-life 7198.95 s, along with its uncertainty 94.4047 s. This method assumes that background radiation has been subtracted and that detector dead time effects are negligible.

<a id="nuc_flux_from_foil_activation"></a>

#### Neutron flux from a measured foil activation — `nuc_flux_from_foil_activation`

The neutron flux that irradiated an activation foil, from the foil's measured activity a decay time after an irradiation of known length, its mass, molar mass, isotopic abundance and activation cross section; with the saturation activity and the reaction rate per atom.

$$
\phi = \frac{A_d\,e^{\lambda t_d}}{N\sigma\left(1-e^{-\lambda t_{irr}}\right)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A_d` | in | foil activity measured at the count | 1/s |
| `m` | in | foil mass (target element) | kg |
| `M` | in | molar mass of the target element | kg/mol |
| `theta` | in | isotopic abundance of the target nuclide | — |
| `sigma` | in | activation cross section (barns) | m^2 |
| `T_half` | in | half-life of the product | s |
| `t_irr` | in | irradiation time | s |
| `t_d` | in | time from the end of irradiation to the count | s |
| `phi` | out | neutron flux | 1/(m^2*s) |
| `A_sat` | out | saturation activity | 1/s |
| `R_atom` | out | reaction rate per target atom sigma phi (1/s) | 1/s |

**Applies when:** constant flux during the irradiation; thin foil (no self-shielding), no target depletion.

```shell
$ nuclear-engineering solve nuc_flux_from_foil_activation "A_d=20000 (1/s)" "m=0.1 g" "M=196.97 g/mol" theta=1 "sigma=9.865e-27 m^2" "T_half=232848 s" "t_irr=1 h" "t_d=30 min"
phi = 6.254299925e+11 1/(m^2*s)
A_sat = 1886368.828 1/s
R_atom = 6.169866876e-15 1/s
  by: Neutron flux from a measured foil activation  (A_sat = A_d e^(lambda t_d)/(1 - e^(-lambda t_irr)); phi = A_sat/(N sigma), N = m N_A theta/M)
  A_d = 20000 1/s
  m = 0.0001 kg
  M = 0.19697 kg/mol
  theta = 1
  sigma = 9.865e-27 m^2
  T_half = 232848 s
  t_irr = 3600 s
  t_d = 1800 s
  assumes: constant flux during the irradiation; thin foil (no self-shielding), no target depletion
```

**What the worked example means:** This calculation determines the neutron flux that irradiated an activation foil by analyzing its induced radioactivity. The result, 6.2543e+11 1/(m^2*s), represents the particle flow rate per unit area necessary to produce the observed decay behavior. It relies on converting the measured activity at counting time, 20000 (1/s), back to the saturation activity, 1.88637e+06 1/s, using the half-life and timing parameters. This saturation value is then divided by the number of target atoms and the cross section to yield the final flux estimate.

<a id="nuc_count_time_split"></a>

#### Optimal division of counting time between sample and background — `nuc_count_time_split`

How to split a fixed total counting time between the sample (gross) count and the background count to minimise the standard deviation of the net count rate, and that minimum standard deviation.

$$
\frac{t_g}{t_b} = \sqrt{\frac{R_g}{R_b}},\quad \sigma_{net} = \frac{\sqrt{R_g}+\sqrt{R_b}}{\sqrt{T}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | total counting time available | s |
| `Rg` | in | gross count rate (sample plus background) | 1/s |
| `Rb` | in | background count rate | 1/s |
| `tg` | out | time for the gross count | s |
| `tb` | out | time for the background count | s |
| `sigma` | out | minimum standard deviation of the net count rate | 1/s |
| `rel` | out | relative standard deviation of the net count rate | — |

**Applies when:** Poisson counting statistics; Rg > Rb > 0.

```shell
$ nuclear-engineering solve nuc_count_time_split "T=1 h" "Rg=100 (1/s)" "Rb=25 (1/s)"
tg = 2400 s
tb = 1200 s
sigma = 0.25 1/s
rel = 0.003333333333
  by: Optimal division of counting time between sample and background  (tg/tb = sqrt(Rg/Rb); sigma_net = (sqrt(Rg) + sqrt(Rb))/sqrt(T))
  T = 3600 s
  Rg = 100 1/s
  Rb = 25 1/s
  assumes: Poisson counting statistics; Rg > Rb > 0
```

**What the worked example means:** Given a total available counting time of 1 h, the algorithm splits this period to minimize statistical uncertainty. The optimal duration for measuring the sample plus background is calculated as 2400 s, while the remaining time of 1200 s is dedicated to measuring the background alone. This allocation results in a minimum standard deviation for the net count rate of 0.25 1/s, which represents the smallest achievable random error under Poisson statistics for these specific rates.

<a id="nuc_escape_peaks"></a>

#### Single and double escape peaks of a gamma ray — `nuc_escape_peaks`

The energies of the single- and double-escape peaks (E - m_e c^2 and E - 2 m_e c^2) that pair production by a gamma ray above 1.022 MeV produces in a detector.

$$
E_{SE} = E_\gamma - m_ec^2,\quad E_{DE} = E_\gamma - 2m_ec^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `E` | in | gamma-ray energy (MeV, keV) | J |
| `single_escape_keV` | out | single-escape peak in keV | — |
| `double_escape_keV` | out | double-escape peak in keV | — |
| `annihilation_keV` | out | annihilation photon energy in keV | — |
| `E_keV` | out | full-energy peak in keV | — |

**Applies when:** E > 2 m_e c^2 = 1.022 MeV.

```shell
$ nuclear-engineering solve nuc_escape_peaks "E=2614 keV"
single_escape_keV = 2103.001049
double_escape_keV = 1592.002099
annihilation_keV = 510.9989507
E_keV = 2614
  by: Single and double escape peaks of a gamma ray  (E_SE = E - m_e c^2, E_DE = E - 2 m_e c^2 (m_e c^2 = 0.51099895069 MeV))
  E = 4.188089721e-13 J
  assumes: E > 2 m_e c^2 = 1.022 MeV
```

**What the worked example means:** This calculation determines the energy positions of spectral artifacts resulting from pair production. Given an incident gamma-ray energy of 2614 keV, it computes the full-energy peak at 2614, the single-escape peak at 2103, and the double-escape peak at 1592. These values represent the residual energy deposited in the detector when one or both annihilation photons, each with energy 510.999, escape the active volume without interacting.

<a id="nuc_solid_angle_on_axis"></a>

#### Solid angle and geometric efficiency of a disk detector (source on axis) — `nuc_solid_angle_on_axis`

The solid angle subtended by a circular detector face (or aperture) at a point source on its axis and the geometric efficiency (fraction of isotropic emissions that hit it).

$$
\Omega = 2\pi\left(1-\frac{d}{\sqrt{d^2+a^2}}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `d` | in | distance from the source to the detector face | m |
| `a` | in | radius of the detector face | m |
| `Omega` | out | solid angle (sr) | — |
| `g` | out | geometric efficiency Omega/(4 pi) | — |

**Applies when:** point isotropic source on the axis.

```shell
$ nuclear-engineering solve nuc_solid_angle_on_axis "d=10 cm" "a=2.54 cm"
Omega = 0.1933749054
g = 0.01538828603
  by: Solid angle and geometric efficiency of a disk detector (source on axis)  (Omega = 2 pi (1 - d/sqrt(d^2 + a^2)))
  d = 0.1 m
  a = 0.0254 m
  assumes: point isotropic source on the axis
```

**What the worked example means:** This calculation determines the solid angle subtended by a circular detector face at an on-axis point source. Using the distance from the source to the detector and the radius of the detector, it computes the fraction of isotropic emissions that intersect the active surface. The resulting solid angle is 0.193375, which corresponds to a geometric efficiency of 0.0153883, meaning this proportion of emitted particles or photons reaches the detector.

<a id="nuc_solid_angle_off_axis"></a>

#### Solid angle of a disk detector for an off-axis point source — `nuc_solid_angle_off_axis`

The solid angle subtended by a circular detector face at a point source displaced sideways from its axis, and the geometric efficiency.

$$
\Omega = \int_0^{2\pi}\left(1-\frac{d}{\sqrt{d^2+r_e(\varphi)^2}}\right)d\varphi
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `d` | in | distance from the source to the plane of the detector face | m |
| `a` | in | radius of the detector face | m |
| `rho0` | in | lateral offset of the source from the detector axis | m |
| `Omega` | out | solid angle (sr) | — |
| `g` | out | geometric efficiency Omega/(4 pi) | — |

**Applies when:** point isotropic source; flat circular face.

```shell
$ nuclear-engineering solve nuc_solid_angle_off_axis "d=5 cm" "a=2.5 cm" "rho0=1 cm"
Omega = 0.6370488883
g = 0.05069473979
  by: Solid angle of a disk detector for an off-axis point source  (Omega = int over azimuth of (1 - d/sqrt(d^2 + r_edge(phi)^2)) dphi (one-dimensional, mpmath tanh-sinh))
  d = 0.05 m
  a = 0.025 m
  rho0 = 0.01 m
  assumes: point isotropic source; flat circular face
```

**What the worked example means:** This calculation determines the fraction of isotropic radiation from a point source that intersects a flat circular detector face. By integrating over the azimuthal angle to account for the lateral offset, the model finds the solid angle subtended by the disk at the source position. The resulting value of 0.637049 represents this solid angle in steradians. Dividing by four pi gives the geometric efficiency of 0.0506947, which indicates the proportion of emitted particles likely to be detected under ideal conditions.

<a id="nuc_detector_efficiency_point_source"></a>

#### Total and intrinsic efficiency of a slab detector for a point source on axis — `nuc_detector_efficiency_point_source`

The total detection efficiency of a flat detector of given thickness and attenuation coefficient for a point source on its axis (every photon entering the front face crosses the thickness along its slant path), the geometric efficiency and the intrinsic efficiency.

$$
\varepsilon = \frac12\int_{\cos\theta_{max}}^{1}\left(1-e^{-\mu t/u}\right)du
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `d` | in | source-to-face distance | m |
| `a` | in | radius of the detector face | m |
| `t` | in | detector thickness | m |
| `mu` | in | total linear attenuation coefficient of the detector material | 1/m |
| `eff` | out | total efficiency (interacting photons per emitted photon) | — |
| `g` | out | geometric efficiency | — |
| `intrinsic` | out | intrinsic efficiency eff/g | — |

**Applies when:** point source on the axis; path length t/cos(theta) for every ray through the front face (side escape neglected).

```shell
$ nuclear-engineering solve nuc_detector_efficiency_point_source "d=10 cm" "a=3.81 cm" "t=7.62 cm" "mu=0.28 1/cm"
eff = 0.02915417276
g = 0.03276353471
intrinsic = 0.8898360029
  by: Total and intrinsic efficiency of a slab detector for a point source on axis  (eff = (1/2) int_{cos theta_max}^1 (1 - e^(-mu t/u)) du, cos theta_max = d/sqrt(d^2 + a^2))
  d = 0.1 m
  a = 0.0381 m
  t = 0.0762 m
  mu = 28 1/m
  assumes: point source on the axis; path length t/cos(theta) for every ray through the front face (side escape neglected)
```

**What the worked example means:** This calculation determines the total detection efficiency for a point source located on the axis of a flat slab detector. By integrating over all angles that allow photons to enter the front face, the model accounts for both the probability of a photon hitting the detector and the likelihood it interacts within the material along its slant path. The resulting total efficiency is 0.0291542, which is composed of a geometric efficiency of 0.0327635 and an intrinsic efficiency of 0.889836. These values indicate how effectively the specific setup captures and registers radiation from the source.

<a id="nuc_two_component_decay_fit"></a>

#### Two-component decay curve: amplitudes of two known half-lives — `nuc_two_component_decay_fit`

Resolve a decay curve that is the sum of two radionuclides of known half-lives into the initial count of each component (Poisson-weighted linear least squares).

$$
\min\sum_i\frac{\left(C_i - C_1e^{-\lambda_1t_i} - C_2e^{-\lambda_2t_i}\right)^2}{C_i}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `t` | in | measurement times, in the unit t_unit | — |
| `C` | in | counts recorded at those times (equal intervals) | — |
| `T1` | in | half-life of component 1 | s |
| `T2` | in | half-life of component 2 | s |
| `t_unit` | in | unit of the times in the list: s, min, h, d or y | — |
| `C1` | out | component 1 count at t = 0 | — |
| `C2` | out | component 2 count at t = 0 | — |
| `frac1` | out | fraction of the initial count due to component 1 | — |

**Applies when:** two components with known, different half-lives; weights 1/C_i (Poisson).

```shell
$ nuclear-engineering solve nuc_two_component_decay_fit "t=[0, 1, 2, 4, 8, 16]" "C=[11000, 6000, 3500, 1625, 1062, 844]" "T1=1 h" "T2=10 h" t_unit=h
C1 = 8973.699227
C2 = 1696.920978
frac1 = 0.8409726009
  by: Two-component decay curve: amplitudes of two known half-lives  (weighted linear least squares of C_i = C1 e^(-lambda1 t_i) + C2 e^(-lambda2 t_i) (2x2 normal equations))
  T1 = 3600 s
  T2 = 36000 s
  t_unit = h
  assumes: two components with known, different half-lives; weights 1/C_i (Poisson)
```

**What the worked example means:** This calculation resolves a measured decay curve into its two constituent radionuclides using known half-lives. By applying Poisson-weighted linear least squares to the count data at times [0, 1, 2, 4, 8, 16], it determines the initial activity of each component. The result shows that component one contributed 8973.7 counts and component two contributed 1696.92 counts at time zero, meaning component one accounted for 0.840973 of the total initial signal.

### Dosimetry (5)

<a id="nuc_committed_dose_coefficient"></a>

#### Committed effective dose from a dose coefficient — `nuc_committed_dose_coefficient`

The committed effective dose E(50) = e(50) I of an intake I of a radionuclide with the dose coefficient e(50) (Sv/Bq, from ICRP tables for the route and chemical form).

$$
E(50) = e(50)\,I
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `I` | in | activity taken in (Bq, kBq, MBq, Ci ...) | 1/s |
| `e` | in | dose coefficient e(50) (Sv/Bq) | J*s/kg |
| `E` | out | committed effective dose (Sv) | J/kg |
| `E_mSv` | out | committed effective dose in mSv | — |
| `I_Bq` | out | intake in Bq | 1/s |

**Applies when:** dose coefficient for the route, particle size and chemical form of the intake; e <= 1e-3 Sv/Bq.

```shell
$ nuclear-engineering solve nuc_committed_dose_coefficient "I=1000000 (1/s)" "e=2.2e-08 (J/kg)/(1/s)"
E = 0.022 J/kg
E_mSv = 22
I_Bq = 1000000 1/s
  by: Committed effective dose from a dose coefficient  (E(50) = e(50) I)
  I = 1000000 1/s
  e = 2.2e-08 J*s/kg
  assumes: dose coefficient for the route, particle size and chemical form of the intake; e <= 1e-3 Sv/Bq
```

**What the worked example means:** This calculation determines the committed effective dose, which represents the total radiation energy absorbed by the body over fifty years following an internal intake of radioactive material. Multiplying the activity taken in, 1e+06 (1/s), by the specific dose coefficient for that radionuclide and exposure route gives a result of 0.022 J/kg. This value quantifies the potential biological harm from the ingested or inhaled substance, expressed in sieverts.

<a id="nuc_committed_dose_retention"></a>

#### Committed equivalent dose to an organ from an intake (retention compartments) — `nuc_committed_dose_retention`

The number of decays in an organ over the commitment period after an intake retained in exponential compartments, and the committed equivalent dose from the energy absorbed per decay, the radiation weighting factor and the organ mass.

$$
H_T(\tau) = \frac{w_R E}{m}\,I\sum_i f_i\frac{1-e^{-(\lambda_p+\lambda_{b,i})\tau}}{\lambda_p+\lambda_{b,i}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `I` | in | activity taken into the organ (intake reaching it) | 1/s |
| `f` | in | fractions of the intake retained in each compartment (sum <= 1) | — |
| `Tb` | in | biological half-lives of the compartments, in the unit t_unit | — |
| `t_unit` | in | unit of the times in the list: s, min, h, d or y | — |
| `T_p` | in | physical half-life | s |
| `E` | in | energy absorbed in the organ per decay (e.g. MeV) | J |
| `w_R` | in | radiation weighting factor (1 for photons and electrons, 20 for alpha particles) | — |
| `m` | in | organ mass | kg |
| `tau` | in | commitment period (50 y for adults) | s |
| `U` | out | number of decays in the organ during tau | — |
| `H` | out | committed equivalent dose (Sv) | J/kg |
| `H_mSv` | out | committed equivalent dose in mSv | — |

**Applies when:** instantaneous intake; retention R(t) = sum f_i e^(-lambda_b,i t); all energy per decay absorbed in the organ as stated.

```shell
$ nuclear-engineering solve nuc_committed_dose_retention "I=1000000 (1/s)" "f=[0.3]" "Tb=[80]" t_unit=d "T_p=692988.48 s" "E=0.18 MeV" w_R=1 "m=20 g" "tau=1577880000 s"
U = 2.726007069e+11
H = 0.3930790347 J/kg
H_mSv = 393.0790347
  by: Committed equivalent dose to an organ from an intake (retention compartments)  (U = I sum_i f_i (1 - e^(-k_i tau))/k_i with k_i = lambda_p + ln 2/Tb_i; H = U E w_R / m)
  I = 1000000 1/s
  T_p = 692988.48 s
  E = 2.883917941e-14 J
  w_R = 1
  m = 0.02 kg
  tau = 1577880000 s
  t_unit = d
  assumes: instantaneous intake; retention R(t) = sum f_i e^(-lambda_b,i t); all energy per decay absorbed in the organ as stated
```

**What the worked example means:** This calculation determines the total number of radioactive decays occurring within the organ during the commitment period, which equals 2.72601e+11. This value is derived from the initial activity taken into the organ, 1e+06 (1/s), and the retention fractions, [0.3], combined with the biological half-lives, [80], and physical half-life, 692988 s. The resulting committed equivalent dose, 0.393079 J/kg, reflects the energy absorbed per decay, 0.18 MeV, scaled by the radiation weighting factor, 1, and divided by the organ mass, 20 g.

<a id="nuc_effective_dose"></a>

#### Effective dose from organ equivalent doses and tissue weighting factors — `nuc_effective_dose`

The effective dose E = sum w_T H_T from the equivalent doses of the irradiated tissues and their tissue weighting factors (stated; ICRP 103: 0.12 for red marrow, colon, lung, stomach, breast and the remainder, 0.08 gonads, 0.04 bladder, oesophagus, liver, thyroid, 0.01 bone surface, brain, salivary glands, skin).

$$
E = \sum_T w_T\,H_T
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `H` | in | equivalent dose of each tissue in mSv | — |
| `w` | in | tissue weighting factor of each tissue (as stated) | — |
| `E_mSv` | out | effective dose in mSv | — |
| `E` | out | effective dose (Sv) | J/kg |
| `w_sum` | out | sum of the weighting factors given | — |

**Applies when:** one weighting factor per tissue, 0 < w_T <= 1, sum at most 1; tissues not listed received no dose.

```shell
$ nuclear-engineering solve nuc_effective_dose "H=[10, 5, 2]" "w=[0.12, 0.04, 0.01]"
E_mSv = 1.42
E = 0.00142 J/kg
w_sum = 0.17
  by: Effective dose from organ equivalent doses and tissue weighting factors  (E = sum_T w_T H_T)
  assumes: one weighting factor per tissue, 0 < w_T <= 1, sum at most 1; tissues not listed received no dose
```

**What the worked example means:** This calculation determines the effective dose, denoted as 1.42, which represents the overall stochastic health risk from radiation exposure. It is derived by multiplying each tissue's equivalent dose, such as [10, 5, 2], by its specific tissue weighting factor, like [0.12, 0.04, 0.01], and summing these products. The result provides a single metric in millisieverts that allows for comparison of risks across different exposure scenarios, accounting for the varying sensitivity of organs to radiation damage.

<a id="nuc_effective_half_life"></a>

#### Effective half-life (physical and biological removal) — `nuc_effective_half_life`

The effective half-life of a radionuclide in the body or another system that removes it biologically while it decays, the effective decay constant and (with the initial activity) the total number of decays.

$$
\frac{1}{T_{eff}} = \frac{1}{T_p} + \frac{1}{T_b}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T_p` | in | physical half-life | s |
| `T_b` | in | biological half-life | s |
| `A0` | in | initial activity in the body (optional) (optional) | 1/s |
| `T_eff` | out | effective half-life | s |
| `lambda_eff` | out | effective removal constant | 1/s |
| `N_decays` | out | total number of decays in the body (cumulated activity, when A0 is given) | — |

**Applies when:** single-exponential biological removal.

```shell
$ nuclear-engineering solve nuc_effective_half_life "T_p=692988.48 s" "T_b=6912000 s"
T_eff = 629841.3714 s
lambda_eff = 1.100510719e-06 1/s
  by: Effective half-life (physical and biological removal)  (1/T_eff = 1/T_p + 1/T_b; cumulated activity A0 T_eff/ln 2)
  T_p = 692988.48 s
  T_b = 6912000 s
  assumes: single-exponential biological removal
```

In the worked example, T_p = 692988.48 s, T_b = 6912000 s go through the equation above and give T_eff = 629841.3714 s, lambda_eff = 1.100510719e-06 1/s. It applies only when single-exponential biological removal: outside that it says what is wrong instead of returning a number.

<a id="nuc_radiation_unit_conversion"></a>

#### Radiation unit conversion (Bq and Ci, Gy and rad, Sv and rem) — `nuc_radiation_unit_conversion`

An activity, absorbed dose or equivalent dose written in any of the SI or traditional units, expressed in both systems (1 Ci = 3.7e10 Bq, 1 rad = 0.01 Gy, 1 rem = 0.01 Sv).

$$
1\ \mathrm{Ci} = 3.7\times10^{10}\ \mathrm{Bq},\quad 1\ \mathrm{rad} = 0.01\ \mathrm{Gy},\quad 1\ \mathrm{rem} = 0.01\ \mathrm{Sv}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A` | in | an activity (Bq, kBq, MBq, Ci, mCi, uCi ...) (optional) | 1/s |
| `D` | in | an absorbed dose (Gy, mGy, rad, mrad) (optional) | J/kg |
| `H` | in | an equivalent or effective dose (Sv, mSv, rem, mrem) (optional) | J/kg |
| `SI_value` | out | value in the SI unit (Bq, Gy or Sv) | — |
| `traditional_value` | out | value in the traditional unit (Ci, rad or rem) | — |
| `si_unit` | out | the SI unit | — |
| `traditional_unit` | out | the traditional unit | — |
| `value_mCi` | out | activity in mCi | — |
| `value_MBq` | out | activity in MBq | — |
| `value_mSv_or_mGy` | out | dose in mSv or mGy | — |
| `value_mrem_or_mrad` | out | dose in mrem or mrad | — |

**Applies when:** exactly one of A, D, H.

```shell
$ nuclear-engineering solve nuc_radiation_unit_conversion "A=370000000 (1/s)"
SI_value = 370000000
traditional_value = 0.01
si_unit = Bq
traditional_unit = Ci
value_mCi = 10
value_MBq = 370
  by: Radiation unit conversion (Bq and Ci, Gy and rad, Sv and rem)  (Ci = A/3.7e10; rad = 100 Gy-value; rem = 100 Sv-value)
  A = 370000000 1/s
  assumes: exactly one of A, D, H
```

In the worked example, A = 370000000 (1/s) go through the equation above and give SI_value = 370000000, traditional_value = 0.01, si_unit = Bq, traditional_unit = Ci. It applies only when exactly one of A, D, H: outside that it says what is wrong instead of returning a number.

### Fuel cycle (1)

<a id="nuc_enrichment_swu"></a>

#### Uranium enrichment: feed, tails and separative work (SWU) — `nuc_enrichment_swu`

The natural-uranium feed and tails masses and the separative work (kg SWU) needed to produce a mass of enriched uranium from stated product, feed and tails assays, per kg of product too.

$$
SWU = P\,V(x_p) + W\,V(x_w) - F\,V(x_f),\quad V(x) = (2x-1)\ln\frac{x}{1-x}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x_p` | in | U-235 assay of the product (weight fraction, e.g. 4.5%) | — |
| `x_f` | in | U-235 assay of the feed (0.711% for natural uranium) | — |
| `x_w` | in | U-235 assay of the tails (e.g. 0.25%) | — |
| `P` | in | mass of product | kg |
| `F` | out | feed mass | kg |
| `W` | out | tails mass | kg |
| `SWU` | out | separative work (kg SWU) | kg |
| `F_per_P` | out | feed per unit product | — |
| `SWU_per_P` | out | kg SWU per kg of product | — |

**Applies when:** tails < feed < product assays; value function V(x) = (2x - 1) ln(x/(1 - x)).

```shell
$ nuclear-engineering solve nuc_enrichment_swu x_p=0.045 x_f=0.00711 x_w=0.0025 "P=1 kg"
F = 9.219088937 kg
W = 8.219088937 kg
SWU = 6.871112989 kg
F_per_P = 9.219088937
SWU_per_P = 6.871112989
  by: Uranium enrichment: feed, tails and separative work (SWU)  (F = P (x_p - x_w)/(x_f - x_w), W = F - P; SWU = P V(x_p) + W V(x_w) - F V(x_f))
  x_p = 0.045
  x_f = 0.00711
  x_w = 0.0025
  P = 1 kg
  assumes: tails < feed < product assays; value function V(x) = (2x - 1) ln(x/(1 - x))
```

In the worked example, x_p = 0.045, x_f = 0.00711, x_w = 0.0025, P = 1 kg go through the equation above and give F = 9.219088937 kg, W = 8.219088937 kg, SWU = 6.871112989 kg, F_per_P = 9.219088937. It applies only when tails < feed < product assays; value function V(x) = (2x - 1) ln(x/(1 - x)): outside that it says what is wrong instead of returning a number.

### Neutron physics (11)

<a id="nuc_logarithmic_energy_decrement"></a>

#### Average logarithmic energy decrement and mean scattering cosine — `nuc_logarithmic_energy_decrement`

For elastic scattering from a nucleus of mass ratio A (isotropic in the centre of mass): the minimum energy ratio alpha, the average logarithmic energy loss per collision xi and the average cosine of the laboratory scattering angle.

$$
\xi = 1 + \frac{\alpha\ln\alpha}{1-\alpha},\quad \alpha = \left(\frac{A-1}{A+1}\right)^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A` | in | mass of the nucleus in neutron masses (mass number) | — |
| `alpha` | out | minimum E'/E, ((A - 1)/(A + 1))^2 | — |
| `xi` | out | average logarithmic energy decrement | — |
| `mu_bar` | out | average cosine of the lab scattering angle, 2/(3A) | — |

**Applies when:** isotropic scattering in the centre-of-mass frame; A >= 1.

```shell
$ nuclear-engineering solve nuc_logarithmic_energy_decrement A=12
alpha = 0.7159763314
xi = 0.1577689898
mu_bar = 0.05555555556
  by: Average logarithmic energy decrement and mean scattering cosine  (xi = 1 + alpha ln(alpha)/(1 - alpha) (xi = 1 for A = 1); mu_bar = 2/(3A))
  A = 12
  assumes: isotropic scattering in the centre-of-mass frame; A >= 1
```

In the worked example, A = 12 go through the equation above and give alpha = 0.7159763314, xi = 0.1577689898, mu_bar = 0.05555555556. It applies only when isotropic scattering in the centre-of-mass frame; A >= 1: outside that it says what is wrong instead of returning a number.

<a id="nuc_collisions_to_thermalize"></a>

#### Average number of collisions to slow a neutron down — `nuc_collisions_to_thermalize`

The average number of elastic collisions with nuclei of mass ratio A needed to slow a neutron from E0 to Ef (for example from 2 MeV to 0.025 eV), from the lethargy change and xi.

$$
\bar n = \frac{\ln(E_0/E_f)}{\xi}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A` | in | mass of the moderator nucleus in neutron masses | — |
| `E0` | in | initial neutron energy | J |
| `Ef` | in | final neutron energy | J |
| `n` | out | average number of collisions | — |
| `xi` | out | average logarithmic energy decrement | — |

**Applies when:** isotropic CM scattering; E0 > Ef; a single nuclide moderator.

```shell
$ nuclear-engineering solve nuc_collisions_to_thermalize A=12 "E0=2 MeV" "Ef=0.025 eV"
n = 115.3429277
xi = 0.1577689898
  by: Average number of collisions to slow a neutron down  (n = ln(E0/Ef)/xi)
  A = 12
  E0 = 3.204353268e-13 J
  Ef = 4.005441585e-21 J
  assumes: isotropic CM scattering; E0 > Ef; a single nuclide moderator
```

**What the worked example means:** This calculation determines the mean number of elastic collisions required for a neutron to reduce its energy from an initial value of 2 MeV to a final value of 0.025 eV within a moderator. The result, 115.343, represents the average collision count derived from the total lethargy change divided by the average logarithmic energy decrement, 0.157769. This metric helps assess how efficiently a specific nuclide with mass ratio 12 thermalizes neutrons under ideal scattering conditions.

<a id="nuc_exponential_pile"></a>

#### Diffusion length from an exponential pile measurement — `nuc_exponential_pile`

The thermal diffusion length of a moderator from fluxes measured along the axis of an exponential pile (a source at the bottom of a prism of extrapolated sides a and b): least-squares relaxation constant gamma, then L from gamma^2 = 1/L^2 + (pi/a)^2 + (pi/b)^2.

$$
\gamma^2 = \frac{1}{L^2} + \left(\frac{\pi}{a}\right)^2 + \left(\frac{\pi}{b}\right)^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `z` | in | heights of the measurements along the axis, in cm (far from the source and the top) | — |
| `phi` | in | measured fluxes or detector readings at those heights (any consistent unit) | — |
| `a` | in | first extrapolated transverse side | m |
| `b` | in | second extrapolated transverse side | m |
| `gamma` | out | fitted relaxation constant | 1/m |
| `L` | out | diffusion length | m |
| `L2` | out | diffusion area L^2 | m^2 |

**Applies when:** fundamental transverse mode only (points far from the source); an effectively infinite pile (points far from the top).

```shell
$ nuclear-engineering solve nuc_exponential_pile "z=[20, 40, 60, 80, 100]" "phi=[10000, 6200, 3850, 2390, 1480]" "a=2 m" "b=2 m"
gamma = 2.387170968 1/m
L = 1.144234392 m
L2 = 1.309272345 m^2
  by: Diffusion length from an exponential pile measurement  (least squares of ln phi = c - gamma z; 1/L^2 = gamma^2 - (pi/a)^2 - (pi/b)^2)
  a = 2 m
  b = 2 m
  assumes: fundamental transverse mode only (points far from the source); an effectively infinite pile (points far from the top)
```

**What the worked example means:** This calculation determines the thermal diffusion length of a moderator by analyzing flux measurements along the axis of an exponential pile. A least-squares fit to the natural logarithm of the flux yields a relaxation constant of 2.38717 1/m. Using this value and the extrapolated transverse dimensions, the diffusion length is computed as 1.14423 m, representing the average distance neutrons travel before absorption.

<a id="nuc_plane_source_slab_flux"></a>

#### Flux from a plane neutron source at the centre of a slab — `nuc_plane_source_slab_flux`

The one-group thermal flux at a position in a non-multiplying slab of stated (extrapolated) thickness with a plane source of neutrons at its mid-plane, the flux at the source plane and the fraction of neutrons that leak from the two faces.

$$
\phi(x) = \frac{SL}{2D}\,\frac{\sinh\left[(a/2-|x|)/L\right]}{\cosh\left[a/(2L)\right]}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `S` | in | source strength per unit area of the plane (neutrons/(m^2 s)) | 1/(m^2*s) |
| `D` | in | diffusion coefficient | m |
| `Sigma_a` | in | absorption cross section | 1/m |
| `a` | in | extrapolated slab thickness (source at the centre) | m |
| `x` | in | distance from the source plane | m |
| `phi` | out | flux at x | 1/(m^2*s) |
| `phi0` | out | flux at the source plane | 1/(m^2*s) |
| `leak` | out | fraction of source neutrons leaking from the two faces | — |

**Applies when:** one-group diffusion in a non-multiplying medium; flux zero at the extrapolated faces.

```shell
$ nuclear-engineering solve nuc_plane_source_slab_flux "S=100000000 1/(cm^2*s)" "D=0.16 cm" "Sigma_a=0.0197 1/cm" "a=1 m" "x=10 cm"
phi = 2.665468272e+11 1/(m^2*s)
phi0 = 8.905881248e+12 1/(m^2*s)
leak = 4.803005487e-08
  by: Flux from a plane neutron source at the centre of a slab  (phi(x) = (S L/(2D)) sinh((a/2 - |x|)/L)/cosh(a/(2L)); leakage 1/cosh(a/(2L)))
  S = 1e+12 1/(m^2*s)
  D = 0.0016 m
  Sigma_a = 1.97 1/m
  a = 1 m
  x = 0.1 m
  assumes: one-group diffusion in a non-multiplying medium; flux zero at the extrapolated faces
```

**What the worked example means:** This calculation determines the one-group thermal neutron flux distribution in a non-multiplying slab of extrapolated thickness 1 m, containing a central plane source of strength 1e+08 1/(cm^2*s). Using diffusion coefficient 0.16 cm and absorption cross section 0.0197 1/cm, it yields the local flux 2.66547e+11 1/(m^2*s) at distance 10 cm, the maximum flux 8.90588e+12 1/(m^2*s) at the source, and the fraction of neutrons escaping the system, 4.80301e-08. These values describe how neutrons spread and are absorbed or leak from the boundaries.

<a id="nuc_point_source_sphere_flux"></a>

#### Flux from a point neutron source at the centre of a sphere — `nuc_point_source_sphere_flux`

The one-group thermal flux at a radius in a non-multiplying sphere (extrapolated radius R) with an isotropic point source at its centre, the fraction of neutrons that leak out, and the flux the same source would give in an infinite medium.

$$
\phi(r) = \frac{S}{4\pi D r}\,\frac{\sinh[(R-r)/L]}{\sinh(R/L)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `S` | in | source strength (neutrons per second) | 1/s |
| `D` | in | diffusion coefficient | m |
| `Sigma_a` | in | absorption cross section | 1/m |
| `R` | in | extrapolated radius of the sphere | m |
| `r` | in | distance from the source | m |
| `phi` | out | flux at r | 1/(m^2*s) |
| `leak` | out | fraction of source neutrons leaking out | — |
| `phi_inf` | out | flux at r in an infinite medium | 1/(m^2*s) |

**Applies when:** one-group diffusion in a non-multiplying medium; 0 < r < R.

```shell
$ nuclear-engineering solve nuc_point_source_sphere_flux "S=1000000000 1/s" "D=0.16 cm" "Sigma_a=0.0197 1/cm" "R=50 cm" "r=10 cm"
phi = 1.488561461e+10 1/(m^2*s)
leak = 8.42667431e-07
phi_inf = 1.488561461e+10 1/(m^2*s)
  by: Flux from a point neutron source at the centre of a sphere  (phi(r) = S sinh((R - r)/L)/(4 pi D r sinh(R/L)); leakage (R/L)/sinh(R/L); infinite medium S e^(-r/L)/(4 pi D r))
  S = 1000000000 1/s
  D = 0.0016 m
  Sigma_a = 1.97 1/m
  R = 0.5 m
  r = 0.1 m
  assumes: one-group diffusion in a non-multiplying medium; 0 < r < R
```

**What the worked example means:** This calculation determines the one-group thermal neutron flux at a specific distance from an isotropic point source located at the center of a non-multiplying spherical medium. Using the diffusion coefficient, absorption cross section, and extrapolated radius, the model computes the flux value of 1.48856e+10 1/(m^2*s) at radius 10 cm. It also calculates the fraction of neutrons escaping the boundary, which is 8.42667e-07, and provides the reference flux in an infinite medium, 1.48856e+10 1/(m^2*s), to highlight the impact of finite geometry on neutron distribution.

<a id="nuc_maxwellian_average_cross_section"></a>

#### Maxwellian-averaged 1/v cross section — `nuc_maxwellian_average_cross_section`

The effective thermal cross section of a 1/v absorber averaged over a Maxwellian neutron flux at a stated neutron temperature, from its tabulated 2200 m/s value.

$$
\bar\sigma = \frac{\sqrt\pi}{2}\,\sigma_0\sqrt{\frac{E_0}{kT}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `sigma0` | in | cross section at 2200 m/s (barns) | m^2 |
| `T` | in | neutron temperature | K |
| `sigma_avg` | out | flux-averaged cross section | m^2 |
| `sigma_avg_b` | out | the same in barns | — |
| `g` | out | ratio sigma_avg/sigma0 = (sqrt(pi)/2) sqrt(E0/kT) | — |

**Applies when:** 1/v cross section; Maxwellian flux spectrum phi(E) ~ E e^(-E/kT); E0 = m_n (2200 m/s)^2/2.

```shell
$ nuclear-engineering solve nuc_maxwellian_average_cross_section "sigma0=6.809e-26 m^2" "T=293.6 K"
sigma_avg = 6.03412493e-26 m^2
sigma_avg_b = 603.412493
g = 0.8861984036
  by: Maxwellian-averaged 1/v cross section  (sigma_avg = sigma0 (sqrt(pi)/2) sqrt(E0/(k T)))
  sigma0 = 6.809e-26 m^2
  T = 293.6 K
  assumes: 1/v cross section; Maxwellian flux spectrum phi(E) ~ E e^(-E/kT); E0 = m_n (2200 m/s)^2/2
```

In the worked example, sigma0 = 6.809e-26 m^2, T = 293.6 K go through the equation above and give sigma_avg = 6.03412493e-26 m^2, sigma_avg_b = 603.412493, g = 0.8861984036. It applies only when 1/v cross section; Maxwellian flux spectrum phi(E) ~ E e^(-E/kT); E0 = m_n (2200 m/s)^2/2: outside that it says what is wrong instead of returning a number.

<a id="nuc_moderator_mixture"></a>

#### Moderating power and ratio of a compound moderator — `nuc_moderator_mixture`

The macroscopic scattering and absorption cross sections of a compound moderator (water, heavy water, graphite, beryllium oxide, ...), its average logarithmic energy decrement (weighted by scattering), moderating power and moderating ratio.

$$
\bar\xi = \frac{\sum_i\xi_i\Sigma_{s,i}}{\sum_i\Sigma_{s,i}},\quad MR = \frac{\bar\xi\Sigma_s}{\Sigma_a}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | atoms of each element per molecule | — |
| `A` | in | mass ratio (mass number) of each element | — |
| `sigma_s` | in | scattering cross section of each element, in barns | — |
| `sigma_a` | in | absorption cross section of each element, in barns | — |
| `rho` | in | density of the moderator | kg/m^3 |
| `M` | in | molar mass of the molecule | kg/mol |
| `xi` | out | average logarithmic energy decrement of the mixture | — |
| `MP` | out | moderating power xi Sigma_s | 1/m |
| `MR` | out | moderating ratio xi Sigma_s/Sigma_a (when Sigma_a > 0) | — |

**Applies when:** isotropic CM scattering for every element; constant cross sections.

```shell
$ nuclear-engineering solve nuc_moderator_mixture "n=[2, 1]" "A=[1, 16]" "sigma_s=[20, 3.8]" "sigma_a=[0.332, 0.00019]" "rho=1 g/cm^3" "M=18.015 g/mol"
xi = 0.9236483396
MP = 135.237583 1/m
MR = 60.90997647
  by: Moderating power and ratio of a compound moderator  (N = rho N_A/M; Sigma_x = N sum n_i sigma_x,i; xi = sum xi_i Sigma_s,i / Sigma_s)
  rho = 1000 kg/m^3
  M = 0.018015 kg/mol
  assumes: isotropic CM scattering for every element; constant cross sections
```

**What the worked example means:** This calculation determines the macroscopic scattering and absorption cross sections for a compound moderator by summing the contributions of its constituent elements, weighted by their atomic densities. The average logarithmic energy decrement is found by weighting each element's decrement by its share of the total scattering cross section. Using these values, the moderating power, which represents the rate at which neutrons lose energy per unit path length, is calculated as 135.238 1/m. The moderating ratio, a figure of merit comparing slowing down to absorption, is determined to be 60.91. These results help assess how effectively the material slows neutrons while minimizing neutron loss.

<a id="nuc_elastic_scattering"></a>

#### Neutron energy and lab angle after an elastic collision — `nuc_elastic_scattering`

The energy of a neutron after elastic scattering from a nucleus of mass ratio A through a stated centre-of-mass angle, the energy ratio, the scattering angle in the laboratory and the lethargy gained.

$$
\frac{E'}{E} = \frac{A^2+2A\cos\theta+1}{(A+1)^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A` | in | mass of the nucleus in neutron masses (mass number; 1 for hydrogen) | — |
| `E` | in | neutron energy before the collision | J |
| `theta` | in | scattering angle in the centre-of-mass frame (deg or rad) | rad |
| `E_out` | out | neutron energy after the collision | J |
| `ratio` | out | E'/E | — |
| `psi` | out | scattering angle in the laboratory frame | rad |
| `du` | out | lethargy gain ln(E/E') | — |

**Applies when:** elastic scattering from a nucleus at rest; A >= 1.

```shell
$ nuclear-engineering solve nuc_elastic_scattering A=12 "E=2 MeV" "theta=90 deg"
E_out = 2.749297183e-13 J
ratio = 0.8579881657
psi = 1.487655095 rad
du = 0.1531649725
  by: Neutron energy and lab angle after an elastic collision  (E'/E = (A^2 + 2A cos theta + 1)/(A + 1)^2; tan psi = A sin theta/(1 + A cos theta))
  A = 12
  E = 3.204353268e-13 J
  theta = 1.570796327 rad
  assumes: elastic scattering from a nucleus at rest; A >= 1
```

**What the worked example means:** This calculation determines the post-collision state of a neutron undergoing elastic scattering from a stationary nucleus. Given the incident neutron energy 2 MeV and the centre-of-mass scattering angle 90 deg for a target with mass number 12, the model computes the energy retention ratio 0.857988. This yields a scattered neutron energy of 2.7493e-13 J and a laboratory frame scattering angle of 1.48766 rad. The change in lethargy, representing the logarithmic energy loss, is calculated as 0.153165. These values describe how effectively the collision moderates the neutron and redirects its path in the lab frame.

<a id="nuc_fuel_cross_sections"></a>

#### Number densities and macroscopic cross sections of UO2 fuel — `nuc_fuel_cross_sections`

Atom densities of U-235, U-238 and oxygen in uranium dioxide of stated density and enrichment, its macroscopic absorption and fission cross sections, eta (neutrons per absorption in the fuel) and the absorption mean free path.

$$
N_{25} = \frac{\rho N_A}{M_U + 2M_O}\,a_5,\quad \Sigma_a = \sum_i N_i\sigma_{a,i}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rho` | in | density of the UO2 (e.g. 10.4 g/cm^3) | kg/m^3 |
| `w` | in | enrichment: mass fraction of U-235 in the uranium | — |
| `sigma_a5` | in | U-235 absorption cross section (barns) | m^2 |
| `sigma_f5` | in | U-235 fission cross section (barns) | m^2 |
| `sigma_a8` | in | U-238 absorption cross section (barns) | m^2 |
| `sigma_aO` | in | oxygen absorption cross section (barns) | m^2 |
| `nu` | in | neutrons per fission of U-235 | — |
| `N5` | out | U-235 atoms per volume | 1/m^3 |
| `N8` | out | U-238 atoms per volume | 1/m^3 |
| `NO` | out | oxygen atoms per volume | 1/m^3 |
| `Sigma_a` | out | macroscopic absorption cross section | 1/m |
| `Sigma_f` | out | macroscopic fission cross section | 1/m |
| `eta` | out | eta = nu Sigma_f/Sigma_a of the fuel | — |
| `mfp_a` | out | absorption mean free path 1/Sigma_a | m |

**Applies when:** stoichiometric UO2 with U-235 and U-238 only; atomic masses U-235 235.043928, U-238 238.050787, O 15.999 g/mol.

```shell
$ nuclear-engineering solve nuc_fuel_cross_sections "rho=10.4 g/cm^3" w=0.03 "sigma_a5=6.809e-26 m^2" "sigma_f5=5.826e-26 m^2" "sigma_a8=2.68e-28 m^2" "sigma_aO=1.9e-32 m^2" nu=2.43
N5 = 7.046348224e+26 1/m^3
N8 = 2.249541432e+28 1/m^3
NO = 4.640009829e+28 1/m^3
Sigma_a = 54.00823769 1/m
Sigma_f = 41.05202475 1/m
eta = 1.847059345
mfp_a = 0.01851569395 m
  by: Number densities and macroscopic cross sections of UO2 fuel  (atom fraction a5 = (w/M5)/(w/M5 + (1 - w)/M8); N(UO2) = rho N_A/(M_U + 2 M_O); Sigma = sum N_i sigma_i)
  rho = 10400 kg/m^3
  w = 0.03
  sigma_a5 = 6.809e-26 m^2
  sigma_f5 = 5.826e-26 m^2
  sigma_a8 = 2.68e-28 m^2
  sigma_aO = 1.9e-32 m^2
  nu = 2.43
  assumes: stoichiometric UO2 with U-235 and U-238 only; atomic masses U-235 235.043928, U-238 238.050787, O 15.999 g/mol
```

**What the worked example means:** This calculation determines the atomic densities of uranium-235, uranium-238, and oxygen in stoichiometric uranium dioxide fuel based on its density 10.4 g/cm^3 and enrichment 0.03. The resulting number densities are 7.04635e+26 1/m^3 for uranium-235, 2.24954e+28 1/m^3 for uranium-238, and 4.64001e+28 1/m^3 for oxygen. These values are combined with microscopic cross sections to compute the macroscopic absorption cross section 54.0082 1/m and fission cross section 41.052 1/m. From these, the neutron reproduction factor eta is calculated as 1.84706, representing neutrons produced per absorption in the fuel, while the absorption mean free path is found to be 0.0185157 m.

<a id="nuc_thermal_neutron_speeds"></a>

#### Speeds and mean energy of thermal neutrons (Maxwell distribution) — `nuc_thermal_neutron_speeds`

The most probable, mean and root-mean-square speeds and the mean kinetic energy of neutrons in thermal equilibrium (Maxwell-Boltzmann distribution) at a stated temperature.

$$
v_p = \sqrt{\frac{2kT}{m_n}},\quad \bar v = \frac{2}{\sqrt\pi}v_p
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | neutron (moderator) temperature | K |
| `v_p` | out | most probable speed sqrt(2kT/m) | m/s |
| `v_mean` | out | mean speed | m/s |
| `v_rms` | out | root-mean-square speed | m/s |
| `E_mean` | out | mean kinetic energy 3kT/2 | J |
| `E_mean_eV` | out | mean kinetic energy in eV | — |

**Applies when:** Maxwell-Boltzmann distribution of the neutron density; neutron mass 1.67492750056e-27 kg.

```shell
$ nuclear-engineering solve nuc_thermal_neutron_speeds "T=293.6 K"
v_p = 2200.070806 m/s
v_mean = 2482.514064 m/s
v_rms = 2694.525436 m/s
E_mean = 6.080378196e-21 J
E_mean_eV = 0.03795073569
  by: Speeds and mean energy of thermal neutrons (Maxwell distribution)  (v_p = sqrt(2kT/m_n), v_mean = 2 v_p/sqrt(pi), v_rms = sqrt(3/2) v_p, E_mean = 3kT/2)
  T = 293.6 K
  assumes: Maxwell-Boltzmann distribution of the neutron density; neutron mass 1.67492750056e-27 kg
```

In the worked example, T = 293.6 K go through the equation above and give v_p = 2200.070806 m/s, v_mean = 2482.514064 m/s, v_rms = 2694.525436 m/s, E_mean = 6.080378196e-21 J. It applies only when Maxwell-Boltzmann distribution of the neutron density; neutron mass 1.67492750056e-27 kg: outside that it says what is wrong instead of returning a number.

<a id="nuc_diffusion_length"></a>

#### Thermal diffusion length and migration length — `nuc_diffusion_length`

The thermal diffusion length L = sqrt(D/Sigma_a) of a medium and, with the Fermi age tau, the migration area M^2 = L^2 + tau and migration length.

$$
L = \sqrt{\frac{D}{\Sigma_a}},\quad M^2 = L^2 + \tau
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | diffusion coefficient (cm) | m |
| `Sigma_a` | in | macroscopic absorption cross section | 1/m |
| `tau` | in | Fermi age to thermal (cm^2, optional) (optional) | m^2 |
| `L` | out | diffusion length | m |
| `L2` | out | diffusion area L^2 | m^2 |
| `L_cm` | out | diffusion length in cm | — |
| `M2` | out | migration area L^2 + tau | m^2 |
| `M` | out | migration length | m |

**Applies when:** D, Sigma_a > 0; one-group (thermal) diffusion theory.

```shell
$ nuclear-engineering solve nuc_diffusion_length "D=0.16 cm" "Sigma_a=0.0197 1/cm"
L = 0.02849882 m
L2 = 0.0008121827411 m^2
L_cm = 2.849882
  by: Thermal diffusion length and migration length  (L^2 = D/Sigma_a; M^2 = L^2 + tau)
  D = 0.0016 m
  Sigma_a = 1.97 1/m
  assumes: D, Sigma_a > 0; one-group (thermal) diffusion theory
```

In the worked example, D = 0.16 cm, Sigma_a = 0.0197 1/cm go through the equation above and give L = 0.02849882 m, L2 = 0.0008121827411 m^2, L_cm = 2.849882. It applies only when D, Sigma_a > 0; one-group (thermal) diffusion theory: outside that it says what is wrong instead of returning a number.

### Reactor kinetics (6)

<a id="nuc_control_rod_worth"></a>

#### Integral and differential control-rod worth (sine-squared weighting) — `nuc_control_rod_worth`

The reactivity worth of a control rod inserted to a depth x in a core of height H with a cosine axial flux (first-order perturbation: worth density ~ sin^2), as a fraction of its total worth, and the differential worth at that depth.

$$
\frac{\rho(x)}{\rho_{tot}} = \frac{x}{H} - \frac{1}{2\pi}\sin\frac{2\pi x}{H}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rho_tot` | in | total worth of the fully inserted rod (absolute, or in pcm, %, dollars or cents) | — |
| `H` | in | core height | m |
| `x` | in | insertion depth from the top of the core | m |
| `beta` | in | delayed-neutron fraction (optional: converts a worth in dollars and reports dollars) (optional) | — |
| `rho` | out | worth inserted (absolute) | — |
| `frac` | out | fraction of the total worth | — |
| `drho_dx` | out | differential worth at x (reactivity per metre) | 1/m |
| `rho_dollars` | out | worth in dollars (when beta is given) | — |

**Applies when:** cosine axial flux, flux zero at the core ends; first-order perturbation (small rod).

```shell
$ nuclear-engineering solve nuc_control_rod_worth rho_tot=0.01 "H=3.66 m" "x=1.83 m"
rho = 0.005
frac = 0.5
drho_dx = 0.005464480874 1/m
  by: Integral and differential control-rod worth (sine-squared weighting)  (rho(x)/rho_tot = x/H - sin(2 pi x/H)/(2 pi); d rho/dx = (rho_tot/H)(1 - cos(2 pi x/H)))
  rho_tot = 0.01
  H = 3.66 m
  x = 1.83 m
  assumes: cosine axial flux, flux zero at the core ends; first-order perturbation (small rod)
```

**What the worked example means:** This calculation determines the reactivity inserted by a control rod at depth x within a core of height H. Using a sine-squared weighting model for a cosine axial flux, it computes the absolute worth as 0.005, which represents 0.5 of the rod's total capability. The differential worth at this position is 0.00546448 1/m, indicating how rapidly reactivity changes with further insertion.

<a id="nuc_power_after_step_one_group"></a>

#### Power after a reactivity step (one delayed group, exact) — `nuc_power_after_step_one_group`

The relative power P(t)/P0 a time t after a step reactivity change from steady state, from the exact two-mode solution with one delayed group, with the amplitude of the stable period (the exact prompt jump or drop) and the inverse period.

$$
\frac{P(t)}{P_0} = \sum_j A_je^{\omega_jt}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rho` | in | step reactivity (absolute, or in pcm, %, dollars or cents) | — |
| `beta` | in | delayed-neutron fraction beta | — |
| `lam` | in | precursor decay constant of the single delayed group (e.g. 0.08 1/s) | 1/s |
| `Lambda` | in | prompt neutron generation time | s |
| `t` | in | time after the step | s |
| `P` | out | relative power P(t)/P0 | — |
| `A1` | out | amplitude of the stable mode (exact prompt jump or drop) | — |
| `omega` | out | inverse stable period | 1/s |

**Applies when:** point kinetics, one delayed group, steady state before the step; rho < beta, rho != 0.

```shell
$ nuclear-engineering solve nuc_power_after_step_one_group rho=0.001 beta=0.0065 "lam=0.08 1/s" "Lambda=0.0001 s" "t=10 s"
P = 1.365788055
A1 = 1.181194942
omega = 0.01452050027 1/s
  by: Power after a reactivity step (one delayed group, exact)  (P/P0 = A1 e^(w1 t) + A2 e^(w2 t), w_j the inhour roots, A_j = 1/(1 - w_j S2(w_j)/N(w_j)) (residues))
  rho = 0.001
  beta = 0.0065
  lam = 0.08 1/s
  Lambda = 0.0001 s
  t = 10 s
  assumes: point kinetics, one delayed group, steady state before the step; rho < beta, rho != 0
```

**What the worked example means:** This calculation determines the reactor's relative power at time 10 s following a step reactivity change of 0.001. Using point kinetics with one delayed group, it solves for the exact two-mode response. The result shows that the power reaches 1.36579, driven by a stable exponential component with amplitude 1.18119 and rate 0.0145205 1/s. This captures both the immediate prompt jump and the subsequent long-term trend.

<a id="nuc_power_after_step_six_group"></a>

#### Power after a reactivity step (six delayed groups, exact) — `nuc_power_after_step_six_group`

The relative power P(t)/P0 a time t after a step reactivity change from steady state with six (or any number up to eight) delayed-neutron groups, from all the inhour roots and their residues, and the stable period.

$$
\rho = \omega\Lambda + \sum_i\frac{\beta_i\,\omega}{\omega+\lambda_i},\quad \frac{P}{P_0} = \sum_j A_je^{\omega_jt}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rho` | in | step reactivity (absolute, or in pcm, %, dollars or cents) | — |
| `beta` | in | delayed-neutron fraction of each group | — |
| `lam` | in | decay constant of each group, in 1/s | — |
| `Lambda` | in | prompt neutron generation time | s |
| `t` | in | time after the step | s |
| `P` | out | relative power P(t)/P0 | — |
| `omega` | out | inverse stable period | 1/s |
| `T` | out | stable period | s |

**Applies when:** point kinetics, steady state before the step; rho < beta (sum of the groups), rho != 0; distinct decay constants.

```shell
$ nuclear-engineering solve nuc_power_after_step_six_group rho=0.001 "beta=[0.000215, 0.001424, 0.001274, 0.002568, 0.000748, 0.000273]" "lam=[0.0124, 0.0305, 0.111, 0.301, 1.14, 3.01]" "Lambda=50 us" "t=10 s"
P = 1.616986969
omega = 0.01819371976 1/s
T = 54.96402127 s
  by: Power after a reactivity step (six delayed groups, exact)  (P/P0 = sum_j A_j e^(w_j t) over the n + 1 inhour roots (bisection between the poles, 40 digits), A_j = 1/(1 - w_j sum beta_i/(w_j + lambda_i)^2 / N(w_j)))
  rho = 0.001
  Lambda = 5e-05 s
  t = 10 s
  assumes: point kinetics, steady state before the step; rho < beta (sum of the groups), rho != 0; distinct decay constants
```

**What the worked example means:** This calculation determines the exact time-dependent power response following a step reactivity change. It uses all delayed neutron group roots to compute the relative power, which resulted in 1.61699 at time 10 s. The inverse stable period is 0.0181937 1/s, corresponding to a stable period of 54.964 s. This approach accounts for the complex interplay between prompt neutrons and multiple delayed groups under point kinetics assumptions.

<a id="nuc_reactivity_from_period"></a>

#### Reactivity from a measured stable period or doubling time (inhour) — `nuc_reactivity_from_period`

The reactivity, in absolute units, dollars and pcm, that produces a measured stable period (or doubling time) with stated delayed-neutron data, from the inhour equation.

$$
\rho = \frac{\Lambda}{T} + \sum_i\frac{\beta_i}{1+\lambda_iT}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `beta` | in | delayed-neutron fraction of each group | — |
| `lam` | in | decay constant of each group, in 1/s | — |
| `Lambda` | in | prompt neutron generation time | s |
| `T` | in | stable period (give this or T_d; negative for falling power) (optional) | s |
| `T_d` | in | doubling time (give this or T) (optional) | s |
| `rho` | out | reactivity | — |
| `rho_dollars` | out | reactivity in dollars | — |
| `rho_pcm` | out | reactivity in pcm | — |

**Applies when:** point kinetics, asymptotic period; for a negative period: 1/T > -lambda_min.

```shell
$ nuclear-engineering solve nuc_reactivity_from_period "beta=[0.000215, 0.001424, 0.001274, 0.002568, 0.000748, 0.000273]" "lam=[0.0124, 0.0305, 0.111, 0.301, 1.14, 3.01]" "Lambda=50 us" "T=60 s"
rho = 0.0009406257256
rho_dollars = 0.1446671371
rho_pcm = 94.06257256
  by: Reactivity from a measured stable period or doubling time (inhour)  (rho = omega Lambda + sum beta_i omega/(omega + lambda_i), omega = 1/T = ln 2/T_d)
  Lambda = 5e-05 s
  T = 60 s
  assumes: point kinetics, asymptotic period; for a negative period: 1/T > -lambda_min
```

**What the worked example means:** This calculation determines the reactor reactivity by applying the inhour equation to a measured asymptotic stable period. The process combines the prompt neutron generation time with the contributions from each delayed neutron group, weighted by their respective decay constants and fractions. Using an input period of 60 s and specific delayed data, the resulting absolute reactivity is found to be 0.000940626. This value corresponds to 0.144667 in dollar units and 94.0626 in pcm, providing a diagnostic measure of the core's state based on its power growth or decay rate.

<a id="nuc_stable_period_six_group"></a>

#### Stable period and prompt root of the inhour equation (six delayed groups) — `nuc_stable_period_six_group`

The stable reactor period, inverse period and doubling time after a step reactivity change with six (or any number up to eight) delayed groups, and the prompt (most negative) root that sets the prompt jump's time constant.

$$
\rho = \omega\Lambda + \sum_i\frac{\beta_i\,\omega}{\omega+\lambda_i}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rho` | in | step reactivity (absolute, or in pcm, %, dollars or cents) | — |
| `beta` | in | delayed-neutron fraction of each group | — |
| `lam` | in | decay constant of each group, in 1/s | — |
| `Lambda` | in | prompt neutron generation time | s |
| `omega` | out | inverse stable period (largest root) | 1/s |
| `T` | out | stable period | s |
| `T_d` | out | doubling time (negative: halving time) | s |
| `omega_p` | out | prompt root (most negative) | 1/s |

**Applies when:** point kinetics; rho < beta, rho != 0; distinct decay constants.

```shell
$ nuclear-engineering solve nuc_stable_period_six_group rho=0.001 "beta=[0.000215, 0.001424, 0.001274, 0.002568, 0.000748, 0.000273]" "lam=[0.0124, 0.0305, 0.111, 0.301, 1.14, 3.01]" "Lambda=50 us"
omega = 0.01819371976 1/s
T = 54.96402127 s
T_d = 38.09815638 s
omega_p = -110.5229924 1/s
  by: Stable period and prompt root of the inhour equation (six delayed groups)  (largest and smallest roots of the inhour equation (bisection between the poles, 40 digits))
  rho = 0.001
  Lambda = 5e-05 s
  assumes: point kinetics; rho < beta, rho != 0; distinct decay constants
```

**What the worked example means:** This calculation determines the asymptotic behavior of reactor power following a step change in reactivity. Using the point kinetics model with six delayed neutron groups, it solves the inhour equation to find the largest root, which defines the stable period 54.964 s and its inverse 0.0181937 1/s. It also identifies the most negative root, known as the prompt root -110.523 1/s, which characterizes the initial rapid adjustment of the neutron population before the delayed neutrons dominate the long-term trend.

<a id="nuc_reactor_period_one_group"></a>

#### Stable reactor period after a reactivity step (one delayed group) — `nuc_reactor_period_one_group`

The stable (asymptotic) period, inverse period and doubling time of a reactor after a step reactivity change, with one effective delayed-neutron group (the stable root of the inhour equation).

$$
\rho = \omega\Lambda + \frac{\beta\,\omega}{\omega+\lambda}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rho` | in | step reactivity (absolute, or in pcm, %, dollars or cents) | — |
| `beta` | in | delayed-neutron fraction beta | — |
| `lam` | in | precursor decay constant of the single delayed group (e.g. 0.08 1/s) | 1/s |
| `Lambda` | in | prompt neutron generation time | s |
| `omega` | out | inverse stable period (largest inhour root) | 1/s |
| `T` | out | stable period (negative: falling power) | s |
| `T_d` | out | doubling time T ln 2 (negative: halving time) | s |

**Applies when:** point kinetics with one delayed group; rho below prompt critical (rho < beta); rho != 0.

```shell
$ nuclear-engineering solve nuc_reactor_period_one_group rho=0.001 beta=0.0065 "lam=0.08 1/s" "Lambda=0.0001 s"
omega = 0.01452050027 1/s
T = 68.86815063 s
T_d = 47.73576444 s
  by: Stable reactor period after a reactivity step (one delayed group)  (largest root of rho = omega Lambda + beta omega/(omega + lambda))
  rho = 0.001
  beta = 0.0065
  lam = 0.08 1/s
  Lambda = 0.0001 s
  assumes: point kinetics with one delayed group; rho below prompt critical (rho < beta); rho != 0
```

**What the worked example means:** This calculation finds the asymptotic reactor behavior after a step change in reactivity using a single delayed neutron group. By solving the inhour equation for the largest root, it determines the inverse stable period 0.0145205 1/s, which defines how fast power changes per second. The stable period 68.8682 s represents the time required for power to increase by a factor of e, while the doubling time 47.7358 s indicates when power will double. These values rely on the prompt generation time 0.0001 s, delayed fraction 0.0065, decay constant 0.08 1/s, and the applied reactivity 0.001.

### Reactor physics (20)

<a id="nuc_inverse_multiplication"></a>

#### Approach to criticality: inverse multiplication (1/M) extrapolation — `nuc_inverse_multiplication`

Predict the critical loading (fuel elements, kg, rod position) from count rates measured as fuel is added, by extrapolating the inverse multiplication 1/M = C_0/C to zero: least-squares line through all points and the line through the last two.

$$
\frac{1}{M} = \frac{C_0}{C},\quad x_{crit}: \frac{1}{M}(x_{crit}) = 0
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | loadings (fuel elements, kg or any consistent measure), first = reference | — |
| `C` | in | count rates at those loadings (any consistent unit) | — |
| `x_crit` | out | predicted critical loading (least-squares line through all points) | — |
| `slope` | out | slope of 1/M per unit loading | — |
| `x_crit_last2` | out | predicted critical loading from the last two points | — |
| `inv_M_last` | out | 1/M at the last loading | — |

**Applies when:** 1/M referred to the first count rate; 1/M decreasing with loading; subcritical measurements.

```shell
$ nuclear-engineering solve nuc_inverse_multiplication "x=[0, 20, 40, 60, 80]" "C=[100, 125, 167, 250, 500]"
x_crit = 99.9760479
slope = -0.01
x_crit_last2 = 100
inv_M_last = 0.2
  by: Approach to criticality: inverse multiplication (1/M) extrapolation  (1/M_i = C_1/C_i; straight lines in x; x_crit where the line reaches 1/M = 0)
  assumes: 1/M referred to the first count rate; 1/M decreasing with loading; subcritical measurements
```

**What the worked example means:** This calculation determines the fuel amount required to reach criticality by analyzing neutron count rates measured during subcritical loading. The inverse multiplication factor, defined as the ratio of the initial count rate to the current count rate, is plotted against the loading. A linear least-squares fit through all data points predicts that the system reaches criticality at a loading of 99.976. An alternative estimate using only the last two measurements yields 100, providing a check on the linearity assumption near the critical point.

<a id="nuc_critical_cylinder"></a>

#### Critical radius of a bare cylindrical reactor of given height (one group) — `nuc_critical_cylinder`

The critical radius of a bare homogeneous cylinder of stated height in one-group diffusion theory, with its extrapolated radius and the physical volume.

$$
\left(\frac{2.405}{R_{ext}}\right)^2 + \left(\frac{\pi}{H_{ext}}\right)^2 = B_m^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | diffusion coefficient (a length, e.g. cm) | m |
| `Sigma_a` | in | macroscopic absorption cross section | 1/m |
| `nuSigma_f` | in | nu times the macroscopic fission cross section | 1/m |
| `extrapolation` | in | extrapolation distance added at each free surface: none, 2D or 2.13D (= 0.71 transport mean free paths) | — |
| `H` | in | physical height of the cylinder | m |
| `R` | out | critical physical radius | m |
| `R_ext` | out | critical extrapolated radius | m |
| `V` | out | critical physical volume | m^3 |

**Applies when:** one-group diffusion; the height exceeds the critical slab thickness.

```shell
$ nuclear-engineering solve nuc_critical_cylinder "D=0.9 cm" "Sigma_a=0.15 1/cm" "nuSigma_f=0.18 1/cm" extrapolation=2.13D "H=50 cm"
R = 0.1198286337 m
R_ext = 0.1390094337 m
V = 0.02255490966 m^3
  by: Critical radius of a bare cylindrical reactor of given height (one group)  ((j01/R_ext)^2 + (pi/H_ext)^2 = B_m^2, j01 = 2.405 (first zero of J0))
  D = 0.009 m
  Sigma_a = 15 1/m
  nuSigma_f = 18 1/m
  H = 0.5 m
  extrapolation = 2.13D
  assumes: one-group diffusion; the height exceeds the critical slab thickness
```

**What the worked example means:** This calculation determines the minimum size for a bare cylindrical reactor to sustain a chain reaction using one-group diffusion theory. Given the material properties and the fixed height, the geometric buckling must match the material buckling derived from the diffusion coefficient and cross sections. The resulting critical physical radius is 0.119829 m, which corresponds to an extrapolated radius of 0.139009 m. This configuration yields a critical physical volume of 0.0225549 m^3, representing the smallest cylinder of this specific height that can achieve criticality without external reflection.

<a id="nuc_critical_sphere"></a>

#### Critical radius of a bare spherical reactor (one group) — `nuc_critical_sphere`

The critical radius (physical and extrapolated) and volume of a bare homogeneous spherical reactor in one-group diffusion theory.

$$
\left(\frac{\pi}{R_{ext}}\right)^2 = \frac{\nu\Sigma_f-\Sigma_a}{D}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | diffusion coefficient (a length, e.g. cm) | m |
| `Sigma_a` | in | macroscopic absorption cross section | 1/m |
| `nuSigma_f` | in | nu times the macroscopic fission cross section | 1/m |
| `extrapolation` | in | extrapolation distance added at each free surface: none, 2D or 2.13D (= 0.71 transport mean free paths) | — |
| `R` | out | critical physical radius | m |
| `R_ext` | out | critical extrapolated radius | m |
| `V` | out | critical physical volume | m^3 |

**Applies when:** one-group diffusion; k_inf > 1; bare homogeneous sphere.

```shell
$ nuclear-engineering solve nuc_critical_sphere "D=0.9 cm" "Sigma_a=0.15 1/cm" "nuSigma_f=0.18 1/cm" extrapolation=2.13D
R = 0.1528913163 m
R_ext = 0.1720721163 m
V = 0.01497052629 m^3
  by: Critical radius of a bare spherical reactor (one group)  ((pi/R_ext)^2 = B_m^2; R = R_ext - d)
  D = 0.009 m
  Sigma_a = 15 1/m
  nuSigma_f = 18 1/m
  extrapolation = 2.13D
  assumes: one-group diffusion; k_inf > 1; bare homogeneous sphere
```

**What the worked example means:** This calculation determines the size at which a bare spherical reactor becomes self-sustaining. Using one-group diffusion theory, it balances material properties against geometric leakage to find the extrapolated radius 0.172072 m, from which the physical radius 0.152891 m and volume 0.0149705 m^3 are derived.

<a id="nuc_critical_slab"></a>

#### Critical thickness of a bare slab reactor (one group) — `nuc_critical_slab`

The critical thickness of an infinite bare slab of multiplying material in one-group diffusion theory, the extrapolated thickness and the material buckling.

$$
\left(\frac{\pi}{a_{ext}}\right)^2 = \frac{\nu\Sigma_f-\Sigma_a}{D}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | diffusion coefficient (a length, e.g. cm) | m |
| `Sigma_a` | in | macroscopic absorption cross section | 1/m |
| `nuSigma_f` | in | nu times the macroscopic fission cross section | 1/m |
| `extrapolation` | in | extrapolation distance added at each free surface: none, 2D or 2.13D (= 0.71 transport mean free paths) | — |
| `a` | out | critical physical thickness | m |
| `a_ext` | out | critical extrapolated thickness | m |

**Applies when:** one-group diffusion; k_inf > 1; bare slab, flux zero at the extrapolated faces.

```shell
$ nuclear-engineering solve nuc_critical_slab "D=0.9 cm" "Sigma_a=0.15 1/cm" "nuSigma_f=0.18 1/cm" extrapolation=2.13D
a = 0.1337105163 m
a_ext = 0.1720721163 m
  by: Critical thickness of a bare slab reactor (one group)  (B_m^2 = (nu Sigma_f - Sigma_a)/D; critical when (pi/a_ext)^2 = B_m^2; a = a_ext - 2d)
  D = 0.009 m
  Sigma_a = 15 1/m
  nuSigma_f = 18 1/m
  extrapolation = 2.13D
  assumes: one-group diffusion; k_inf > 1; bare slab, flux zero at the extrapolated faces
```

**What the worked example means:** This calculation determines the minimum size required for a bare slab reactor to sustain a chain reaction using one-group diffusion theory. Equating the material buckling derived from nuclear properties with the geometric buckling of the slab shows that the extrapolated thickness must be 0.172072 m. The actual physical thickness, accounting for neutron leakage at the boundaries, is calculated as 0.133711 m. This result indicates the smallest dimension at which neutron production balances absorption and leakage.

<a id="nuc_reflected_slab_critical"></a>

#### Critical thickness of a reflected slab reactor and reflector savings (one group) — `nuc_reflected_slab_critical`

The critical core thickness of a slab reactor with a reflector of stated thickness on both faces (one-group diffusion in core and reflector) and the reflector savings compared with the bare core.

$$
D_cB\tan\frac{Ba}{2} = \frac{D_r}{L_r}\coth\frac{T_r+d}{L_r}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D_c` | in | core diffusion coefficient | m |
| `Sigma_ac` | in | core absorption cross section | 1/m |
| `nuSigma_fc` | in | core nu Sigma_f | 1/m |
| `D_r` | in | reflector diffusion coefficient | m |
| `Sigma_ar` | in | reflector absorption cross section | 1/m |
| `T_r` | in | reflector thickness on each face | m |
| `extrapolation` | in | extrapolation distance added at each free surface: none, 2D or 2.13D (= 0.71 transport mean free paths) | — |
| `a` | out | critical core thickness | m |
| `delta` | out | reflector savings per face (bare extrapolated half-thickness minus reflected half-thickness) | m |

**Applies when:** one-group diffusion; non-multiplying reflector; extrapolation distance of the reflector at its outer face.

```shell
$ nuclear-engineering solve nuc_reflected_slab_critical "D_c=0.9 cm" "Sigma_ac=0.15 1/cm" "nuSigma_fc=0.18 1/cm" "D_r=0.16 cm" "Sigma_ar=0.0197 1/cm" "T_r=20 cm" extrapolation=2.13D
a = 0.03606623394 m
delta = 0.06800294117 m
  by: Critical thickness of a reflected slab reactor and reflector savings (one group)  (D_c B tan(B a/2) = (D_r/L_r) coth((T_r + d)/L_r), L_r^2 = D_r/Sigma_ar)
  D_c = 0.009 m
  Sigma_ac = 15 1/m
  nuSigma_fc = 18 1/m
  D_r = 0.0016 m
  Sigma_ar = 1.97 1/m
  T_r = 0.2 m
  extrapolation = 2.13D
  assumes: one-group diffusion; non-multiplying reflector; extrapolation distance of the reflector at its outer face
```

**What the worked example means:** This calculation determines the minimum core thickness required for a slab reactor to reach criticality when surrounded by non-multiplying reflectors. Using one-group diffusion theory, it solves for the geometric buckling that balances neutron production in the core against leakage and absorption in both the fuel and the surrounding reflector material. The result, 0.0360662 m, represents the physical width of the active fuel region needed to sustain a chain reaction given the specified reflector properties. Additionally, the model computes the reflector savings, 0.0680029 m, which quantifies how much thinner the reflected core can be compared to an identical bare core due to neutrons being scattered back into the fuel.

<a id="nuc_critical_cuboid"></a>

#### Critical third side of a bare rectangular reactor (one group) — `nuc_critical_cuboid`

The third side that makes a bare rectangular parallelepiped of two stated sides critical in one-group diffusion theory, its extrapolated value and the physical volume.

$$
\left(\frac{\pi}{a_{ext}}\right)^2+\left(\frac{\pi}{b_{ext}}\right)^2+\left(\frac{\pi}{c_{ext}}\right)^2 = B_m^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | diffusion coefficient (a length, e.g. cm) | m |
| `Sigma_a` | in | macroscopic absorption cross section | 1/m |
| `nuSigma_f` | in | nu times the macroscopic fission cross section | 1/m |
| `extrapolation` | in | extrapolation distance added at each free surface: none, 2D or 2.13D (= 0.71 transport mean free paths) | — |
| `a` | in | first physical side | m |
| `b` | in | second physical side | m |
| `c` | out | critical physical third side | m |
| `c_ext` | out | critical extrapolated third side | m |
| `V` | out | critical physical volume | m^3 |

**Applies when:** one-group diffusion; the two given sides are large enough.

```shell
$ nuclear-engineering solve nuc_critical_cuboid "D=0.9 cm" "Sigma_a=0.15 1/cm" "nuSigma_f=0.18 1/cm" extrapolation=2.13D "a=60 cm" "b=60 cm"
c = 0.1477648343 m
c_ext = 0.1861264343 m
V = 0.05319534036 m^3
  by: Critical third side of a bare rectangular reactor (one group)  ((pi/a_ext)^2 + (pi/b_ext)^2 + (pi/c_ext)^2 = B_m^2)
  D = 0.009 m
  Sigma_a = 15 1/m
  nuSigma_f = 18 1/m
  a = 0.6 m
  b = 0.6 m
  extrapolation = 2.13D
  assumes: one-group diffusion; the two given sides are large enough
```

**What the worked example means:** This calculation determines the third dimension required for a bare rectangular reactor to reach criticality. Given the first two sides, the method solves the buckling equation using one-group diffusion theory. The resulting critical physical third side is 0.147765 m, while the extrapolated boundary distance yields an effective size of 0.186126 m. These values ensure neutron production balances absorption and leakage in all three dimensions.

<a id="nuc_decay_heat_power"></a>

#### Decay heat after shutdown (Way-Wigner) — `nuc_decay_heat_power`

The fission-product decay power a time t after shutdown of a reactor that ran at constant power for an operating time T, by the Way-Wigner correlation.

$$
\frac{P_d}{P_0} = 0.0622\left[t^{-0.2} - (t+T)^{-0.2}\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P0` | in | thermal power before shutdown | W |
| `T_op` | in | operating time at that power | s |
| `t` | in | time after shutdown (10 s to about 1e8 s) | s |
| `P_d` | out | decay power | W |
| `ratio` | out | decay power / operating power | — |

**Applies when:** constant power before shutdown; 10 s <= t <= 1e8 s.

```shell
$ nuclear-engineering solve nuc_decay_heat_power "P0=3000 MW" "T_op=31557600 s" "t=1 h"
P_d = 30375540.96 W
ratio = 0.01012518032
  by: Decay heat after shutdown (Way-Wigner)  (P/P0 = 0.0622 [t^-0.2 - (t + T)^-0.2], t and T in seconds (Way-Wigner))
  P0 = 3000000000 W
  T_op = 31557600 s
  t = 3600 s
  assumes: constant power before shutdown; 10 s <= t <= 1e8 s
```

**What the worked example means:** This calculation determines the residual thermal power produced by fission products after a reactor shutdown. Using the Way-Wigner correlation, it takes the pre-shutdown thermal power 3000 MW, the operating time 3.15576e+07 s, and the time since shutdown 1 h to compute the decay power 3.03755e+07 W. The result indicates that 0.0101252 of the original power remains as heat at this specific moment.

<a id="nuc_decay_heat_energy"></a>

#### Decay heat energy released between two times after shutdown — `nuc_decay_heat_energy`

The total fission-product decay energy released between two times after shutdown (the Way-Wigner power integrated), and the mean decay power over that interval.

$$
E = 0.0622\,P_0\left[\frac{t^{0.8}-(t+T)^{0.8}}{0.8}\right]_{t_1}^{t_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P0` | in | thermal power before shutdown | W |
| `T_op` | in | operating time at that power | s |
| `t1` | in | start of the interval after shutdown | s |
| `t2` | in | end of the interval after shutdown | s |
| `E` | out | decay energy released | J |
| `E_MWh` | out | the same energy in MWh | — |
| `P_mean` | out | mean decay power over the interval | W |

**Applies when:** 10 s <= t1 < t2 <= 1e8 s; constant power before shutdown.

```shell
$ nuclear-engineering solve nuc_decay_heat_energy "P0=3000 MW" "T_op=31557600 s" "t1=10 s" "t2=86400 s"
E = 1.563755897e+12 J
E_MWh = 434.3766382
P_mean = 18101121.63 W
  by: Decay heat energy released between two times after shutdown  (E = 0.0622 P0 [(t^0.8 - (t + T)^0.8)/0.8] from t1 to t2)
  P0 = 3000000000 W
  T_op = 31557600 s
  t1 = 10 s
  t2 = 86400 s
  assumes: 10 s <= t1 < t2 <= 1e8 s; constant power before shutdown
```

**What the worked example means:** This calculation determines the cumulative thermal energy from fission product decay between two specific times after reactor shutdown. Using the pre-shutdown thermal power 3000 MW and operating duration 3.15576e+07 s, it integrates the Way-Wigner correlation over the interval starting at 10 s and ending at 86400 s. The result, 1.56376e+12 J, represents the total joules of heat generated during this window, while 1.81011e+07 W indicates the average rate of that energy release.

<a id="nuc_xenon_equilibrium"></a>

#### Equilibrium iodine-135 and xenon-135 concentrations — `nuc_xenon_equilibrium`

The equilibrium I-135 and Xe-135 atom densities in fuel operating at a steady flux, the xenon absorptions per fission (its poisoning strength) and the flux at which xenon burn-out equals its decay.

$$
X_\infty = \frac{(\gamma_I+\gamma_X)\Sigma_f\phi}{\lambda_X+\sigma_X\phi}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Sigma_f` | in | macroscopic fission cross section of the fuel | 1/m |
| `phi` | in | thermal flux during operation | 1/(m^2*s) |
| `gamma_I` | in | cumulative fission yield of I-135 (e.g. 0.0639) | — |
| `gamma_X` | in | direct fission yield of Xe-135 (e.g. 0.00237) | — |
| `sigma_X` | in | Xe-135 absorption cross section (e.g. 2.65e6 b) | m^2 |
| `I_eq` | out | equilibrium I-135 atoms per volume | 1/m^3 |
| `X_eq` | out | equilibrium Xe-135 atoms per volume | 1/m^3 |
| `abs_per_fission` | out | Xe-135 absorptions per fission at equilibrium | — |

**Applies when:** constant flux; half-lives of I-135 (6.57 h) and Xe-135 (9.14 h) of radioactivedecay 0.6.1; Xe-135m neglected.

```shell
$ nuclear-engineering solve nuc_xenon_equilibrium "Sigma_f=0.1 1/cm" "phi=3e+13 1/(cm^2*s)" gamma_I=0.0639 gamma_X=0.00237 "sigma_X=2.65e-22 m^2"
I_eq = 6.54130685e+21 1/m^3
X_eq = 1.976915754e+21 1/m^3
abs_per_fission = 0.05238826748
  by: Equilibrium iodine-135 and xenon-135 concentrations  (I = gamma_I Sigma_f phi/lambda_I; X = (gamma_I + gamma_X) Sigma_f phi/(lambda_X + sigma_X phi))
  Sigma_f = 10 1/m
  phi = 3e+17 1/(m^2*s)
  gamma_I = 0.0639
  gamma_X = 0.00237
  sigma_X = 2.65e-22 m^2
  assumes: constant flux; half-lives of I-135 (6.57 h) and Xe-135 (9.14 h) of radioactivedecay 0.6.1; Xe-135m neglected
```

**What the worked example means:** This calculation determines the steady-state concentrations of iodine-135 and xenon-135 in a nuclear reactor core operating at constant flux. Using the macroscopic fission cross section 0.1 1/cm, thermal flux 3e+13 1/(cm^2*s), and fission yields 0.0639 for iodine and 0.00237 for direct xenon production gives the equilibrium inventory. The resulting iodine density is 6.54131e+21 1/m^3 atoms per cubic meter, while the xenon density is 1.97692e+21 1/m^3 atoms per cubic meter. This xenon level leads to 0.0523883 neutron absorptions by xenon for every fission event, which is critical for understanding reactor reactivity control.

<a id="nuc_flux_shape"></a>

#### Fundamental-mode flux shape of a bare reactor at a point — `nuc_flux_shape`

The neutron flux at a point of a bare slab, sphere or finite cylinder relative to its maximum at the centre (cosine, sin(x)/x, J0 times cosine) and the geometric buckling, for extrapolated dimensions.

$$
\frac{\phi}{\phi_{max}} = J_0\!\left(\frac{2.405\,r}{R}\right)\cos\frac{\pi z}{H}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `geometry` | in | shape: slab (a, x), sphere (R, r) or cylinder (R, H, r, z) | — |
| `a` | in | extrapolated slab thickness (optional) | m |
| `R` | in | extrapolated radius (optional) | m |
| `H` | in | extrapolated cylinder height (optional) | m |
| `x` | in | distance from the slab mid-plane (optional) | m |
| `r` | in | distance from the centre or axis (optional) | m |
| `z` | in | height above the cylinder mid-plane (optional) | m |
| `ratio` | out | phi/phi_max at the point | — |
| `B2` | out | geometric buckling | 1/m^2 |

**Applies when:** fundamental mode of a bare homogeneous reactor; extrapolated dimensions; the point inside the reactor.

```shell
$ nuclear-engineering solve nuc_flux_shape geometry=cylinder "R=1.5 m" "H=3 m" "r=75 cm" "z=50 cm"
ratio = 0.5801761727
B2 = 3.666927584 1/m^2
  by: Fundamental-mode flux shape of a bare reactor at a point  (slab cos(pi x/a); sphere sin(pi r/R)/(pi r/R); cylinder J0(2.405 r/R) cos(pi z/H))
  R = 1.5 m
  H = 3 m
  r = 0.75 m
  z = 0.5 m
  geometry = cylinder
  assumes: fundamental mode of a bare homogeneous reactor; extrapolated dimensions; the point inside the reactor
```

**What the worked example means:** This calculation determines the neutron flux distribution within a bare homogeneous reactor core. Using the fundamental mode solution for the selected geometry, it computes the ratio of the local flux to the maximum central flux as 0.580176. It also derives the geometric buckling 3.66693 1/m^2, which characterizes the curvature of the flux shape based on the extrapolated dimensions provided.

<a id="nuc_bare_reactor_keff"></a>

#### k-effective and geometric buckling of a bare reactor (one group) — `nuc_bare_reactor_keff`

The geometric buckling of a bare homogeneous slab, sphere, cylinder or rectangular reactor of stated size and its effective multiplication factor in one-group diffusion theory, with the non-leakage probability and the reactivity.

$$
k_{eff} = \frac{\nu\Sigma_f}{\Sigma_a + D B_g^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | diffusion coefficient (a length, e.g. cm) | m |
| `Sigma_a` | in | macroscopic absorption cross section | 1/m |
| `nuSigma_f` | in | nu times the macroscopic fission cross section | 1/m |
| `extrapolation` | in | extrapolation distance added at each free surface: none, 2D or 2.13D (= 0.71 transport mean free paths) | — |
| `geometry` | in | bare reactor shape: slab (a), sphere (R), cylinder (R, H) or cuboid (a, b, c) | — |
| `a` | in | slab thickness or first side of a cuboid (optional) | m |
| `b` | in | second side of a cuboid (optional) | m |
| `c` | in | third side of a cuboid (optional) | m |
| `R` | in | radius of a sphere or cylinder (optional) | m |
| `H` | in | height of a cylinder (optional) | m |
| `B2` | out | geometric buckling (extrapolated dimensions) | 1/m^2 |
| `k_eff` | out | effective multiplication factor | — |
| `P_NL` | out | non-leakage probability 1/(1 + L^2 B^2) | — |
| `rho` | out | reactivity (k - 1)/k | — |

**Applies when:** one-group diffusion; bare homogeneous reactor; the dimensions of the stated geometry only.

```shell
$ nuclear-engineering solve nuc_bare_reactor_keff "D=0.9 cm" "Sigma_a=0.15 1/cm" "nuSigma_f=0.18 1/cm" extrapolation=2.13D geometry=sphere "R=12 cm"
B2 = 509.4963634 1/m^2
k_eff = 0.9190487902
P_NL = 0.7658739918
rho = -0.08808151501
  by: k-effective and geometric buckling of a bare reactor (one group)  (B_g^2 from the shape (pi/a)^2, (pi/R)^2, (2.405/R)^2 + (pi/H)^2, sum (pi/x_i)^2; k = nu Sigma_f/(Sigma_a + D B_g^2))
  D = 0.009 m
  Sigma_a = 15 1/m
  nuSigma_f = 18 1/m
  R = 0.12 m
  geometry = sphere
  extrapolation = 2.13D
  assumes: one-group diffusion; bare homogeneous reactor; the dimensions of the stated geometry only
```

**What the worked example means:** This calculation determines the effective multiplication factor and geometric buckling for a bare homogeneous reactor using one-group diffusion theory. By combining material properties like the diffusion coefficient and absorption cross section with the physical dimensions of the shape, it computes the leakage probability. The resulting multiplication factor of 0.919049 indicates whether the system is subcritical, critical, or supercritical based on the balance between production and loss terms.

<a id="nuc_two_group_bare_keff"></a>

#### k-effective of a bare reactor in two-group diffusion theory — `nuc_two_group_bare_keff`

The effective and infinite multiplication factors of a bare homogeneous reactor in two-group (fast and thermal) diffusion theory, with fission in both groups and downscatter from fast to thermal, and the thermal-to-fast flux ratio of the fundamental mode.

$$
k = \frac{\nu\Sigma_{f1} + \nu\Sigma_{f2}\frac{\Sigma_{12}}{D_2B^2+\Sigma_{a2}}}{D_1B^2+\Sigma_{a1}+\Sigma_{12}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `geometry` | in | bare reactor shape: slab (a), sphere (R), cylinder (R, H) or cuboid (a, b, c) | — |
| `D1` | in | fast-group diffusion coefficient | m |
| `D2` | in | thermal-group diffusion coefficient | m |
| `Sigma_a1` | in | fast-group absorption cross section | 1/m |
| `Sigma_12` | in | downscatter (removal) cross section from fast to thermal | 1/m |
| `Sigma_a2` | in | thermal-group absorption cross section | 1/m |
| `nuSigma_f1` | in | fast-group nu Sigma_f | 1/m |
| `nuSigma_f2` | in | thermal-group nu Sigma_f | 1/m |
| `a` | in | slab thickness or first side of a cuboid (extrapolated) (optional) | m |
| `b` | in | second side of a cuboid (extrapolated) (optional) | m |
| `c` | in | third side of a cuboid (extrapolated) (optional) | m |
| `R` | in | radius of a sphere or cylinder (extrapolated) (optional) | m |
| `H` | in | height of a cylinder (extrapolated) (optional) | m |
| `B2` | out | geometric buckling | 1/m^2 |
| `k_eff` | out | effective multiplication factor | — |
| `k_inf` | out | infinite multiplication factor | — |
| `flux_ratio` | out | thermal/fast flux ratio of the fundamental mode | — |

**Applies when:** two-group diffusion, no upscatter; all fission neutrons born fast; extrapolated dimensions.

```shell
$ nuclear-engineering solve nuc_two_group_bare_keff geometry=cylinder "D1=1.43 cm" "D2=0.37 cm" "Sigma_a1=0.01 1/cm" "Sigma_12=0.018 1/cm" "Sigma_a2=0.08 1/cm" "nuSigma_f1=0.005 1/cm" "nuSigma_f2=0.135 1/cm" "R=170 cm" "H=370 cm"
B2 = 2.72203772 1/m^2
k_eff = 1.244724886
k_inf = 1.263392857
flux_ratio = 0.2247170941
  by: k-effective of a bare reactor in two-group diffusion theory  (k = [nu Sigma_f1 + nu Sigma_f2 Sigma_12/(D2 B^2 + Sigma_a2)] / (D1 B^2 + Sigma_a1 + Sigma_12))
  D1 = 0.0143 m
  D2 = 0.0037 m
  Sigma_a1 = 1 1/m
  Sigma_12 = 1.8 1/m
  Sigma_a2 = 8 1/m
  nuSigma_f1 = 0.5 1/m
  nuSigma_f2 = 13.5 1/m
  R = 1.7 m
  H = 3.7 m
  geometry = cylinder
  assumes: two-group diffusion, no upscatter; all fission neutrons born fast; extrapolated dimensions
```

**What the worked example means:** This calculation determines the criticality state of a bare homogeneous reactor using two-group diffusion theory. By combining material properties like diffusion coefficients and cross sections with the geometric buckling 2.72204 1/m^2, it yields an effective multiplication factor 1.24472 and an infinite multiplication factor 1.26339. The result indicates whether the system is subcritical, critical, or supercritical, while also providing the thermal-to-fast flux ratio 0.224717 to describe the neutron spectrum.

<a id="nuc_critical_cylinder_minimum_volume"></a>

#### Minimum-volume critical bare cylinder (one group) — `nuc_critical_cylinder_minimum_volume`

The radius and height of the bare cylindrical reactor that is critical with the smallest volume (extrapolated height-to-diameter ratio 0.924), and that volume.

$$
H_{ext} = \frac{\sqrt3\,\pi}{B_m},\quad R_{ext} = \sqrt{\tfrac32}\,\frac{2.405}{B_m}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | diffusion coefficient (a length, e.g. cm) | m |
| `Sigma_a` | in | macroscopic absorption cross section | 1/m |
| `nuSigma_f` | in | nu times the macroscopic fission cross section | 1/m |
| `extrapolation` | in | extrapolation distance added at each free surface: none, 2D or 2.13D (= 0.71 transport mean free paths) | — |
| `R` | out | physical radius | m |
| `H` | out | physical height | m |
| `V_ext` | out | minimum critical volume (extrapolated dimensions) | m^3 |
| `HD` | out | extrapolated height/diameter | — |

**Applies when:** one-group diffusion; k_inf > 1.

```shell
$ nuclear-engineering solve nuc_critical_cylinder_minimum_volume "D=0.9 cm" "Sigma_a=0.15 1/cm" "nuSigma_f=0.18 1/cm" extrapolation=none
R = 0.1613206026 m
H = 0.298037648 m
V_ext = 0.02436692191 m^3
HD = 0.9237432886
  by: Minimum-volume critical bare cylinder (one group)  (minimise pi R^2 H subject to (j01/R)^2 + (pi/H)^2 = B_m^2: H_ext = sqrt(3) pi/B_m, R_ext = sqrt(3/2) j01/B_m)
  D = 0.009 m
  Sigma_a = 15 1/m
  nuSigma_f = 18 1/m
  extrapolation = none
  assumes: one-group diffusion; k_inf > 1
```

**What the worked example means:** This calculation determines the smallest possible dimensions for a bare cylindrical reactor core to reach criticality under one-group diffusion theory. By minimizing the extrapolated volume subject to the buckling constraint, it yields an optimal height-to-diameter ratio. The resulting physical radius is 0.161321 m and the height is 0.298038 m, producing a total minimum volume of 0.0243669 m^3. These values depend on the material properties such as the diffusion coefficient and macroscopic cross sections.

<a id="nuc_six_factor_formula"></a>

#### Multiplication factor by the four- and six-factor formulas — `nuc_six_factor_formula`

k_inf = eta f p eps (four-factor formula) and, with the fast and thermal non-leakage probabilities, k_eff = k_inf P_FNL P_TNL (six-factor formula), with the reactivity (k - 1)/k.

$$
k_{eff} = \eta f p \varepsilon\,P_{FNL}P_{TNL}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `eta` | in | reproduction factor eta | — |
| `f` | in | thermal utilization f | — |
| `p` | in | resonance escape probability p | — |
| `eps` | in | fast fission factor epsilon | — |
| `P_FNL` | in | fast non-leakage probability (optional) (optional) | — |
| `P_TNL` | in | thermal non-leakage probability (optional) (optional) | — |
| `k_inf` | out | infinite multiplication factor | — |
| `k_eff` | out | effective multiplication factor (k_inf without leakage factors) | — |
| `P_NL` | out | total non-leakage probability | — |
| `rho` | out | reactivity (k_eff - 1)/k_eff | — |

**Applies when:** 0 < f, p, P_FNL, P_TNL <= 1; missing non-leakage factors are taken as 1 (an infinite medium).

```shell
$ nuclear-engineering solve nuc_six_factor_formula eta=2.02 f=0.71 p=0.87 eps=1.03
k_inf = 1.28518662
k_eff = 1.28518662
P_NL = 1
rho = 0.221902886
  by: Multiplication factor by the four- and six-factor formulas  (k_inf = eta f p eps; k_eff = k_inf P_FNL P_TNL)
  eta = 2.02
  f = 0.71
  p = 0.87
  eps = 1.03
  assumes: 0 < f, p, P_FNL, P_TNL <= 1; missing non-leakage factors are taken as 1 (an infinite medium)
```

**What the worked example means:** This calculation determines the neutron economy of the reactor by applying the four-factor formula to find the infinite multiplication factor, which equals 1.28519. When non-leakage probabilities are included, the effective multiplication factor becomes 1.28519. The resulting reactivity, defined as the ratio of the excess neutrons to the total population, is calculated as 0.221903. These values indicate whether the system is subcritical, critical, or supercritical based on the balance between neutron production via fission and losses due to absorption and leakage.

<a id="nuc_power_from_reaction_rate"></a>

#### Power density and power from the fission reaction rate — `nuc_power_from_reaction_rate`

The fission reaction rate density R = N sigma_f phi, the power density q''' = E_f N sigma_f phi and, with a volume, the power, from the fissile number density, the fission cross section, the flux and the energy per fission.

$$
q''' = E_f N \sigma_f \phi,\quad P = q''' V
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `E_f` | in | recoverable energy per fission (e.g. 200 MeV) | J |
| `N` | in | fissile atoms per unit volume | 1/m^3 |
| `sigma` | in | fission cross section (barns) | m^2 |
| `phi` | in | neutron flux | 1/(m^2*s) |
| `V` | in | volume (optional, for the power) (optional) | m^3 |
| `R` | out | fission rate density (1/(m^3 s)) | 1/(m^3*s) |
| `q` | out | power density | W/m^3 |
| `q_MW_m3` | out | power density in MW/m^3 | — |
| `P` | out | power E_f N sigma phi V | W |
| `P_MW` | out | power in MW | — |

**Applies when:** one-group cross section and flux; uniform over the volume.

```shell
$ nuclear-engineering solve nuc_power_from_reaction_rate "E_f=200 MeV" "N=1e+21 1/cm^3" "sigma=5.826e-26 m^2" "phi=3e+13 1/(cm^2*s)"
R = 1.7478e+19 1/(m^3*s)
q = 560056864.2 W/m^3
q_MW_m3 = 560.0568642
  by: Power density and power from the fission reaction rate  (R = N sigma phi, q''' = E_f R, P = q''' V)
  E_f = 3.204353268e-11 J
  N = 1e+27 1/m^3
  sigma = 5.826e-26 m^2
  phi = 3e+17 1/(m^2*s)
  assumes: one-group cross section and flux; uniform over the volume
```

In the worked example, E_f = 200 MeV, N = 1e+21 1/cm^3, sigma = 5.826e-26 m^2, phi = 3e+13 1/(cm^2*s) go through the equation above and give R = 1.7478e+19 1/(m^3*s), q = 560056864.2 W/m^3, q_MW_m3 = 560.0568642. It applies only when one-group cross section and flux; uniform over the volume: outside that it says what is wrong instead of returning a number.

<a id="nuc_core_power_from_flux"></a>

#### Reactor power from the peak flux (or peak flux from power) of a bare core — `nuc_core_power_from_flux`

The thermal power of a bare homogeneous sphere, cylinder or rectangular core from its peak thermal flux (or the peak flux from the power), with the average flux, the peak-to-average ratio and the volume, for the fundamental-mode flux shape.

$$
P = E_f\Sigma_f\int_V\phi\,dV = E_f\Sigma_f\,\phi_{max}\,\frac{\bar\phi}{\phi_{max}}\,V
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `geometry` | in | core shape: sphere (R), cylinder (R, H) or cuboid (a, b, c) | — |
| `Sigma_f` | in | macroscopic fission cross section | 1/m |
| `E_f` | in | recoverable energy per fission (e.g. 200 MeV) | J |
| `phi_max` | in | peak (centre) thermal flux (give this or P) (optional) | 1/(m^2*s) |
| `P` | in | thermal power (give this or phi_max) (optional) | W |
| `R` | in | extrapolated radius (optional) | m |
| `H` | in | extrapolated cylinder height (optional) | m |
| `a` | in | first extrapolated side of a cuboid (optional) | m |
| `b` | in | second side (optional) | m |
| `c` | in | third side (optional) | m |
| `P` | out | thermal power | W |
| `phi_max` | out | peak flux | 1/(m^2*s) |
| `phi_avg` | out | volume-average flux | 1/(m^2*s) |
| `peaking` | out | peak-to-average flux ratio | — |

**Applies when:** fundamental-mode flux of a bare homogeneous core; extrapolated dimensions taken as the core size; uniform Sigma_f.

```shell
$ nuclear-engineering solve nuc_core_power_from_flux geometry=cylinder "Sigma_f=0.002 1/cm" "E_f=200 MeV" "phi_max=1e+13 1/(cm^2*s)" "R=170 cm" "H=370 cm"
P = 5917484.501 W
phi_max = 1e+17 1/(m^2*s)
phi_avg = 2.74863647e+16 1/(m^2*s)
peaking = 3.638167546
  by: Reactor power from the peak flux (or peak flux from power) of a bare core  (P = E_f Sigma_f phi_max <shape> V; <shape> = 3/pi^2 (sphere), (2 J1(2.405)/2.405)(2/pi) (cylinder), (2/pi)^3 (cuboid))
  Sigma_f = 0.2 1/m
  E_f = 3.204353268e-11 J
  phi_max = 1e+17 1/(m^2*s)
  R = 1.7 m
  H = 3.7 m
  geometry = cylinder
  assumes: fundamental-mode flux of a bare homogeneous core; extrapolated dimensions taken as the core size; uniform Sigma_f
```

**What the worked example means:** This calculation determines the total thermal power, 5.91748e+06 W, for a bare homogeneous core based on its fundamental-mode flux distribution. It uses the provided peak flux, 1e+17 1/(m^2*s), along with the geometry-specific average-to-peak ratio and volume to integrate the flux over the entire core. The result represents the energy released per second, assuming uniform fission cross sections and extrapolated dimensions define the effective active volume.

<a id="nuc_samarium"></a>

#### Samarium-149 poisoning: equilibrium and after shutdown — `nuc_samarium`

The equilibrium Pm-149 and Sm-149 atom densities during operation (Sm-149 is independent of the flux), the Sm-149 level reached after shutdown when all the promethium has decayed, and (optionally) the level a time t after shutdown.

$$
S_\infty = \frac{\gamma_{Pm}\Sigma_f}{\sigma_{Sm}},\quad P_\infty = \frac{\gamma_{Pm}\Sigma_f\phi}{\lambda_{Pm}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Sigma_f` | in | macroscopic fission cross section | 1/m |
| `phi` | in | thermal flux during operation | 1/(m^2*s) |
| `gamma_Pm` | in | fission yield of Pm-149 (e.g. 0.0113) | — |
| `sigma_Sm` | in | Sm-149 absorption cross section (e.g. 40800 b) | m^2 |
| `t` | in | time after shutdown (optional) (optional) | s |
| `Pm_eq` | out | equilibrium Pm-149 | 1/m^3 |
| `Sm_eq` | out | equilibrium Sm-149 | 1/m^3 |
| `Sm_max` | out | Sm-149 after all Pm-149 has decayed | 1/m^3 |
| `max_ratio` | out | Sm_max/Sm_eq | — |
| `Sm_t` | out | Sm-149 at t after shutdown (when t is given) | 1/m^3 |

**Applies when:** constant flux before shutdown; Pm-149 half-life of radioactivedecay 0.6.1 (53.08 h); Pm-149 absorption neglected.

```shell
$ nuclear-engineering solve nuc_samarium "Sigma_f=0.1 1/cm" "phi=3e+13 1/(cm^2*s)" gamma_Pm=0.0113 "sigma_Sm=4.08e-24 m^2"
Pm_eq = 9.345609968e+21 1/m^3
Sm_eq = 2.769607843e+22 1/m^3
Sm_max = 3.70416884e+22 1/m^3
max_ratio = 1.337434413
  by: Samarium-149 poisoning: equilibrium and after shutdown  (Pm = gamma Sigma_f phi/lambda_Pm; Sm = gamma Sigma_f/sigma_Sm; after shutdown Sm + Pm(1 - e^(-lambda_Pm t)))
  Sigma_f = 10 1/m
  phi = 3e+17 1/(m^2*s)
  gamma_Pm = 0.0113
  sigma_Sm = 4.08e-24 m^2
  assumes: constant flux before shutdown; Pm-149 half-life of radioactivedecay 0.6.1 (53.08 h); Pm-149 absorption neglected
```

**What the worked example means:** This calculation determines the steady-state concentration of samarium-149 during reactor operation, which is independent of neutron flux and equals 2.76961e+22 1/m^3. It also computes the peak samarium-149 level reached after shutdown once all promethium-149 has decayed, given by 3.70417e+22 1/m^3. The ratio of these values, 1.33743, indicates the severity of the poisoning effect relative to the operating state.

<a id="nuc_depletion_constant_flux"></a>

#### U-235 depletion and energy released at constant flux — `nuc_depletion_constant_flux`

How much of an initial mass of U-235 remains after irradiation at a constant flux, the fraction burnt, the fission energy released (J and MWd) and the power at the start and at the end.

$$
N(t) = N_0e^{-\sigma_a\phi t},\quad E = E_f\frac{\sigma_f}{\sigma_a}\left(N_0-N\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m0` | in | initial mass of U-235 | kg |
| `sigma_a` | in | U-235 absorption cross section (barns) | m^2 |
| `sigma_f` | in | U-235 fission cross section (barns) | m^2 |
| `phi` | in | neutron flux | 1/(m^2*s) |
| `t` | in | irradiation time | s |
| `E_f` | in | recoverable energy per fission (e.g. 200 MeV) | J |
| `m` | out | U-235 mass remaining | kg |
| `burnt` | out | fraction of U-235 consumed | — |
| `E` | out | fission energy released | J |
| `E_MWd` | out | energy released in MWd | — |
| `P_t` | out | fission power at t | W |

**Applies when:** constant flux, one group; U-235 only (no breeding); M(U-235) = 235.043928 g/mol.

```shell
$ nuclear-engineering solve nuc_depletion_constant_flux "m0=1 kg" "sigma_a=6.809e-26 m^2" "sigma_f=5.826e-26 m^2" "phi=3e+13 1/(cm^2*s)" "t=31557600 s" "E_f=200 MeV"
m = 0.5248582278 kg
burnt = 0.4751417722
E = 3.337740854e+13 J
E_MWd = 386.3125989
P_t = 753140.4958 W
  by: U-235 depletion and energy released at constant flux  (N = N0 e^(-sigma_a phi t); E = E_f (sigma_f/sigma_a) (N0 - N); P = E_f sigma_f phi N)
  m0 = 1 kg
  sigma_a = 6.809e-26 m^2
  sigma_f = 5.826e-26 m^2
  phi = 3e+17 1/(m^2*s)
  t = 31557600 s
  E_f = 3.204353268e-11 J
  assumes: constant flux, one group; U-235 only (no breeding); M(U-235) = 235.043928 g/mol
```

**What the worked example means:** This calculation determines the remaining mass of uranium-235 and the total energy generated during constant flux irradiation. Starting with an initial mass of 1 kg, the model applies exponential decay based on the absorption cross section 6.809e-26 m^2 and neutron flux 3e+13 1/(cm^2*s) over time 3.15576e+07 s. The result shows that 0.524858 kg of fuel is left, meaning a fraction of 0.475142 was consumed. This consumption produced 3.33774e+13 J of fission energy, equivalent to 386.313 megawatt days, assuming a recoverable energy per fission of 200 MeV.

<a id="nuc_constant_power_depletion"></a>

#### U-235 depletion at constant power — `nuc_constant_power_depletion`

How much of an initial mass of U-235 remains after operation at a constant power (the flux rising as the fuel is consumed), the fraction burnt, the mass consumed and the factor by which the flux has risen.

$$
N(t) = N_0 - \frac{P\,t}{E_f}\frac{\sigma_a}{\sigma_f},\quad \frac{\phi(t)}{\phi_0} = \frac{N_0}{N(t)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m0` | in | initial mass of U-235 | kg |
| `P` | in | constant thermal power | W |
| `t` | in | operating time | s |
| `E_f` | in | recoverable energy per fission (e.g. 200 MeV) | J |
| `sigma_a` | in | U-235 absorption cross section (barns) | m^2 |
| `sigma_f` | in | U-235 fission cross section (barns) | m^2 |
| `m` | out | U-235 mass remaining | kg |
| `burnt` | out | fraction of U-235 consumed | — |
| `consumed` | out | U-235 mass consumed | kg |
| `flux_ratio` | out | flux at t over the initial flux (N0/N) | — |
| `fissions` | out | number of fissions P t/E_f | — |

**Applies when:** constant power, one group; U-235 only (no breeding); M(U-235) = 235.043928 g/mol.

```shell
$ nuclear-engineering solve nuc_constant_power_depletion "m0=100 kg" "P=30 MW" "t=31557600 s" "E_f=200 MeV" "sigma_a=6.809e-26 m^2" "sigma_f=5.826e-26 m^2"
m = 86.52291956 kg
burnt = 0.1347708044
consumed = 13.47708044 kg
flux_ratio = 1.155763126
fissions = 2.954505702e+25
  by: U-235 depletion at constant power  (fissions = P t/E_f; atoms absorbed = fissions sigma_a/sigma_f; N = N0 - P t sigma_a/(E_f sigma_f); phi/phi0 = N0/N)
  m0 = 100 kg
  P = 30000000 W
  t = 31557600 s
  E_f = 3.204353268e-11 J
  sigma_a = 6.809e-26 m^2
  sigma_f = 5.826e-26 m^2
  assumes: constant power, one group; U-235 only (no breeding); M(U-235) = 235.043928 g/mol
```

**What the worked example means:** This calculation determines the amount of Uranium-235 fuel left after operating a reactor core at a constant thermal power. By converting the total energy produced into the number of fissions and then accounting for neutron absorption versus fission probabilities, it computes the remaining atom count and mass. The result shows that as fuel is consumed, the neutron flux must increase to maintain constant power, yielding a final mass of 86.5229 kg kg and a flux ratio of 1.15576.

<a id="nuc_xenon_after_shutdown"></a>

#### Xenon-135 after shutdown: concentration and peak — `nuc_xenon_after_shutdown`

The Xe-135 and I-135 atom densities a time t after shutdown from equilibrium operation, the xenon relative to its equilibrium value, and the time and height of the post-shutdown xenon peak (none at low flux).

$$
X(t) = X_0e^{-\lambda_Xt} + \frac{\lambda_II_0}{\lambda_X-\lambda_I}\left(e^{-\lambda_It}-e^{-\lambda_Xt}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Sigma_f` | in | macroscopic fission cross section of the fuel | 1/m |
| `phi` | in | thermal flux during operation | 1/(m^2*s) |
| `gamma_I` | in | cumulative fission yield of I-135 (e.g. 0.0639) | — |
| `gamma_X` | in | direct fission yield of Xe-135 (e.g. 0.00237) | — |
| `sigma_X` | in | Xe-135 absorption cross section (e.g. 2.65e6 b) | m^2 |
| `t` | in | time after shutdown | s |
| `X` | out | Xe-135 atoms per volume at t | 1/m^3 |
| `ratio` | out | X(t)/X_eq | — |
| `t_peak` | out | time of the xenon peak after shutdown (0 when xenon only falls) | s |
| `peak_ratio` | out | peak xenon / equilibrium xenon | — |
| `I` | out | I-135 atoms per volume at t | 1/m^3 |

**Applies when:** equilibrium before shutdown; no flux after shutdown.

```shell
$ nuclear-engineering solve nuc_xenon_after_shutdown "Sigma_f=0.1 1/cm" "phi=3e+13 1/(cm^2*s)" gamma_I=0.0639 gamma_X=0.00237 "sigma_X=2.65e-22 m^2" "t=10 h"
X = 3.723382215e+21 1/m^3
ratio = 1.883429887
t_peak = 30167.01507 s
peak_ratio = 1.901549197
I = 2.277591284e+21 1/m^3
  by: Xenon-135 after shutdown: concentration and peak  (X(t) = X0 e^(-lambda_X t) + I0 lambda_I/(lambda_X - lambda_I)(e^(-lambda_I t) - e^(-lambda_X t)); dX/dt = 0 for the peak)
  Sigma_f = 10 1/m
  phi = 3e+17 1/(m^2*s)
  gamma_I = 0.0639
  gamma_X = 0.00237
  sigma_X = 2.65e-22 m^2
  t = 36000 s
  assumes: equilibrium before shutdown; no flux after shutdown
```

**What the worked example means:** This calculation determines the xenon-135 density 3.72338e+21 1/m^3 and iodine-135 density 2.27759e+21 1/m^3 after a shutdown from equilibrium. The result shows that xenon rises to a peak value 1.90155 times its original level at time 30167 s, meaning immediate restart may be difficult due to negative reactivity.

### Shielding (1)

<a id="nuc_half_value_layer"></a>

#### Half-value and tenth-value layers — `nuc_half_value_layer`

The half-value layer ln 2/mu, tenth-value layer ln 10/mu and mean free path 1/mu of a narrow beam in an absorber of linear attenuation coefficient mu, and with a thickness the transmission and number of half-value layers.

$$
HVL = \frac{\ln 2}{\mu},\quad TVL = \frac{\ln 10}{\mu}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `mu` | in | linear attenuation coefficient | 1/m |
| `x` | in | absorber thickness (optional) (optional) | m |
| `HVL` | out | half-value layer | m |
| `TVL` | out | tenth-value layer | m |
| `mfp` | out | mean free path 1/mu | m |
| `HVL_cm` | out | half-value layer in cm | — |
| `TVL_cm` | out | tenth-value layer in cm | — |
| `transmission` | out | narrow-beam transmission exp(-mu x) | — |
| `n_HVL` | out | thickness in half-value layers | — |

**Applies when:** narrow beam (no buildup); mu > 0.

```shell
$ nuclear-engineering solve nuc_half_value_layer "mu=0.656 1/cm"
HVL = 0.010566268 m
TVL = 0.03510038252 m
mfp = 0.01524390244 m
HVL_cm = 1.0566268
TVL_cm = 3.510038252
  by: Half-value and tenth-value layers  (HVL = ln 2/mu, TVL = ln 10/mu, T = e^(-mu x))
  mu = 65.6 1/m
  assumes: narrow beam (no buildup); mu > 0
```

**What the worked example means:** The half-value layer is 0.0105663 m and the tenth-value layer is 0.0351004 m, with a mean free path of 0.0152439 m.

### Shielding and dose (10)

<a id="nuc_accumulated_dose_decaying_source"></a>

#### Dose accumulated near a decaying source — `nuc_accumulated_dose_decaying_source`

The dose accumulated over an exposure time when the dose rate falls with the source's half-life, the dose if the exposure lasted until complete decay, and the mean dose rate.

$$
D = \dot H_0\,e^{-\lambda t_d}\,\frac{1-e^{-\lambda t}}{\lambda}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `H0` | in | dose rate at the reference time | J/(kg*s) |
| `T_half` | in | half-life of the source | s |
| `t` | in | exposure time | s |
| `td` | in | delay from the reference time to the start (0 when omitted) (optional) | s |
| `D` | out | dose accumulated | J/kg |
| `D_mSv` | out | dose in mSv (or mGy) | — |
| `D_inf` | out | dose until complete decay | J/kg |
| `H_mean` | out | mean dose rate during the exposure | J/(kg*s) |

**Applies when:** the dose rate follows one half-life.

```shell
$ nuclear-engineering solve nuc_accumulated_dose_decaying_source "H0=0.001 (J/kg)/h" "T_half=6586.2 s" "t=2 h"
D = 0.001402260154 J/kg
D_mSv = 1.402260154
D_inf = 0.002639410577 J/kg
H_mean = 1.947583547e-07 J/(kg*s)
  by: Dose accumulated near a decaying source  (D = H0 e^(-lambda td) (1 - e^(-lambda t))/lambda)
  H0 = 2.777777778e-07 J/(kg*s)
  T_half = 6586.2 s
  t = 7200 s
  assumes: the dose rate follows one half-life
```

**What the worked example means:** The accumulated dose is 0.00140226 J/kg, equivalent to 1.40226, with a mean rate of 1.94758e-07 J/(kg*s).

<a id="nuc_point_source_dose_rate"></a>

#### Dose rate from a shielded point gamma source with buildup — `nuc_point_source_dose_rate`

The dose rate at a distance from a point gamma source of stated dose-rate constant, behind a slab shield, with the buildup factor in the stated form (none, linear or Taylor), plus the buildup factor and the transmission.

$$
\dot H = \frac{\Gamma A}{r^2}\,B(\mu x)\,e^{-\mu x}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Gamma` | in | dose-rate constant of the nuclide (e.g. uSv*m^2/(MBq*h)) | J*m^2/kg |
| `A` | in | source activity | 1/s |
| `r` | in | source-to-point distance | m |
| `mu` | in | linear attenuation coefficient of the shield | 1/m |
| `x` | in | shield thickness (0 for none) | m |
| `buildup` | in | buildup form: none (B = 1), linear (B = 1 + mu x) or taylor | — |
| `A1` | in | Taylor buildup parameter A1 (only with buildup = taylor) (optional) | — |
| `alpha1` | in | Taylor buildup parameter alpha1 (only with buildup = taylor) (optional) | — |
| `alpha2` | in | Taylor buildup parameter alpha2 (only with buildup = taylor) (optional) | — |
| `H_dot` | out | dose rate (same quantity as Gamma: Sv/s, Gy/s) | J/(kg*s) |
| `H_uSv_h` | out | dose rate in uSv/h (or uGy/h) | — |
| `B` | out | buildup factor | — |
| `T` | out | transmission B e^(-mu x) | — |

**Applies when:** point isotropic source; shield between source and point (x < r); Taylor parameters only with the Taylor form.

```shell
$ nuclear-engineering solve nuc_point_source_dose_rate "Gamma=3.51e-13 (J/kg)*m^2/((1/s)*h)" "A=3.7e+11 (1/s)" "r=2 m" "mu=0.0656 1/cm" "x=10 cm" buildup=linear
H_dot = 7.750139161e-06 J/(kg*s)
H_uSv_h = 27900.50098
B = 1.656
T = 0.8593362895
  by: Dose rate from a shielded point gamma source with buildup  (H = Gamma A B(mu x) e^(-mu x) / r^2; B = 1, 1 + mu x, or A1 e^(-alpha1 mu x) + (1 - A1) e^(-alpha2 mu x))
  Gamma = 9.75e-17 J*m^2/kg
  A = 3.7e+11 1/s
  r = 2 m
  mu = 6.56 1/m
  x = 0.1 m
  buildup = linear
  assumes: point isotropic source; shield between source and point (x < r); Taylor parameters only with the Taylor form
```

**What the worked example means:** This calculation determines the dose rate, 7.75014e-06 J/(kg*s), from a point gamma source with activity 3.7e+11 (1/s) located at a distance 2 m. It accounts for geometric spreading and attenuation through a shield of thickness 10 cm with linear attenuation coefficient 0.0656 1/cm. The result incorporates the buildup factor 1.656 to approximate scattered radiation, yielding a total transmission of 0.859336.

<a id="nuc_transit_dose"></a>

#### Dose received passing a point source — `nuc_transit_dose`

The dose received by a person or detector moving at constant speed along a straight path past an unshielded point source, from the closest distance, the speed and the path length (centred on the closest approach).

$$
D = \frac{\Gamma A}{d\,v}\,2\arctan\frac{L}{2d}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Gamma` | in | dose-rate constant of the nuclide | J*m^2/kg |
| `A` | in | source activity | 1/s |
| `d` | in | closest distance to the source | m |
| `v` | in | speed along the path | m/s |
| `L` | in | length of the path, centred on the point of closest approach | m |
| `D` | out | dose received (Sv or Gy, as Gamma) | J/kg |
| `D_uSv` | out | dose in uSv (or uGy) | — |
| `frac` | out | fraction of the dose of an infinitely long path | — |

**Applies when:** unshielded point source; straight path at constant speed.

```shell
$ nuclear-engineering solve nuc_transit_dose "Gamma=3.51e-13 (J/kg)*m^2/((1/s)*h)" "A=3.7e+12 (1/s)" "d=2 m" "v=1 m/s" "L=20 m"
D = 0.0004954543267 J/kg
D_uSv = 495.4543267
frac = 0.8743340836
  by: Dose received passing a point source  (D = (Gamma A/(d v)) 2 atan(L/(2 d)); infinite path Gamma A pi/(d v))
  Gamma = 9.75e-17 J*m^2/kg
  A = 3.7e+12 1/s
  d = 2 m
  v = 1 m/s
  L = 20 m
  assumes: unshielded point source; straight path at constant speed
```

**What the worked example means:** This calculation determines the total radiation dose accumulated by a person moving at constant speed past an unshielded point source. Using the source activity 3.7e+12 (1/s), dose-rate constant 3.51e-13 (J/kg)*m^2/((1/s)*h), closest approach distance 2 m, speed 1 m/s, and path length 20 m, the model integrates the inverse-square law over time. The resulting dose is 0.000495454 J/kg gray or sievert, equivalent to 495.454 microsieverts. This value represents 0.874334 of the maximum possible dose for an infinitely long path, indicating how much of the potential exposure was captured during the specific transit segment.

<a id="nuc_klein_nishina"></a>

#### Klein-Nishina Compton cross sections per electron — `nuc_klein_nishina`

The total Compton (Klein-Nishina) cross section per electron for a photon energy, the energy-transfer cross section and the mean kinetic energy given to the recoil electron.

$$
\sigma = 2\pi r_e^2\left\{\frac{1+k}{k^2}\left[\frac{2(1+k)}{1+2k}-\frac{\ln(1+2k)}{k}\right]+\frac{\ln(1+2k)}{2k}-\frac{1+3k}{(1+2k)^2}\right\}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `E` | in | photon energy (10 keV to 100 MeV) | J |
| `sigma` | out | Klein-Nishina cross section per electron | m^2 |
| `sigma_b` | out | the same in barns per electron | — |
| `sigma_tr` | out | energy-transfer cross section per electron | m^2 |
| `T_mean` | out | mean kinetic energy of the Compton electron (MeV) | MeV |

**Applies when:** free electrons at rest (no binding); 10 keV <= E <= 100 MeV.

```shell
$ nuclear-engineering solve nuc_klein_nishina "E=0.662 MeV"
sigma = 2.561403522e-29 m^2
sigma_b = 0.2561403522
sigma_tr = 9.774937387e-30 m^2
T_mean = 0.252635264 MeV
  by: Klein-Nishina Compton cross sections per electron  (closed Klein-Nishina integrals over the scattering angle, k = E/(m_e c^2); T_mean = E sigma_tr/sigma)
  E = 1.060640932e-13 J
  assumes: free electrons at rest (no binding); 10 keV <= E <= 100 MeV
```

**What the worked example means:** This calculation determines the probability of Compton scattering for a photon with energy 0.662 MeV. It yields the total cross section 2.5614e-29 m^2 and its conversion to barns 0.25614, alongside the energy-transfer cross section 9.77494e-30 m^2. The mean kinetic energy imparted to the recoil electron is reported as 0.252635 MeV MeV, providing insight into how much energy the photon loses during the interaction.

<a id="nuc_attenuation_coefficient_fit"></a>

#### Linear attenuation coefficient from narrow-beam transmission measurements — `nuc_attenuation_coefficient_fit`

The linear attenuation coefficient of an absorber from intensities measured behind several thicknesses (least-squares fit of ln I against x), with the half- and tenth-value layers and the fitted unattenuated intensity.

$$
\ln I = \ln I_0 - \mu x
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | absorber thicknesses, in cm | — |
| `I` | in | measured intensities or count rates (any consistent unit) | — |
| `mu` | out | fitted linear attenuation coefficient | 1/m |
| `HVL` | out | half-value layer | m |
| `TVL` | out | tenth-value layer | m |
| `I0` | out | fitted intensity with no absorber | — |

**Applies when:** narrow-beam geometry (no buildup); background already subtracted; unweighted least squares of ln I.

```shell
$ nuclear-engineering solve nuc_attenuation_coefficient_fit "x=[0, 1, 2, 3, 4]" "I=[1000, 520, 270, 141, 73]"
mu = 65.39660597 1/m
HVL = 0.0105991308 m
TVL = 0.03520955039 m
I0 = 1000.021917
  by: Linear attenuation coefficient from narrow-beam transmission measurements  (least squares of ln I = ln I0 - mu x; HVL = ln 2/mu, TVL = ln 10/mu)
  assumes: narrow-beam geometry (no buildup); background already subtracted; unweighted least squares of ln I
```

**What the worked example means:** This calculation determines the linear attenuation coefficient by performing an unweighted least-squares fit of the natural logarithm of measured intensities against absorber thicknesses. The resulting coefficient, 65.3966 1/m, quantifies how rapidly radiation intensity decreases per unit distance in the material. From this value, the half-value layer (0.0105991 m) and tenth-value layer (0.0352096 m) are derived to characterize shielding effectiveness, while the intercept provides the estimated unattenuated intensity (1000.02).

<a id="nuc_multilayer_transmission"></a>

#### Narrow-beam transmission through a multilayer shield — `nuc_multilayer_transmission`

The uncollided (narrow-beam) transmission through several shield layers, the total optical thickness, its value in half-value layers, the attenuation factor and (optionally) the dose rate behind the shield.

$$
T = \exp\Big(-\sum_i \mu_i x_i\Big)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `mu` | in | linear attenuation coefficient of each layer, in 1/cm | — |
| `x` | in | thickness of each layer, in cm | — |
| `H0` | in | uncollided dose rate in front of the shield (optional) (optional) | J/(kg*s) |
| `T` | out | uncollided transmission | — |
| `tau` | out | total optical thickness sum mu_i x_i | — |
| `n_hvl` | out | thickness in half-value layers | — |
| `attenuation` | out | attenuation factor 1/T | — |
| `H` | out | uncollided dose rate behind the shield (when H0 is given) | J/(kg*s) |

**Applies when:** narrow beam (no buildup); layers perpendicular to the beam.

```shell
$ nuclear-engineering solve nuc_multilayer_transmission "mu=[0.656, 0.149]" "x=[5, 30]"
T = 0.0004307425406
tau = 7.75
n_hvl = 11.18088657
attenuation = 2321.572415
  by: Narrow-beam transmission through a multilayer shield  (T = exp(-sum mu_i x_i))
  assumes: narrow beam (no buildup); layers perpendicular to the beam
```

**What the worked example means:** The uncollided transmission is 0.000430743, corresponding to a total optical thickness of 7.75 and an attenuation factor of 2321.57.

<a id="nuc_shield_thickness_for_dose_rate"></a>

#### Shield thickness for a target dose rate (point source, buildup) — `nuc_shield_thickness_for_dose_rate`

The slab thickness that brings the dose rate from a point gamma source down to a target value, with no buildup or the linear buildup form B = 1 + mu x (the root of the attenuation equation).

$$
\frac{\Gamma A}{r^2}B(\mu x)e^{-\mu x} = \dot H_{target}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Gamma` | in | dose-rate constant of the nuclide | J*m^2/kg |
| `A` | in | source activity | 1/s |
| `r` | in | source-to-point distance | m |
| `mu` | in | linear attenuation coefficient of the shield | 1/m |
| `H_target` | in | target dose rate (e.g. uSv/h) | J/(kg*s) |
| `buildup` | in | buildup form: none (B = 1) or linear (B = 1 + mu x) | — |
| `x` | out | required shield thickness | m |
| `n_mfp` | out | thickness in mean free paths | — |
| `B` | out | buildup factor at that thickness | — |

**Applies when:** target below the unshielded dose rate; the shield fits between source and point.

```shell
$ nuclear-engineering solve nuc_shield_thickness_for_dose_rate "Gamma=3.51e-13 (J/kg)*m^2/((1/s)*h)" "A=3.7e+11 (1/s)" "r=2 m" "mu=0.656 1/cm" "H_target=2.5e-06 (J/kg)/h" buildup=linear
x = 0.1835320565 m
n_mfp = 12.03970291
B = 13.03970291
  by: Shield thickness for a target dose rate (point source, buildup)  (solve B(mu x) e^(-mu x) = H_target r^2/(Gamma A): x = ln(H0/H)/mu (no buildup) or (1 + b) e^(-b) = q by bisection)
  Gamma = 9.75e-17 J*m^2/kg
  A = 3.7e+11 1/s
  r = 2 m
  mu = 65.6 1/m
  H_target = 6.944444444e-10 J/(kg*s)
  buildup = linear
  assumes: target below the unshielded dose rate; the shield fits between source and point
```

**What the worked example means:** This calculation determines the slab thickness needed to attenuate radiation from a point source down to a specific safety limit. By balancing the unshielded dose rate against the target value using the linear attenuation coefficient and buildup factor, the model solves for the physical barrier size. The resulting thickness of 0.183532 m corresponds to 12.0397 mean free paths, ensuring the dose rate at the measurement point does not exceed 2.5e-06 (J/kg)/h.

<a id="nuc_stay_time_decaying_source"></a>

#### Stay time to a dose limit near a decaying source — `nuc_stay_time_decaying_source`

How long one can stay at a place where the dose rate decays with the source's half-life before receiving a stated dose, compared with the stay time if the rate were constant.

$$
t = -\frac{1}{\lambda}\ln\left(1-\frac{\lambda D_{lim}}{\dot H_0}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `H0` | in | dose rate at the start of the stay | J/(kg*s) |
| `T_half` | in | half-life of the source | s |
| `D_lim` | in | dose limit for the stay | J/kg |
| `t_stay` | out | allowed stay time | s |
| `frac` | out | dose limit as a fraction of the dose until complete decay | — |

**Applies when:** D_lim < H0 T_half/ln 2 (otherwise the limit is never reached).

```shell
$ nuclear-engineering solve nuc_stay_time_decaying_source "H0=0.002 (J/kg)/h" "T_half=6586.2 s" "D_lim=0.001 (J/kg)"
t_stay = 1995.634472 s
frac = 0.1894362341
  by: Stay time to a dose limit near a decaying source  (H0 (1 - e^(-lambda t))/lambda = D_lim -> t = -ln(1 - lambda D_lim/H0)/lambda)
  H0 = 5.555555556e-07 J/(kg*s)
  T_half = 6586.2 s
  D_lim = 0.001 J/kg
  assumes: D_lim < H0 T_half/ln 2 (otherwise the limit is never reached)
```

**What the worked example means:** This calculation determines how long you can remain near a decaying source before reaching a specific dose limit. By accounting for the exponential decrease in dose rate, it finds that the maximum safe duration is 1995.63 s. This result assumes the initial dose rate was 0.002 (J/kg)/h and the source has a half-life of 6586.2 s. The fraction of the total potential dose consumed by this limit is 0.189436, showing that as the source activity drops, each second contributes less to your total exposure than if the rate had remained constant.

<a id="nuc_line_source_flux"></a>

#### Uncollided flux from a shielded line source (Sievert integral) — `nuc_line_source_flux`

The uncollided photon flux at a point at perpendicular distance h from a straight line source (a pipe, a wire) running from y1 to y2 along the line, behind a slab shield parallel to the line, with the unshielded flux and the transmission.

$$
\phi = \frac{S_L}{4\pi h}\left[F(\theta_2,\mu t) - F(\theta_1,\mu t)\right],\ F(\theta,b)=\int_0^{\theta}e^{-b\sec u}\,du
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `S_L` | in | source strength per unit length (photons per second per metre) | 1/(m*s) |
| `h` | in | perpendicular distance from the line to the point | m |
| `y1` | in | position of one end along the line, measured from the foot of the perpendicular (may be negative) | m |
| `y2` | in | position of the other end (y2 > y1) | m |
| `mu` | in | linear attenuation coefficient of the shield | 1/m |
| `t` | in | shield thickness (0 for none) | m |
| `phi` | out | uncollided flux at the point | 1/(m^2*s) |
| `phi0` | out | flux without the shield | 1/(m^2*s) |
| `T` | out | transmission phi/phi0 | — |

**Applies when:** isotropic line source in vacuum; slab shield parallel to the line (t < h); no buildup.

```shell
$ nuclear-engineering solve nuc_line_source_flux "S_L=100000000 1/(m*s)" "h=1 m" "y1=-2 m" "y2=2 m" "mu=0.1 1/cm" "t=10 cm"
phi = 5012706.821 1/(m^2*s)
phi0 = 17620819.12 1/(m^2*s)
T = 0.2844763792
  by: Uncollided flux from a shielded line source (Sievert integral)  (phi = S_L/(4 pi h) [F(theta2, mu t) - F(theta1, mu t)], F(theta, b) = int_0^theta e^(-b sec u) du (Sievert integral), theta_i = atan(y_i/h))
  S_L = 100000000 1/(m*s)
  h = 1 m
  y1 = -2 m
  y2 = 2 m
  mu = 10 1/m
  t = 0.1 m
  assumes: isotropic line source in vacuum; slab shield parallel to the line (t < h); no buildup
```

**What the worked example means:** This calculation determines the uncollided photon flux at a point located at perpendicular distance 1 m from a line source extending from -2 m to 2 m, shielded by a slab of thickness 10 cm and attenuation coefficient 0.1 1/cm. The result, 5.01271e+06 1/(m^2*s), represents the intensity of photons that have passed through the shield without scattering. The unshielded flux would be 1.76208e+07 1/(m^2*s), yielding a transmission factor of 0.284476. This method uses the Sievert integral to account for the varying path lengths through the shield across the length of the source.

<a id="nuc_disk_source_flux"></a>

#### Uncollided flux on the axis of a shielded disk source — `nuc_disk_source_flux`

The uncollided flux on the axis of a uniform disk source (a contaminated floor patch, a tank bottom) at distance h, behind a slab shield parallel to the disk, with the unshielded flux and the transmission.

$$
\phi = \frac{S_A}{2}\left[E_1(\mu t) - E_1(\mu t\sec\theta)\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `S_A` | in | source strength per unit area (photons per second per square metre) | 1/(m^2*s) |
| `R` | in | disk radius | m |
| `h` | in | distance from the disk on its axis | m |
| `mu` | in | linear attenuation coefficient of the shield | 1/m |
| `t` | in | shield thickness (0 for none) | m |
| `phi` | out | uncollided flux on the axis | 1/(m^2*s) |
| `phi0` | out | flux without the shield | 1/(m^2*s) |
| `T` | out | transmission phi/phi0 | — |

**Applies when:** uniform isotropic disk source; slab shield parallel to the disk (t < h); no buildup.

```shell
$ nuclear-engineering solve nuc_disk_source_flux "S_A=10000000 1/(m^2*s)" "R=1 m" "h=50 cm" "mu=0.1 1/cm" "t=5 cm"
phi = 1895736.597 1/(m^2*s)
phi0 = 4023594.781 1/(m^2*s)
T = 0.4711549497
  by: Uncollided flux on the axis of a shielded disk source  (phi = (S_A/2) [E1(b) - E1(b sec theta)], sec theta = sqrt(h^2 + R^2)/h, b = mu t; unshielded (S_A/4) ln(1 + R^2/h^2))
  S_A = 10000000 1/(m^2*s)
  R = 1 m
  h = 0.5 m
  mu = 10 1/m
  t = 0.05 m
  assumes: uniform isotropic disk source; slab shield parallel to the disk (t < h); no buildup
```

**What the worked example means:** This calculation determines the uncollided photon flux on the central axis of a uniform disk source located behind a parallel slab shield. The result, 1.89574e+06 1/(m^2*s), represents the intensity of photons that have not interacted with the shield material, derived from the source strength per unit area, 1e+07 1/(m^2*s), the disk radius, 1 m, the axial distance, 50 cm, the linear attenuation coefficient, 0.1 1/cm, and the shield thickness, 5 cm. For comparison, the flux without any shielding is calculated as 4.02359e+06 1/(m^2*s), while the ratio of these two values gives the transmission factor, 0.471155, indicating how effectively the shield attenuates the direct beam.

### Thermal hydraulics (4)

<a id="nuc_channel_coolant_temperature"></a>

#### Coolant temperature along a channel with a chopped-cosine power shape — `nuc_channel_coolant_temperature`

The bulk coolant temperature at a height z (from the core mid-plane) in a heated channel whose linear heat rate follows a chopped cosine, the outlet temperature and the channel's total power.

$$
T(z) = T_{in} + \frac{q_0H_e}{\pi\dot mc_p}\left[\sin\frac{\pi z}{H_e} + \sin\frac{\pi H}{2H_e}\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `q0` | in | peak linear heat rate (at the core mid-plane) | W/m |
| `H` | in | heated (core) height | m |
| `H_e` | in | extrapolated height of the chopped-cosine power shape (H_e >= H) | m |
| `m_dot` | in | coolant mass flow rate in the channel | kg/s |
| `c_p` | in | coolant specific heat | J/(kg*K) |
| `T_in` | in | coolant inlet temperature | K |
| `z` | in | height above the core mid-plane (negative below) | m |
| `T` | out | coolant temperature at z | K |
| `T_out` | out | coolant outlet temperature | K |
| `Q` | out | total heat added to the channel | W |

**Applies when:** steady state, single-phase coolant, constant c_p; q'(z) = q0 cos(pi z/H_e).

```shell
$ nuclear-engineering solve nuc_channel_coolant_temperature "q0=43 kW/m" "H=3.66 m" "H_e=4 m" "m_dot=0.3 kg/s" "c_p=5500 J/(kg*K)" "T_in=565 K" "z=0 m"
T = 597.8860714 K
T_out = 630.7721428 K
Q = 108524.0356 W
  by: Coolant temperature along a channel with a chopped-cosine power shape  (T(z) = T_in + (q0 H_e/(pi m c_p)) [sin(pi z/H_e) + sin(pi H/(2 H_e))])
  q0 = 43000 W/m
  H = 3.66 m
  H_e = 4 m
  m_dot = 0.3 kg/s
  c_p = 5500 J/(kg*K)
  T_in = 565 K
  z = 0 m
  assumes: steady state, single-phase coolant, constant c_p; q'(z) = q0 cos(pi z/H_e)
```

**What the worked example means:** This calculation determines the bulk coolant temperature at a specific axial position within a heated channel, assuming a chopped cosine power distribution. The result, 597.886 K, reflects the cumulative heat transfer from the inlet up to height 0 m. Additionally, the method computes the overall thermal output of the channel, represented by 108524 W, and the final coolant temperature exiting the core region, denoted as 630.772 K. These values are derived from the peak linear heat rate 43 kW/m, the geometric parameters 3.66 m and 4 m, and the flow conditions defined by mass flow rate 0.3 kg/s and specific heat 5500 J/(kg*K).

<a id="nuc_centreline_temperature_variable_k"></a>

#### Fuel centreline temperature with temperature-dependent conductivity — `nuc_centreline_temperature_variable_k`

The centreline temperature of a cylindrical fuel pellet at a stated linear heat rate and surface temperature when its conductivity varies as k = 1/(A + B T) (the usual form for UO2), through the conductivity integral, with the conductivities at the surface and the centre.

$$
\int_{T_s}^{T_0}k(T)\,dT = \frac{q'}{4\pi}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `q_lin` | in | linear heat rate q' | W/m |
| `T_s` | in | pellet surface temperature | K |
| `A` | in | A in k = 1/(A + B T) (m*K/W) | m*K/W |
| `B` | in | B in k = 1/(A + B T) (m/W) | m/W |
| `T_0` | out | centreline temperature | K |
| `dT` | out | centre-to-surface temperature rise | K |
| `k_s` | out | conductivity at the surface | W/(m*K) |
| `k_0` | out | conductivity at the centre | W/(m*K) |

**Applies when:** uniform heat generation; radial conduction, steady state; k(T) = 1/(A + B T) with T in K.

```shell
$ nuclear-engineering solve nuc_centreline_temperature_variable_k "q_lin=300 W/cm" "T_s=800 K" "A=0.0452 m*K/W" "B=0.000246 m/W"
T_0 = 1586.098009 K
dT = 786.0980091 K
k_s = 4.132231405 W/(m*K)
k_0 = 2.296843555 W/(m*K)
  by: Fuel centreline temperature with temperature-dependent conductivity  (int_{Ts}^{T0} k dT = q'/(4 pi): T0 = [(A + B Ts) e^(B q'/(4 pi)) - A]/B)
  q_lin = 30000 W/m
  T_s = 800 K
  A = 0.0452 m*K/W
  B = 0.000246 m/W
  assumes: uniform heat generation; radial conduction, steady state; k(T) = 1/(A + B T) with T in K
```

**What the worked example means:** This calculation determines the peak temperature at the center of a fuel pellet given the linear heat rate 300 W/cm, surface temperature 800 K, and conductivity parameters 0.0452 m*K/W and 0.000246 m/W. The result, a centreline temperature of 1586.1 K, accounts for how thermal conductivity varies with temperature, yielding a centre-to-surface difference of 786.098 K. This approach is essential for accurate safety analysis because ignoring conductivity variations can lead to significant errors in predicting fuel performance.

<a id="nuc_fuel_rod_temperatures"></a>

#### Fuel-rod radial temperatures: centreline, pellet surface and cladding — `nuc_fuel_rod_temperatures`

The temperatures at the fuel centreline, the pellet surface and the cladding inner and outer surfaces of a cylindrical fuel rod at a stated linear heat rate, from the coolant temperature through the film, cladding, gap and pellet (constant conductivities).

$$
T_0 - T_b = q'\left[\frac{1}{4\pi k_f} + \frac{1}{2\pi R_fh_g} + \frac{\ln(R_o/R_f)}{2\pi k_c} + \frac{1}{2\pi R_oh}\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `q_lin` | in | linear heat rate q' (W/m) | W/m |
| `R_f` | in | pellet radius | m |
| `k_f` | in | fuel thermal conductivity | W/(m*K) |
| `h_g` | in | gap conductance | W/(m^2*K) |
| `t_c` | in | cladding thickness | m |
| `k_c` | in | cladding thermal conductivity | W/(m*K) |
| `h` | in | coolant heat transfer coefficient | W/(m^2*K) |
| `T_cool` | in | bulk coolant temperature | K |
| `T_0` | out | fuel centreline temperature | K |
| `T_fs` | out | pellet surface temperature | K |
| `T_ci` | out | cladding inner surface temperature | K |

**Applies when:** steady state, radial conduction only; uniform heat generation in the pellet; thin gap of conductance h_g at the pellet radius.

```shell
$ nuclear-engineering solve nuc_fuel_rod_temperatures "q_lin=200 W/cm" "R_f=4.1 mm" "k_f=3 W/(m*K)" "h_g=5000 W/(m^2*K)" "t_c=0.57 mm" "k_c=15 W/(m*K)" "h=34000 W/(m^2*K)" "T_cool=580 K"
T_0 = 1313.460197 K
T_fs = 782.9437203 K
T_ci = 627.6706051 K
  by: Fuel-rod radial temperatures: centreline, pellet surface and cladding  (T_co = T_b + q'/(2 pi R_o h); T_ci = T_co + q' ln(R_o/R_f)/(2 pi k_c); T_fs = T_ci + q'/(2 pi R_f h_g); T_0 = T_fs + q'/(4 pi k_f))
  q_lin = 20000 W/m
  R_f = 0.0041 m
  k_f = 3 W/(m*K)
  h_g = 5000 W/(m^2*K)
  t_c = 0.00057 m
  k_c = 15 W/(m*K)
  h = 34000 W/(m^2*K)
  T_cool = 580 K
  assumes: steady state, radial conduction only; uniform heat generation in the pellet; thin gap of conductance h_g at the pellet radius
```

**What the worked example means:** This calculation determines the steady-state radial temperature profile of a nuclear fuel rod. Starting from the bulk coolant temperature, it accounts for heat transfer across the film, conduction through the cladding, and conduction within the fuel pellet to find the temperatures at key interfaces. The resulting centreline temperature of 1313.46 K represents the hottest point in the fuel, which must remain below safety limits to prevent melting.

<a id="nuc_max_clad_temperature"></a>

#### Location and value of the maximum cladding surface temperature — `nuc_max_clad_temperature`

Where along a channel with a chopped-cosine power shape the cladding surface is hottest (above the mid-plane, because the coolant warms up), its temperature there and the coolant outlet temperature.

$$
\tan\frac{\pi z_{max}}{H_e} = \frac{2R_ohH_e}{\dot mc_p}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `q0` | in | peak linear heat rate (at the core mid-plane) | W/m |
| `H` | in | heated (core) height | m |
| `H_e` | in | extrapolated height of the chopped-cosine power shape (H_e >= H) | m |
| `m_dot` | in | coolant mass flow rate in the channel | kg/s |
| `c_p` | in | coolant specific heat | J/(kg*K) |
| `T_in` | in | coolant inlet temperature | K |
| `R_o` | in | cladding outer radius | m |
| `h` | in | coolant heat transfer coefficient | W/(m^2*K) |
| `z_max` | out | height of the maximum cladding temperature above the mid-plane | m |
| `T_max` | out | maximum cladding surface temperature | K |
| `T_out` | out | coolant outlet temperature | K |

**Applies when:** single-phase coolant, constant h and c_p; q'(z) = q0 cos(pi z/H_e); the maximum may sit at the core exit.

```shell
$ nuclear-engineering solve nuc_max_clad_temperature "q0=43 kW/m" "H=3.66 m" "H_e=4 m" "m_dot=0.3 kg/s" "c_p=5500 J/(kg*K)" "T_in=565 K" "R_o=4.75 mm" "h=34000 W/(m^2*K)"
z_max = 0.8458226745 m
T_max = 651.7070529 K
T_out = 630.7721428 K
  by: Location and value of the maximum cladding surface temperature  (dT_c/dz = 0: tan(pi z/H_e) = 2 R_o h H_e/(m c_p); T_c = T(z) + q'(z)/(2 pi R_o h))
  q0 = 43000 W/m
  H = 3.66 m
  H_e = 4 m
  m_dot = 0.3 kg/s
  c_p = 5500 J/(kg*K)
  T_in = 565 K
  R_o = 0.00475 m
  h = 34000 W/(m^2*K)
  assumes: single-phase coolant, constant h and c_p; q'(z) = q0 cos(pi z/H_e); the maximum may sit at the core exit
```

**What the worked example means:** This calculation finds the hottest point on the fuel rod's cladding. Using the coolant inlet temperature, mass flow rate, and power distribution, it determines that the peak temperature reaches 651.707 K. This maximum occurs at a vertical position of 0.845823 m relative to the core center. The result highlights where thermal stress is greatest, which is critical for ensuring structural integrity during operation.

Questions and reports: support@ai999x.com. AI999X — The Future Is Self-Improving.
