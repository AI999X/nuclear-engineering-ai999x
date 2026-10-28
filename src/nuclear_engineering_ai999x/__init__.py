"""nuclear-engineering-ai999x — offline nuclear engineering answers with their formula, steps and verification.

    import nuclear_engineering_ai999x as m
    m.ask("Specific activity: nuclide = Co-60, m = 1 g").show()            # A = 4.19e13 Bq = 1132 Ci
    m.ask("Critical sphere: D = 0.9 cm, Sigma_a = 0.15 1/cm, nuSigma_f = 0.18 1/cm, extrapolation = 2.13D").show()   # R = 15.3 cm
    m.solve("nuc_enrichment_swu", x_p=0.045, x_f=0.00711, x_w=0.0025, P="1 kg")     # 6.87 kg SWU, 9.22 kg feed
    m.capabilities()                   # every certified capability

73 certified capabilities, the engineering view of nuclear science: activity and decay chains of real radionuclides
(specific activity and mass of a source, progeny build-up with real branching fractions, three-member Bateman chains
with initial daughters, time of maximum daughter activity, transient and secular equilibrium, generators, neutron
activation with and without burn-up, irradiation time, decay-curve and two-component fits, Way-Wigner decay heat and
energy, committed dose from retention compartments, effective half-life), shielding and dose (shielded point sources
with none, linear or Taylor buildup, shield thickness for a target dose rate, multilayer transmission, Sievert line
sources, disk sources, transit, accumulated and stay-time doses, Klein-Nishina cross sections, Compton edge),
detectors and counting (paralyzable and non-paralyzable dead time, counting-time optimisation and precision, solid
angle on and off axis, detector efficiency, expected count rates, foil activation, attenuation fits, 1/M approach to
criticality), neutron physics (elastic-scattering kinematics, xi and collisions to thermalize, compound moderators,
Maxwellian-averaged cross sections, thermal-neutron speeds, UO2 number densities and cross sections, plane and point
sources, exponential piles), reactor physics (one-group critical slab, sphere, cylinder, minimum-volume cylinder and
cuboid, k-effective of bare reactors, two-group theory, reflected slabs and reflector savings, flux shapes, core power
from peak flux), kinetics (one- and six-group point kinetics, inhour periods, reactivity from period, rod worth),
poisons and fuel (xenon equilibrium and the post-shutdown peak, samarium, U-235 depletion, enrichment SWU) and
fuel-rod heat transfer (radial temperatures, temperature-dependent conductivity, channel coolant and hottest-clad
temperatures). Educational and civil content only.

Questions use the form "<capability phrase>: name = value, ..." (see templates()); physical values carry their units
(Bq, Ci, Gy, Sv, rad, rem, barn, cps, cpm, d, y, MWd/kgU and pcm, $ or cents for reactivity are converted), fractions
may be written with %, lists in brackets, and conventions (buildup form, extrapolation distance, geometry, dead-time
branch, energy per fission, delayed-neutron data) are stated, never assumed. Built by the AI999X model machine from the
same engine as every domain; a question that is not determined is refused rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.0.1"
_P = Product("nuclear_engineering", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "__version__"]
