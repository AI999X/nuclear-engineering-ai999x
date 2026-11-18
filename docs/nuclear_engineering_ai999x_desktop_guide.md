# Nuclear Engineering: the desktop app, user guide

**Nuclear Engineering**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Nuclear Engineering covers radioactive decay and decay chains, binding energy, reactor physics basics, radiation shielding and attenuation, and dosimetry. Its desktop app holds all 84 calculations in 10 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-nuclear-engineering-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-nuclear-engineering-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-nuclear-engineering-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-nuclear-engineering.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-nuclear-engineering-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-nuclear-engineering-1.6.5-linux-x86_64.tar.gz
./ai999x-nuclear-engineering-1.6.5-linux-x86_64/ai999x-nuclear-engineering
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Nuclear Engineering app, opened on Activity and specific activity of a mass of a radionuclide with its worked example.](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/hero.webp)

*The Nuclear Engineering app, opened on Activity and specific activity of a mass of a radionuclide with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Decay and activity › Activity and specific activity of a mass of a radionuclide (or type part of its name in the search box). It opens with a worked example filled in: “Specific activity: nuclide = Co-60, m = 1 g” The answer: A = 4.18689443e+13 1/s, A_Ci = 1131.593089, a = 4.18689443e+16 1/(kg*s), a_Ci_per_g = 1131.593089.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Activity and specific activity of a mass of a radionuclide: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/scene-1.webp)

*Activity and specific activity of a mass of a radionuclide: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “Specific activity: nuclide = Co-60, m = 1 g”. The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: Activity and specific activity of a mass of a radionuclide
A = 4.18689443e+13 1/s
A_Ci = 1131.593089
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Dose rate from a shielded point gamma source with buildup](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/scene-2.webp)

*Dose rate from a shielded point gamma source with buildup*

![Critical radius of a bare spherical reactor (one group)](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/scene-3.webp)

*Critical radius of a bare spherical reactor (one group)*

![Uranium enrichment: feed, tails and separative work (SWU)](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/scene-4.webp)

*Uranium enrichment: feed, tails and separative work (SWU)*

![Compton edge and backscatter peak of a gamma line](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/scene-5.webp)

*Compton edge and backscatter peak of a gamma line*

![Committed effective dose from a dose coefficient](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/scene-6.webp)

*Committed effective dose from a dose coefficient*

![Average number of collisions to slow a neutron down](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/scene-7.webp)

*Average number of collisions to slow a neutron down*

![Integral and differential control-rod worth (sine-squared weighting)](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/scene-8.webp)

*Integral and differential control-rod worth (sine-squared weighting)*

![Activity and specific activity of a mass of a radionuclide: a chart, enlarged.](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/chart-1.webp)

*Activity and specific activity of a mass of a radionuclide: a chart, enlarged.*

![Dose rate from a shielded point gamma source with buildup: a chart, enlarged.](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/chart-2.webp)

*Dose rate from a shielded point gamma source with buildup: a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Nuclear Engineering app in the dark theme.](https://www.ai999x.com/?ax_shot=nuclear-engineering-ai999x/hero-dark.webp)

*The Nuclear Engineering app in the dark theme.*

## 7. Every calculation

84 calculations in 10 topics; each has its own entry on the model's page (https://www.ai999x.com/models/nuclear-engineering-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Decay and activity | Activation with target depletion and product burn-up, Activity and specific activity of a mass of a radionuclide, Activity of a radionuclide and its progeny after a decay time, Irradiation time to reach a fraction of saturation activity, Mass and number of atoms of a radionuclide with a given activity, Neutron activation: activity after irradiation and decay, Radionuclide generator: daughter activity after elution (with branching), Three-member decay chain with initial activities (Bateman), Time of maximum daughter activity, Transient and secular equilibrium: activity ratio and approach |
| Detectors and counting | Compton edge and backscatter peak of a gamma line, Counting time for a target precision of the net count rate, Counts recorded from a decaying source (decay during counting), Currie critical level, detection limit and minimum detectable activity, Dead-time correction, non-paralyzable detector, Dead-time correction, paralyzable detector, Expected count rate from a point source on a detector's axis, Half-life from a measured decay curve (weighted log-linear fit), Neutron flux from a measured foil activation, Optimal division of counting time between sample and background, Single and double escape peaks of a gamma ray, Solid angle and geometric efficiency of a disk detector (source on axis), Solid angle of a disk detector for an off-axis point source, Total and intrinsic efficiency of a slab detector for a point source on axis, Two-component decay curve: amplitudes of two known half-lives |
| Dosimetry | Committed effective dose from a dose coefficient, Committed equivalent dose to an organ from an intake (retention compartments), Effective dose from organ equivalent doses and tissue weighting factors, Effective half-life (physical and biological removal), Radiation unit conversion (Bq and Ci, Gy and rad, Sv and rem) |
| Fuel cycle | Uranium enrichment: feed, tails and separative work (SWU) |
| Neutron physics | Average logarithmic energy decrement and mean scattering cosine, Average number of collisions to slow a neutron down, Diffusion length from an exponential pile measurement, Flux from a plane neutron source at the centre of a slab, Flux from a point neutron source at the centre of a sphere, Macroscopic cross section, mean free path and reaction rate of a material, Maxwellian-averaged 1/v cross section, Moderating power and ratio of a compound moderator, Neutron energy and lab angle after an elastic collision, Number densities and macroscopic cross sections of UO2 fuel, Speeds and mean energy of thermal neutrons (Maxwell distribution), Thermal diffusion length and migration length |
| Reactor kinetics | Integral and differential control-rod worth (sine-squared weighting), Power after a reactivity step (one delayed group, exact), Power after a reactivity step (six delayed groups, exact), Reactivity from a measured stable period or doubling time (inhour), Stable period and prompt root of the inhour equation (six delayed groups), Stable reactor period after a reactivity step (one delayed group) |
| Reactor physics | Approach to criticality: inverse multiplication (1/M) extrapolation, Critical radius of a bare cylindrical reactor of given height (one group), Critical radius of a bare spherical reactor (one group), Critical thickness of a bare slab reactor (one group), Critical thickness of a reflected slab reactor and reflector savings (one group), Critical third side of a bare rectangular reactor (one group), Decay heat after shutdown (Way-Wigner), Decay heat energy released between two times after shutdown, Equilibrium iodine-135 and xenon-135 concentrations, Fundamental-mode flux shape of a bare reactor at a point, k-effective and geometric buckling of a bare reactor (one group), k-effective of a bare reactor in two-group diffusion theory, Minimum-volume critical bare cylinder (one group), Multiplication factor by the four- and six-factor formulas, Power density and power from the fission reaction rate, Reactor power from the peak flux (or peak flux from power) of a bare core, Samarium-149 poisoning: equilibrium and after shutdown, U-235 depletion and energy released at constant flux, U-235 depletion at constant power, Xenon-135 after shutdown: concentration and peak |
| Shielding | Half-value and tenth-value layers |
| Shielding and dose | Dose accumulated near a decaying source, Dose rate from a shielded point gamma source with buildup, Dose received passing a point source, Klein-Nishina Compton cross sections per electron, Linear attenuation coefficient from narrow-beam transmission measurements, Narrow-beam transmission through a multilayer shield, Shield thickness for a target dose rate (point source, buildup), Stay time to a dose limit near a decaying source, Uncollided flux from a shielded line source (Sievert integral), Uncollided flux on the axis of a shielded disk source |
| Thermal hydraulics | Coolant temperature along a channel with a chopped-cosine power shape, Fuel centreline temperature with temperature-dependent conductivity, Fuel-rod radial temperatures: centreline, pellet surface and cladding, Location and value of the maximum cladding surface temperature |

## 8. Other ways to use it

The same engine runs on the command line (`nuclear-engineering`, its own guide), in Python (`pip install nuclear-engineering-ai999x`, `import nuclear_engineering_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/nuclear-engineering-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/nuclear-engineering-ai999x/
