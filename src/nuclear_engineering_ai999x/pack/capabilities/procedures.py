"""Nuclear engineering procedures: deterministic capabilities with typed inputs and several outputs (domain code, loaded
by the pack; enabled only once tools/certify_procedures.py has certified them).

Topics (the engineering view; the decay law, radioactive dating, two-body decay, Q values and the pure parent-daughter
pair belong to quantum physics, radiometric ages to earth science): activity and decay chains of real radionuclides
(specific activity, progeny build-up, three-member chains, equilibrium, generators, activation, decay-curve fits,
decay heat, committed dose), radiation shielding and dose (point, line and disk sources with buildup, multilayer
shields, transit and stay-time doses, Klein-Nishina cross sections), detectors and counting (dead time, counting-time
optimisation, solid angle and efficiency, approach to criticality), neutron physics (elastic scattering and
moderation, cross sections of fuel, Maxwellian averages, diffusion of a source, critical size and k-effective of bare
and reflected reactors, two-group theory, power and flux shapes), reactor kinetics (one- and six-group point kinetics,
inhour, rod worth), poisons and fuel (xenon, samarium, depletion, enrichment) and fuel-rod heat transfer.

Each procedure states its inputs, outputs, conditions, method, LaTeX, the documented question form it parses, worked
examples, an INDEPENDENT reference computation (used only by certification; the checking libraries are imported inside
the reference functions) and a deliberately wrong variant that the checks must reject. The shipped code (run, verify,
parse) is plain Python (math, re, fractions) plus mpmath where stated.

How the references differ from the shipped code:
- named radionuclides: radioactivedecay 0.6.1's own activity, mass and decay calculations (its ICRP-107 decay data);
  the shipped half-lives, atomic masses and branching fractions are copied from it and must agree exactly;
- user-specified chains and kinetics: matrix exponentials (mpmath expm, scipy expm), scipy DOP853 integration of the
  rate equations (decay, activation, burn-up, xenon, samarium, depletion), convolution integrals evaluated by scipy
  quadrature, root finding with scipy brentq on those numerical solutions, numpy/scipy eigenvalues;
- shielding: attenuation integrated layer by layer as an ODE through the shield (buildup terms as coupled ODEs),
  scipy quadrature of line and disk sources in a different variable from the shipped Sievert/E1 forms, quadrature
  of the Klein-Nishina differential cross section, photon kinematics solved from four-momentum conservation;
- detectors: renewal-theory quadrature (non-paralyzable dead time), scipy lambertw (paralyzable), quadrature over
  the detector face (solid angle, efficiency), numpy regressions for fits;
- neutron physics: vector (lab/centre-of-mass) collision kinematics, quadrature over the scattering angle (xi, mean
  cosine), quadrature over the Maxwellian flux spectrum, the diffusion equation solved numerically by shooting
  (DOP853 + brentq) for critical sizes, k-effective, reflected cores, flux shapes and sources, a generalized matrix
  eigenproblem for two-group theory, quadrature of flux shapes for core power, ideal-cascade integrals for
  separative work.

Conventions (fixed, so run and reference agree exactly):
- SI internally. Activities are read in Bq or Ci (1 Ci = 3.7e10 Bq exactly) and reported in Bq (1/s) with Ci beside
  them; doses in Gy or Sv (also rad and rem = 0.01 Gy/Sv), reported in J/kg (= Gy or Sv) with mSv or uSv beside them;
  cross sections in barns (1 b = 1e-28 m^2); count rates in cps or cpm; burnup in MWd/kgU or GWd/tU; times in s, min,
  h, d (day), y (Julian year 365.25 d) and the shared reader's units. These spellings are converted HERE (the shared
  unit reader lacks them) and the conversion is exact. radioactivedecay's own year (365.2422 d) is never used:
  every time is passed to it in seconds.
- Constants: Avogadro 6.02214076e23 1/mol, e = 1.602176634e-19 C, m_e c^2 = 0.51099895069 MeV, classical electron
  radius 2.8179403205e-15 m, neutron mass 1.67492750056e-27 kg (CODATA 2022).
- Named radionuclides: half-lives (s), atomic masses (g/mol) and branching fractions of radioactivedecay 0.6.1
  (ICRP-107; masses from its atomic-mass table).
- Energy per fission, buildup-factor parameters, gamma-ray dose-rate constants, delayed-neutron data, cross sections
  and fission yields are INPUTS: the question states them; nothing is assumed. Buildup forms: none (B = 1), linear
  (B = 1 + mu x) or Taylor (B = A1 e^{-alpha1 mu x} + (1 - A1) e^{-alpha2 mu x}); the form is a required choice.
- One-group diffusion: extrapolation distance chosen by the question (none, 2D, or 2.1312 D = 0.7104 transport mean
  free paths). Dimensions called "extrapolated" include it; physical ones do not.
- Reactivity is read as an absolute number (0.002), in pcm (1e-5), in percent, or in dollars/cents (then the stated
  total delayed fraction beta converts it). Periods are e-folding times; doubling time = period x ln 2.
- Way-Wigner decay heat: P/P0 = 0.0622 [t^-0.2 - (t + T)^-0.2] (t, T in seconds), valid 10 s to about 1e7 s.
- Results built from exp, log or roots are rounded to 12 significant digits (identical on every platform) and values
  that are zero by construction are written as exactly 0.

Question form (one per procedure): the capability phrase, a colon, then `name = value` pairs separated by commas
(`Specific activity: nuclide = Co-60, m = 1 g`). Physical values carry a unit; fractions may be written with %; lists
are written in brackets in the unit their meaning states (a unit written after a list must be that unit); choices are
words. The reading guard (tools/ask_sweep.py, 2026-10-23) is built into this form:
  1. every stated number is bound: the question is split into `name = value` pairs and any other text or number
     refuses the whole question; nuclide names (Co-60, U-235) are words, not numbers;
  2. the capability phrase must open the question and be followed by the colon, so wording that names another
     capability's task (paralyzable vs non-paralyzable, slab vs cylinder vs sphere, one vs six delayed groups) never
     reaches this reader; geometry, buildup form, dead-time branch and extrapolation are required choices;
  3. a convention is an input: it is read when stated and refused (NEEDS_INPUT) when not, never assumed;
  4. physical values are passed to the runtime with their units (converted here only for units the shared reader
     lacks), so nothing is converted twice.
"""
from __future__ import annotations

import math
import re

import mpmath as _mp

# ================================================================================================ constants (CODATA 2022)
NA = 6.02214076e23              # 1/mol
QE = 1.602176634e-19            # C
MEV = 1.602176634e-13           # J
ME_MEV = 0.51099895069          # electron rest energy, MeV
RE = 2.8179403205e-15           # classical electron radius, m
MN = 1.67492750056e-27          # neutron mass, kg
KB = 1.380649e-23               # J/K
CI = 3.7e10                     # Bq
BARN = 1e-28                    # m^2
DAY = 86400.0
YEAR = 365.25 * DAY             # Julian year (the shared reader's "yr")
LN2 = math.log(2.0)
J01 = 2.404825557695773         # first zero of J0
EXTRAP = {"none": 0.0, "2d": 2.0, "2.13d": 2.1312}   # extrapolation distance / D


# ================================================================================================ shared helpers
def _r(x):
    """12 significant digits, no negative zero (identical results on every platform and Python version)."""
    if isinstance(x, bool) or isinstance(x, str) or x is None:
        return x
    if isinstance(x, int):
        return x
    if isinstance(x, (list, tuple)):
        return [_r(v) for v in x]
    x = float(x)
    if x == 0 or not math.isfinite(x):
        return 0.0 if x == 0 else x
    return float(f"{x:.12g}") + 0.0


def _out(**kw):
    return {k: _r(v) for k, v in kw.items()}


def _rel(a, b):
    if isinstance(a, (list, tuple)) or isinstance(b, (list, tuple)):
        if not (isinstance(a, (list, tuple)) and isinstance(b, (list, tuple))) or len(a) != len(b):
            return math.inf
        return max([_rel(x, y) for x, y in zip(a, b)] + [0.0])
    if isinstance(a, (str, bool)) or isinstance(b, (str, bool)):
        return 0.0 if a == b else math.inf
    try:
        a, b = float(a), float(b)
    except (TypeError, ValueError):
        return math.inf
    d = abs(a - b)
    if d == 0:
        return 0.0
    if abs(a) < 1e-300 and abs(b) < 1e-300:
        return 0.0
    return d / max(abs(a), abs(b))


def _recheck(fn):
    """A verifier: every output the procedure gives is recomputed and compared (relative); a missing output fails."""
    def verify(inputs, outputs):
        try:
            want = fn(**inputs)
        except Exception:
            return math.inf
        worst = 0.0
        for k, v in want.items():
            if k not in outputs:
                return math.inf
            worst = max(worst, _rel(outputs[k], v))
        return worst
    return verify


def _safe(fn):
    """Guard a wrong variant against exceptions (a variant that raises cannot be compared)."""
    def g(**kw):
        try:
            return fn(**kw)
        except Exception:
            return {"_failed": True}
    return g


def _need(ok, msg):
    if not ok:
        raise ValueError(msg)


def _isnum(v):
    return isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v)


def _pos(name, *vals):
    for v in vals:
        _need(_isnum(v) and v > 0, f"{name} must be a positive number")


def _nonneg(name, *vals):
    for v in vals:
        _need(_isnum(v) and v >= 0, f"{name} must be zero or positive")


def _frac(name, v, lo_open=False, hi_open=False):
    _need(_isnum(v), f"{name} must be a number")
    _need((v > 0 if lo_open else v >= 0) and (v < 1 if hi_open else v <= 1), f"{name} must be a fraction between 0 and 1")


def _lu(rng, a, b):
    return math.exp(rng.uniform(math.log(a), math.log(b)))


def _sig(x, n=4):
    """x rounded to n significant digits (random inputs written as a user would write them)."""
    return float(f"{x:.{n}g}")


def _bisect(f, a, b, n=400):
    """Root of f on [a, b] by bisection (f(a), f(b) of opposite signs), to full double precision."""
    fa, fb = f(a), f(b)
    if fa == 0:
        return a
    if fb == 0:
        return b
    _need((fa > 0) != (fb > 0), "no root in the interval")
    for _ in range(n):
        m = 0.5 * (a + b)
        if m == a or m == b:
            break
        fm = f(m)
        if fm == 0:
            return m
        if (fm > 0) == (fa > 0):
            a, fa = m, fm
        else:
            b = m
    return 0.5 * (a + b)


def _mpf(x):
    return _mp.mpf(x)


# ---------------------------------------------------------------------------------------------- reference helpers
def _ivp(fun, t_end, y0, rtol=1e-13, atol=None, t0=0.0):
    """Reference helper: scipy DOP853 integration from t0 to t_end; returns the final state."""
    from scipy.integrate import solve_ivp
    if abs(t_end - t0) < 1e-6:                      # a tiny interval: classical RK4 in 20 steps (error ~ h^5)
        y, t, h = [float(v) for v in y0], t0, (t_end - t0) / 20.0
        for _ in range(20):
            k1 = fun(t, y)
            k2 = fun(t + h / 2, [a + h / 2 * b for a, b in zip(y, k1)])
            k3 = fun(t + h / 2, [a + h / 2 * b for a, b in zip(y, k2)])
            k4 = fun(t + h, [a + h * b for a, b in zip(y, k3)])
            y = [a + h / 6 * (p + 2 * q + 2 * r + s) for a, p, q, r, s in zip(y, k1, k2, k3, k4)]
            t += h
        return y
    if atol is None:
        atol = [1e-30 + 1e-16 * abs(v) for v in y0]
    sol = solve_ivp(fun, (t0, t_end), list(y0), method="DOP853", rtol=rtol, atol=atol)
    if not sol.success:
        raise RuntimeError(sol.message)
    return [float(v) for v in sol.y[:, -1]]


def _quad(f, a, b, **kw):
    from scipy.integrate import quad
    v, _ = quad(f, a, b, epsabs=0.0, epsrel=1e-13, limit=500, **kw)
    return float(v)


def _brentq(f, a, b):
    from scipy.optimize import brentq
    return float(brentq(f, a, b, xtol=1e-300, rtol=8.9e-16, maxiter=500))


# ================================================================================================ units
_KU = {"inverse_length": "1/m", "time": "s", "mass": "kg", "length": "m", "area": "m^2", "volume": "m^3", "energy": "J", "power": "W",
       "temperature": "K", "temperature_difference": "K", "radioactivity": "1/s", "decay_constant": "1/s",
       "specific_activity": "1/(kg*s)", "count_rate": "1/s", "molar_mass": "kg/mol", "density": "kg/m^3",
       "number_density": "1/m^3", "microscopic_cross_section": "m^2", "macroscopic_cross_section": "1/m",
       "linear_attenuation_coefficient": "1/m", "neutron_flux": "1/(m^2*s)", "particle_flux": "1/(m^2*s)",
       "linear_source_strength": "1/(m*s)", "surface_source_strength": "1/(m^2*s)", "emission_rate": "1/s",
       "dose_rate": "J/(kg*s)", "dose": "J/kg", "absorbed_dose": "J/kg", "equivalent_dose": "J/kg", "dose_rate_constant": "J*m^2/kg",
       "neutron_diffusion_coefficient": "m", "buckling": "1/m^2", "inverse_period": "1/s",
       "linear_heat_rate": "W/m", "volumetric_heat_generation": "W/m^3", "thermal_conductivity": "W/(m*K)",
       "heat_transfer_coefficient": "W/(m^2*K)", "mass_flow_rate": "kg/s", "specific_heat": "J/(kg*K)",
       "heat_flux": "W/m^2", "burnup": "J/kg", "separative_work": "kg", "speed": "m/s", "angle": "rad",
       "thermal_resistivity": "m*K/W", "thermal_resistivity_slope": "m/W", "energy_mev": "MeV", "number": ""}

_DOSE_KINDS = {"dose_rate", "dose", "absorbed_dose", "equivalent_dose", "dose_rate_constant"}
_XS_KINDS = {"microscopic_cross_section"}
_PFX = {"": 1.0, "k": 1e3, "M": 1e6, "G": 1e9, "T": 1e12, "P": 1e15, "m": 1e-3, "u": 1e-6, "µ": 1e-6, "μ": 1e-6,
        "n": 1e-9, "p": 1e-12, "c": 1e-2}


def _atoms_table():
    t = {}
    for p, f in _PFX.items():
        if p in ("", "k", "M", "G", "T", "P"):
            t[p + "Bq"] = (f, "(1/s)", None)
        if p in ("", "k", "M", "m", "u", "µ", "μ", "n", "p"):
            t[p + "Ci"] = (f * CI, "(1/s)", None)
        if p in ("", "m", "u", "µ", "μ", "n", "c"):
            t[p + "Gy"] = (f, "(J/kg)", None)
            t[p + "Sv"] = (f, "(J/kg)", None)
        if p in ("", "m", "u", "µ", "μ", "k", "M"):
            t[p + "b"] = (f * BARN, "m^2", _XS_KINDS)
    for p in ("", "m"):
        t[p + "rad"] = (_PFX[p] * 0.01, "(J/kg)", _DOSE_KINDS)
        t[p + "rem"] = (_PFX[p] * 0.01, "(J/kg)", _DOSE_KINDS)
    t.update({"barn": (BARN, "m^2", _XS_KINDS), "barns": (BARN, "m^2", _XS_KINDS),
              "cps": (1.0, "(1/s)", None), "cpm": (1.0 / 60.0, "(1/s)", None),
              "n": (1.0, "1", {"neutron_flux", "particle_flux", "linear_source_strength", "surface_source_strength", "emission_rate"}),
              "d": (DAY, "s", None), "y": (YEAR, "s", None), "a": (YEAR, "s", None),
              "kWd": (1e3 * DAY, "J", None), "MWd": (1e6 * DAY, "J", None), "GWd": (1e9 * DAY, "J", None),
              "kgU": (1.0, "kg", None), "kgHM": (1.0, "kg", None), "tU": (1e3, "kg", None), "tHM": (1e3, "kg", None),
              "MTU": (1e3, "kg", None)})
    return t


_ATOMS = _atoms_table()
_UTOK = re.compile(r"\s*(\^\s*[-+]?\d+|[*/()·]|1(?![\w.])|[A-Za-zµμ°]+)")


def _unit_expr(text: str, kind: str):
    """(factor, unit text for the shared reader) for a written unit; the spellings the shared reader lacks are
    replaced by their SI equivalents (exact factors). None when the unit cannot be read here."""
    toks, pos = [], 0
    text = text.strip()
    while pos < len(text):
        m = _UTOK.match(text, pos)
        if not m or m.end() == pos:
            return None
        toks.append(m.group(1).replace(" ", "").replace("·", "*"))
        pos = m.end()
    i = [0]

    def peek():
        return toks[i[0]] if i[0] < len(toks) else None

    def factor():
        t = peek()
        if t is None:
            raise ValueError
        i[0] += 1
        if t == "(":
            f, s = expr()
            if peek() != ")":
                raise ValueError
            i[0] += 1
            out = (f, "(" + s + ")")
        elif t == "1":
            out = (1.0, "1")
        elif re.fullmatch(r"[A-Za-zµμ°]+", t):
            a = _ATOMS.get(t)
            if a is not None and (a[2] is None or kind in a[2]):
                out = (a[0], a[1])
            elif t in ("rad", "mrad", "rem", "mrem") and kind in _DOSE_KINDS:
                raise ValueError
            else:
                out = (1.0, t)
        else:
            raise ValueError
        t = peek()
        if t is not None and t.startswith("^"):
            i[0] += 1
            e = int(t[1:])
            out = (out[0] ** e, out[1] + "^" + str(e) if not out[1].startswith("(") else out[1] + "^" + str(e))
        return out

    def expr():
        f, s = factor()
        while peek() in ("*", "/"):
            op = peek()
            i[0] += 1
            g, u = factor()
            if op == "*":
                f, s = f * g, s + "*" + u
            else:
                f, s = f / g, s + "/" + (u if re.fullmatch(r"[A-Za-z0-9µμ°^]+|\(.*\)", u) else "(" + u + ")")
        return f, s
    try:
        f, s = expr()
    except (ValueError, IndexError):
        return None
    if i[0] != len(toks):
        return None
    return f, s


def _written_unit(num: str, unit: str, kind: str):
    """'value unit' for the runtime, with units the shared reader lacks converted here; None when it cannot be read."""
    u = unit.strip()
    atoms = re.findall(r"[A-Za-zµμ°]+", u)
    if not any(a in _ATOMS and (_ATOMS[a][2] is None or kind in _ATOMS[a][2]) for a in atoms):
        return f"{num} {u}"
    e = _unit_expr(u, kind)
    if e is None:
        return None
    return f"{float(num) * e[0]!r} {e[1]}"


# ================================================================================================ the question reader
_NUM = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"
_PAIR = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(.*?)\s*$")
_PCT = {"%": 0.01, "percent": 0.01, "per cent": 0.01}
_RHO_UNITS = {"pcm": 1e-5, "%": 0.01, "percent": 0.01}
_DOLLAR = {"$": 1.0, "dollar": 1.0, "dollars": 1.0, "cent": 0.01, "cents": 0.01, "¢": 0.01}


def _split(rest):
    """Split on commas outside brackets."""
    parts, depth, cur = [], 0, ""
    for ch in rest:
        if ch == "[":
            depth += 1
        elif ch == "]":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    parts.append(cur)
    return parts


def _norm_choice(s):
    return re.sub(r"[\s_\-]", "", str(s)).lower()


def _reader(phrases, inputs, aliases=None, post=None):
    """The one documented question form, '<phrase>: name = value, ...', for a procedure with these inputs (the reading
    guard of the module docstring is built in). `post(out)` may finish the reading (dollar reactivities need the
    stated beta) and returns None to refuse."""
    rx = re.compile(r"\s*(?:" + "|".join(re.escape(p) for p in phrases) + r")\s*:\s*", re.I)
    names, by = {}, {}
    for i in inputs:
        names[i["symbol"]] = i["symbol"]
        by[i["symbol"]] = i
    for i in inputs:
        names.setdefault(i["symbol"].lower(), i["symbol"])
    for a, s in (aliases or {}).items():
        names[a] = s
        names.setdefault(a.lower(), s)

    def key_of(n):
        return names.get(n, names.get(n.lower()))

    def parse(question):
        if not isinstance(question, str):
            return None
        m = rx.match(question)
        if not m:
            return None
        rest = re.sub(r"[.?!]\s*$", "", question[m.end():].strip()).strip()
        if not rest:
            return {}
        out = {}
        for seg in _split(rest):
            mp = _PAIR.match(seg)
            if not mp or not mp.group(2):
                return None
            key = key_of(mp.group(1))
            if key is None or key in out:
                return None
            spec = by[key]
            t = spec.get("type", "number")
            raw = mp.group(2)
            if t == "vector":
                mv = re.fullmatch(r"(\[[^\]]*\])\s*(.*)", raw)
                if not mv:
                    return None
                unit = mv.group(2).strip()
                if unit and unit != spec.get("list_unit"):
                    return None
                items = [p.strip() for p in mv.group(1)[1:-1].split(",")]
                if not items or not all(re.fullmatch(_NUM, p) for p in items):
                    return None
                out[key] = ", ".join(items)
                continue
            if t in ("choice", "text"):
                if not re.fullmatch(r"[A-Za-z0-9][A-Za-z_0-9 .\-]*", raw):
                    return None
                if t == "choice":
                    want = _norm_choice(raw)
                    hit = [c for c in spec["choices"] if _norm_choice(c) == want]
                    out[key] = hit[0] if len(hit) == 1 else raw    # an unknown choice: the runtime refuses it
                else:
                    out[key] = raw
                continue
            ms = re.fullmatch(r"(" + _NUM + r")\s*(.*)", raw)
            if not ms:
                return None
            num, unit = ms.group(1), ms.group(2).strip()
            if t == "integer":
                if unit:
                    return None
                out[key] = num
                continue
            kind = spec.get("kind", "number")
            if kind != "number":
                if not unit:
                    return None
                w = _written_unit(num, unit, kind)
                if w is None:
                    return None
                out[key] = w
                continue
            if spec.get("rho"):
                ul = unit.lower()
                if not unit:
                    out[key] = float(num)
                elif ul in _RHO_UNITS:
                    out[key] = float(num) * _RHO_UNITS[ul]
                elif ul in _DOLLAR:
                    out[key] = ("$", float(num) * _DOLLAR[ul])     # finished by post() with the stated beta
                else:
                    return None
                continue
            if unit:
                if spec.get("pct") and unit.lower() in _PCT:
                    out[key] = float(num) * _PCT[unit.lower()]
                    continue
                return None
            if spec.get("pct") and abs(float(num)) > 1:
                return None                       # '20' for a fraction: 20 % or 20? refused; write 20% or 0.2
            out[key] = float(num)
        if post is not None:
            out = post(out)
        elif any(isinstance(v, tuple) for v in out.values()):
            return None
        return out
    return parse


def _dollars_with(beta_key):
    """post(): a reactivity in dollars or cents is converted with the stated beta (a number or a list summed)."""
    def post(out):
        if out is None:
            return None
        dol = [k for k, v in out.items() if isinstance(v, tuple)]
        if not dol:
            return out
        b = out.get(beta_key)
        if b is None:
            return None                                  # dollars without beta: not converted, refused
        if isinstance(b, str):
            try:
                b = math.fsum(float(x) for x in b.split(","))
            except ValueError:
                return None
        out = dict(out)
        for k in dol:
            out[k] = out[k][1] * float(b)
        return out
    return post


def _written(spec, v):
    """An input value as a user writes it in the documented form."""
    t = spec.get("type", "number")
    if isinstance(v, str) and t not in ("choice", "text"):
        return v
    if t == "vector":
        return "[" + ", ".join(repr(float(x)) for x in v) + "]"
    if t in ("choice", "text"):
        return str(v)
    if t == "integer":
        return str(int(v))
    kind = spec.get("kind", "number")
    if kind != "number":
        return f"{float(v)!r} {_KU[kind]}"
    return repr(float(v))


# ================================================================================================ input/output specs
def _phys(sym, kind, meaning, required=True, **extra):
    _KU[kind]                                   # every physical kind has its SI unit here
    return {"symbol": sym, "kind": kind, "meaning": meaning, "required": required, **extra}


def _numin(sym, meaning, required=True, pct=False, **extra):
    d = {"symbol": sym, "kind": "number", "meaning": meaning, "required": required, **extra}
    if pct:
        d["pct"] = True
    return d


def _rhoin(sym, meaning, required=True):
    return {"symbol": sym, "kind": "number", "meaning": meaning + " (absolute, or in pcm, %, dollars or cents)",
            "required": required, "rho": True}


def _intin(sym, meaning, lo=None, hi=None, required=True):
    d = {"symbol": sym, "type": "integer", "meaning": meaning, "required": required}
    if lo is not None:
        d["min"] = lo
    if hi is not None:
        d["max"] = hi
    return d


def _vecin(sym, meaning, lo=1, hi=50, required=True, unit=None):
    d = {"symbol": sym, "type": "vector", "kind": "number", "meaning": meaning, "required": required,
         "min_length": lo, "max_length": hi}
    if unit:
        d["list_unit"] = unit
    return d


def _choice(sym, choices, meaning, required=True):
    return {"symbol": sym, "type": "choice", "choices": list(choices), "meaning": meaning, "required": required}


def _o(sym, kind, meaning):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


PROCEDURES: dict = {}
LEVELS: dict = {}


def _add(key, *, name, topic, purpose, inputs, outputs, primary, method, latex, run, reference, wrong, rand,
         phrases, examples, level, conditions=(), aliases=None, extra_q=(), verify=None, post=None):
    """Register a procedure: the documented question form is built from its phrases and inputs; templates and
    questions are written from the worked examples exactly as a user writes them (an example may give the written
    form of a value, e.g. "37 MBq", next to its SI number)."""
    parse = _reader(phrases, inputs, aliases, post)
    by = {i["symbol"]: i for i in inputs}

    def ask_text(e, drop=None):
        w = e.get("written", {})
        return f"{phrases[0]}: " + ", ".join(f"{k} = {w[k] if k in w else _written(by[k], v)}"
                                              for k, v in e["inputs"].items() if k != drop)
    templates = [ask_text(e) for e in examples[:2]]
    questions = [{"q": ask_text(e), "outputs": e["outputs"]} for e in examples]
    req = [i["symbol"] for i in inputs if i.get("required")]
    if req and examples:
        questions.append({"q": ask_text(examples[0], drop=req[0]), "outcome": "NEEDS_INPUT"})
        questions.append({"q": ask_text(examples[0]) + ", zz = 1", "outcome": "UNRECOGNIZED_FORMAT"})
    questions += [{"q": q["q"], "outputs": examples[q["like"]]["outputs"]} if "like" in q else q for q in extra_q]
    PROCEDURES[key] = {
        "name": name, "topic": topic, "purpose": purpose, "inputs": inputs, "outputs": outputs, "primary": primary,
        "conditions": list(conditions), "method": method, "latex": latex, "run": run,
        "verify": verify or _recheck(run), "parse": parse, "reference": reference, "wrong": _safe(wrong),
        "random_inputs": rand, "templates": templates, "examples": [{"inputs": e["inputs"], "outputs": e["outputs"]}
                                                                     for e in examples], "questions": questions}
    LEVELS[key] = level


def _ex(inputs, outputs=None, written=None):
    return {"inputs": inputs, "outputs": outputs or {}, "written": written or {}}


# ================================================================================================ DECAY AND ACTIVITY
# half-life (s) and atomic mass (g/mol) of radioactivedecay 0.6.1 (ICRP-107 decay data, its atomic-mass table)
_ND = {
    "H-3": (388781329.30560005, 3.01604928132), "C-14": (179874478656.0, 14.00324198862),
    "Na-22": (82107965.967552, 21.994437547), "P-32": (1232323.2, 31.973907643), "K-40": (3.947771452608e+16, 39.963998165),
    "Co-57": (23478336.0, 56.936289819), "Co-60": (166346024.445504, 59.933815536), "Kr-85": (339426296.91648, 84.91252726),
    "Sr-89": (4365792.0, 88.907450808), "Sr-90": (908523901.8432001, 89.90772787), "Y-90": (230759.99999999997, 89.907141749),
    "Tc-99m": (21654.0, 98.9064028581), "Mo-99": (237384.0, 98.907707299), "I-125": (5132160.0, 124.90463061),
    "I-131": (692988.48, 130.906126375), "I-135": (23652.0, 134.910059355), "Xe-133": (452995.2, 132.905910748),
    "Xe-135": (32904.0, 134.907231441), "Cs-134": (65158740.969984, 133.906718501), "Cs-137": (951980944.7479681, 136.907089296),
    "Ir-192": (6378652.8, 191.962602414), "Tl-201": (262483.2, 200.970820235), "Po-210": (11955686.4, 209.982873686),
    "Rn-222": (330350.4, 222.017576017), "Ra-223": (987552.0, 223.018500648), "Ra-226": (50491081728.0, 226.025408186),
    "Th-230": (2378761087910.4, 230.033132267), "Th-232": (4.43374811424e+17, 232.038053606),
    "U-234": (7747225352640.0, 234.040950296), "U-235": (2.221607596032e+16, 235.043928117),
    "U-238": (1.4099634572544002e+17, 238.050786936), "Np-237": (67658049515520.01, 237.04817164),
    "Pu-238": (2767542417.216, 238.049558175), "Pu-239": (760837487788.8, 239.052161596),
    "Pu-240": (207139662789.12003, 240.05381174), "Pu-241": (452841889.248, 241.056849651),
    "Am-241": (13638903451.776001, 241.056827343), "Cm-244": (571180362.0480001, 244.062750622),
    "Cf-252": (83468069.4816, 252.081626507), "F-18": (6586.2, 18.000937324), "Ga-67": (281767.68, 66.928202276),
    "In-111": (242326.08, 110.905107236), "Fe-55": (86371306.68096001, 54.938291158), "Ni-63": (3158848300.608, 62.929669021),
    "Se-75": (10348905.6, 74.92252287), "Mn-54": (26967168.0, 53.940355772), "Zn-65": (21086784.0, 64.929240534),
    "Ba-133": (331978862.3616, 132.906007443), "Eu-152": (427186108.34496003, 151.92175098),
    "Lu-177": (574300.8, 176.94376357), "Sm-153": (167400.0, 152.922103576), "Ho-166": (96480.0, 165.932291209),
    "Re-188": (61214.4, 187.958113658), "Pm-149": (191088.0, 148.918341507)}

# decay chains of radioactivedecay 0.6.1: parent -> (first daughter, {member: half-life s}, [(from, to, branching)]);
# stable members are left out (they carry no activity)
_CHAIN_DATA = {
    "Sr-90": ("Y-90", {'Sr-90': 908523901.8432001, 'Y-90': 230759.99999999997}, [('Sr-90', 'Y-90', 1.0)]),
    "Cs-137": ("Ba-137m", {'Cs-137': 951980944.7479681, 'Ba-137m': 153.12}, [('Cs-137', 'Ba-137m', 0.94399)]),
    "Mo-99": ("Tc-99m", {'Mo-99': 237384.0, 'Tc-99m': 21654.0, 'Tc-99': 6661667095488.0},
              [('Mo-99', 'Tc-99m', 0.8773), ('Mo-99', 'Tc-99', 0.1227), ('Tc-99m', 'Tc-99', 0.99996)]),
    "Ge-68": ("Ga-68", {'Ge-68': 23410080.0, 'Ga-68': 4062.5999999999995}, [('Ge-68', 'Ga-68', 1.0)]),
    "W-188": ("Re-188", {'W-188': 6028992.0, 'Re-188': 61214.4}, [('W-188', 'Re-188', 1.0)]),
    "Ru-106": ("Rh-106", {'Ru-106': 32278175.999999996, 'Rh-106': 29.8}, [('Ru-106', 'Rh-106', 1.0)]),
    "Ce-144": ("Pr-144", {'Ce-144': 24616224.000000004, 'Pr-144': 1036.8000000000002, 'Pr-144m': 432.0,
                          'Nd-144': 7.22653607232e+22},
               [('Ce-144', 'Pr-144', 0.99023), ('Ce-144', 'Pr-144m', 0.0097699), ('Pr-144', 'Nd-144', 1.0),
                ('Pr-144m', 'Pr-144', 0.9993), ('Pr-144m', 'Nd-144', 0.0007)]),
    "Ba-140": ("La-140", {'Ba-140': 1101772.8, 'La-140': 144987.84}, [('Ba-140', 'La-140', 1.0)]),
    "Te-132": ("I-132", {'Te-132': 276825.60000000003, 'I-132': 8262.0}, [('Te-132', 'I-132', 1.0)]),
    "Sn-113": ("In-113m", {'Sn-113': 9943776.0, 'In-113m': 5968.44}, [('Sn-113', 'In-113m', 0.99998)]),
    "Sr-82": ("Rb-82", {'Sr-82': 2191104.0, 'Rb-82': 76.38}, [('Sr-82', 'Rb-82', 1.0)]),
    "Zr-95": ("Nb-95", {'Zr-95': 5532364.8, 'Nb-95': 3023222.4, 'Nb-95m': 311904.0},
              [('Zr-95', 'Nb-95', 0.9892), ('Zr-95', 'Nb-95m', 0.010802), ('Nb-95m', 'Nb-95', 0.944)]),
    "Sb-125": ("Te-125m", {'Sb-125': 87051674.00724481, 'Te-125m': 4959360.0}, [('Sb-125', 'Te-125m', 0.23136)]),
    "Pb-212": ("Bi-212", {'Pb-212': 38304.0, 'Bi-212': 3633.0, 'Po-212': 2.99e-07, 'Tl-208': 183.18},
               [('Pb-212', 'Bi-212', 1.0), ('Bi-212', 'Po-212', 0.6406), ('Bi-212', 'Tl-208', 0.3594)]),
    "Te-131m": ("I-131", {'Te-131m': 108000.0, 'I-131': 692988.48, 'Te-131': 1500.0, 'Xe-131m': 1022976.0},
                [('Te-131m', 'I-131', 0.778), ('Te-131m', 'Te-131', 0.222), ('I-131', 'Xe-131m', 0.011759),
                 ('Te-131', 'I-131', 1.0)]),
    "I-133": ("Xe-133", {'I-133': 74880.0, 'Xe-133': 452995.2, 'Xe-133m': 189216.0},
              [('I-133', 'Xe-133', 0.97115), ('I-133', 'Xe-133m', 0.028846), ('Xe-133m', 'Xe-133', 1.0)]),
    "Pd-103": ("Rh-103m", {'Pd-103': 1468022.4, 'Rh-103m': 3366.8399999999997}, [('Pd-103', 'Rh-103m', 0.99875)]),
    "Ce-134": ("La-134", {'Ce-134': 273024.0, 'La-134': 387.0}, [('Ce-134', 'La-134', 1.0)]),
    "Ba-131": ("Cs-131", {'Ba-131': 993600.0, 'Cs-131': 837129.6}, [('Ba-131', 'Cs-131', 1.0)]),
    "Hg-195m": ("Hg-195", {'Hg-195m': 149760.0, 'Hg-195': 37908.0, 'Au-195': 16078867.200000001},
                [('Hg-195m', 'Hg-195', 0.542), ('Hg-195m', 'Au-195', 0.458), ('Hg-195', 'Au-195', 1.0)]),
    "Sr-91": ("Y-91m", {'Sr-91': 34668.0, 'Y-91m': 2982.6, 'Y-91': 5055264.0},
              [('Sr-91', 'Y-91m', 0.58247), ('Sr-91', 'Y-91', 0.41753), ('Y-91m', 'Y-91', 1.0)]),
}


def _topo(members, edges):
    """Members in an order where every parent comes before its daughters (Kahn; ties in the table's order)."""
    order, left = [], list(members)
    while left:
        for m in left:
            if not any(b == m and a in left for a, b, _f in edges):
                order.append(m)
                left.remove(m)
                break
        else:
            raise ValueError("the chain has a cycle")
    return order


def _bateman_dag(order, lam, edges, n0, t):
    """Numbers of every member at time t (mpmath, 40 digits): N_i = sum_j c_ij exp(-lambda_j t) with
    c_ij = sum_parents b_pi lambda_p c_pj / (lambda_i - lambda_j) for j != i, c_ii = N_i(0) - sum_{j != i} c_ij."""
    with _mp.workdps(40):
        L = {m: _mpf(lam[m]) for m in order}
        c = {}
        for m in order:
            row = {}
            for a, b, f in edges:
                if b != m:
                    continue
                for j, cj in c[a].items():
                    row[j] = row.get(j, 0) + _mpf(f) * L[a] * cj / (L[m] - L[j])
            row[m] = _mpf(n0.get(m, 0.0)) - sum(row.values(), _mpf(0))
            c[m] = row
        tt = _mpf(t)
        return {m: float(sum((cj * _mp.exp(-L[j] * tt) for j, cj in c[m].items()), _mpf(0))) for m in order}


_CHAINS = {}
for _p, (_d, _hl, _ed) in _CHAIN_DATA.items():
    _CHAINS[_p] = {"daughter": _d, "order": _topo(list(_hl), _ed), "lam": {m: LN2 / h for m, h in _hl.items()},
                   "edges": _ed}


# ------------------------------------------------------------------------------------------------ specific activity
def specific_activity(nuclide, m):
    _need(nuclide in _ND, "the nuclide must be one of the tabulated radionuclides")
    _pos("the mass m", m)
    T, M = _ND[nuclide]
    N = m * 1000.0 / M * NA
    A = LN2 / T * N
    a = LN2 / T * NA / M * 1000.0
    return _out(A=A, A_Ci=A / CI, a=a, a_Ci_per_g=a / 1000.0 / CI, N=N)


def _ref_specific_activity(nuclide, m):
    import radioactivedecay as rd
    inv = rd.Inventory({nuclide: m * 1000.0}, "g")
    A = float(inv.activities("Bq")[nuclide])
    a = float(rd.Inventory({nuclide: 1.0}, "g").activities("Bq")[nuclide]) * 1000.0
    return {"A": A, "A_Ci": A / CI, "a": a, "a_Ci_per_g": a / 1000.0 / CI, "N": float(inv.numbers()[nuclide])}


_NUC = _choice("nuclide", list(_ND), "the radionuclide, written like Co-60, Cs-137, Tc-99m, U-235")
_add("nuc_specific_activity",
     name="Activity and specific activity of a mass of a radionuclide", topic="decay and activity / specific activity",
     purpose="The activity of a stated mass of a named radionuclide, its specific activity (activity per unit mass) and the number of atoms, from its tabulated half-life and atomic mass.",
     inputs=[_NUC, _phys("m", "mass", "mass of the radionuclide (the pure nuclide, not the compound)")],
     outputs=[_o("A", "radioactivity", "activity (Bq)"), _o("A_Ci", "number", "activity in curies"),
              _o("a", "specific_activity", "specific activity (Bq/kg)"), _o("a_Ci_per_g", "number", "specific activity in Ci/g"),
              _o("N", "number", "number of atoms")],
     primary="A", conditions=["the nuclide is in the table (ICRP-107 half-lives, radioactivedecay 0.6.1)", "m > 0"],
     method="N = m N_A / M, A = lambda N with lambda = ln 2 / T_half",
     latex=r"A = \lambda N = \frac{\ln 2}{T_{1/2}}\,\frac{m\,N_A}{M}",
     run=specific_activity, reference=_ref_specific_activity,
     wrong=lambda nuclide, m: (lambda T, M: _out(A=m * 1000 / M * NA / T, A_Ci=m * 1000 / M * NA / T / CI, a=NA / M * 1000 / T,
                                                 a_Ci_per_g=NA / M / T / CI, N=m * 1000 / M * NA))(*_ND[nuclide]),
     rand=lambda rng: {"nuclide": rng.choice(list(_ND)), "m": _sig(_lu(rng, 1e-12, 10.0))},
     phrases=("Specific activity", "Activity of a mass of a radionuclide", "Activity from mass"),
     examples=[_ex({"nuclide": "Co-60", "m": 0.001}, written={"m": "1 g"}),
               _ex({"nuclide": "Ra-226", "m": 0.001}, written={"m": "1 g"}),
               _ex({"nuclide": "Cs-137", "m": 1e-9}, written={"m": "1 ug"})],
     level="beginner",
     extra_q=[{"q": "Specific activity: nuclide = co60, m = 1000 mg", "like": 0},
              {"q": "Specific activity: nuclide = Xx-999, m = 1 g", "outcome": "INVALID_INPUT"},
              {"q": "Specific activity: nuclide = Co-60, m = 1", "outcome": "UNRECOGNIZED_FORMAT"}])


def mass_for_activity(nuclide, A):
    _need(nuclide in _ND, "the nuclide must be one of the tabulated radionuclides")
    _pos("the activity A", A)
    T, M = _ND[nuclide]
    N = A * T / LN2
    m = N * M / NA / 1000.0
    return _out(m=m, m_ug=m * 1e9, N=N)


def _ref_mass_for_activity(nuclide, A):
    import radioactivedecay as rd
    inv = rd.Inventory({nuclide: A}, "Bq")
    m = float(inv.masses("g")[nuclide]) / 1000.0
    return {"m": m, "m_ug": m * 1e9, "N": float(inv.numbers()[nuclide])}


_add("nuc_mass_for_activity",
     name="Mass and number of atoms of a radionuclide with a given activity", topic="decay and activity / specific activity",
     purpose="The mass and the number of atoms of a named radionuclide that has a stated activity (how much material a source of that strength contains).",
     inputs=[_NUC, _phys("A", "radioactivity", "activity (Bq, or Ci)")],
     outputs=[_o("m", "mass", "mass of the nuclide"), _o("m_ug", "number", "mass in micrograms"), _o("N", "number", "number of atoms")],
     primary="m", conditions=["the nuclide is in the table", "A > 0"],
     method="N = A T_half / ln 2, m = N M / N_A",
     latex=r"m = \frac{A\,T_{1/2}}{\ln 2}\,\frac{M}{N_A}",
     run=mass_for_activity, reference=_ref_mass_for_activity,
     wrong=lambda nuclide, A: (lambda T, M: _out(m=A * T * M / NA / 1000, m_ug=A * T * M / NA * 1e6, N=A * T))(*_ND[nuclide]),
     rand=lambda rng: {"nuclide": rng.choice(list(_ND)), "A": _sig(_lu(rng, 1e3, 1e16))},
     phrases=("Mass for activity", "Mass of a radionuclide for an activity", "Mass from activity"),
     examples=[_ex({"nuclide": "Tc-99m", "A": 3.7e8}, written={"A": "10 mCi"}),
               _ex({"nuclide": "Am-241", "A": 37000.0}, written={"A": "37 kBq"})],
     level="beginner",
     extra_q=[{"q": "Mass for activity: nuclide = Tc-99m, A = 370 MBq", "like": 0},
              {"q": "Mass for activity: nuclide = Tc-99m, A = -5 Bq", "outcome": "INVALID_INPUT"}])


# ------------------------------------------------------------------------------------------------ named decay chains
def chain_activity(nuclide, A0, t):
    _need(nuclide in _CHAINS, "the parent must be one of the tabulated chain parents")
    _pos("the initial activity A0", A0)
    _nonneg("the time t", t)
    ch = _CHAINS[nuclide]
    lam = ch["lam"]
    N = _bateman_dag(ch["order"], lam, ch["edges"], {nuclide: A0 / lam[nuclide]}, t)
    A = {m: lam[m] * max(N[m], 0.0) for m in ch["order"]}
    Ap, Ad = A[nuclide], A[ch["daughter"]]
    return _out(A_parent=Ap, A_daughter=Ad, A_total=math.fsum(A.values()), ratio=Ad / Ap)


def _ref_chain_activity(nuclide, A0, t):
    import radioactivedecay as rd
    act = rd.Inventory({nuclide: A0}, "Bq").decay(t, "s").activities("Bq")
    d = _CHAIN_DATA[nuclide][0]
    return {"A_parent": float(act[nuclide]), "A_daughter": float(act[d]), "A_total": float(sum(act.values())),
            "ratio": float(act[d]) / float(act[nuclide])}


def _wrong_chain_activity(nuclide, A0, t):
    ch = _CHAINS[nuclide]
    lp, ld = ch["lam"][nuclide], ch["lam"][ch["daughter"]]
    Ap = A0 * math.exp(-lp * t)
    Ad = A0 * (1 - math.exp(-ld * t))               # the secular build-up applied to every chain (branching ignored)
    return _out(A_parent=Ap, A_daughter=Ad, A_total=Ap + Ad, ratio=Ad / Ap)


def _rand_chain(rng):
    p = rng.choice(list(_CHAINS))
    return {"nuclide": p, "A0": _sig(_lu(rng, 1e3, 1e15)), "t": _sig(_CHAIN_DATA[p][1][p] * _lu(rng, 0.02, 4.0))}


_add("nuc_chain_activity",
     name="Activity of a radionuclide and its progeny after a decay time", topic="decay and activity / decay chains of real nuclides",
     purpose="Starting from a pure parent of stated activity, the activities of the parent, of its first daughter and of the whole chain (all radioactive progeny) after a time t, with the real branching fractions (Sr-90/Y-90, Mo-99/Tc-99m, Cs-137/Ba-137m, Ge-68/Ga-68, ...).",
     inputs=[_choice("nuclide", list(_CHAINS), "the parent nuclide (Sr-90, Cs-137, Mo-99, Ge-68, W-188, Ru-106, Ce-144, Ba-140, Te-132, Sn-113, Sr-82, Zr-95, Sb-125, Pb-212, Te-131m, I-133, Pd-103, Ce-134, Ba-131, Hg-195m, Sr-91)"),
             _phys("A0", "radioactivity", "activity of the pure parent at t = 0"), _phys("t", "time", "decay time")],
     outputs=[_o("A_parent", "radioactivity", "activity of the parent at t"),
              _o("A_daughter", "radioactivity", "activity of the first daughter at t (Y-90, Ba-137m, Tc-99m, Ga-68, Re-188, Rh-106, Pr-144, La-140, I-132, In-113m, Rb-82, Nb-95, Te-125m, Bi-212, I-131, Xe-133, Rh-103m, La-134, Cs-131, Hg-195, Y-91m)"),
              _o("A_total", "radioactivity", "total activity of the parent and all radioactive progeny"),
              _o("ratio", "number", "daughter-to-parent activity ratio")],
     primary="A_daughter", conditions=["pure parent at t = 0", "t >= 0", "branching fractions and half-lives of radioactivedecay 0.6.1"],
     method="Bateman solution of the whole chain (every branch), evaluated with 40-digit arithmetic",
     latex=r"N_i(t) = \sum_j c_{ij}\,e^{-\lambda_j t},\quad c_{ij} = \frac{\sum_p b_{pi}\lambda_p c_{pj}}{\lambda_i - \lambda_j}",
     run=chain_activity, reference=_ref_chain_activity, wrong=_wrong_chain_activity, rand=_rand_chain,
     phrases=("Chain activity", "Decay chain activity", "Progeny activity"),
     examples=[_ex({"nuclide": "Mo-99", "A0": 3.7e10, "t": 86400.0}, written={"A0": "1 Ci", "t": "1 d"}),
               _ex({"nuclide": "Sr-90", "A0": 1e6, "t": 864000.0}, written={"A0": "1 MBq", "t": "10 d"}),
               _ex({"nuclide": "Cs-137", "A0": 1000.0, "t": 3600.0}, written={"A0": "1 kBq", "t": "1 h"})],
     level="intermediate",
     extra_q=[{"q": "Chain activity: nuclide = Mo-99, A0 = 37 GBq, t = 24 h", "like": 0},
              {"q": "Chain activity: nuclide = Co-60, A0 = 1 Ci, t = 1 d", "outcome": "INVALID_INPUT"}])


# ------------------------------------------------------------------------------------------------ three-member chain
def three_member_chain(T1, T2, T3, A10, A20, A30, t):
    _pos("the half-lives", T1, T2, T3)
    _pos("the initial parent activity A10", A10)
    _nonneg("the initial activities A20, A30", A20, A30)
    _nonneg("the time t", t)
    Ts = sorted([T1, T2, T3])
    _need(Ts[1] / Ts[0] > 1.0001 and Ts[2] / Ts[1] > 1.0001, "the three half-lives must differ (by more than 0.01 %)")
    lam = {"1": LN2 / T1, "2": LN2 / T2, "3": LN2 / T3}
    n0 = {"1": A10 / lam["1"], "2": A20 / lam["2"], "3": A30 / lam["3"]}
    N = _bateman_dag(["1", "2", "3"], lam, [("1", "2", 1.0), ("2", "3", 1.0)], n0, t)
    A = [lam[k] * N[k] for k in ("1", "2", "3")]
    return _out(A1=A[0], A2=A[1], A3=A[2], A_total=math.fsum(A))


def _ref_three_member_chain(T1, T2, T3, A10, A20, A30, t):
    import mpmath as mp
    with mp.workdps(40):
        l1, l2, l3 = (mp.log(2) / mp.mpf(T) for T in (T1, T2, T3))
        Mx = mp.matrix([[-l1, 0, 0], [l1, -l2, 0], [0, l2, -l3]])
        n0 = mp.matrix([mp.mpf(A10) / l1, mp.mpf(A20) / l2, mp.mpf(A30) / l3])
        n = mp.expm(Mx * mp.mpf(t)) * n0
        A = [float(l1 * n[0]), float(l2 * n[1]), float(l3 * n[2])]
    return {"A1": A[0], "A2": A[1], "A3": A[2], "A_total": sum(A)}


def _wrong_three(T1, T2, T3, A10, A20, A30, t):
    l1, l2, l3 = LN2 / T1, LN2 / T2, LN2 / T3
    e1, e2, e3 = math.exp(-l1 * t), math.exp(-l2 * t), math.exp(-l3 * t)
    N1 = A10 / l1 * e1
    N2 = A20 / l2 * e2 + A10 / l1 * l1 / (l1 - l2) * (e1 - e2)        # sign of the denominator swapped
    N3 = A30 / l3 * e3
    return _out(A1=l1 * N1, A2=l2 * N2, A3=l3 * N3, A_total=l1 * N1 + l2 * N2 + l3 * N3)


def _rand_three(rng):
    while True:
        T1 = _lu(rng, 1.0, 1e9)
        T2, T3 = T1 * _lu(rng, 0.02, 50.0), T1 * _lu(rng, 0.02, 50.0)
        Ts = sorted([T1, T2, T3])
        if Ts[1] / Ts[0] > 1.05 and Ts[2] / Ts[1] > 1.05:
            break
    A10 = _lu(rng, 1e3, 1e12)
    A20 = rng.choice([0.0, A10 * rng.uniform(0.0, 2.0)])
    A30 = rng.choice([0.0, A10 * rng.uniform(0.0, 2.0)])
    return {"T1": _sig(T1), "T2": _sig(T2), "T3": _sig(T3), "A10": _sig(A10), "A20": _sig(A20), "A30": _sig(A30),
            "t": _sig(T1 * _lu(rng, 0.02, 5.0))}


_add("nuc_three_member_chain",
     name="Three-member decay chain with initial activities (Bateman)", topic="decay and activity / decay chains",
     purpose="Activities of the three radioactive members of a chain 1 -> 2 -> 3 after a time t, from their half-lives and the initial activity of every member (the daughters need not start at zero).",
     inputs=[_phys("T1", "time", "half-life of member 1 (the parent)"), _phys("T2", "time", "half-life of member 2"),
             _phys("T3", "time", "half-life of member 3"), _phys("A10", "radioactivity", "initial activity of member 1"),
             _phys("A20", "radioactivity", "initial activity of member 2 (0 if none)"),
             _phys("A30", "radioactivity", "initial activity of member 3 (0 if none)"), _phys("t", "time", "elapsed time")],
     outputs=[_o("A1", "radioactivity", "activity of member 1"), _o("A2", "radioactivity", "activity of member 2"),
              _o("A3", "radioactivity", "activity of member 3"), _o("A_total", "radioactivity", "total activity")],
     primary="A3", conditions=["every decay of 1 gives 2 and of 2 gives 3 (no branching)", "the half-lives differ", "t >= 0"],
     method="Bateman solution with the initial numbers N_i0 = A_i0 / lambda_i, in 40-digit arithmetic",
     latex=r"N_3 = N_{30}e^{-\lambda_3 t} + \frac{\lambda_2 N_{20}}{\lambda_3-\lambda_2}(e^{-\lambda_2 t}-e^{-\lambda_3 t}) + \lambda_1\lambda_2 N_{10}\sum_{i}\frac{e^{-\lambda_i t}}{\prod_{j\ne i}(\lambda_j-\lambda_i)}",
     run=three_member_chain, reference=_ref_three_member_chain, wrong=_wrong_three, rand=_rand_three,
     phrases=("Three-member chain", "Three-member decay chain", "Bateman three-member chain"),
     examples=[_ex({"T1": 100.0, "T2": 10.0, "T3": 1.0, "A10": 1000.0, "A20": 0.0, "A30": 0.0, "t": 50.0}),
               _ex({"T1": 3600.0, "T2": 7200.0, "T3": 600.0, "A10": 1e6, "A20": 5e5, "A30": 0.0, "t": 1800.0},
                   written={"T1": "1 h", "T2": "2 h", "T3": "10 min", "A10": "1 MBq", "A20": "0.5 MBq", "A30": "0 Bq", "t": "30 min"})],
     level="advanced",
     extra_q=[{"q": "Three-member chain: T1 = 100 s, T2 = 100 s, T3 = 1 s, A10 = 1000 Bq, A20 = 0 Bq, A30 = 0 Bq, t = 50 s", "outcome": "INVALID_INPUT"}])


# ------------------------------------------------------------------------------------------------ daughter maximum
def _tmax(lp, ld):
    r = ld / lp
    if r == 1.0:
        return 1.0 / lp
    return math.log1p(r - 1.0) / (lp * (r - 1.0))


def daughter_max_time(T_P, T_D, A_P0=None):
    _pos("the half-lives", T_P, T_D)
    lp, ld = LN2 / T_P, LN2 / T_D
    t = _tmax(lp, ld)
    ratio = math.exp(-lp * t)                     # at the maximum the daughter activity equals the parent activity
    out = dict(t_max=t, ratio_max=ratio, t_max_half_lives=t / T_D)
    if A_P0 is not None:
        _pos("the initial parent activity A_P0", A_P0)
        out["A_D_max"] = A_P0 * ratio
    return _out(**out)


def _conv(a, b, t):
    """Reference helper: the convolution integral of e^(-a s) with e^(-b (t - s)) over [0, t] by quadrature. The
    common factor e^(-min(a, b) t) is taken out first (exact algebra), so the remaining integrand starts at 1 and the
    quadrature never misses a narrow peak at one end of a long interval."""
    if t <= 0:
        return 0.0
    k = abs(b - a)
    lo = min(a, b)
    if k == 0:
        return t * math.exp(-a * t)
    return math.exp(-lo * t) * _quad(lambda u: math.exp(-k * u), 0.0, min(t, 60.0 / k))


def _conv_daughter(lp, ld, t):
    """N_D(t) for N_P(0) = 1, N_D(0) = 0: lambda_P times the convolution of the parent's decay with the daughter's."""
    return lp * _conv(lp, ld, t)


def _ref_daughter_max_time(T_P, T_D, A_P0=None):
    lp, ld = LN2 / T_P, LN2 / T_D
    g = lambda t: lp * math.exp(-lp * t) - ld * _conv_daughter(lp, ld, t)       # dN_D/dt
    lo, hi = 1e-3 / (lp + ld), 60.0 / min(lp, ld)
    t = _brentq(g, lo, hi)
    ratio = ld * _conv_daughter(lp, ld, t) / lp
    out = {"t_max": t, "ratio_max": ratio, "t_max_half_lives": t / T_D}
    if A_P0 is not None:
        out["A_D_max"] = A_P0 * ratio
    return out


_add("nuc_daughter_max_activity_time",
     name="Time of maximum daughter activity", topic="decay and activity / decay chains and equilibrium",
     purpose="When the activity of a radioactive daughter, growing from a pure parent, reaches its maximum, and how large it is then (it equals the parent's activity at that moment).",
     inputs=[_phys("T_P", "time", "half-life of the parent"), _phys("T_D", "time", "half-life of the daughter"),
             _phys("A_P0", "radioactivity", "initial parent activity (optional: gives the maximum daughter activity)", required=False)],
     outputs=[_o("t_max", "time", "time of maximum daughter activity"),
              _o("ratio_max", "number", "maximum daughter activity / initial parent activity"),
              _o("t_max_half_lives", "number", "t_max in daughter half-lives"),
              _o("A_D_max", "radioactivity", "maximum daughter activity (when A_P0 is given)")],
     primary="t_max", conditions=["pure parent at t = 0", "every parent decay gives the daughter"],
     method="dA_D/dt = 0: t_max = ln(lambda_D/lambda_P)/(lambda_D - lambda_P) (1/lambda when equal); A_D(t_max) = A_P(t_max)",
     latex=r"t_{max} = \frac{\ln(\lambda_D/\lambda_P)}{\lambda_D - \lambda_P}",
     run=daughter_max_time, reference=_ref_daughter_max_time,
     wrong=lambda T_P, T_D, A_P0=None: _out(t_max=math.log(T_P / T_D) / (LN2 / T_D + LN2 / T_P), ratio_max=0.5, t_max_half_lives=0.0),
     rand=lambda rng: (lambda TP: {"T_P": _sig(TP), "T_D": _sig(TP * _lu(rng, 0.01, 100.0)),
                                   **({"A_P0": _sig(_lu(rng, 1e3, 1e12))} if rng.random() < 0.5 else {})})(_lu(rng, 1.0, 1e9)),
     phrases=("Daughter maximum activity time", "Time of maximum daughter activity", "Daughter maximum"),
     examples=[_ex({"T_P": 237384.0, "T_D": 21654.0}, written={"T_P": "65.94 h", "T_D": "6.015 h"}),
               _ex({"T_P": 10.0, "T_D": 10.0, "A_P0": 1000.0})],
     level="intermediate")


# ------------------------------------------------------------------------------------------------ equilibrium ratio
def equilibrium_ratio(T_P, T_D):
    _pos("the half-lives", T_P, T_D)
    _need(T_P > T_D, "no equilibrium: the parent must live longer than the daughter (T_P > T_D)")
    lp, ld = LN2 / T_P, LN2 / T_D
    return _out(ratio_A=T_P / (T_P - T_D), ratio_N=T_D / (T_P - T_D), t_99=math.log(100.0) / (ld - lp))


def _ref_equilibrium_ratio(T_P, T_D):
    lp, ld = LN2 / T_P, LN2 / T_D
    k = ld - lp
    v = _ivp(lambda t, y: [lp - k * y[0]], 45.0 / k, [0.0], atol=[1e-30])[0]   # v = N_D/N_P from a pure parent
    ratio_A = ld / lp * v
    frac = lambda t: lp * _quad(lambda u: math.exp(-k * u), 0.0, t) / v - 0.99
    return {"ratio_A": ratio_A, "ratio_N": v, "t_99": _brentq(frac, 1e-6 / k, 40.0 / k)}


_add("nuc_equilibrium_ratio",
     name="Transient and secular equilibrium: activity ratio and approach", topic="decay and activity / decay chains and equilibrium",
     purpose="The daughter-to-parent activity and atom ratios once equilibrium is reached (transient; close to 1 in secular equilibrium) and the time after which the activity ratio is within 1 % of it, starting from pure parent.",
     inputs=[_phys("T_P", "time", "half-life of the parent"), _phys("T_D", "time", "half-life of the daughter (shorter than the parent's)")],
     outputs=[_o("ratio_A", "number", "equilibrium activity ratio A_D/A_P = T_P/(T_P - T_D)"),
              _o("ratio_N", "number", "equilibrium atom ratio N_D/N_P"),
              _o("t_99", "time", "time to reach 99 % of the equilibrium activity ratio")],
     primary="ratio_A", conditions=["T_P > T_D", "pure parent at t = 0", "every parent decay gives the daughter"],
     method="A_D/A_P -> lambda_D/(lambda_D - lambda_P); the ratio approaches it as 1 - exp(-(lambda_D - lambda_P) t)",
     latex=r"\frac{A_D}{A_P} = \frac{\lambda_D}{\lambda_D-\lambda_P}\left(1 - e^{-(\lambda_D-\lambda_P)t}\right)",
     run=equilibrium_ratio, reference=_ref_equilibrium_ratio,
     wrong=lambda T_P, T_D: _out(ratio_A=T_D / (T_P - T_D), ratio_N=T_P / (T_P - T_D), t_99=math.log(100.0) / (LN2 / T_D)),
     rand=lambda rng: (lambda TP: {"T_P": _sig(TP), "T_D": _sig(TP * _lu(rng, 1e-4, 0.8))})(_lu(rng, 1.0, 1e10)),
     phrases=("Equilibrium ratio", "Transient equilibrium", "Secular equilibrium", "Parent-daughter equilibrium"),
     examples=[_ex({"T_P": 237384.0, "T_D": 21654.0}), _ex({"T_P": 908523901.8432001, "T_D": 230760.0})],
     level="beginner",
     extra_q=[{"q": "Transient equilibrium: T_P = 1 h, T_D = 2 h", "outcome": "INVALID_INPUT"}])


# ------------------------------------------------------------------------------------------------ generator
def _ad_build(lp, ld, t):
    """N_D(t)/N_P(0) for a pure parent, no cancellation (40 digits)."""
    with _mp.workdps(40):
        a, b, tt = _mpf(lp), _mpf(ld), _mpf(t)
        if lp == ld:
            return float(a * tt * _mp.exp(-a * tt))
        return float(a / (b - a) * (_mp.exp(-a * tt) - _mp.exp(-b * tt)))


def generator(A_P0, T_P, T_D, b, t):
    _pos("the parent activity A_P0", A_P0)
    _pos("the half-lives", T_P, T_D)
    _frac("the branching fraction b", b, lo_open=True)
    _nonneg("the time t", t)
    lp, ld = LN2 / T_P, LN2 / T_D
    nd = _ad_build(lp, ld, t)
    A_D = b * A_P0 / lp * ld * nd
    A_P = A_P0 * math.exp(-lp * t)
    tm = _tmax(lp, ld)
    A_Dmax = b * A_P0 / lp * ld * _ad_build(lp, ld, tm)
    return _out(A_D=A_D, A_P=A_P, ratio=A_D / A_P, t_max=tm, A_D_max=A_Dmax)


def _ref_generator(A_P0, T_P, T_D, b, t):
    lp, ld = LN2 / T_P, LN2 / T_D
    A_D = b * A_P0 / lp * ld * _conv_daughter(lp, ld, t) if t > 0 else 0.0
    g = lambda s: lp * math.exp(-lp * s) - ld * _conv_daughter(lp, ld, s)
    tm = _brentq(g, 1e-3 / (lp + ld), 60.0 / min(lp, ld))
    A_P = A_P0 * float(_ivp(lambda s, y: [-lp * y[0]], t, [1.0])[0]) if t > 0 else A_P0
    return {"A_D": A_D, "A_P": A_P, "ratio": A_D / A_P, "t_max": tm, "A_D_max": b * A_P0 / lp * ld * _conv_daughter(lp, ld, tm)}


_add("nuc_generator_daughter_activity",
     name="Radionuclide generator: daughter activity after elution (with branching)", topic="decay and activity / generators",
     purpose="The daughter activity available in a generator (Mo-99/Tc-99m, Ge-68/Ga-68, Sr-82/Rb-82 ...) a time t after the last complete elution, with the branching fraction to the daughter, plus when the daughter activity peaks and its peak value.",
     inputs=[_phys("A_P0", "radioactivity", "parent activity at the elution"), _phys("T_P", "time", "half-life of the parent"),
             _phys("T_D", "time", "half-life of the daughter"),
             _numin("b", "fraction of parent decays that produce the daughter (0 < b <= 1, e.g. 0.8773 for Mo-99 -> Tc-99m)", pct=True),
             _phys("t", "time", "time since the elution")],
     outputs=[_o("A_D", "radioactivity", "daughter activity at t"), _o("A_P", "radioactivity", "parent activity at t"),
              _o("ratio", "number", "A_D / A_P"), _o("t_max", "time", "time of maximum daughter activity"),
              _o("A_D_max", "radioactivity", "maximum daughter activity")],
     primary="A_D", conditions=["daughter fully removed at the elution", "0 < b <= 1"],
     method="A_D(t) = b A_P0 lambda_D/(lambda_D - lambda_P) (e^(-lambda_P t) - e^(-lambda_D t)) (40 digits); t_max = ln(lambda_D/lambda_P)/(lambda_D - lambda_P)",
     latex=r"A_D(t) = b\,A_{P0}\frac{\lambda_D}{\lambda_D-\lambda_P}\left(e^{-\lambda_P t}-e^{-\lambda_D t}\right)",
     run=generator, reference=_ref_generator,
     wrong=lambda A_P0, T_P, T_D, b, t: (lambda o: {**o, "A_D": _r(o["A_D"] / b), "A_D_max": _r(o["A_D_max"] / b)})(generator(A_P0, T_P, T_D, b, t)),
     rand=lambda rng: (lambda TP, TD: {"A_P0": _sig(_lu(rng, 1e6, 1e12)), "T_P": _sig(TP), "T_D": _sig(TD), "b": round(rng.uniform(0.3, 0.95), 4),
                                       "t": _sig(TD * _lu(rng, 0.05, 20.0))})(*(lambda TP: (TP, TP * _lu(rng, 0.005, 0.5)))(_lu(rng, 1e3, 1e8))),
     phrases=("Generator daughter activity", "Radionuclide generator", "Generator elution"),
     examples=[_ex({"A_P0": 3.7e10, "T_P": 237384.0, "T_D": 21654.0, "b": 0.8773, "t": 86400.0},
                   written={"A_P0": "1 Ci", "T_P": "65.94 h", "T_D": "6.015 h", "t": "24 h"}),
               _ex({"A_P0": 1e9, "T_P": 23410080.0, "T_D": 4062.6, "b": 1.0, "t": 3600.0},
                   written={"A_P0": "1 GBq", "t": "1 h"})],
     level="intermediate",
     extra_q=[{"q": "Generator elution: A_P0 = 1 Ci, T_P = 65.94 h, T_D = 6.015 h, b = 87.73%, t = 24 h", "like": 0},
              {"q": "Generator elution: A_P0 = 1 Ci, T_P = 65.94 h, T_D = 6.015 h, b = 87.73, t = 24 h", "outcome": "UNRECOGNIZED_FORMAT"}])


# ------------------------------------------------------------------------------------------------ counting a decaying source
def counts_during_decay(A0, T_half, tc, eff, y, td=0.0):
    _pos("the activity A0", A0)
    _pos("the half-life and the counting time", T_half, tc)
    _nonneg("the delay td", td)
    _frac("the efficiency", eff, lo_open=True)
    _frac("the emission probability", y, lo_open=True)
    lam = LN2 / T_half
    x = lam * tc
    decays = A0 * math.exp(-lam * td) * (-math.expm1(-x)) / lam
    N = eff * y * decays
    return _out(N=N, A_mean=decays / tc, F=x / (-math.expm1(-x)))


def _ref_counts_during_decay(A0, T_half, tc, eff, y, td=0.0):
    lam = LN2 / T_half
    decays = _quad(lambda s: A0 * math.exp(-lam * (td + s)), 0.0, tc)
    start = A0 * math.exp(-lam * td)
    return {"N": eff * y * decays, "A_mean": decays / tc, "F": start * tc / decays}


_add("nuc_counts_during_decay",
     name="Counts recorded from a decaying source (decay during counting)", topic="detectors and counting / decay corrections",
     purpose="The expected number of counts recorded from a source that decays during the count, the mean activity during the count and the decay-correction factor that converts the mean activity to the activity at the start of counting.",
     inputs=[_phys("A0", "radioactivity", "activity at the reference time"), _phys("T_half", "time", "half-life"),
             _phys("tc", "time", "counting time"), _numin("eff", "detection efficiency (fraction of emitted radiations counted)", pct=True),
             _numin("y", "emission probability per decay (1 when every decay is counted)", pct=True),
             _phys("td", "time", "delay from the reference time to the start of counting (0 when omitted)", required=False)],
     outputs=[_o("N", "number", "expected counts"), _o("A_mean", "radioactivity", "mean activity during the count"),
              _o("F", "number", "decay-correction factor: activity at the start of counting / mean activity")],
     primary="N", conditions=["0 < eff <= 1", "0 < y <= 1", "no dead-time losses"],
     method="N = eff y A0 e^(-lambda td) (1 - e^(-lambda tc))/lambda; F = lambda tc/(1 - e^(-lambda tc))",
     latex=r"N = \varepsilon\,y\,A_0e^{-\lambda t_d}\frac{1-e^{-\lambda t_c}}{\lambda}",
     run=counts_during_decay, reference=_ref_counts_during_decay,
     wrong=lambda A0, T_half, tc, eff, y, td=0.0: _out(N=eff * y * A0 * math.exp(-LN2 / T_half * td) * tc, A_mean=A0, F=1.0),
     rand=lambda rng: (lambda T: {"A0": _sig(_lu(rng, 10.0, 1e9)), "T_half": _sig(T), "tc": _sig(T * _lu(rng, 0.05, 5.0)),
                                  "eff": round(rng.uniform(0.01, 0.9), 4), "y": round(rng.uniform(0.05, 1.0), 4),
                                  **({"td": _sig(T * _lu(rng, 0.01, 3.0))} if rng.random() < 0.6 else {})})(_lu(rng, 10.0, 1e8)),
     phrases=("Counts during decay", "Counting a decaying source", "Decay during counting"),
     examples=[_ex({"A0": 1e5, "T_half": 6586.2, "tc": 3600.0, "eff": 0.2, "y": 0.967},
                   written={"A0": "100 kBq", "tc": "1 h", "eff": "20%", "y": "96.7%"}),
               _ex({"A0": 3.7e4, "T_half": 600.0, "tc": 600.0, "eff": 0.1, "y": 1.0, "td": 300.0},
                   written={"A0": "1 uCi", "T_half": "10 min", "tc": "10 min", "td": "5 min"})],
     level="beginner")


# ------------------------------------------------------------------------------------------------ effective half-life
def effective_half_life(T_p, T_b, A0=None):
    _pos("the half-lives", T_p, T_b)
    Te = T_p * T_b / (T_p + T_b)
    out = dict(T_eff=Te, lambda_eff=LN2 / Te)
    if A0 is not None:
        _pos("the initial activity A0", A0)
        out["N_decays"] = A0 * Te / LN2
    return _out(**out)


def _ref_effective_half_life(T_p, T_b, A0=None):
    surv = lambda t: 2.0 ** (-t / T_p) * 2.0 ** (-t / T_b)          # two independent removal processes
    Te = _brentq(lambda t: surv(t) - 0.5, 0.0, min(T_p, T_b))
    out = {"T_eff": Te, "lambda_eff": LN2 / Te}
    if A0 is not None:
        k = LN2 / T_p + LN2 / T_b
        out["N_decays"] = A0 * _quad(lambda s: math.exp(-s), 0.0, math.inf) / k
    return out


_add("nuc_effective_half_life",
     name="Effective half-life (physical and biological removal)", topic="dosimetry / internal emitters",
     purpose="The effective half-life of a radionuclide in the body or another system that removes it biologically while it decays, the effective decay constant and (with the initial activity) the total number of decays.",
     inputs=[_phys("T_p", "time", "physical half-life"), _phys("T_b", "time", "biological half-life"),
             _phys("A0", "radioactivity", "initial activity in the body (optional)", required=False)],
     outputs=[_o("T_eff", "time", "effective half-life"), _o("lambda_eff", "decay_constant", "effective removal constant"),
              _o("N_decays", "number", "total number of decays in the body (cumulated activity, when A0 is given)")],
     primary="T_eff", conditions=["single-exponential biological removal"],
     method="1/T_eff = 1/T_p + 1/T_b; cumulated activity A0 T_eff/ln 2",
     latex=r"\frac{1}{T_{eff}} = \frac{1}{T_p} + \frac{1}{T_b}",
     run=effective_half_life, reference=_ref_effective_half_life,
     wrong=lambda T_p, T_b, A0=None: _out(T_eff=(T_p + T_b) / 2, lambda_eff=2 * LN2 / (T_p + T_b), **({"N_decays": A0 * (T_p + T_b) / 2} if A0 else {})),
     rand=lambda rng: {"T_p": _sig(_lu(rng, 60.0, 1e10)), "T_b": _sig(_lu(rng, 60.0, 1e10)),
                       **({"A0": _sig(_lu(rng, 1e3, 1e10))} if rng.random() < 0.5 else {})},
     phrases=("Effective half-life", "Effective half life"),
     examples=[_ex({"T_p": 692988.48, "T_b": 6912000.0}, written={"T_p": "8.0207 d", "T_b": "80 d"}),
               _ex({"T_p": 21654.0, "T_b": 86400.0, "A0": 3.7e8}, written={"T_b": "1 d", "A0": "10 mCi"})],
     level="beginner")


# ------------------------------------------------------------------------------------------------ neutron activation
def activation(m, M, theta, sigma, phi, T_half, t_irr, t_d=0.0):
    _pos("the target mass, molar mass, cross section, flux, half-life and irradiation time", m, M, sigma, phi, T_half, t_irr)
    _frac("the isotopic abundance theta", theta, lo_open=True)
    _nonneg("the decay time t_d", t_d)
    lam = LN2 / T_half
    R = m / M * NA * theta * sigma * phi
    S = -math.expm1(-lam * t_irr)
    A_eoi = R * S
    return _out(A_eoi=A_eoi, A_d=A_eoi * math.exp(-lam * t_d), A_sat=R, S=S)


def _ref_activation(m, M, theta, sigma, phi, T_half, t_irr, t_d=0.0):
    lam = LN2 / T_half
    R = m / M * NA * theta * sigma * phi
    # dN*/ds = 1 - N* in units of 1/lambda (rate R, decay lambda), then free decay
    n = _ivp(lambda s, y: [1.0 - y[0]], lam * t_irr, [0.0], atol=[1e-30])[0]
    nd = _ivp(lambda s, y: [-y[0]], lam * t_d, [n], atol=[1e-300])[0] if t_d > 0 else n
    nsat = _ivp(lambda s, y: [1.0 - y[0]], 60.0, [0.0], atol=[1e-30])[0]
    return {"A_eoi": R * n, "A_d": R * nd, "A_sat": R * nsat, "S": n}


_add("nuc_activation_activity",
     name="Neutron activation: activity after irradiation and decay", topic="decay and activity / neutron activation",
     purpose="The activity induced in a target by a neutron flux: at the end of irradiation, after a decay time, at saturation, and the saturation fraction reached (target depletion neglected).",
     inputs=[_phys("m", "mass", "mass of the target element"), _phys("M", "molar_mass", "molar mass of the target element"),
             _numin("theta", "isotopic abundance of the target nuclide", pct=True),
             _phys("sigma", "microscopic_cross_section", "activation cross section (barns)"),
             _phys("phi", "neutron_flux", "neutron flux"), _phys("T_half", "time", "half-life of the product"),
             _phys("t_irr", "time", "irradiation time"), _phys("t_d", "time", "decay time after irradiation (0 when omitted)", required=False)],
     outputs=[_o("A_eoi", "radioactivity", "activity at the end of irradiation"), _o("A_d", "radioactivity", "activity after the decay time"),
              _o("A_sat", "radioactivity", "saturation activity N sigma phi"), _o("S", "number", "saturation fraction 1 - e^(-lambda t_irr)")],
     primary="A_eoi", conditions=["constant flux", "target depletion and product burn-up neglected", "0 < theta <= 1"],
     method="A(t_irr) = N sigma phi (1 - e^(-lambda t_irr)), N = m N_A theta / M; then A e^(-lambda t_d)",
     latex=r"A = N\sigma\phi\left(1-e^{-\lambda t_{irr}}\right)e^{-\lambda t_d}",
     run=activation, reference=_ref_activation,
     wrong=lambda m, M, theta, sigma, phi, T_half, t_irr, t_d=0.0: (lambda R: _out(A_eoi=R * math.exp(-LN2 / T_half * t_irr), A_d=R * math.exp(-LN2 / T_half * (t_irr + t_d)),
                                                                                    A_sat=R, S=math.exp(-LN2 / T_half * t_irr)))(m / M * NA * theta * sigma * phi),
     rand=lambda rng: (lambda T: {"m": _sig(_lu(rng, 1e-7, 1e-2)), "M": _sig(rng.uniform(0.01, 0.24)), "theta": round(rng.uniform(0.01, 1.0), 4),
                                  "sigma": _sig(_lu(rng, 0.01, 100.0)) * BARN, "phi": _sig(_lu(rng, 1e12, 1e18)), "T_half": _sig(T),
                                  "t_irr": _sig(T * _lu(rng, 0.01, 8.0)), **({"t_d": _sig(T * _lu(rng, 0.01, 5.0))} if rng.random() < 0.6 else {})})(_lu(rng, 10.0, 1e8)),
     phrases=("Neutron activation", "Activation activity", "Activation"),
     examples=[_ex({"m": 1e-3, "M": 0.058933195, "theta": 1.0, "sigma": 37.18e-28, "phi": 1e17, "T_half": 166346024.445504, "t_irr": 2592000.0},
                   written={"m": "1 g", "M": "58.933195 g/mol", "sigma": "37.18 b", "phi": "1e13 n/(cm^2*s)", "t_irr": "30 d"}),
               _ex({"m": 1e-4, "M": 0.02698, "theta": 1.0, "sigma": 0.231e-28, "phi": 1e16, "T_half": 134.7, "t_irr": 600.0, "t_d": 60.0},
                   written={"m": "0.1 g", "M": "26.98 g/mol", "sigma": "0.231 barn", "phi": "1e12 1/(cm^2*s)", "t_irr": "10 min", "t_d": "1 min"})],
     level="intermediate")


def activation_burnup(N_T0, sigma_T, sigma_P, phi, T_half, t):
    _pos("N_T0, the cross sections, the flux and the half-life", N_T0, sigma_T, sigma_P, phi, T_half)
    _nonneg("the time t", t)
    lam = LN2 / T_half
    LT, LP = sigma_T * phi, lam + sigma_P * phi
    _need(abs(LP / LT - 1.0) > 1e-6, "the product's removal rate equals the target's burn-up rate: not supported")
    with _mp.workdps(40):
        a, b, tt = _mpf(LT), _mpf(LP), _mpf(t)
        NP = float(_mpf(N_T0) * a / (b - a) * (_mp.exp(-a * tt) - _mp.exp(-b * tt)))
        tm = float(_mp.log(b / a) / (b - a))
        NPm = float(_mpf(N_T0) * a / (b - a) * (_mp.exp(-a * tm) - _mp.exp(-b * tm)))
    return _out(A_P=lam * NP, f_T=math.exp(-LT * t), t_max=tm, A_max=lam * NPm)


def _ref_activation_burnup(N_T0, sigma_T, sigma_P, phi, T_half, t):
    lam = LN2 / T_half
    LT, LP = sigma_T * phi, lam + sigma_P * phi
    conv = lambda s: N_T0 * LT * _conv(LT, LP, s)
    fT = _ivp(lambda s, y: [-y[0]], LT * t, [1.0], atol=[1e-300])[0] if t > 0 else 1.0
    tm = _brentq(lambda s: LT * N_T0 * math.exp(-LT * s) - LP * conv(s), 1e-3 / (LT + LP), 60.0 / min(LT, LP))
    return {"A_P": lam * conv(t), "f_T": fT, "t_max": tm, "A_max": lam * conv(tm)}


_add("nuc_activation_with_burnup",
     name="Activation with target depletion and product burn-up", topic="decay and activity / neutron activation",
     purpose="The product activity when a high flux both depletes the target nuclide and burns up the product by its own absorption, the fraction of target left, and the time and value of the maximum product activity.",
     inputs=[_numin("N_T0", "initial number of target atoms"), _phys("sigma_T", "microscopic_cross_section", "target capture cross section (barns)"),
             _phys("sigma_P", "microscopic_cross_section", "product absorption cross section (barns)"),
             _phys("phi", "neutron_flux", "neutron flux"), _phys("T_half", "time", "half-life of the product"), _phys("t", "time", "irradiation time")],
     outputs=[_o("A_P", "radioactivity", "product activity at t"), _o("f_T", "number", "fraction of target atoms left"),
              _o("t_max", "time", "irradiation time of maximum product activity"), _o("A_max", "radioactivity", "maximum product activity")],
     primary="A_P", conditions=["constant flux", "removal rates sigma_T phi and lambda + sigma_P phi differ"],
     method="dN_T/dt = -sigma_T phi N_T, dN_P/dt = sigma_T phi N_T - (lambda + sigma_P phi) N_P: N_P = N_T0 sigma_T phi (e^(-a t) - e^(-b t))/(b - a) (40 digits)",
     latex=r"N_P = \frac{N_{T0}\sigma_T\phi}{\Lambda_P-\Lambda_T}\left(e^{-\Lambda_T t}-e^{-\Lambda_P t}\right),\ \Lambda_T=\sigma_T\phi,\ \Lambda_P=\lambda+\sigma_P\phi",
     run=activation_burnup, reference=_ref_activation_burnup,
     wrong=lambda N_T0, sigma_T, sigma_P, phi, T_half, t: (lambda lam: _out(A_P=N_T0 * sigma_T * phi * -math.expm1(-lam * t), f_T=1.0, t_max=0.0, A_max=N_T0 * sigma_T * phi))(LN2 / T_half),
     rand=lambda rng: (lambda T, phi: {"N_T0": _sig(_lu(rng, 1e18, 1e24)), "sigma_T": _sig(_lu(rng, 1.0, 1e4)) * BARN, "sigma_P": _sig(_lu(rng, 0.1, 1e5)) * BARN,
                                       "phi": phi, "T_half": _sig(T), "t": _sig(T * _lu(rng, 0.05, 5.0))})(_lu(rng, 3600.0, 1e9), _sig(_lu(rng, 1e17, 1e19))),
     phrases=("Activation with burnup", "Activation with burn-up", "Activation with target depletion"),
     examples=[_ex({"N_T0": 1e22, "sigma_T": 37.18e-28, "sigma_P": 2e-28, "phi": 1e18, "T_half": 166346024.445504, "t": 31557600.0},
                   written={"sigma_T": "37.18 b", "sigma_P": "2 b", "phi": "1e14 n/(cm^2*s)", "t": "1 y"}),
               _ex({"N_T0": 1e20, "sigma_T": 1000e-28, "sigma_P": 5000e-28, "phi": 5e18, "T_half": 3.0e6, "t": 1e6},
                   written={"sigma_T": "1000 b", "sigma_P": "5000 b", "phi": "5e14 1/(cm^2*s)"})],
     level="expert")


def irradiation_time(T_half, f):
    _pos("the half-life", T_half)
    _frac("the saturation fraction f", f, lo_open=True, hi_open=True)
    t = -math.log1p(-f) * T_half / LN2
    return _out(t_irr=t, n_half=t / T_half)


def _ref_irradiation_time(T_half, f):
    lam = LN2 / T_half
    S = lambda t: lam * _quad(lambda u: math.exp(-lam * u), 0.0, t)
    t = _brentq(lambda t: S(t) - f, 0.0, 80.0 / lam)
    return {"t_irr": t, "n_half": t / T_half}


_add("nuc_irradiation_time_for_saturation",
     name="Irradiation time to reach a fraction of saturation activity", topic="decay and activity / neutron activation",
     purpose="How long a target must be irradiated (constant flux) for the induced activity to reach a stated fraction of its saturation value, in time and in product half-lives.",
     inputs=[_phys("T_half", "time", "half-life of the product"), _numin("f", "fraction of the saturation activity (0 < f < 1)", pct=True)],
     outputs=[_o("t_irr", "time", "irradiation time"), _o("n_half", "number", "irradiation time in half-lives")],
     primary="t_irr", conditions=["constant flux", "0 < f < 1"],
     method="1 - e^(-lambda t) = f -> t = -ln(1 - f)/lambda",
     latex=r"t_{irr} = -\frac{T_{1/2}}{\ln 2}\ln(1-f)",
     run=irradiation_time, reference=_ref_irradiation_time,
     wrong=lambda T_half, f: _out(t_irr=f * T_half / LN2, n_half=f / LN2),
     rand=lambda rng: {"T_half": _sig(_lu(rng, 1.0, 1e9)), "f": round(rng.uniform(0.01, 0.999), 4)},
     phrases=("Irradiation time for saturation", "Irradiation time", "Time to reach saturation fraction"),
     examples=[_ex({"T_half": 54000.0, "f": 0.9}, written={"T_half": "15 h", "f": "90%"}),
               _ex({"T_half": 134.7, "f": 0.5})],
     level="beginner")


# ------------------------------------------------------------------------------------------------ decay-curve fits
_TU = {"s": 1.0, "min": 60.0, "h": 3600.0, "d": DAY, "y": YEAR}
_TUNIT = _choice("t_unit", list(_TU), "unit of the times in the list: s, min, h, d or y")


def _lists(t, C, n_min):
    _need(len(t) == len(C), "give as many times as counts")
    _need(len(t) >= n_min, f"give at least {n_min} measurements")
    _need(all(math.isfinite(x) for x in t), "the times must be numbers")
    _need(all(math.isfinite(c) and c > 0 for c in C), "every count must be positive")
    _need(len(set(t)) == len(t), "the times must differ")


def decay_fit(t, C, t_unit):
    _need(t_unit in _TU, "t_unit must be s, min, h, d or y")
    _lists(t, C, 3)
    ts = [x * _TU[t_unit] for x in t]
    w = list(C)
    S, St, Stt = math.fsum(w), math.fsum(wi * x for wi, x in zip(w, ts)), math.fsum(wi * x * x for wi, x in zip(w, ts))
    Sy = math.fsum(wi * math.log(c) for wi, c in zip(w, C))
    Sty = math.fsum(wi * x * math.log(c) for wi, x, c in zip(w, ts, C))
    det = S * Stt - St * St
    _need(det > 0, "the times do not determine a slope")
    b = (S * Sty - St * Sy) / det
    a = (Stt * Sy - St * Sty) / det
    lam = -b
    _need(lam > 0, "the counts do not decrease: no decay constant can be fitted")
    sig_l = math.sqrt(S / det)
    T = LN2 / lam
    return _out(lambda_=lam, T_half=T, C0=math.exp(a), sigma_T=T * sig_l / lam)


def _ref_decay_fit(t, C, t_unit):
    import numpy as np
    ts = np.array(t, dtype=float) * _TU[t_unit]
    c = np.array(C, dtype=float)
    p, cov = np.polyfit(ts, np.log(c), 1, w=np.sqrt(c), cov="unscaled")
    lam = -float(p[0])
    T = LN2 / lam
    return {"lambda_": lam, "T_half": T, "C0": float(np.exp(p[1])), "sigma_T": T * float(np.sqrt(cov[0, 0])) / lam}


def _rand_decay_fit(rng):
    unit = rng.choice(list(_TU))
    T = _lu(rng, 2.0, 50.0)
    n = rng.randint(3, 10)
    C0 = _lu(rng, 1e3, 1e6)
    ts = sorted(rng.sample(range(0, 60), n))
    C = [max(1.0, round(C0 * 2 ** (-x / T) * rng.uniform(0.95, 1.05))) for x in ts]
    return {"t": [float(x) for x in ts], "C": C, "t_unit": unit}


_add("nuc_decay_curve_fit",
     name="Half-life from a measured decay curve (weighted log-linear fit)", topic="detectors and counting / decay-curve analysis",
     purpose="The decay constant, half-life, initial count and the standard error of the half-life from counts measured at several times (equal counting intervals), by a Poisson-weighted least-squares fit of ln C against t.",
     inputs=[_vecin("t", "measurement times, in the unit t_unit", 3, 200), _vecin("C", "counts recorded at those times (equal counting intervals)", 3, 200),
             _TUNIT],
     outputs=[_o("lambda_", "decay_constant", "fitted decay constant"), _o("T_half", "time", "fitted half-life"),
              _o("C0", "number", "fitted count at t = 0"), _o("sigma_T", "time", "standard error of the half-life (Poisson weights)")],
     primary="T_half", conditions=["a single radionuclide", "counts positive and decreasing overall", "weights w_i = C_i (Var ln C = 1/C)"],
     method="weighted least squares of ln C_i = ln C0 - lambda t_i with weights C_i; sigma_lambda^2 = S/(S Stt - St^2)",
     latex=r"\min\sum_i C_i\left(\ln C_i - \ln C_0 + \lambda t_i\right)^2",
     run=decay_fit, reference=_ref_decay_fit,
     wrong=lambda t, C, t_unit: (lambda ts: (lambda b: _out(lambda_=-b, T_half=LN2 / -b, C0=C[0], sigma_T=0.0))(
         (math.log(C[-1]) - math.log(C[0])) / (ts[-1] - ts[0])))([x * _TU[t_unit] for x in t]),
     rand=_rand_decay_fit,
     phrases=("Decay curve fit", "Half-life from a decay curve", "Decay-curve fit"),
     examples=[_ex({"t": [0.0, 1.0, 2.0, 3.0, 4.0], "C": [10000.0, 7100.0, 5000.0, 3540.0, 2500.0], "t_unit": "h"}),
               _ex({"t": [0.0, 10.0, 20.0, 30.0], "C": [5120.0, 2560.0, 1280.0, 640.0], "t_unit": "min"})],
     level="intermediate",
     extra_q=[{"q": "Decay curve fit: t = [0, 1, 2], C = [100, 200, 400], t_unit = h", "outcome": "INVALID_INPUT"},
              {"q": "Decay curve fit: t = [0, 1, 2, 3, 4] h, C = [10000, 7100, 5000, 3540, 2500]", "outcome": "UNRECOGNIZED_FORMAT"}])


def two_component_fit(t, C, T1, T2, t_unit):
    _need(t_unit in _TU, "t_unit must be s, min, h, d or y")
    _lists(t, C, 3)
    _pos("the half-lives", T1, T2)
    _need(abs(T1 / T2 - 1.0) > 1e-3, "the two half-lives must differ")
    ts = [x * _TU[t_unit] for x in t]
    l1, l2 = LN2 / T1, LN2 / T2
    f1 = [math.exp(-l1 * x) for x in ts]
    f2 = [math.exp(-l2 * x) for x in ts]
    w = [1.0 / c for c in C]                       # Poisson variance of each count
    a11 = math.fsum(wi * x * x for wi, x in zip(w, f1))
    a22 = math.fsum(wi * x * x for wi, x in zip(w, f2))
    a12 = math.fsum(wi * x * y for wi, x, y in zip(w, f1, f2))
    b1 = math.fsum(wi * x * c for wi, x, c in zip(w, f1, C))
    b2 = math.fsum(wi * x * c for wi, x, c in zip(w, f2, C))
    det = a11 * a22 - a12 * a12
    _need(det > 1e-3 * a11 * a22, "the two components cannot be separated from these times")
    c1 = (b1 * a22 - b2 * a12) / det
    c2 = (a11 * b2 - a12 * b1) / det
    return _out(C1=c1, C2=c2, frac1=c1 / (c1 + c2))


def _ref_two_component_fit(t, C, T1, T2, t_unit):
    import numpy as np
    ts = np.array(t, dtype=float) * _TU[t_unit]
    c = np.array(C, dtype=float)
    A = np.column_stack([np.exp(-LN2 / T1 * ts), np.exp(-LN2 / T2 * ts)])
    sw = 1.0 / np.sqrt(c)
    x, *_ = np.linalg.lstsq(A * sw[:, None], c * sw, rcond=None)
    return {"C1": float(x[0]), "C2": float(x[1]), "frac1": float(x[0] / (x[0] + x[1]))}


def _rand_two_comp(rng):
    unit = rng.choice(list(_TU))
    T1 = _lu(rng, 1.0, 5.0)
    T2 = T1 * _lu(rng, 4.0, 30.0)
    n = rng.randint(5, 12)
    ts = sorted([0] + rng.sample(range(1, int(4 * T2) + 6), n - 1))
    a1, a2 = _lu(rng, 1e3, 1e6), _lu(rng, 1e2, 1e5)
    C = [max(1.0, round((a1 * 2 ** (-x / T1) + a2 * 2 ** (-x / T2)) * rng.uniform(0.97, 1.03))) for x in ts]
    return {"t": [float(x) for x in ts], "C": C, "T1": _sig(T1 * _TU[unit]), "T2": _sig(T2 * _TU[unit]), "t_unit": unit}


_add("nuc_two_component_decay_fit",
     name="Two-component decay curve: amplitudes of two known half-lives", topic="detectors and counting / decay-curve analysis",
     purpose="Resolve a decay curve that is the sum of two radionuclides of known half-lives into the initial count of each component (Poisson-weighted linear least squares).",
     inputs=[_vecin("t", "measurement times, in the unit t_unit", 3, 200), _vecin("C", "counts recorded at those times (equal intervals)", 3, 200),
             _phys("T1", "time", "half-life of component 1"), _phys("T2", "time", "half-life of component 2"), _TUNIT],
     outputs=[_o("C1", "number", "component 1 count at t = 0"), _o("C2", "number", "component 2 count at t = 0"),
              _o("frac1", "number", "fraction of the initial count due to component 1")],
     primary="C1", conditions=["two components with known, different half-lives", "weights 1/C_i (Poisson)"],
     method="weighted linear least squares of C_i = C1 e^(-lambda1 t_i) + C2 e^(-lambda2 t_i) (2x2 normal equations)",
     latex=r"\min\sum_i\frac{\left(C_i - C_1e^{-\lambda_1t_i} - C_2e^{-\lambda_2t_i}\right)^2}{C_i}",
     run=two_component_fit, reference=_ref_two_component_fit,
     wrong=lambda t, C, T1, T2, t_unit: _out(C1=C[0] / 2, C2=C[0] / 2, frac1=0.5),
     rand=_rand_two_comp,
     phrases=("Two-component decay fit", "Two-component decay curve", "Resolve a two-component decay curve"),
     examples=[_ex({"t": [0.0, 1.0, 2.0, 4.0, 8.0, 16.0], "C": [11000.0, 6000.0, 3500.0, 1625.0, 1062.0, 844.0], "T1": 3600.0, "T2": 36000.0, "t_unit": "h"},
                   written={"T1": "1 h", "T2": "10 h"}),
               _ex({"t": [0.0, 5.0, 10.0, 20.0, 40.0], "C": [2000.0, 1150.0, 800.0, 560.0, 330.0], "T1": 300.0, "T2": 3000.0, "t_unit": "min"},
                   written={"T1": "5 min", "T2": "50 min"})],
     level="advanced")


# ------------------------------------------------------------------------------------------------ decay heat
_WW = 0.0622


def _ww_check(P0, T_op, *ts):
    _pos("the power before shutdown and the operating time", P0, T_op)
    for t in ts:
        _need(_isnum(t) and 10.0 <= t <= 1e8, "the Way-Wigner correlation holds from 10 s to about 1e8 s after shutdown")


def decay_heat(P0, T_op, t):
    _ww_check(P0, T_op, t)
    ratio = _WW * (t ** -0.2 - (t + T_op) ** -0.2)
    return _out(P_d=P0 * ratio, ratio=ratio)


def _ref_decay_heat(P0, T_op, t):
    k = _WW * 0.2                                  # decay power of one burst of fissions per unit energy, s^-1.2 kernel
    ratio = _quad(lambda tau: k * (t + T_op - tau) ** -1.2, 0.0, T_op)      # superposition over the operating history
    return {"P_d": P0 * ratio, "ratio": ratio}


_add("nuc_decay_heat_power",
     name="Decay heat after shutdown (Way-Wigner)", topic="reactor physics / decay heat",
     purpose="The fission-product decay power a time t after shutdown of a reactor that ran at constant power for an operating time T, by the Way-Wigner correlation.",
     inputs=[_phys("P0", "power", "thermal power before shutdown"), _phys("T_op", "time", "operating time at that power"),
             _phys("t", "time", "time after shutdown (10 s to about 1e8 s)")],
     outputs=[_o("P_d", "power", "decay power"), _o("ratio", "number", "decay power / operating power")],
     primary="P_d", conditions=["constant power before shutdown", "10 s <= t <= 1e8 s"],
     method="P/P0 = 0.0622 [t^-0.2 - (t + T)^-0.2], t and T in seconds (Way-Wigner)",
     latex=r"\frac{P_d}{P_0} = 0.0622\left[t^{-0.2} - (t+T)^{-0.2}\right]",
     run=decay_heat, reference=_ref_decay_heat,
     wrong=lambda P0, T_op, t: (lambda r: _out(P_d=P0 * r, ratio=r))(_WW * (t ** -0.2 - T_op ** -0.2) if T_op != t else _WW * t ** -0.2),
     rand=lambda rng: {"P0": _sig(_lu(rng, 1e6, 4e9)), "T_op": _sig(_lu(rng, 3600.0, 1e8)), "t": _sig(_lu(rng, 10.0, 1e7))},
     phrases=("Decay heat", "Decay heat power", "Way-Wigner decay heat"),
     examples=[_ex({"P0": 3e9, "T_op": 31557600.0, "t": 3600.0}, written={"P0": "3000 MW", "T_op": "1 y", "t": "1 h"}),
               _ex({"P0": 1e8, "T_op": 2592000.0, "t": 86400.0}, written={"P0": "100 MW", "T_op": "30 d", "t": "1 d"})],
     level="intermediate",
     extra_q=[{"q": "Decay heat: P0 = 3000 MW, T_op = 1 y, t = 1 s", "outcome": "INVALID_INPUT"}])


def decay_heat_energy(P0, T_op, t1, t2):
    _ww_check(P0, T_op, t1, t2)
    _need(t2 > t1, "t2 must be later than t1")
    F = lambda t: (t ** 0.8 - (t + T_op) ** 0.8) / 0.8
    E = _WW * P0 * (F(t2) - F(t1))
    return _out(E=E, E_MWh=E / 3.6e9, P_mean=E / (t2 - t1))


def _ref_decay_heat_energy(P0, T_op, t1, t2):
    E = _quad(lambda t: _WW * P0 * (t ** -0.2 - (t + T_op) ** -0.2), t1, t2)
    return {"E": E, "E_MWh": E / 3.6e9, "P_mean": E / (t2 - t1)}


_add("nuc_decay_heat_energy",
     name="Decay heat energy released between two times after shutdown", topic="reactor physics / decay heat",
     purpose="The total fission-product decay energy released between two times after shutdown (the Way-Wigner power integrated), and the mean decay power over that interval.",
     inputs=[_phys("P0", "power", "thermal power before shutdown"), _phys("T_op", "time", "operating time at that power"),
             _phys("t1", "time", "start of the interval after shutdown"), _phys("t2", "time", "end of the interval after shutdown")],
     outputs=[_o("E", "energy", "decay energy released"), _o("E_MWh", "number", "the same energy in MWh"),
              _o("P_mean", "power", "mean decay power over the interval")],
     primary="E", conditions=["10 s <= t1 < t2 <= 1e8 s", "constant power before shutdown"],
     method="E = 0.0622 P0 [(t^0.8 - (t + T)^0.8)/0.8] from t1 to t2",
     latex=r"E = 0.0622\,P_0\left[\frac{t^{0.8}-(t+T)^{0.8}}{0.8}\right]_{t_1}^{t_2}",
     run=decay_heat_energy, reference=_ref_decay_heat_energy,
     wrong=lambda P0, T_op, t1, t2: (lambda E: _out(E=E, E_MWh=E / 3.6e9, P_mean=E / (t2 - t1)))(_WW * P0 * (t1 ** -0.2 - (t1 + T_op) ** -0.2) * (t2 - t1)),
     rand=lambda rng: (lambda t1: {"P0": _sig(_lu(rng, 1e6, 4e9)), "T_op": _sig(_lu(rng, 3600.0, 1e8)), "t1": _sig(t1),
                                   "t2": _sig(min(1e8, t1 * _lu(rng, 1.5, 100.0)))})(_lu(rng, 10.0, 1e6)),
     phrases=("Decay heat energy", "Decay energy after shutdown"),
     examples=[_ex({"P0": 3e9, "T_op": 31557600.0, "t1": 10.0, "t2": 86400.0}, written={"P0": "3000 MW", "T_op": "1 y", "t2": "1 d"}),
               _ex({"P0": 1e8, "T_op": 2592000.0, "t1": 3600.0, "t2": 604800.0}, written={"P0": "100 MW", "T_op": "30 d", "t1": "1 h", "t2": "7 d"})],
     level="advanced")


# ------------------------------------------------------------------------------------------------ committed dose
def committed_dose(I, f, Tb, t_unit, T_p, E, w_R, m, tau):
    _need(t_unit in _TU, "t_unit must be s, min, h, d or y")
    _pos("the intake, physical half-life, energy, weighting factor, organ mass and integration period", I, T_p, E, w_R, m, tau)
    _need(len(f) == len(Tb) and 1 <= len(f) <= 6, "give one retention fraction per biological half-life (1 to 6)")
    _need(all(_isnum(x) and x >= 0 for x in f) and math.fsum(f) <= 1 + 1e-12, "retention fractions must be >= 0 and sum to at most 1")
    _need(all(_isnum(x) and x > 0 for x in Tb), "biological half-lives must be positive")
    lp = LN2 / T_p
    U = math.fsum(I * fi * -math.expm1(-(lp + LN2 / (Tb_i * _TU[t_unit])) * tau) / (lp + LN2 / (Tb_i * _TU[t_unit]))
                  for fi, Tb_i in zip(f, Tb))
    H = U * E * w_R / m
    return _out(U=U, H=H, H_mSv=H * 1000.0)


def _ref_committed_dose(I, f, Tb, t_unit, T_p, E, w_R, m, tau):
    lp = LN2 / T_p
    ks = [lp + LN2 / (x * _TU[t_unit]) for x in Tb]
    U = math.fsum(I * fi * _quad(lambda s, k=k: math.exp(-k * s), 0.0, min(tau, 60.0 / k)) for fi, k in zip(f, ks))
    H = U * E * w_R / m
    return {"U": U, "H": H, "H_mSv": H * 1000.0}


def _rand_committed(rng):
    n = rng.randint(1, 3)
    f = [rng.uniform(0.05, 1.0) for _ in range(n)]
    s = sum(f) / rng.uniform(0.5, 1.0)
    f = [round(x / s, 4) for x in f]
    unit = rng.choice(["h", "d", "y"])
    return {"I": _sig(_lu(rng, 1e2, 1e8)), "f": f, "Tb": [_sig(_lu(rng, 0.1, 500.0)) for _ in range(n)], "t_unit": unit,
            "T_p": _sig(_lu(rng, 3600.0, 1e10)), "E": _sig(_lu(rng, 0.01, 5.0)) * MEV, "w_R": rng.choice([1.0, 1.0, 20.0]),
            "m": _sig(_lu(rng, 0.01, 70.0)), "tau": rng.choice([50 * YEAR, 70 * YEAR, _sig(_lu(rng, 1e5, 2e9))])}


_add("nuc_committed_dose_retention",
     name="Committed equivalent dose to an organ from an intake (retention compartments)", topic="dosimetry / internal emitters",
     purpose="The number of decays in an organ over the commitment period after an intake retained in exponential compartments, and the committed equivalent dose from the energy absorbed per decay, the radiation weighting factor and the organ mass.",
     inputs=[_phys("I", "radioactivity", "activity taken into the organ (intake reaching it)"),
             _vecin("f", "fractions of the intake retained in each compartment (sum <= 1)", 1, 6),
             _vecin("Tb", "biological half-lives of the compartments, in the unit t_unit", 1, 6), _TUNIT,
             _phys("T_p", "time", "physical half-life"), _phys("E", "energy", "energy absorbed in the organ per decay (e.g. MeV)"),
             _numin("w_R", "radiation weighting factor (1 for photons and electrons, 20 for alpha particles)"),
             _phys("m", "mass", "organ mass"), _phys("tau", "time", "commitment period (50 y for adults)")],
     outputs=[_o("U", "number", "number of decays in the organ during tau"), _o("H", "equivalent_dose", "committed equivalent dose (Sv)"),
              _o("H_mSv", "number", "committed equivalent dose in mSv")],
     primary="H", conditions=["instantaneous intake", "retention R(t) = sum f_i e^(-lambda_b,i t)", "all energy per decay absorbed in the organ as stated"],
     method="U = I sum_i f_i (1 - e^(-k_i tau))/k_i with k_i = lambda_p + ln 2/Tb_i; H = U E w_R / m",
     latex=r"H_T(\tau) = \frac{w_R E}{m}\,I\sum_i f_i\frac{1-e^{-(\lambda_p+\lambda_{b,i})\tau}}{\lambda_p+\lambda_{b,i}}",
     run=committed_dose, reference=_ref_committed_dose,
     wrong=lambda I, f, Tb, t_unit, T_p, E, w_R, m, tau: (lambda U: _out(U=U, H=U * E * w_R / m, H_mSv=U * E * w_R / m * 1000))(
         math.fsum(I * fi * (Tb_i * _TU[t_unit]) / LN2 for fi, Tb_i in zip(f, Tb))),
     rand=_rand_committed,
     phrases=("Committed dose", "Committed equivalent dose", "Committed dose from intake"),
     examples=[_ex({"I": 1e6, "f": [0.3], "Tb": [80.0], "t_unit": "d", "T_p": 692988.48, "E": 0.18 * MEV, "w_R": 1.0, "m": 0.02, "tau": 50 * YEAR},
                   written={"I": "1 MBq", "T_p": "8.0207 d", "E": "0.18 MeV", "m": "20 g", "tau": "50 y"}),
               _ex({"I": 1000.0, "f": [0.6, 0.4], "Tb": [10.0, 100.0], "t_unit": "y", "T_p": 760837487788.8, "E": 5.2 * MEV, "w_R": 20.0, "m": 5.0, "tau": 50 * YEAR},
                   written={"I": "1 kBq", "E": "5.2 MeV", "m": "5 kg", "tau": "50 y"})],
     level="advanced")


# ================================================================================================ SHIELDING AND DOSE
_BUILDUPS = ("none", "linear", "taylor")


def _buildup(form, b, A1, alpha1, alpha2):
    """Buildup factor at b mean free paths in the stated form; Taylor parameters only with the Taylor form."""
    if form == "taylor":
        _need(A1 is not None and alpha1 is not None and alpha2 is not None, "the Taylor form needs A1, alpha1 and alpha2")
        _need(1 + alpha1 > 0 and 1 + alpha2 > 0, "the Taylor exponents need 1 + alpha > 0")
        B = A1 * math.exp(-alpha1 * b) + (1.0 - A1) * math.exp(-alpha2 * b)
        _need(B > 0, "these Taylor parameters give a buildup factor <= 0 here")
        return B
    _need(A1 is None and alpha1 is None and alpha2 is None, "Taylor parameters are given but the buildup form is not taylor")
    _need(form in ("none", "linear"), "buildup must be none, linear or taylor")
    return 1.0 if form == "none" else 1.0 + b


def _ref_transmission(form, b, A1, alpha1, alpha2):
    """Reference: the transmitted fraction (with buildup) integrated through the shield as an ODE in optical depth s.
    none: y' = -y; linear: y1' = -y1, y2' = y1 - y2 (y2 = s e^-s); Taylor: z_k' = -(1 + alpha_k) z_k."""
    if b == 0:
        return 1.0, 1.0
    if form == "none":
        y = _ivp(lambda s, y: [-y[0]], b, [1.0], atol=[1e-300])
        return y[0], 1.0
    if form == "linear":
        y = _ivp(lambda s, y: [-y[0], y[0] - y[1]], b, [1.0, 0.0], atol=[1e-30, 1e-30])
        return y[0] + y[1], (y[0] + y[1]) / y[0]
    y = _ivp(lambda s, y: [-y[0], -(1 + alpha1) * y[1], -(1 + alpha2) * y[2]], b, [1.0, A1, 1.0 - A1],
             atol=[1e-30, 1e-30, 1e-30])
    return y[1] + y[2], (y[1] + y[2]) / y[0]


_TAYLOR_IN = [_numin("A1", "Taylor buildup parameter A1 (only with buildup = taylor)", required=False),
              _numin("alpha1", "Taylor buildup parameter alpha1 (only with buildup = taylor)", required=False),
              _numin("alpha2", "Taylor buildup parameter alpha2 (only with buildup = taylor)", required=False)]


def point_dose_rate(Gamma, A, r, mu, x, buildup, A1=None, alpha1=None, alpha2=None):
    _pos("Gamma, the activity and the distance", Gamma, A, r)
    _nonneg("mu and the shield thickness", mu, x)
    _need(x < r, "the shield must lie between the source and the dose point (x < r)")
    b = mu * x
    B = _buildup(buildup, b, A1, alpha1, alpha2)
    T = B * math.exp(-b)
    H = Gamma * A * T / (r * r)
    return _out(H_dot=H, H_uSv_h=H * 3.6e9, B=B, T=T)


def _ref_point_dose_rate(Gamma, A, r, mu, x, buildup, A1=None, alpha1=None, alpha2=None):
    T, B = _ref_transmission(buildup, mu * x, A1, alpha1, alpha2)
    H = Gamma * A * T / (r * r)
    return {"H_dot": H, "H_uSv_h": H * 3.6e9, "B": B, "T": T}


def _rand_buildup(rng, d):
    form = rng.choice(_BUILDUPS)
    d["buildup"] = form
    if form == "taylor":
        d.update({"A1": round(rng.uniform(5.0, 30.0), 3), "alpha1": round(rng.uniform(-0.12, -0.02), 4),
                  "alpha2": round(rng.uniform(0.0, 0.15), 4)})
    return d


def _rand_point(rng):
    mu = _sig(_lu(rng, 2.0, 200.0))
    r = _sig(_lu(rng, 0.3, 30.0))
    x = _sig(min(rng.uniform(0.0, 15.0) / mu, 0.9 * r))
    d = {"Gamma": _sig(_lu(rng, 1e-18, 1e-15)), "A": _sig(_lu(rng, 1e6, 1e14)), "r": r, "mu": mu, "x": x}
    d = _rand_buildup(rng, d)
    if d["buildup"] == "taylor":
        d["x"] = _sig(min(rng.uniform(1.0, 10.0) / mu, 0.9 * r))
    return d


_add("nuc_point_source_dose_rate",
     name="Dose rate from a shielded point gamma source with buildup", topic="shielding and dose / point sources",
     purpose="The dose rate at a distance from a point gamma source of stated dose-rate constant, behind a slab shield, with the buildup factor in the stated form (none, linear or Taylor), plus the buildup factor and the transmission.",
     inputs=[_phys("Gamma", "dose_rate_constant", "dose-rate constant of the nuclide (e.g. uSv*m^2/(MBq*h))"),
             _phys("A", "radioactivity", "source activity"), _phys("r", "length", "source-to-point distance"),
             _phys("mu", "linear_attenuation_coefficient", "linear attenuation coefficient of the shield"),
             _phys("x", "length", "shield thickness (0 for none)"),
             _choice("buildup", _BUILDUPS, "buildup form: none (B = 1), linear (B = 1 + mu x) or taylor"), *_TAYLOR_IN],
     outputs=[_o("H_dot", "dose_rate", "dose rate (same quantity as Gamma: Sv/s, Gy/s)"), _o("H_uSv_h", "number", "dose rate in uSv/h (or uGy/h)"),
              _o("B", "number", "buildup factor"), _o("T", "number", "transmission B e^(-mu x)")],
     primary="H_dot", conditions=["point isotropic source", "shield between source and point (x < r)", "Taylor parameters only with the Taylor form"],
     method="H = Gamma A B(mu x) e^(-mu x) / r^2; B = 1, 1 + mu x, or A1 e^(-alpha1 mu x) + (1 - A1) e^(-alpha2 mu x)",
     latex=r"\dot H = \frac{\Gamma A}{r^2}\,B(\mu x)\,e^{-\mu x}",
     run=point_dose_rate, reference=_ref_point_dose_rate,
     wrong=lambda Gamma, A, r, mu, x, buildup, A1=None, alpha1=None, alpha2=None: (lambda o: {**o, "H_dot": _r(o["H_dot"] * r), "H_uSv_h": _r(o["H_uSv_h"] * r)})(
         point_dose_rate(Gamma, A, r, mu, x, buildup, A1, alpha1, alpha2)),
     rand=_rand_point,
     phrases=("Point source dose rate", "Shielded point source dose rate", "Dose rate from a point source"),
     examples=[_ex({"Gamma": 0.351e-12 / 3600.0, "A": 3.7e11, "r": 2.0, "mu": 6.56, "x": 0.1, "buildup": "linear"},
                   written={"Gamma": "0.351 uSv*m^2/(MBq*h)", "A": "10 Ci", "r": "2 m", "mu": "0.0656 1/cm", "x": "10 cm"}),
               _ex({"Gamma": 0.351e-12 / 3600.0, "A": 3.7e11, "r": 2.0, "mu": 6.56, "x": 0.1, "buildup": "taylor", "A1": 10.0, "alpha1": -0.07, "alpha2": 0.06},
                   written={"Gamma": "0.351 uSv*m^2/(MBq*h)", "A": "10 Ci", "r": "2 m", "mu": "0.0656 1/cm", "x": "10 cm"}),
               _ex({"Gamma": 0.0927e-12 / 3600.0, "A": 1e9, "r": 1.0, "mu": 0.0, "x": 0.0, "buildup": "none"},
                   written={"Gamma": "0.0927 uSv*m^2/(MBq*h)", "A": "1 GBq", "r": "1 m", "mu": "0 1/m", "x": "0 m"})],
     level="beginner",
     extra_q=[{"q": "Point source dose rate: Gamma = 0.351 uSv*m^2/(MBq*h), A = 10 Ci, r = 2 m, mu = 0.0656 1/cm, x = 10 cm, buildup = linear, A1 = 10", "outcome": "INVALID_INPUT"},
              {"q": "Point source dose rate: Gamma = 0.351 uSv*m^2/(MBq*h), A = 10 Ci, r = 2 m, mu = 0.0656 1/cm, x = 10 cm", "outcome": "NEEDS_INPUT"}])


def shield_thickness(Gamma, A, r, mu, H_target, buildup):
    _pos("Gamma, the activity, the distance, mu and the target dose rate", Gamma, A, r, mu, H_target)
    _need(buildup in ("none", "linear"), "buildup must be none or linear for a thickness")
    H0 = Gamma * A / (r * r)
    q = H_target / H0
    _need(q < 1, "the unshielded dose rate is already below the target: no shield is needed")
    if buildup == "none":
        b = -math.log(q)
    else:
        lq = math.log(q)
        b = _bisect(lambda z: math.log1p(z) - z - lq, 0.0, -2.0 * lq + 10.0)
    x = b / mu
    _need(x < r, "the shield would be thicker than the source distance")
    return _out(x=x, n_mfp=b, B=1.0 if buildup == "none" else 1.0 + b)


def _ref_shield_thickness(Gamma, A, r, mu, H_target, buildup):
    from scipy.special import lambertw
    H0 = Gamma * A / (r * r)
    q = H_target / H0
    if buildup == "linear":
        b = float((-1.0 - lambertw(-q / math.e, -1)).real)
    else:
        b = _brentq(lambda s: _ref_transmission("none", s, None, None, None)[0] - q, 0.0, -math.log(q) + 5.0)
    return {"x": b / mu, "n_mfp": b, "B": 1.0 if buildup == "none" else 1.0 + b}


def _rand_thick(rng):
    mu = _sig(_lu(rng, 2.0, 200.0))
    r = _sig(_lu(rng, 2.0, 30.0))
    G, A = _sig(_lu(rng, 1e-18, 1e-15)), _sig(_lu(rng, 1e8, 1e14))
    H0 = G * A / (r * r)
    b = rng.uniform(0.5, min(15.0, 0.8 * r * mu))
    return {"Gamma": G, "A": A, "r": r, "mu": mu, "H_target": _sig(H0 * math.exp(-b)), "buildup": rng.choice(["none", "linear"])}


_add("nuc_shield_thickness_for_dose_rate",
     name="Shield thickness for a target dose rate (point source, buildup)", topic="shielding and dose / shield design",
     purpose="The slab thickness that brings the dose rate from a point gamma source down to a target value, with no buildup or the linear buildup form B = 1 + mu x (the root of the attenuation equation).",
     inputs=[_phys("Gamma", "dose_rate_constant", "dose-rate constant of the nuclide"), _phys("A", "radioactivity", "source activity"),
             _phys("r", "length", "source-to-point distance"), _phys("mu", "linear_attenuation_coefficient", "linear attenuation coefficient of the shield"),
             _phys("H_target", "dose_rate", "target dose rate (e.g. uSv/h)"),
             _choice("buildup", ("none", "linear"), "buildup form: none (B = 1) or linear (B = 1 + mu x)")],
     outputs=[_o("x", "length", "required shield thickness"), _o("n_mfp", "number", "thickness in mean free paths"),
              _o("B", "number", "buildup factor at that thickness")],
     primary="x", conditions=["target below the unshielded dose rate", "the shield fits between source and point"],
     method="solve B(mu x) e^(-mu x) = H_target r^2/(Gamma A): x = ln(H0/H)/mu (no buildup) or (1 + b) e^(-b) = q by bisection",
     latex=r"\frac{\Gamma A}{r^2}B(\mu x)e^{-\mu x} = \dot H_{target}",
     run=shield_thickness, reference=_ref_shield_thickness,
     wrong=lambda Gamma, A, r, mu, H_target, buildup: (lambda H0: _out(x=math.log(H0 / H_target) / mu * 0.5, n_mfp=0.0, B=1.0))(Gamma * A / r ** 2),
     rand=_rand_thick,
     phrases=("Shield thickness for dose rate", "Shield thickness", "Required shield thickness"),
     examples=[_ex({"Gamma": 0.351e-12 / 3600.0, "A": 3.7e11, "r": 2.0, "mu": 65.6, "H_target": 2.5e-6 / 3600.0, "buildup": "linear"},
                   written={"Gamma": "0.351 uSv*m^2/(MBq*h)", "A": "10 Ci", "r": "2 m", "mu": "0.656 1/cm", "H_target": "2.5 uSv/h"}),
               _ex({"Gamma": 0.351e-12 / 3600.0, "A": 3.7e11, "r": 2.0, "mu": 65.6, "H_target": 2.5e-6 / 3600.0, "buildup": "none"},
                   written={"Gamma": "0.351 uSv*m^2/(MBq*h)", "A": "10 Ci", "r": "2 m", "mu": "0.656 1/cm", "H_target": "2.5 uSv/h"})],
     level="intermediate",
     extra_q=[{"q": "Shield thickness: Gamma = 0.351 uSv*m^2/(MBq*h), A = 1 kBq, r = 2 m, mu = 0.656 1/cm, H_target = 2.5 uSv/h, buildup = none", "outcome": "INVALID_INPUT"}])


def multilayer(mu, x, H0=None):
    _need(len(mu) == len(x) and 1 <= len(mu) <= 10, "give one attenuation coefficient per layer thickness (1 to 10 layers)")
    _need(all(_isnum(m) and m >= 0 for m in mu), "attenuation coefficients must be >= 0")
    _need(all(_isnum(v) and v >= 0 for v in x), "thicknesses must be >= 0")
    tau = math.fsum(m * v for m, v in zip(mu, x))
    T = math.exp(-tau)
    out = dict(T=T, tau=tau, n_hvl=tau / LN2, attenuation=1.0 / T)
    if H0 is not None:
        _pos("the dose rate in front of the shield", H0)
        out["H"] = H0 * T
    return _out(**out)


def _ref_multilayer(mu, x, H0=None):
    I, tau = 1.0, 0.0
    for m, v in zip(mu, x):                        # slab by slab: dI/ds = -mu(s) I through each layer in turn
        if m * v > 0:
            I = _ivp(lambda s, y, m=m: [-m * y[0]], v, [I], atol=[1e-300])[0]
    tau = -math.log(I)
    out = {"T": I, "tau": tau, "n_hvl": tau / LN2, "attenuation": 1.0 / I}
    if H0 is not None:
        out["H"] = H0 * I
    return out


_add("nuc_multilayer_transmission",
     name="Narrow-beam transmission through a multilayer shield", topic="shielding and dose / attenuation",
     purpose="The uncollided (narrow-beam) transmission through several shield layers, the total optical thickness, its value in half-value layers, the attenuation factor and (optionally) the dose rate behind the shield.",
     inputs=[_vecin("mu", "linear attenuation coefficient of each layer, in 1/cm", 1, 10, unit="1/cm"),
             _vecin("x", "thickness of each layer, in cm", 1, 10, unit="cm"),
             _phys("H0", "dose_rate", "uncollided dose rate in front of the shield (optional)", required=False)],
     outputs=[_o("T", "number", "uncollided transmission"), _o("tau", "number", "total optical thickness sum mu_i x_i"),
              _o("n_hvl", "number", "thickness in half-value layers"), _o("attenuation", "number", "attenuation factor 1/T"),
              _o("H", "dose_rate", "uncollided dose rate behind the shield (when H0 is given)")],
     primary="T", conditions=["narrow beam (no buildup)", "layers perpendicular to the beam"],
     method="T = exp(-sum mu_i x_i)",
     latex=r"T = \exp\Big(-\sum_i \mu_i x_i\Big)",
     run=lambda mu, x, H0=None: multilayer([m * 100.0 for m in mu], [v / 100.0 for v in x], H0),
     reference=lambda mu, x, H0=None: _ref_multilayer([m * 100.0 for m in mu], [v / 100.0 for v in x], H0),
     wrong=lambda mu, x, H0=None: (lambda T: _out(T=T, tau=0.0, n_hvl=0.0, attenuation=1 / T if T else 0.0, **({"H": H0 * T} if H0 else {})))(
         math.prod(max(1e-300, 1 - m * v) for m, v in zip(mu, x))),
     rand=lambda rng: (lambda n: {"mu": [_sig(_lu(rng, 0.02, 2.0)) for _ in range(n)], "x": [_sig(rng.uniform(0.0, 12.0) / n) for _ in range(n)],
                                  **({"H0": _sig(_lu(rng, 1e-9, 1e-3))} if rng.random() < 0.5 else {})})(rng.randint(1, 5)),
     phrases=("Multilayer transmission", "Multilayer shield transmission", "Narrow-beam transmission"),
     examples=[_ex({"mu": [0.656, 0.149], "x": [5.0, 30.0]}),
               _ex({"mu": [1.2, 0.2, 0.07], "x": [2.0, 10.0, 20.0], "H0": 1e-3 / 3600.0}, written={"H0": "1 mSv/h"})],
     level="beginner",
     extra_q=[{"q": "Multilayer transmission: mu = [0.656, 0.149] 1/cm, x = [5, 30] cm", "like": 0},
              {"q": "Multilayer transmission: mu = [0.656, 0.149] 1/m, x = [5, 30] cm", "outcome": "UNRECOGNIZED_FORMAT"},
              {"q": "Multilayer transmission: mu = [0.656, 0.149], x = [5, 30, 4]", "outcome": "INVALID_INPUT"}])


# ------------------------------------------------------------------------------------------------ extended sources
def _sievert(theta, b):
    """F(theta, b) = int_0^theta exp(-b sec u) du (odd in theta), mpmath tanh-sinh quadrature."""
    if b == 0:
        return theta
    with _mp.workdps(25):
        s = 1.0 if theta >= 0 else -1.0
        v = _mp.quad(lambda u: _mp.exp(-b / _mp.cos(u)), [0, abs(theta)])
        return s * float(v)


def line_source(S_L, h, y1, y2, mu, t):
    _pos("the source strength and the perpendicular distance", S_L, h)
    _need(_isnum(y1) and _isnum(y2) and y2 > y1, "the line runs from y1 to y2 with y2 > y1")
    _nonneg("mu and the shield thickness", mu, t)
    _need(t < h, "the shield must lie between the line and the point (t < h)")
    b = mu * t
    th1, th2 = math.atan2(y1, h), math.atan2(y2, h)
    phi = S_L / (4 * math.pi * h) * (_sievert(th2, b) - _sievert(th1, b))
    phi0 = S_L / (4 * math.pi * h) * (th2 - th1)
    return _out(phi=phi, phi0=phi0, T=phi / phi0)


def _ref_line_source(S_L, h, y1, y2, mu, t):
    b = mu * t
    f = lambda y, bb: S_L * math.exp(-bb * math.sqrt(h * h + y * y) / h) / (4 * math.pi * (h * h + y * y))
    phi = _quad(lambda y: f(y, b), y1, y2, points=[0.0] if y1 < 0 < y2 else None)
    phi0 = _quad(lambda y: f(y, 0.0), y1, y2, points=[0.0] if y1 < 0 < y2 else None)
    return {"phi": phi, "phi0": phi0, "T": phi / phi0}


def _rand_line(rng):
    h = _sig(_lu(rng, 0.2, 20.0))
    y1 = _sig(rng.uniform(-3.0, 2.0) * h)
    y2 = _sig(y1 + _lu(rng, 0.1, 6.0) * h)
    mu = _sig(_lu(rng, 1.0, 100.0))
    return {"S_L": _sig(_lu(rng, 1e5, 1e12)), "h": h, "y1": y1, "y2": y2, "mu": mu, "t": _sig(min(rng.uniform(0.0, 6.0) / mu, 0.9 * h))}


_add("nuc_line_source_flux",
     name="Uncollided flux from a shielded line source (Sievert integral)", topic="shielding and dose / extended sources",
     purpose="The uncollided photon flux at a point at perpendicular distance h from a straight line source (a pipe, a wire) running from y1 to y2 along the line, behind a slab shield parallel to the line, with the unshielded flux and the transmission.",
     inputs=[_phys("S_L", "linear_source_strength", "source strength per unit length (photons per second per metre)"),
             _phys("h", "length", "perpendicular distance from the line to the point"),
             _phys("y1", "length", "position of one end along the line, measured from the foot of the perpendicular (may be negative)"),
             _phys("y2", "length", "position of the other end (y2 > y1)"),
             _phys("mu", "linear_attenuation_coefficient", "linear attenuation coefficient of the shield"),
             _phys("t", "length", "shield thickness (0 for none)")],
     outputs=[_o("phi", "particle_flux", "uncollided flux at the point"), _o("phi0", "particle_flux", "flux without the shield"),
              _o("T", "number", "transmission phi/phi0")],
     primary="phi", conditions=["isotropic line source in vacuum", "slab shield parallel to the line (t < h)", "no buildup"],
     method="phi = S_L/(4 pi h) [F(theta2, mu t) - F(theta1, mu t)], F(theta, b) = int_0^theta e^(-b sec u) du (Sievert integral), theta_i = atan(y_i/h)",
     latex=r"\phi = \frac{S_L}{4\pi h}\left[F(\theta_2,\mu t) - F(\theta_1,\mu t)\right],\ F(\theta,b)=\int_0^{\theta}e^{-b\sec u}\,du",
     run=line_source, reference=_ref_line_source,
     wrong=lambda S_L, h, y1, y2, mu, t: (lambda p: _out(phi=p, phi0=S_L * (y2 - y1) / (4 * math.pi * h * h), T=math.exp(-mu * t)))(
         S_L * (y2 - y1) * math.exp(-mu * t) / (4 * math.pi * h * h)),
     rand=_rand_line,
     phrases=("Line source flux", "Shielded line source", "Line source"),
     examples=[_ex({"S_L": 1e8, "h": 1.0, "y1": -2.0, "y2": 2.0, "mu": 10.0, "t": 0.1},
                   written={"S_L": "1e8 1/(m*s)", "h": "1 m", "y1": "-2 m", "y2": "2 m", "mu": "0.1 1/cm", "t": "10 cm"}),
               _ex({"S_L": 5e9, "h": 2.0, "y1": 0.0, "y2": 3.0, "mu": 0.0, "t": 0.0},
                   written={"S_L": "5e9 1/(m*s)", "h": "2 m", "y1": "0 m", "y2": "3 m", "mu": "0 1/m", "t": "0 m"})],
     level="expert")


def disk_source(S_A, R, h, mu, t):
    _pos("the source strength, the disk radius and the distance", S_A, R, h)
    _nonneg("mu and the shield thickness", mu, t)
    _need(t < h, "the shield must lie between the disk and the point (t < h)")
    b = mu * t
    sec = math.sqrt(h * h + R * R) / h
    phi0 = S_A / 4.0 * math.log1p((R / h) ** 2)
    if b == 0:
        phi = phi0
    else:
        with _mp.workdps(25):
            phi = float(S_A / 2.0 * (_mp.e1(b) - _mp.e1(b * sec)))
    return _out(phi=phi, phi0=phi0, T=phi / phi0)


def _ref_disk_source(S_A, R, h, mu, t):
    b = mu * t
    f = lambda rho, bb: S_A / 2.0 * rho * math.exp(-bb * math.sqrt(h * h + rho * rho) / h) / (h * h + rho * rho)
    phi = _quad(lambda rho: f(rho, b), 0.0, R)
    phi0 = _quad(lambda rho: f(rho, 0.0), 0.0, R)
    return {"phi": phi, "phi0": phi0, "T": phi / phi0}


_add("nuc_disk_source_flux",
     name="Uncollided flux on the axis of a shielded disk source", topic="shielding and dose / extended sources",
     purpose="The uncollided flux on the axis of a uniform disk source (a contaminated floor patch, a tank bottom) at distance h, behind a slab shield parallel to the disk, with the unshielded flux and the transmission.",
     inputs=[_phys("S_A", "surface_source_strength", "source strength per unit area (photons per second per square metre)"),
             _phys("R", "length", "disk radius"), _phys("h", "length", "distance from the disk on its axis"),
             _phys("mu", "linear_attenuation_coefficient", "linear attenuation coefficient of the shield"),
             _phys("t", "length", "shield thickness (0 for none)")],
     outputs=[_o("phi", "particle_flux", "uncollided flux on the axis"), _o("phi0", "particle_flux", "flux without the shield"),
              _o("T", "number", "transmission phi/phi0")],
     primary="phi", conditions=["uniform isotropic disk source", "slab shield parallel to the disk (t < h)", "no buildup"],
     method="phi = (S_A/2) [E1(b) - E1(b sec theta)], sec theta = sqrt(h^2 + R^2)/h, b = mu t; unshielded (S_A/4) ln(1 + R^2/h^2)",
     latex=r"\phi = \frac{S_A}{2}\left[E_1(\mu t) - E_1(\mu t\sec\theta)\right]",
     run=disk_source, reference=_ref_disk_source,
     wrong=lambda S_A, R, h, mu, t: (lambda p0: _out(phi=p0 * math.exp(-mu * t) * 4, phi0=p0 * 4, T=math.exp(-mu * t)))(S_A / 4.0 * math.log1p((R / h) ** 2)),
     rand=lambda rng: (lambda h, mu: {"S_A": _sig(_lu(rng, 1e4, 1e12)), "R": _sig(h * _lu(rng, 0.05, 30.0)), "h": h, "mu": mu,
                                      "t": _sig(min(rng.uniform(0.0, 6.0) / mu, 0.9 * h))})(_sig(_lu(rng, 0.05, 10.0)), _sig(_lu(rng, 1.0, 100.0))),
     phrases=("Disk source flux", "Shielded disk source", "Disk source"),
     examples=[_ex({"S_A": 1e7, "R": 1.0, "h": 0.5, "mu": 10.0, "t": 0.05},
                   written={"S_A": "1e7 1/(m^2*s)", "R": "1 m", "h": "50 cm", "mu": "0.1 1/cm", "t": "5 cm"}),
               _ex({"S_A": 1e6, "R": 2.0, "h": 1.0, "mu": 0.0, "t": 0.0},
                   written={"S_A": "1e6 1/(m^2*s)", "R": "2 m", "h": "1 m", "mu": "0 1/m", "t": "0 m"})],
     level="advanced")


# ------------------------------------------------------------------------------------------------ dose over time
def transit_dose(Gamma, A, d, v, L):
    _pos("Gamma, the activity, the closest distance, the speed and the path length", Gamma, A, d, v, L)
    D = Gamma * A / (d * v) * 2.0 * math.atan(L / (2.0 * d))
    Dinf = Gamma * A * math.pi / (d * v)
    return _out(D=D, D_uSv=D * 1e6, frac=D / Dinf)


def _ref_transit_dose(Gamma, A, d, v, L):
    T = L / (2.0 * v)
    D = _quad(lambda t: Gamma * A / (d * d + v * v * t * t), -T, T, points=[0.0])
    Dinf = 2.0 * _quad(lambda t: Gamma * A / (d * d + v * v * t * t), 0.0, math.inf)
    return {"D": D, "D_uSv": D * 1e6, "frac": D / Dinf}


_add("nuc_transit_dose",
     name="Dose received passing a point source", topic="shielding and dose / dose over time",
     purpose="The dose received by a person or detector moving at constant speed along a straight path past an unshielded point source, from the closest distance, the speed and the path length (centred on the closest approach).",
     inputs=[_phys("Gamma", "dose_rate_constant", "dose-rate constant of the nuclide"), _phys("A", "radioactivity", "source activity"),
             _phys("d", "length", "closest distance to the source"), _phys("v", "speed", "speed along the path"),
             _phys("L", "length", "length of the path, centred on the point of closest approach")],
     outputs=[_o("D", "dose", "dose received (Sv or Gy, as Gamma)"), _o("D_uSv", "number", "dose in uSv (or uGy)"),
              _o("frac", "number", "fraction of the dose of an infinitely long path")],
     primary="D", conditions=["unshielded point source", "straight path at constant speed"],
     method="D = (Gamma A/(d v)) 2 atan(L/(2 d)); infinite path Gamma A pi/(d v)",
     latex=r"D = \frac{\Gamma A}{d\,v}\,2\arctan\frac{L}{2d}",
     run=transit_dose, reference=_ref_transit_dose,
     wrong=lambda Gamma, A, d, v, L: (lambda D: _out(D=D, D_uSv=D * 1e6, frac=1.0))(Gamma * A / (d * d) * L / v),
     rand=lambda rng: (lambda d: {"Gamma": _sig(_lu(rng, 1e-18, 1e-15)), "A": _sig(_lu(rng, 1e8, 1e14)), "d": d,
                                  "v": _sig(_lu(rng, 0.2, 30.0)), "L": _sig(d * _lu(rng, 0.2, 100.0))})(_sig(_lu(rng, 0.3, 50.0))),
     phrases=("Transit dose", "Dose passing a point source", "Dose passing a source"),
     examples=[_ex({"Gamma": 0.351e-12 / 3600.0, "A": 3.7e12, "d": 2.0, "v": 1.0, "L": 20.0},
                   written={"Gamma": "0.351 uSv*m^2/(MBq*h)", "A": "100 Ci", "d": "2 m", "v": "1 m/s", "L": "20 m"}),
               _ex({"Gamma": 0.0927e-12 / 3600.0, "A": 1e12, "d": 5.0, "v": 10.0, "L": 1000.0},
                   written={"Gamma": "0.0927 uSv*m^2/(MBq*h)", "A": "1 TBq", "d": "5 m", "v": "10 m/s", "L": "1 km"})],
     level="intermediate")


def accumulated_dose(H0, T_half, t, td=0.0):
    _pos("the initial dose rate, the half-life and the exposure time", H0, T_half, t)
    _nonneg("the delay td", td)
    lam = LN2 / T_half
    Hs = H0 * math.exp(-lam * td)
    D = Hs * -math.expm1(-lam * t) / lam
    return _out(D=D, D_mSv=D * 1e3, D_inf=Hs / lam, H_mean=D / t)


def _ref_accumulated_dose(H0, T_half, t, td=0.0):
    lam = LN2 / T_half
    D = _quad(lambda s: H0 * math.exp(-lam * s), td, td + t)
    Dinf = _quad(lambda s: H0 * math.exp(-lam * s), td, td + 60.0 / lam)
    return {"D": D, "D_mSv": D * 1e3, "D_inf": Dinf, "H_mean": D / t}


_add("nuc_accumulated_dose_decaying_source",
     name="Dose accumulated near a decaying source", topic="shielding and dose / dose over time",
     purpose="The dose accumulated over an exposure time when the dose rate falls with the source's half-life, the dose if the exposure lasted until complete decay, and the mean dose rate.",
     inputs=[_phys("H0", "dose_rate", "dose rate at the reference time"), _phys("T_half", "time", "half-life of the source"),
             _phys("t", "time", "exposure time"), _phys("td", "time", "delay from the reference time to the start (0 when omitted)", required=False)],
     outputs=[_o("D", "dose", "dose accumulated"), _o("D_mSv", "number", "dose in mSv (or mGy)"),
              _o("D_inf", "dose", "dose until complete decay"), _o("H_mean", "dose_rate", "mean dose rate during the exposure")],
     primary="D", conditions=["the dose rate follows one half-life"],
     method="D = H0 e^(-lambda td) (1 - e^(-lambda t))/lambda",
     latex=r"D = \dot H_0\,e^{-\lambda t_d}\,\frac{1-e^{-\lambda t}}{\lambda}",
     run=accumulated_dose, reference=_ref_accumulated_dose,
     wrong=lambda H0, T_half, t, td=0.0: _out(D=H0 * t, D_mSv=H0 * t * 1e3, D_inf=H0 * T_half, H_mean=H0),
     rand=lambda rng: (lambda T: {"H0": _sig(_lu(rng, 1e-9, 1e-3)), "T_half": _sig(T), "t": _sig(T * _lu(rng, 0.02, 6.0)),
                                  **({"td": _sig(T * _lu(rng, 0.01, 3.0))} if rng.random() < 0.5 else {})})(_lu(rng, 60.0, 1e9)),
     phrases=("Accumulated dose from a decaying source", "Accumulated dose", "Dose from a decaying source"),
     examples=[_ex({"H0": 1e-3 / 3600.0, "T_half": 6586.2, "t": 7200.0}, written={"H0": "1 mSv/h", "t": "2 h"}),
               _ex({"H0": 50e-6 / 3600.0, "T_half": 692988.48, "t": 86400.0 * 7, "td": 86400.0}, written={"H0": "50 uSv/h", "t": "7 d", "td": "1 d"})],
     level="beginner")


def stay_time(H0, T_half, D_lim):
    _pos("the dose rate, the half-life and the dose limit", H0, T_half, D_lim)
    lam = LN2 / T_half
    q = lam * D_lim / H0
    _need(q < 1, "the dose limit is never reached: the source decays first (D_lim >= H0 T_half/ln 2)")
    t = -math.log1p(-q) / lam
    return _out(t_stay=t, frac=q)


def _ref_stay_time(H0, T_half, D_lim):
    lam = LN2 / T_half
    D = lambda t: _quad(lambda s: H0 * math.exp(-lam * s), 0.0, t)
    Dinf = _quad(lambda s: H0 * math.exp(-lam * s), 0.0, 60.0 / lam)
    t = _brentq(lambda t: D(t) - D_lim, 0.0, 60.0 / lam)
    return {"t_stay": t, "frac": D_lim / Dinf}


_add("nuc_stay_time_decaying_source",
     name="Stay time to a dose limit near a decaying source", topic="shielding and dose / dose over time",
     purpose="How long one can stay at a place where the dose rate decays with the source's half-life before receiving a stated dose, compared with the stay time if the rate were constant.",
     inputs=[_phys("H0", "dose_rate", "dose rate at the start of the stay"), _phys("T_half", "time", "half-life of the source"),
             _phys("D_lim", "dose", "dose limit for the stay")],
     outputs=[_o("t_stay", "time", "allowed stay time"),
              _o("frac", "number", "dose limit as a fraction of the dose until complete decay")],
     primary="t_stay", conditions=["D_lim < H0 T_half/ln 2 (otherwise the limit is never reached)"],
     method="H0 (1 - e^(-lambda t))/lambda = D_lim -> t = -ln(1 - lambda D_lim/H0)/lambda",
     latex=r"t = -\frac{1}{\lambda}\ln\left(1-\frac{\lambda D_{lim}}{\dot H_0}\right)",
     run=stay_time, reference=_ref_stay_time,
     wrong=lambda H0, T_half, D_lim: _out(t_stay=D_lim / H0, frac=0.0),
     rand=lambda rng: (lambda H0, T: {"H0": H0, "T_half": T, "D_lim": _sig(H0 * T / LN2 * rng.uniform(0.02, 0.95))})(_sig(_lu(rng, 1e-9, 1e-3)), _sig(_lu(rng, 60.0, 1e9))),
     phrases=("Stay time", "Stay time near a decaying source", "Allowed stay time"),
     examples=[_ex({"H0": 2e-3 / 3600.0, "T_half": 6586.2, "D_lim": 1e-3}, written={"H0": "2 mSv/h", "D_lim": "1 mSv"}),
               _ex({"H0": 1e-4 / 3600.0, "T_half": 21654.0, "D_lim": 2e-4}, written={"H0": "100 uSv/h", "D_lim": "200 uSv"})],
     level="intermediate",
     extra_q=[{"q": "Stay time: H0 = 2 mSv/h, T_half = 6586.2 s, D_lim = 100 mSv", "outcome": "INVALID_INPUT"}])


# ------------------------------------------------------------------------------------------------ photon interactions
def klein_nishina(E):
    _pos("the photon energy", E)
    Emev = E / MEV
    _need(0.01 <= Emev <= 100.0, "the photon energy must lie between 10 keV and 100 MeV")
    with _mp.workdps(30):
        k = _mpf(Emev) / _mpf(ME_MEV)
        L = _mp.log(1 + 2 * k)
        s = 2 * _mp.pi * ((1 + k) / k ** 2 * (2 * (1 + k) / (1 + 2 * k) - L / k) + L / (2 * k) - (1 + 3 * k) / (1 + 2 * k) ** 2)
        st = 2 * _mp.pi * (2 * (1 + k) ** 2 / (k ** 2 * (1 + 2 * k)) - (1 + 3 * k) / (1 + 2 * k) ** 2
                           - (1 + k) * (2 * k ** 2 - 2 * k - 1) / (k ** 2 * (1 + 2 * k) ** 2) - 4 * k ** 2 / (3 * (1 + 2 * k) ** 3)
                           - ((1 + k) / k ** 3 - 1 / (2 * k) + 1 / (2 * k ** 3)) * L)
        sig, sigtr = float(s) * RE * RE, float(st) * RE * RE
    return _out(sigma=sig, sigma_b=sig / BARN, sigma_tr=sigtr, T_mean=Emev * sigtr / sig)


def _ref_klein_nishina(E):
    k = E / MEV / ME_MEV
    d = lambda mu: (lambda r: math.pi * RE * RE * r * r * (r + 1 / r - (1 - mu * mu)))(1 / (1 + k * (1 - mu)))
    s = _quad(d, -1.0, 1.0)
    st = _quad(lambda mu: d(mu) * (1 - 1 / (1 + k * (1 - mu))), -1.0, 1.0)
    return {"sigma": s, "sigma_b": s / BARN, "sigma_tr": st, "T_mean": E / MEV * st / s}


_add("nuc_klein_nishina",
     name="Klein-Nishina Compton cross sections per electron", topic="shielding and dose / photon interactions",
     purpose="The total Compton (Klein-Nishina) cross section per electron for a photon energy, the energy-transfer cross section and the mean kinetic energy given to the recoil electron.",
     inputs=[_phys("E", "energy", "photon energy (10 keV to 100 MeV)")],
     outputs=[_o("sigma", "microscopic_cross_section", "Klein-Nishina cross section per electron"),
              _o("sigma_b", "number", "the same in barns per electron"),
              _o("sigma_tr", "microscopic_cross_section", "energy-transfer cross section per electron"),
              _o("T_mean", "energy_mev", "mean kinetic energy of the Compton electron (MeV)")],
     primary="sigma", conditions=["free electrons at rest (no binding)", "10 keV <= E <= 100 MeV"],
     method="closed Klein-Nishina integrals over the scattering angle, k = E/(m_e c^2); T_mean = E sigma_tr/sigma",
     latex=r"\sigma = 2\pi r_e^2\left\{\frac{1+k}{k^2}\left[\frac{2(1+k)}{1+2k}-\frac{\ln(1+2k)}{k}\right]+\frac{\ln(1+2k)}{2k}-\frac{1+3k}{(1+2k)^2}\right\}",
     run=klein_nishina, reference=_ref_klein_nishina,
     wrong=lambda E: (lambda o: {**o, "sigma": _r(8 * math.pi / 3 * RE * RE), "sigma_b": _r(8 * math.pi / 3 * RE * RE / BARN)})(klein_nishina(E)),
     rand=lambda rng: {"E": _sig(_lu(rng, 0.02, 100.0)) * MEV},
     phrases=("Klein-Nishina cross section", "Klein-Nishina", "Compton cross section per electron"),
     examples=[_ex({"E": 0.662 * MEV}, written={"E": "0.662 MeV"}), _ex({"E": 1.25 * MEV}, written={"E": "1.25 MeV"})],
     level="expert",
     extra_q=[{"q": "Klein-Nishina: E = 1 keV", "outcome": "INVALID_INPUT"}])


def compton_edge(E):
    _pos("the photon energy", E)
    Emev = E / MEV
    k = Emev / ME_MEV
    Ebs = Emev / (1 + 2 * k)
    return _out(T_max=Emev - Ebs, E_bs=Ebs)


def _ref_compton_edge(E):
    from scipy.optimize import brentq
    Emev = E / MEV
    m = ME_MEV
    # backscatter: the photon reverses, the electron takes momentum E + E'; energy balance solved numerically
    f = lambda Ep: Emev + m - Ep - math.sqrt(m * m + (Emev + Ep) ** 2)
    Ep = brentq(f, 0.0, Emev, xtol=1e-300, rtol=8.9e-16, maxiter=500)
    return {"T_max": Emev - Ep, "E_bs": Ep}


_add("nuc_compton_edge",
     name="Compton edge and backscatter peak of a gamma line", topic="detectors and counting / gamma spectra",
     purpose="Where a gamma line's Compton edge (maximum electron energy) and backscatter peak (energy of a photon scattered through 180 degrees) appear in a detector's spectrum.",
     inputs=[_phys("E", "energy", "gamma-ray energy")],
     outputs=[_o("T_max", "energy_mev", "Compton edge: maximum electron recoil energy (MeV)"),
              _o("E_bs", "energy_mev", "backscatter peak: energy of the photon scattered through 180 degrees (MeV)")],
     primary="T_max", conditions=["free electron at rest"],
     method="E_bs = E/(1 + 2E/(m_e c^2)), T_max = E - E_bs",
     latex=r"T_{max} = \frac{2E^2}{m_ec^2 + 2E},\quad E_{bs} = \frac{E}{1+2E/m_ec^2}",
     run=compton_edge, reference=_ref_compton_edge,
     wrong=lambda E: (lambda Em: _out(T_max=Em - Em / (1 + Em / ME_MEV), E_bs=Em / (1 + Em / ME_MEV)))(E / MEV),
     rand=lambda rng: {"E": _sig(_lu(rng, 0.01, 20.0)) * MEV},
     phrases=("Compton edge", "Compton edge and backscatter peak", "Backscatter peak"),
     examples=[_ex({"E": 0.662 * MEV}, written={"E": "662 keV"}), _ex({"E": 1.332 * MEV}, written={"E": "1.332 MeV"})],
     level="beginner")


# ================================================================================================ DETECTORS AND COUNTING
def dead_time_nonparalyzable(m, tau):
    _pos("the observed count rate and the dead time", m, tau)
    _need(m * tau < 1, "m tau must be below 1 for a non-paralyzable detector")
    n = m / (1.0 - m * tau)
    return _out(n=n, loss=m * tau, corr=n / m)


def _ref_dead_time_nonparalyzable(m, tau):
    # renewal theory: after each count the detector is dead for tau, then waits an exponential time for the next
    # event; observed rate = 1 / mean interval, the mean interval computed by quadrature of its density
    def observed(n):
        mean = _quad(lambda t: t * n * math.exp(-n * (t - tau)), tau, tau + 60.0 / n)
        return 1.0 / mean
    n = _brentq(lambda n: observed(n) - m, m, m / (1.0 - m * tau) * 4.0 + 1.0)
    return {"n": n, "loss": (n - m) / n, "corr": n / m}


_add("nuc_dead_time_nonparalyzable",
     name="Dead-time correction, non-paralyzable detector", topic="detectors and counting / dead time",
     purpose="The true event rate from the observed count rate of a non-paralyzable (non-extendable) detector with a stated dead time, the fraction of events lost and the correction factor.",
     inputs=[_phys("m", "count_rate", "observed count rate (cps, cpm)"), _phys("tau", "time", "dead time (e.g. us)")],
     outputs=[_o("n", "count_rate", "true event rate"), _o("loss", "number", "fraction of events lost"), _o("corr", "number", "correction factor n/m")],
     primary="n", conditions=["non-paralyzable (non-extendable) dead time", "m tau < 1"],
     method="n = m / (1 - m tau)",
     latex=r"n = \frac{m}{1 - m\tau}",
     run=dead_time_nonparalyzable, reference=_ref_dead_time_nonparalyzable,
     wrong=lambda m, tau: _out(n=m / (1 + m * tau), loss=m * tau, corr=1 / (1 + m * tau)),
     rand=lambda rng: (lambda tau: {"m": _sig(rng.uniform(0.01, 0.7) / tau), "tau": tau})(_sig(_lu(rng, 1e-7, 1e-3))),
     phrases=("Non-paralyzable dead time", "Dead time non-paralyzable", "Non-paralyzable dead-time correction"),
     examples=[_ex({"m": 50000.0, "tau": 5e-6}, written={"m": "50000 cps", "tau": "5 us"}),
               _ex({"m": 1e6 / 60.0, "tau": 1e-5}, written={"m": "1e6 cpm", "tau": "10 us"})],
     level="intermediate",
     extra_q=[{"q": "Non-paralyzable dead time: m = 300000 cps, tau = 5 us", "outcome": "INVALID_INPUT"}])


def dead_time_paralyzable(m, tau, branch):
    _pos("the observed count rate and the dead time", m, tau)
    _need(branch in ("low", "high"), "branch must be low (n tau < 1) or high (n tau > 1)")
    y = m * tau
    _need(y <= math.exp(-1.0), "m tau above 1/e is impossible for a paralyzable detector")
    ly = math.log(y)
    g = lambda x: math.log(x) - x - ly
    if branch == "low":
        x = _bisect(g, y, 1.0)
    else:
        x = _bisect(g, 1.0, -2.0 * ly + 10.0)
    n = x / tau
    return _out(n=n, loss=1.0 - m / n, m_max=1.0 / (math.e * tau))


def _ref_dead_time_paralyzable(m, tau, branch):
    from scipy.optimize import minimize_scalar
    from scipy.special import lambertw
    y = m * tau
    x = float((-lambertw(-y, 0 if branch == "low" else -1)).real)
    res = minimize_scalar(lambda n: -n * math.exp(-n * tau), bounds=(0.0, 5.0 / tau), method="bounded",
                          options={"xatol": 1e-10 / tau})
    return {"n": x / tau, "loss": 1.0 - m * tau / x, "m_max": float(-res.fun)}


_add("nuc_dead_time_paralyzable",
     name="Dead-time correction, paralyzable detector", topic="detectors and counting / dead time",
     purpose="The true event rate from the observed count rate of a paralyzable (extendable) detector, on the stated branch of the rate curve (below or above the maximum), with the fraction lost and the largest observable count rate.",
     inputs=[_phys("m", "count_rate", "observed count rate"), _phys("tau", "time", "dead time"),
             _choice("branch", ("low", "high"), "branch of the rate curve: low (true rate below 1/tau) or high (above it)")],
     outputs=[_o("n", "count_rate", "true event rate"), _o("loss", "number", "fraction of events lost"),
              _o("m_max", "count_rate", "largest observable count rate 1/(e tau)")],
     primary="n", conditions=["paralyzable (extendable) dead time", "m tau <= 1/e", "the branch is stated (two true rates give the same m)"],
     method="m = n e^(-n tau) solved for n on the stated branch (bisection in x = n tau)",
     latex=r"m = n\,e^{-n\tau}",
     run=dead_time_paralyzable, reference=_ref_dead_time_paralyzable,
     wrong=lambda m, tau, branch: _out(n=m / (1 - m * tau), loss=m * tau, m_max=1.0 / tau),
     rand=lambda rng: (lambda tau, br: {"m": _sig((lambda x: x * math.exp(-x))(rng.uniform(0.02, 0.9) if br == "low" else rng.uniform(1.1, 5.0)) / tau),
                                        "tau": tau, "branch": br})(_sig(_lu(rng, 1e-7, 1e-3)), rng.choice(["low", "high"])),
     phrases=("Paralyzable dead time", "Dead time paralyzable", "Paralyzable dead-time correction"),
     examples=[_ex({"m": 50000.0, "tau": 5e-6, "branch": "low"}, written={"m": "50000 cps", "tau": "5 us"}),
               _ex({"m": 50000.0, "tau": 5e-6, "branch": "high"}, written={"m": "50000 cps", "tau": "5 us"})],
     level="advanced",
     extra_q=[{"q": "Paralyzable dead time: m = 100000 cps, tau = 5 us, branch = low", "outcome": "INVALID_INPUT"}])


def count_time_split(T, Rg, Rb):
    _pos("the total time and the count rates", T, Rg, Rb)
    _need(Rg > Rb, "the gross count rate must exceed the background rate")
    sg, sb = math.sqrt(Rg), math.sqrt(Rb)
    tg = T * sg / (sg + sb)
    tb = T - tg
    sig = (sg + sb) / math.sqrt(T)
    return _out(tg=tg, tb=tb, sigma=sig, rel=sig / (Rg - Rb))


def _ref_count_time_split(T, Rg, Rb):
    # minimise Var(net rate) = Rg/tg + Rb/(T - tg) numerically: root of its derivative in tg
    tg = _brentq(lambda t: -Rg / (t * t) + Rb / ((T - t) ** 2), T * 1e-12, T * (1 - 1e-12))
    sig = math.sqrt(Rg / tg + Rb / (T - tg))
    return {"tg": tg, "tb": T - tg, "sigma": sig, "rel": sig / (Rg - Rb)}


_add("nuc_count_time_split",
     name="Optimal division of counting time between sample and background", topic="detectors and counting / counting statistics",
     purpose="How to split a fixed total counting time between the sample (gross) count and the background count to minimise the standard deviation of the net count rate, and that minimum standard deviation.",
     inputs=[_phys("T", "time", "total counting time available"), _phys("Rg", "count_rate", "gross count rate (sample plus background)"),
             _phys("Rb", "count_rate", "background count rate")],
     outputs=[_o("tg", "time", "time for the gross count"), _o("tb", "time", "time for the background count"),
              _o("sigma", "count_rate", "minimum standard deviation of the net count rate"),
              _o("rel", "number", "relative standard deviation of the net count rate")],
     primary="tg", conditions=["Poisson counting statistics", "Rg > Rb > 0"],
     method="tg/tb = sqrt(Rg/Rb); sigma_net = (sqrt(Rg) + sqrt(Rb))/sqrt(T)",
     latex=r"\frac{t_g}{t_b} = \sqrt{\frac{R_g}{R_b}},\quad \sigma_{net} = \frac{\sqrt{R_g}+\sqrt{R_b}}{\sqrt{T}}",
     run=count_time_split, reference=_ref_count_time_split,
     wrong=lambda T, Rg, Rb: (lambda tg: _out(tg=tg, tb=T - tg, sigma=math.sqrt(Rg / tg + Rb / (T - tg)), rel=0.0))(T / 2),
     rand=lambda rng: (lambda Rb: {"T": _sig(_lu(rng, 60.0, 1e5)), "Rb": Rb, "Rg": _sig(Rb * _lu(rng, 1.5, 1e3))})(_sig(_lu(rng, 0.01, 100.0))),
     phrases=("Counting time split", "Optimal counting time split", "Divide counting time"),
     examples=[_ex({"T": 3600.0, "Rg": 100.0, "Rb": 25.0}, written={"T": "1 h", "Rg": "100 cps", "Rb": "25 cps"}),
               _ex({"T": 600.0, "Rg": 600.0 / 60.0, "Rb": 30.0 / 60.0}, written={"T": "10 min", "Rg": "600 cpm", "Rb": "30 cpm"})],
     level="intermediate")


def count_time_for_precision(Rg, Rb, r):
    _pos("the count rates", Rg, Rb)
    _frac("the target relative standard deviation r", r, lo_open=True, hi_open=True)
    _need(Rg > Rb, "the gross count rate must exceed the background rate")
    sg, sb = math.sqrt(Rg), math.sqrt(Rb)
    T = (sg + sb) ** 2 / ((Rg - Rb) ** 2 * r * r)
    tg = T * sg / (sg + sb)
    return _out(T=T, tg=tg, tb=T - tg)


def _ref_count_time_for_precision(Rg, Rb, r):
    def rel(T):
        tg = _brentq(lambda t: -Rg / (t * t) + Rb / ((T - t) ** 2), T * 1e-12, T * (1 - 1e-12))
        return math.sqrt(Rg / tg + Rb / (T - tg)) / (Rg - Rb)
    lo = 1e-12
    hi = 1.0
    while rel(hi) > r:
        hi *= 10.0
    T = _brentq(lambda T: rel(T) - r, lo, hi)
    tg = _brentq(lambda t: -Rg / (t * t) + Rb / ((T - t) ** 2), T * 1e-12, T * (1 - 1e-12))
    return {"T": T, "tg": tg, "tb": T - tg}


_add("nuc_count_time_for_precision",
     name="Counting time for a target precision of the net count rate", topic="detectors and counting / counting statistics",
     purpose="The total counting time (optimally split between sample and background) needed for the net count rate to have a stated relative standard deviation, and the two counting times.",
     inputs=[_phys("Rg", "count_rate", "gross count rate"), _phys("Rb", "count_rate", "background count rate"),
             _numin("r", "target relative standard deviation of the net count rate (e.g. 1%)", pct=True)],
     outputs=[_o("T", "time", "total counting time"), _o("tg", "time", "gross counting time"), _o("tb", "time", "background counting time")],
     primary="T", conditions=["Poisson statistics", "optimal split tg/tb = sqrt(Rg/Rb)", "Rg > Rb"],
     method="T = (sqrt(Rg) + sqrt(Rb))^2 / ((Rg - Rb)^2 r^2)",
     latex=r"T = \frac{(\sqrt{R_g}+\sqrt{R_b})^2}{(R_g-R_b)^2\,r^2}",
     run=count_time_for_precision, reference=_ref_count_time_for_precision,
     wrong=lambda Rg, Rb, r: (lambda T: _out(T=T, tg=T / 2, tb=T / 2))((Rg + Rb) / ((Rg - Rb) ** 2 * r * r)),
     rand=lambda rng: (lambda Rb: {"Rb": Rb, "Rg": _sig(Rb * _lu(rng, 1.5, 1e3)), "r": round(rng.uniform(0.002, 0.2), 4)})(_sig(_lu(rng, 0.01, 100.0))),
     phrases=("Counting time for precision", "Counting time for a target precision", "Required counting time"),
     examples=[_ex({"Rg": 100.0, "Rb": 25.0, "r": 0.01}, written={"Rg": "100 cps", "Rb": "25 cps", "r": "1%"}),
               _ex({"Rg": 5.0, "Rb": 1.0, "r": 0.05}, written={"Rg": "300 cpm", "Rb": "60 cpm", "r": "5%"})],
     level="intermediate")


def solid_angle_axis(d, a):
    _pos("the distance and the radius", d, a)
    c = d / math.hypot(d, a)
    om = 2 * math.pi * (1 - c)
    return _out(Omega=om, g=om / (4 * math.pi))


def _ref_solid_angle_axis(d, a):
    om = _quad(lambda rho: 2 * math.pi * rho * d / (d * d + rho * rho) ** 1.5, 0.0, a)
    return {"Omega": om, "g": om / (4 * math.pi)}


_add("nuc_solid_angle_on_axis",
     name="Solid angle and geometric efficiency of a disk detector (source on axis)", topic="detectors and counting / geometry",
     purpose="The solid angle subtended by a circular detector face (or aperture) at a point source on its axis and the geometric efficiency (fraction of isotropic emissions that hit it).",
     inputs=[_phys("d", "length", "distance from the source to the detector face"), _phys("a", "length", "radius of the detector face")],
     outputs=[_o("Omega", "number", "solid angle (sr)"), _o("g", "number", "geometric efficiency Omega/(4 pi)")],
     primary="Omega", conditions=["point isotropic source on the axis"],
     method="Omega = 2 pi (1 - d/sqrt(d^2 + a^2))",
     latex=r"\Omega = 2\pi\left(1-\frac{d}{\sqrt{d^2+a^2}}\right)",
     run=solid_angle_axis, reference=_ref_solid_angle_axis,
     wrong=lambda d, a: (lambda om: _out(Omega=om, g=om / (4 * math.pi)))(math.pi * a * a / (d * d)),
     rand=lambda rng: (lambda a: {"a": a, "d": _sig(a * _lu(rng, 0.05, 50.0))})(_sig(_lu(rng, 0.001, 0.2))),
     phrases=("Solid angle on axis", "Solid angle of a disk detector", "Geometric efficiency on axis"),
     examples=[_ex({"d": 0.1, "a": 0.0254}, written={"d": "10 cm", "a": "2.54 cm"}), _ex({"d": 0.01, "a": 0.01}, written={"d": "1 cm", "a": "1 cm"})],
     level="beginner")


def solid_angle_off_axis(d, a, rho0):
    _pos("the distance and the radius", d, a)
    _nonneg("the lateral offset", rho0)
    with _mp.workdps(20):
        D, A, P = _mpf(d), _mpf(a), _mpf(rho0)
        if rho0 <= a:
            f = lambda ph: 1 - D / _mp.sqrt(D * D + (P * _mp.cos(ph) + _mp.sqrt(A * A - (P * _mp.sin(ph)) ** 2)) ** 2)
            om = 2 * _mp.quad(f, [0, _mp.pi / 2, _mp.pi])
        else:
            ph0 = _mp.asin(A / P)

            def f(ph):
                s = _mp.sqrt(max(A * A - (P * _mp.sin(ph)) ** 2, 0))
                r1, r2 = P * _mp.cos(ph) - s, P * _mp.cos(ph) + s
                return D / _mp.sqrt(D * D + r1 * r1) - D / _mp.sqrt(D * D + r2 * r2)
            om = 2 * _mp.quad(f, [0, ph0])
        om = float(om)
    return _out(Omega=om, g=om / (4 * math.pi))


def _ref_solid_angle_off_axis(d, a, rho0):
    # direct integration over the detector face: dOmega = d dA / r^3 (polar coordinates about the face centre)
    inner = lambda rho: _quad(lambda psi: d * rho / (d * d + rho * rho + rho0 * rho0 - 2 * rho * rho0 * math.cos(psi)) ** 1.5, 0.0, math.pi)
    om = 2.0 * _quad(inner, 0.0, a, points=[rho0] if 0 < rho0 < a else None)
    return {"Omega": om, "g": om / (4 * math.pi)}


_add("nuc_solid_angle_off_axis",
     name="Solid angle of a disk detector for an off-axis point source", topic="detectors and counting / geometry",
     purpose="The solid angle subtended by a circular detector face at a point source displaced sideways from its axis, and the geometric efficiency.",
     inputs=[_phys("d", "length", "distance from the source to the plane of the detector face"), _phys("a", "length", "radius of the detector face"),
             _phys("rho0", "length", "lateral offset of the source from the detector axis")],
     outputs=[_o("Omega", "number", "solid angle (sr)"), _o("g", "number", "geometric efficiency Omega/(4 pi)")],
     primary="Omega", conditions=["point isotropic source", "flat circular face"],
     method="Omega = int over azimuth of (1 - d/sqrt(d^2 + r_edge(phi)^2)) dphi (one-dimensional, mpmath tanh-sinh)",
     latex=r"\Omega = \int_0^{2\pi}\left(1-\frac{d}{\sqrt{d^2+r_e(\varphi)^2}}\right)d\varphi",
     run=solid_angle_off_axis, reference=_ref_solid_angle_off_axis,
     wrong=lambda d, a, rho0: (lambda c: _out(Omega=2 * math.pi * (1 - c), g=(1 - c) / 2))(d / math.hypot(d, a)),
     rand=lambda rng: (lambda a: {"a": a, "d": _sig(a * _lu(rng, 0.2, 10.0)), "rho0": _sig(a * rng.uniform(0.05, 3.0))})(_sig(_lu(rng, 0.005, 0.1))),
     phrases=("Solid angle off axis", "Off-axis solid angle", "Solid angle for an off-axis source"),
     examples=[_ex({"d": 0.05, "a": 0.025, "rho0": 0.01}, written={"d": "5 cm", "a": "2.5 cm", "rho0": "1 cm"}),
               _ex({"d": 0.05, "a": 0.025, "rho0": 0.05}, written={"d": "5 cm", "a": "2.5 cm", "rho0": "5 cm"})],
     level="expert")


def detector_efficiency(d, a, t, mu):
    _pos("the distance, radius, thickness and attenuation coefficient", d, a, t, mu)
    c0 = d / math.hypot(d, a)
    b = mu * t
    with _mp.workdps(20):
        tot = float(_mp.quad(lambda u: (1 - _mp.exp(-b / u)) / 2, [c0, 1]))
    g = (1 - c0) / 2
    return _out(eff=tot, g=g, intrinsic=tot / g)


def _ref_detector_efficiency(d, a, t, mu):
    # integrate over the detector face instead of the angle: each element sees the source through its own path length
    f = lambda rho: (1 - math.exp(-mu * t * math.sqrt(d * d + rho * rho) / d)) * d * rho / (2 * (d * d + rho * rho) ** 1.5)
    tot = _quad(f, 0.0, a)
    g = _quad(lambda rho: d * rho / (2 * (d * d + rho * rho) ** 1.5), 0.0, a)
    return {"eff": tot, "g": g, "intrinsic": tot / g}


_add("nuc_detector_efficiency_point_source",
     name="Total and intrinsic efficiency of a slab detector for a point source on axis", topic="detectors and counting / efficiency",
     purpose="The total detection efficiency of a flat detector of given thickness and attenuation coefficient for a point source on its axis (every photon entering the front face crosses the thickness along its slant path), the geometric efficiency and the intrinsic efficiency.",
     inputs=[_phys("d", "length", "source-to-face distance"), _phys("a", "length", "radius of the detector face"),
             _phys("t", "length", "detector thickness"), _phys("mu", "linear_attenuation_coefficient", "total linear attenuation coefficient of the detector material")],
     outputs=[_o("eff", "number", "total efficiency (interacting photons per emitted photon)"),
              _o("g", "number", "geometric efficiency"), _o("intrinsic", "number", "intrinsic efficiency eff/g")],
     primary="eff", conditions=["point source on the axis", "path length t/cos(theta) for every ray through the front face (side escape neglected)"],
     method="eff = (1/2) int_{cos theta_max}^1 (1 - e^(-mu t/u)) du, cos theta_max = d/sqrt(d^2 + a^2)",
     latex=r"\varepsilon = \frac12\int_{\cos\theta_{max}}^{1}\left(1-e^{-\mu t/u}\right)du",
     run=detector_efficiency, reference=_ref_detector_efficiency,
     wrong=lambda d, a, t, mu: (lambda g: _out(eff=g * -math.expm1(-mu * t), g=g, intrinsic=-math.expm1(-mu * t) * 0.999))((1 - d / math.hypot(d, a)) / 2),
     rand=lambda rng: (lambda a: {"a": a, "d": _sig(a * _lu(rng, 0.1, 20.0)), "t": _sig(_lu(rng, 0.001, 0.1)), "mu": _sig(_lu(rng, 1.0, 200.0))})(_sig(_lu(rng, 0.005, 0.1))),
     phrases=("Detector efficiency for a point source", "Detector efficiency", "Slab detector efficiency"),
     examples=[_ex({"d": 0.1, "a": 0.0381, "t": 0.0762, "mu": 28.0}, written={"d": "10 cm", "a": "3.81 cm", "t": "7.62 cm", "mu": "0.28 1/cm"}),
               _ex({"d": 0.02, "a": 0.025, "t": 0.005, "mu": 50.0}, written={"d": "2 cm", "a": "2.5 cm", "t": "5 mm", "mu": "0.5 1/cm"})],
     level="advanced")


def inverse_multiplication(x, C):
    _need(len(x) == len(C) and len(x) >= 3, "give at least three loadings with their count rates")
    _need(all(_isnum(v) for v in x) and len(set(x)) == len(x), "loadings must be distinct numbers")
    _need(all(_isnum(c) and c > 0 for c in C), "count rates must be positive")
    inv = [C[0] / c for c in C]
    n = len(x)
    mx, my = math.fsum(x) / n, math.fsum(inv) / n
    sxx = math.fsum((v - mx) ** 2 for v in x)
    sxy = math.fsum((v - mx) * (w - my) for v, w in zip(x, inv))
    slope = sxy / sxx
    _need(slope < 0, "1/M does not decrease with loading: no extrapolation to criticality")
    icpt = my - slope * mx
    s2 = (inv[-1] - inv[-2]) / (x[-1] - x[-2])
    _need(s2 < 0, "the last two points do not decrease: no two-point extrapolation")
    return _out(x_crit=-icpt / slope, slope=slope, x_crit_last2=x[-1] - inv[-1] / s2, inv_M_last=inv[-1])


def _ref_inverse_multiplication(x, C):
    import numpy as np
    inv = C[0] / np.array(C, dtype=float)
    p = np.polyfit(np.array(x, dtype=float), inv, 1)
    q = np.polyfit(np.array(x[-2:], dtype=float), inv[-2:], 1)
    return {"x_crit": float(-p[1] / p[0]), "slope": float(p[0]), "x_crit_last2": float(-q[1] / q[0]), "inv_M_last": float(inv[-1])}


def _rand_inv_m(rng):
    xc = _lu(rng, 10.0, 300.0)
    n = rng.randint(3, 8)
    xs = sorted(rng.sample(range(0, int(0.9 * xc)), n))
    C0 = _lu(rng, 10.0, 1000.0)
    C = [_sig(C0 / max(0.02, (1 - v / xc) * rng.uniform(0.97, 1.03))) for v in xs]
    C[0] = _sig(C0)
    return {"x": [float(v) for v in xs], "C": C}


_add("nuc_inverse_multiplication",
     name="Approach to criticality: inverse multiplication (1/M) extrapolation", topic="reactor physics / approach to criticality",
     purpose="Predict the critical loading (fuel elements, kg, rod position) from count rates measured as fuel is added, by extrapolating the inverse multiplication 1/M = C_0/C to zero: least-squares line through all points and the line through the last two.",
     inputs=[_vecin("x", "loadings (fuel elements, kg or any consistent measure), first = reference", 3, 50),
             _vecin("C", "count rates at those loadings (any consistent unit)", 3, 50)],
     outputs=[_o("x_crit", "number", "predicted critical loading (least-squares line through all points)"),
              _o("slope", "number", "slope of 1/M per unit loading"),
              _o("x_crit_last2", "number", "predicted critical loading from the last two points"),
              _o("inv_M_last", "number", "1/M at the last loading")],
     primary="x_crit", conditions=["1/M referred to the first count rate", "1/M decreasing with loading", "subcritical measurements"],
     method="1/M_i = C_1/C_i; straight lines in x; x_crit where the line reaches 1/M = 0",
     latex=r"\frac{1}{M} = \frac{C_0}{C},\quad x_{crit}: \frac{1}{M}(x_{crit}) = 0",
     run=inverse_multiplication, reference=_ref_inverse_multiplication,
     wrong=lambda x, C: (lambda inv: _out(x_crit=x[-1] / (1 - inv[-1]), slope=0.0, x_crit_last2=x[-1] / (1 - inv[-1]), inv_M_last=inv[-1]))([C[0] / c for c in C]),
     rand=_rand_inv_m,
     phrases=("Inverse multiplication", "Approach to criticality", "1/M extrapolation"),
     examples=[_ex({"x": [0.0, 20.0, 40.0, 60.0, 80.0], "C": [100.0, 125.0, 167.0, 250.0, 500.0]}),
               _ex({"x": [10.0, 30.0, 50.0], "C": [50.0, 80.0, 160.0]})],
     level="intermediate")


# ================================================================================================ NEUTRON PHYSICS
def _alpha(A):
    return ((A - 1.0) / (A + 1.0)) ** 2


def _xi(A):
    a = _alpha(A)
    if a == 0:
        return 1.0
    return 1.0 + a * math.log(a) / (1.0 - a)


def _ref_kinematics(A, theta):
    """Reference: elastic collision as vectors (neutron mass 1, target mass A at rest, incident speed 1): the CM moves
    at 1/(A+1); in the CM the neutron's velocity A/(A+1) turns through theta; back to the lab."""
    import numpy as np
    V = np.array([1.0 / (A + 1.0), 0.0])
    vc = A / (A + 1.0) * np.array([math.cos(theta), math.sin(theta)])
    v = V + vc
    return float(v @ v), float(math.atan2(v[1], v[0]))


def _ref_xi(A):
    ratio = lambda mu: (A * A + 2 * A * mu + 1) / (A + 1) ** 2
    return _quad(lambda mu: -math.log(ratio(mu)) / 2, -1.0, 1.0)


def _ref_mubar(A):
    return _quad(lambda mu: (1 + A * mu) / math.sqrt(A * A + 2 * A * mu + 1) / 2, -1.0, 1.0)


def elastic_scattering(A, E, theta):
    _need(_isnum(A) and A >= 1, "the mass ratio A must be at least 1")
    _pos("the neutron energy", E)
    _need(_isnum(theta) and 0 <= theta <= math.pi, "the centre-of-mass angle must lie between 0 and 180 degrees")
    _need(not (A == 1 and theta == math.pi), "a head-on collision with A = 1 stops the neutron: no lab angle")
    c = math.cos(theta)
    ratio = (A * A + 2 * A * c + 1) / (A + 1) ** 2
    psi = math.atan2(A * math.sin(theta), 1 + A * c)
    return _out(E_out=E * ratio, ratio=ratio, psi=psi, du=-math.log(ratio))


def _ref_elastic_scattering(A, E, theta):
    r, psi = _ref_kinematics(A, theta)
    return {"E_out": E * r, "ratio": r, "psi": psi, "du": -math.log(r)}


_add("nuc_elastic_scattering",
     name="Neutron energy and lab angle after an elastic collision", topic="neutron physics / moderation",
     purpose="The energy of a neutron after elastic scattering from a nucleus of mass ratio A through a stated centre-of-mass angle, the energy ratio, the scattering angle in the laboratory and the lethargy gained.",
     inputs=[_numin("A", "mass of the nucleus in neutron masses (mass number; 1 for hydrogen)"), _phys("E", "energy", "neutron energy before the collision"),
             _phys("theta", "angle", "scattering angle in the centre-of-mass frame (deg or rad)")],
     outputs=[_o("E_out", "energy", "neutron energy after the collision"), _o("ratio", "number", "E'/E"),
              _o("psi", "angle", "scattering angle in the laboratory frame"), _o("du", "number", "lethargy gain ln(E/E')")],
     primary="E_out", conditions=["elastic scattering from a nucleus at rest", "A >= 1"],
     method="E'/E = (A^2 + 2A cos theta + 1)/(A + 1)^2; tan psi = A sin theta/(1 + A cos theta)",
     latex=r"\frac{E'}{E} = \frac{A^2+2A\cos\theta+1}{(A+1)^2}",
     run=elastic_scattering, reference=_ref_elastic_scattering,
     wrong=lambda A, E, theta: (lambda r: _out(E_out=E * r, ratio=r, psi=theta, du=-math.log(r)))((A * A + 1) / (A + 1) ** 2),
     rand=lambda rng: {"A": rng.choice([1.0, 2.0, 9.0, 12.0, 16.0, 238.0, round(rng.uniform(1.0, 240.0), 3)]),
                       "E": _sig(_lu(rng, 0.01, 2e7)) * QE, "theta": round(rng.uniform(0.01, 3.1), 6)},
     phrases=("Elastic scattering", "Neutron elastic scattering", "Energy after an elastic collision"),
     examples=[_ex({"A": 12.0, "E": 2e6 * QE, "theta": math.pi / 2}, written={"E": "2 MeV", "theta": "90 deg"}),
               _ex({"A": 1.0, "E": 1e6 * QE, "theta": math.pi / 3}, written={"E": "1 MeV", "theta": "60 deg"})],
     level="beginner")


def log_decrement(A):
    _need(_isnum(A) and A >= 1, "the mass ratio A must be at least 1")
    return _out(alpha=_alpha(A), xi=_xi(A), mu_bar=2.0 / (3.0 * A))


def _ref_log_decrement(A):
    r, _psi = _ref_kinematics(A, math.pi)               # the largest energy loss: a head-on collision
    return {"alpha": r, "xi": _ref_xi(A), "mu_bar": _ref_mubar(A)}


_add("nuc_logarithmic_energy_decrement",
     name="Average logarithmic energy decrement and mean scattering cosine", topic="neutron physics / moderation",
     purpose="For elastic scattering from a nucleus of mass ratio A (isotropic in the centre of mass): the minimum energy ratio alpha, the average logarithmic energy loss per collision xi and the average cosine of the laboratory scattering angle.",
     inputs=[_numin("A", "mass of the nucleus in neutron masses (mass number)")],
     outputs=[_o("alpha", "number", "minimum E'/E, ((A - 1)/(A + 1))^2"), _o("xi", "number", "average logarithmic energy decrement"),
              _o("mu_bar", "number", "average cosine of the lab scattering angle, 2/(3A)")],
     primary="xi", conditions=["isotropic scattering in the centre-of-mass frame", "A >= 1"],
     method="xi = 1 + alpha ln(alpha)/(1 - alpha) (xi = 1 for A = 1); mu_bar = 2/(3A)",
     latex=r"\xi = 1 + \frac{\alpha\ln\alpha}{1-\alpha},\quad \alpha = \left(\frac{A-1}{A+1}\right)^2",
     run=log_decrement, reference=_ref_log_decrement,
     wrong=lambda A: _out(alpha=(A - 1) / (A + 1), xi=2.0 / (A + 2.0 / 3.0), mu_bar=1.0 / (3 * A)),
     rand=lambda rng: {"A": rng.choice([1.0, 2.0, 4.0, 9.0, 12.0, 16.0, 238.0, round(rng.uniform(1.0, 240.0), 3)])},
     phrases=("Logarithmic energy decrement", "Average logarithmic energy decrement", "Xi for a moderator"),
     examples=[_ex({"A": 12.0}), _ex({"A": 1.0}), _ex({"A": 2.0})],
     level="beginner",
     extra_q=[{"q": "Logarithmic energy decrement: A = 0.5", "outcome": "INVALID_INPUT"}])


def collisions_to_thermalize(A, E0, Ef):
    _need(_isnum(A) and A >= 1, "the mass ratio A must be at least 1")
    _pos("the energies", E0, Ef)
    _need(E0 > Ef, "the initial energy must exceed the final energy")
    xi = _xi(A)
    du = math.log(E0 / Ef)
    return _out(n=du / xi, xi=xi)


def _ref_collisions(A, E0, Ef):
    xi = _ref_xi(A)
    return {"n": math.log(E0 / Ef) / xi, "xi": xi}


_add("nuc_collisions_to_thermalize",
     name="Average number of collisions to slow a neutron down", topic="neutron physics / moderation",
     purpose="The average number of elastic collisions with nuclei of mass ratio A needed to slow a neutron from E0 to Ef (for example from 2 MeV to 0.025 eV), from the lethargy change and xi.",
     inputs=[_numin("A", "mass of the moderator nucleus in neutron masses"), _phys("E0", "energy", "initial neutron energy"),
             _phys("Ef", "energy", "final neutron energy")],
     outputs=[_o("n", "number", "average number of collisions"), _o("xi", "number", "average logarithmic energy decrement"),
              ],
     primary="n", conditions=["isotropic CM scattering", "E0 > Ef", "a single nuclide moderator"],
     method="n = ln(E0/Ef)/xi",
     latex=r"\bar n = \frac{\ln(E_0/E_f)}{\xi}",
     run=collisions_to_thermalize, reference=_ref_collisions,
     wrong=lambda A, E0, Ef: _out(n=math.log10(E0 / Ef) / _xi(A), xi=2.0 / (A + 2.0 / 3.0)),
     rand=lambda rng: {"A": rng.choice([1.0, 2.0, 9.0, 12.0, 16.0, 238.0, round(rng.uniform(1.0, 240.0), 3)]),
                       "E0": _sig(_lu(rng, 1e3, 2e7)) * QE, "Ef": _sig(_lu(rng, 0.01, 1.0)) * QE},
     phrases=("Collisions to thermalize", "Number of collisions to thermalize", "Collisions to slow down"),
     examples=[_ex({"A": 12.0, "E0": 2e6 * QE, "Ef": 0.025 * QE}, written={"E0": "2 MeV", "Ef": "0.025 eV"}),
               _ex({"A": 1.0, "E0": 2e6 * QE, "Ef": 0.025 * QE}, written={"E0": "2 MeV", "Ef": "0.025 eV"})],
     level="beginner")


def moderator_mixture(n, A, sigma_s, sigma_a, rho, M):
    k = len(n)
    _need(1 <= k <= 8 and len(A) == k and len(sigma_s) == k and len(sigma_a) == k, "give n, A, sigma_s and sigma_a for every element")
    _need(all(_isnum(x) and x > 0 for x in n), "atoms per molecule must be positive")
    _need(all(_isnum(x) and x >= 1 for x in A), "mass ratios must be at least 1")
    _need(all(_isnum(x) and x >= 0 for x in sigma_s + sigma_a), "cross sections must be >= 0")
    _pos("the density and the molar mass", rho, M)
    Nm = rho * NA / M
    Ss = [Nm * ni * s * BARN for ni, s in zip(n, sigma_s)]
    Sa = math.fsum(Nm * ni * s * BARN for ni, s in zip(n, sigma_a))
    S = math.fsum(Ss)
    _need(S > 0, "the scattering cross section is zero")
    xi = math.fsum(_xi(a) * s for a, s in zip(A, Ss)) / S
    out = dict(xi=xi, MP=xi * S)
    if Sa > 0:
        out["MR"] = xi * S / Sa
    return _out(**out)


def _ref_moderator_mixture(n, A, sigma_s, sigma_a, rho, M):
    import numpy as np
    Nm = rho * NA / M
    Ss = Nm * np.array(n) * np.array(sigma_s) * BARN
    Sa = float(Nm * np.dot(n, sigma_a) * BARN)
    xis = np.array([_ref_xi(a) for a in A])
    S = float(Ss.sum())
    xi = float(np.dot(xis, Ss) / S)
    out = {"xi": xi, "MP": xi * S}
    if Sa > 0:
        out["MR"] = xi * S / Sa
    return out


_add("nuc_moderator_mixture",
     name="Moderating power and ratio of a compound moderator", topic="neutron physics / moderation",
     purpose="The macroscopic scattering and absorption cross sections of a compound moderator (water, heavy water, graphite, beryllium oxide, ...), its average logarithmic energy decrement (weighted by scattering), moderating power and moderating ratio.",
     inputs=[_vecin("n", "atoms of each element per molecule", 1, 8), _vecin("A", "mass ratio (mass number) of each element", 1, 8),
             _vecin("sigma_s", "scattering cross section of each element, in barns", 1, 8, unit="b"),
             _vecin("sigma_a", "absorption cross section of each element, in barns", 1, 8, unit="b"),
             _phys("rho", "density", "density of the moderator"), _phys("M", "molar_mass", "molar mass of the molecule")],
     outputs=[_o("xi", "number", "average logarithmic energy decrement of the mixture"),
              _o("MP", "macroscopic_cross_section", "moderating power xi Sigma_s"),
              _o("MR", "number", "moderating ratio xi Sigma_s/Sigma_a (when Sigma_a > 0)")],
     primary="MR", conditions=["isotropic CM scattering for every element", "constant cross sections"],
     method="N = rho N_A/M; Sigma_x = N sum n_i sigma_x,i; xi = sum xi_i Sigma_s,i / Sigma_s",
     latex=r"\bar\xi = \frac{\sum_i\xi_i\Sigma_{s,i}}{\sum_i\Sigma_{s,i}},\quad MR = \frac{\bar\xi\Sigma_s}{\Sigma_a}",
     run=moderator_mixture, reference=_ref_moderator_mixture,
     wrong=lambda n, A, sigma_s, sigma_a, rho, M: (lambda o: {**o, "xi": _r(math.fsum(_xi(a) * k for a, k in zip(A, n)) / math.fsum(n)),
                                                             "MP": 0.0, "MR": 0.0})(moderator_mixture(n, A, sigma_s, sigma_a, rho, M)),
     rand=lambda rng: (lambda k: {"n": [float(rng.randint(1, 3)) for _ in range(k)], "A": [rng.choice([1.0, 2.0, 9.0, 12.0, 16.0]) for _ in range(k)],
                                  "sigma_s": [_sig(_lu(rng, 1.0, 50.0)) for _ in range(k)], "sigma_a": [_sig(_lu(rng, 1e-4, 1.0)) for _ in range(k)],
                                  "rho": _sig(rng.uniform(800.0, 3000.0)), "M": _sig(rng.uniform(0.01, 0.06))})(rng.randint(1, 3)),
     phrases=("Moderator mixture", "Moderating ratio", "Moderating power"),
     examples=[_ex({"n": [2.0, 1.0], "A": [1.0, 16.0], "sigma_s": [20.0, 3.8], "sigma_a": [0.332, 0.00019], "rho": 1000.0, "M": 0.018015},
                   written={"rho": "1 g/cm^3", "M": "18.015 g/mol"}),
               _ex({"n": [1.0], "A": [12.0], "sigma_s": [4.8], "sigma_a": [0.0034], "rho": 1600.0, "M": 0.012011},
                   written={"rho": "1.6 g/cm^3", "M": "12.011 g/mol"})],
     level="intermediate")


_E0_2200 = 0.5 * MN * 2200.0 ** 2           # J


def maxwellian_average(sigma0, T):
    _pos("the 2200 m/s cross section and the neutron temperature", sigma0, T)
    f = math.sqrt(math.pi) / 2.0 * math.sqrt(_E0_2200 / (KB * T))
    return _out(sigma_avg=sigma0 * f, sigma_avg_b=sigma0 * f / BARN, g=f)


def _ref_maxwellian_average(sigma0, T):
    kT = KB * T
    # flux-weighted Maxwellian (phi(E) ~ E e^(-E/kT)) average of sigma0 sqrt(E0/E), in x = E/kT
    num = _quad(lambda x: math.sqrt(_E0_2200 / (kT * x)) * x * math.exp(-x) if x > 0 else 0.0, 0.0, math.inf)
    den = _quad(lambda x: x * math.exp(-x), 0.0, math.inf)
    f = num / den
    return {"sigma_avg": sigma0 * f, "sigma_avg_b": sigma0 * f / BARN, "g": f}


_add("nuc_maxwellian_average_cross_section",
     name="Maxwellian-averaged 1/v cross section", topic="neutron physics / cross sections",
     purpose="The effective thermal cross section of a 1/v absorber averaged over a Maxwellian neutron flux at a stated neutron temperature, from its tabulated 2200 m/s value.",
     inputs=[_phys("sigma0", "microscopic_cross_section", "cross section at 2200 m/s (barns)"), _phys("T", "temperature", "neutron temperature")],
     outputs=[_o("sigma_avg", "microscopic_cross_section", "flux-averaged cross section"), _o("sigma_avg_b", "number", "the same in barns"),
              _o("g", "number", "ratio sigma_avg/sigma0 = (sqrt(pi)/2) sqrt(E0/kT)")],
     primary="sigma_avg", conditions=["1/v cross section", "Maxwellian flux spectrum phi(E) ~ E e^(-E/kT)", "E0 = m_n (2200 m/s)^2/2"],
     method="sigma_avg = sigma0 (sqrt(pi)/2) sqrt(E0/(k T))",
     latex=r"\bar\sigma = \frac{\sqrt\pi}{2}\,\sigma_0\sqrt{\frac{E_0}{kT}}",
     run=maxwellian_average, reference=_ref_maxwellian_average,
     wrong=lambda sigma0, T: (lambda f: _out(sigma_avg=sigma0 * f, sigma_avg_b=sigma0 * f / BARN, g=f))(math.sqrt(_E0_2200 / (KB * T))),
     rand=lambda rng: {"sigma0": _sig(_lu(rng, 0.01, 1e5)) * BARN, "T": _sig(rng.uniform(200.0, 1500.0))},
     phrases=("Maxwellian average cross section", "Maxwellian-averaged cross section", "Thermal average cross section"),
     examples=[_ex({"sigma0": 680.9e-28, "T": 293.6}, written={"sigma0": "680.9 b", "T": "293.6 K"}),
               _ex({"sigma0": 2.68e-28, "T": 600.0}, written={"sigma0": "2.68 b", "T": "600 K"})],
     level="intermediate")


_M5, _M8, _MO = 235.043928117, 238.050786936, 15.999     # g/mol (U masses of radioactivedecay; O standard weight)


def fuel_cross_sections(rho, w, sigma_a5, sigma_f5, sigma_a8, sigma_aO, nu):
    _pos("the density, the cross sections and nu", rho, sigma_a5, sigma_f5, nu)
    _nonneg("sigma_a8 and sigma_aO", sigma_a8, sigma_aO)
    _frac("the enrichment w", w, lo_open=True)
    _need(sigma_f5 <= sigma_a5, "the fission cross section cannot exceed the absorption cross section")
    rg = rho / 1000.0                                  # g/cm^3 -> per cm^3 below, then to 1/m^3
    a5 = (w / _M5) / (w / _M5 + (1 - w) / _M8)
    MU = a5 * _M5 + (1 - a5) * _M8
    Nmol = rg * NA / (MU + 2 * _MO) * 1e6              # UO2 molecules per m^3
    N5, N8, NO = Nmol * a5, Nmol * (1 - a5), 2 * Nmol
    Sa = (N5 * sigma_a5 + N8 * sigma_a8 + NO * sigma_aO)
    Sf = N5 * sigma_f5
    return _out(N5=N5, N8=N8, NO=NO, Sigma_a=Sa, Sigma_f=Sf, eta=nu * Sf / Sa, mfp_a=1.0 / Sa)


def _ref_fuel_cross_sections(rho, w, sigma_a5, sigma_f5, sigma_a8, sigma_aO, nu):
    import numpy as np
    import radioactivedecay as rd
    M5 = rd.converters.AVOGADRO / float(rd.Inventory({"U-235": 1.0}, "g").numbers()["U-235"])
    M8 = rd.converters.AVOGADRO / float(rd.Inventory({"U-238": 1.0}, "g").numbers()["U-238"])
    # unknowns N5, N8, NO (per m^3): mass balance, UO2 stoichiometry, enrichment by mass
    Mx = np.array([[M5, M8, _MO], [2.0, 2.0, -1.0], [M5 * (1 - w), -M8 * w, 0.0]])
    N5, N8, NO = np.linalg.solve(Mx, np.array([rho * 1000.0 * NA, 0.0, 0.0]))
    Sa = float(np.dot([N5, N8, NO], [sigma_a5, sigma_a8, sigma_aO]))
    # fixed-seed Monte Carlo of free paths: each nuclide's absorption competes as an independent exponential flight
    rng = np.random.default_rng(20261023)
    rates = np.array([N5 * sigma_a5, N8 * sigma_a8, NO * sigma_aO])
    rates = rates[rates > 0]
    paths = np.min(rng.exponential(1.0 / rates[:, None], size=(len(rates), 200000)), axis=0)
    mc, err = float(paths.mean()), float(paths.std() / math.sqrt(paths.size))
    if abs(mc - 1.0 / Sa) > 6 * err:
        raise RuntimeError("Monte Carlo free paths disagree with 1/Sigma_a")
    return {"N5": float(N5), "N8": float(N8), "NO": float(NO), "Sigma_a": Sa, "Sigma_f": float(N5 * sigma_f5),
            "eta": nu * float(N5 * sigma_f5) / Sa, "mfp_a": 1.0 / Sa}


_add("nuc_fuel_cross_sections",
     name="Number densities and macroscopic cross sections of UO2 fuel", topic="neutron physics / cross sections",
     purpose="Atom densities of U-235, U-238 and oxygen in uranium dioxide of stated density and enrichment, its macroscopic absorption and fission cross sections, eta (neutrons per absorption in the fuel) and the absorption mean free path.",
     inputs=[_phys("rho", "density", "density of the UO2 (e.g. 10.4 g/cm^3)"), _numin("w", "enrichment: mass fraction of U-235 in the uranium", pct=True),
             _phys("sigma_a5", "microscopic_cross_section", "U-235 absorption cross section (barns)"),
             _phys("sigma_f5", "microscopic_cross_section", "U-235 fission cross section (barns)"),
             _phys("sigma_a8", "microscopic_cross_section", "U-238 absorption cross section (barns)"),
             _phys("sigma_aO", "microscopic_cross_section", "oxygen absorption cross section (barns)"),
             _numin("nu", "neutrons per fission of U-235")],
     outputs=[_o("N5", "number_density", "U-235 atoms per volume"), _o("N8", "number_density", "U-238 atoms per volume"),
              _o("NO", "number_density", "oxygen atoms per volume"), _o("Sigma_a", "macroscopic_cross_section", "macroscopic absorption cross section"),
              _o("Sigma_f", "macroscopic_cross_section", "macroscopic fission cross section"), _o("eta", "number", "eta = nu Sigma_f/Sigma_a of the fuel"),
              _o("mfp_a", "length", "absorption mean free path 1/Sigma_a")],
     primary="Sigma_a", conditions=["stoichiometric UO2 with U-235 and U-238 only", "atomic masses U-235 235.043928, U-238 238.050787, O 15.999 g/mol"],
     method="atom fraction a5 = (w/M5)/(w/M5 + (1 - w)/M8); N(UO2) = rho N_A/(M_U + 2 M_O); Sigma = sum N_i sigma_i",
     latex=r"N_{25} = \frac{\rho N_A}{M_U + 2M_O}\,a_5,\quad \Sigma_a = \sum_i N_i\sigma_{a,i}",
     run=fuel_cross_sections, reference=_ref_fuel_cross_sections,
     wrong=lambda rho, w, sigma_a5, sigma_f5, sigma_a8, sigma_aO, nu: (lambda Nm: (lambda N5, N8: _out(
         N5=N5, N8=N8, NO=2 * Nm, Sigma_a=N5 * sigma_a5 + N8 * sigma_a8, Sigma_f=N5 * sigma_f5, eta=nu * sigma_f5 / sigma_a5, mfp_a=1 / (N5 * sigma_a5 + N8 * sigma_a8)))(
         Nm * w, Nm * (1 - w)))(rho * NA / (0.238 * 1000 + 2 * _MO) * 1e3),
     rand=lambda rng: (lambda sa5: {"rho": _sig(rng.uniform(9000.0, 10970.0)), "w": round(rng.uniform(0.007, 0.2), 4),
                                    "sigma_a5": sa5 * BARN, "sigma_f5": _sig(sa5 * rng.uniform(0.8, 0.87)) * BARN,
                                    "sigma_a8": _sig(rng.uniform(1.5, 3.0)) * BARN, "sigma_aO": _sig(_lu(rng, 1e-4, 3e-4)) * BARN,
                                    "nu": round(rng.uniform(2.4, 2.5), 3)})(_sig(rng.uniform(400.0, 700.0))),
     phrases=("Fuel cross sections", "UO2 fuel cross sections", "Macroscopic cross sections of UO2"),
     examples=[_ex({"rho": 10400.0, "w": 0.03, "sigma_a5": 680.9e-28, "sigma_f5": 582.6e-28, "sigma_a8": 2.68e-28, "sigma_aO": 0.00019e-28, "nu": 2.43},
                   written={"rho": "10.4 g/cm^3", "w": "3%", "sigma_a5": "680.9 b", "sigma_f5": "582.6 b", "sigma_a8": "2.68 b", "sigma_aO": "0.00019 b"}),
               _ex({"rho": 10970.0, "w": 0.00711, "sigma_a5": 680.9e-28, "sigma_f5": 582.6e-28, "sigma_a8": 2.68e-28, "sigma_aO": 0.00019e-28, "nu": 2.43},
                   written={"rho": "10.97 g/cm^3", "w": "0.711%", "sigma_a5": "680.9 b", "sigma_f5": "582.6 b", "sigma_a8": "2.68 b", "sigma_aO": "0.00019 b"})],
     level="intermediate")



# ------------------------------------------------------------------------------------------------ one-group diffusion
_GEOMS = ("slab", "sphere", "cylinder", "cuboid")
_DIMS = {"slab": ("a",), "sphere": ("R",), "cylinder": ("R", "H"), "cuboid": ("a", "b", "c")}


def _dims(geometry, given):
    _need(geometry in _GEOMS, "geometry must be slab, sphere, cylinder or cuboid")
    need = _DIMS[geometry]
    for k, v in given.items():
        if k in need:
            _need(v is not None, f"the {geometry} needs {', '.join(need)}")
            _pos(f"the dimension {k}", v)
        else:
            _need(v is None, f"{k} is not a dimension of a {geometry} (it needs {', '.join(need)})")
    return [given[k] for k in need]


def _bg2(geometry, dims):
    """Geometric buckling of a bare reactor of EXTRAPOLATED dimensions."""
    if geometry == "slab":
        return (math.pi / dims[0]) ** 2
    if geometry == "sphere":
        return (math.pi / dims[0]) ** 2
    if geometry == "cylinder":
        return (J01 / dims[0]) ** 2 + (math.pi / dims[1]) ** 2
    return sum((math.pi / x) ** 2 for x in dims)


_REF_XI0 = {}


def _ref_shape(geom, xi):
    """Reference: phi(xi), phi'(xi) of the one-group flux equation with buckling 1, phi(0) = 1, phi'(0) = 0, integrated
    numerically (DOP853); sphere and cylinder start just off the centre on their series."""
    if xi == 0:
        return 1.0, 0.0
    if geom == "slab":
        return tuple(_ivp(lambda s, y: [y[1], -y[0]], xi, [1.0, 0.0], atol=[1e-16, 1e-16]))
    s0 = min(1e-4, xi / 2)
    if geom == "sphere":
        y0 = [1 - s0 ** 2 / 6 + s0 ** 4 / 120, -s0 / 3 + s0 ** 3 / 30]
        f = lambda s, y: [y[1], -y[0] - 2 * y[1] / s]
    else:
        y0 = [1 - s0 ** 2 / 4 + s0 ** 4 / 64, -s0 / 2 + s0 ** 3 / 16]
        f = lambda s, y: [y[1], -y[0] - y[1] / s]
    return tuple(_ivp(f, xi, y0, atol=[1e-16, 1e-16], t0=s0))


def _ref_xi0(geom):
    """First zero of the reference flux shape (pi/2 for a slab, measured from the mid-plane; pi for a sphere; j01 for a
    cylinder), found by root-finding on the numerical solution."""
    if geom not in _REF_XI0:
        lo, hi = {"slab": (1.2, 2.0), "cylinder": (2.0, 2.8), "sphere": (2.8, 3.5)}[geom]
        _REF_XI0[geom] = _brentq(lambda x: _ref_shape(geom, x)[0], lo, hi)
    return _REF_XI0[geom]


def _ref_bg2(geometry, dims):
    if geometry in ("slab", "cuboid"):
        return sum((2 * _ref_xi0("slab") / x) ** 2 for x in dims)       # slab half-width a/2 holds the first zero
    if geometry == "sphere":
        return (_ref_xi0("sphere") / dims[0]) ** 2
    return (_ref_xi0("cylinder") / dims[0]) ** 2 + (2 * _ref_xi0("slab") / dims[1]) ** 2


def _group1(D, Sigma_a, nuSigma_f):
    _pos("D, Sigma_a and nu Sigma_f", D, Sigma_a, nuSigma_f)
    return (nuSigma_f - Sigma_a) / D


_EXTRAP_IN = _choice("extrapolation", ("none", "2D", "2.13D"),
                     "extrapolation distance added at each free surface: none, 2D or 2.13D (= 0.71 transport mean free paths)")
_ONEGROUP_IN = [_phys("D", "neutron_diffusion_coefficient", "diffusion coefficient (a length, e.g. cm)"),
                _phys("Sigma_a", "macroscopic_cross_section", "macroscopic absorption cross section"),
                _phys("nuSigma_f", "macroscopic_cross_section", "nu times the macroscopic fission cross section")]


def critical_slab(D, Sigma_a, nuSigma_f, extrapolation):
    Bm2 = _group1(D, Sigma_a, nuSigma_f)
    _need(Bm2 > 0, "k-infinity <= 1: the medium cannot be critical at any size")
    d = EXTRAP[extrapolation.lower()] * D
    ae = math.pi / math.sqrt(Bm2)
    _need(ae - 2 * d > 0, "the extrapolation distance exceeds the critical size")
    return _out(a=ae - 2 * d, a_ext=ae)


def _ref_critical_slab(D, Sigma_a, nuSigma_f, extrapolation):
    Bm = math.sqrt((nuSigma_f - Sigma_a) / D)
    ae = 2 * _ref_xi0("slab") / Bm
    d = EXTRAP[extrapolation.lower()] * D
    return {"a": ae - 2 * d, "a_ext": ae}


def _rand_onegroup(rng):
    D = _sig(_lu(rng, 0.003, 0.02))
    Sa = _sig(_lu(rng, 0.5, 30.0))
    kinf = rng.uniform(1.03, 1.8)
    return {"D": D, "Sigma_a": Sa, "nuSigma_f": _sig(Sa * kinf), "extrapolation": rng.choice(["none", "2D", "2.13D"])}


_add("nuc_critical_slab",
     name="Critical thickness of a bare slab reactor (one group)", topic="reactor physics / criticality of bare reactors",
     purpose="The critical thickness of an infinite bare slab of multiplying material in one-group diffusion theory, the extrapolated thickness and the material buckling.",
     inputs=[*_ONEGROUP_IN, _EXTRAP_IN],
     outputs=[_o("a", "length", "critical physical thickness"), _o("a_ext", "length", "critical extrapolated thickness")],
     primary="a", conditions=["one-group diffusion", "k_inf > 1", "bare slab, flux zero at the extrapolated faces"],
     method="B_m^2 = (nu Sigma_f - Sigma_a)/D; critical when (pi/a_ext)^2 = B_m^2; a = a_ext - 2d",
     latex=r"\left(\frac{\pi}{a_{ext}}\right)^2 = \frac{\nu\Sigma_f-\Sigma_a}{D}",
     run=critical_slab, reference=_ref_critical_slab,
     wrong=lambda D, Sigma_a, nuSigma_f, extrapolation: (lambda Bm2: _out(a=math.pi / math.sqrt(Bm2) / 2, a_ext=math.pi / math.sqrt(Bm2) / 2))(
         (nuSigma_f - Sigma_a) / D),
     rand=_rand_onegroup,
     phrases=("Critical slab", "Bare slab critical thickness", "Critical thickness of a bare slab"),
     examples=[_ex({"D": 0.009, "Sigma_a": 15.0, "nuSigma_f": 18.0, "extrapolation": "2.13D"},
                   written={"D": "0.9 cm", "Sigma_a": "0.15 1/cm", "nuSigma_f": "0.18 1/cm"}),
               _ex({"D": 0.009, "Sigma_a": 15.0, "nuSigma_f": 18.0, "extrapolation": "none"},
                   written={"D": "0.9 cm", "Sigma_a": "0.15 1/cm", "nuSigma_f": "0.18 1/cm"})],
     level="intermediate",
     extra_q=[{"q": "Critical slab: D = 0.9 cm, Sigma_a = 0.15 1/cm, nuSigma_f = 0.12 1/cm, extrapolation = none", "outcome": "INVALID_INPUT"},
              {"q": "Critical slab: D = 0.9 cm, Sigma_a = 0.15 1/cm, nuSigma_f = 0.18 1/cm", "outcome": "NEEDS_INPUT"}])


def critical_sphere(D, Sigma_a, nuSigma_f, extrapolation):
    Bm2 = _group1(D, Sigma_a, nuSigma_f)
    _need(Bm2 > 0, "k-infinity <= 1: the medium cannot be critical at any size")
    d = EXTRAP[extrapolation.lower()] * D
    Re = math.pi / math.sqrt(Bm2)
    _need(Re - d > 0, "the extrapolation distance exceeds the critical radius")
    return _out(R=Re - d, R_ext=Re, V=4.0 / 3.0 * math.pi * (Re - d) ** 3)


def _ref_critical_sphere(D, Sigma_a, nuSigma_f, extrapolation):
    Bm = math.sqrt((nuSigma_f - Sigma_a) / D)
    Re = _ref_xi0("sphere") / Bm
    d = EXTRAP[extrapolation.lower()] * D
    return {"R": Re - d, "R_ext": Re, "V": 4.0 / 3.0 * math.pi * (Re - d) ** 3}


_add("nuc_critical_sphere",
     name="Critical radius of a bare spherical reactor (one group)", topic="reactor physics / criticality of bare reactors",
     purpose="The critical radius (physical and extrapolated) and volume of a bare homogeneous spherical reactor in one-group diffusion theory.",
     inputs=[*_ONEGROUP_IN, _EXTRAP_IN],
     outputs=[_o("R", "length", "critical physical radius"), _o("R_ext", "length", "critical extrapolated radius"),
              _o("V", "volume", "critical physical volume")],
     primary="R", conditions=["one-group diffusion", "k_inf > 1", "bare homogeneous sphere"],
     method="(pi/R_ext)^2 = B_m^2; R = R_ext - d",
     latex=r"\left(\frac{\pi}{R_{ext}}\right)^2 = \frac{\nu\Sigma_f-\Sigma_a}{D}",
     run=critical_sphere, reference=_ref_critical_sphere,
     wrong=lambda D, Sigma_a, nuSigma_f, extrapolation: (lambda Re: _out(R=Re, R_ext=Re, V=4.0 / 3.0 * math.pi * Re ** 3))(J01 / math.sqrt((nuSigma_f - Sigma_a) / D)),
     rand=_rand_onegroup,
     phrases=("Critical sphere", "Bare sphere critical radius", "Critical radius of a bare sphere"),
     examples=[_ex({"D": 0.009, "Sigma_a": 15.0, "nuSigma_f": 18.0, "extrapolation": "2.13D"},
                   written={"D": "0.9 cm", "Sigma_a": "0.15 1/cm", "nuSigma_f": "0.18 1/cm"}),
               _ex({"D": 0.01, "Sigma_a": 10.0, "nuSigma_f": 12.5, "extrapolation": "2D"},
                   written={"D": "1 cm", "Sigma_a": "0.1 1/cm", "nuSigma_f": "0.125 1/cm"})],
     level="intermediate")


def critical_cylinder(D, Sigma_a, nuSigma_f, extrapolation, H):
    Bm2 = _group1(D, Sigma_a, nuSigma_f)
    _need(Bm2 > 0, "k-infinity <= 1: the medium cannot be critical at any size")
    _pos("the height H", H)
    d = EXTRAP[extrapolation.lower()] * D
    He = H + 2 * d
    Br2 = Bm2 - (math.pi / He) ** 2
    _need(Br2 > 0, "this height is too small: no radius makes the cylinder critical")
    Re = J01 / math.sqrt(Br2)
    _need(Re - d > 0, "the extrapolation distance exceeds the critical radius")
    return _out(R=Re - d, R_ext=Re, V=math.pi * (Re - d) ** 2 * H)


def _ref_critical_cylinder(D, Sigma_a, nuSigma_f, extrapolation, H):
    Bm2 = (nuSigma_f - Sigma_a) / D
    d = EXTRAP[extrapolation.lower()] * D
    Bz = 2 * _ref_xi0("slab") / (H + 2 * d)
    Re = _ref_xi0("cylinder") / math.sqrt(Bm2 - Bz * Bz)
    return {"R": Re - d, "R_ext": Re, "V": math.pi * (Re - d) ** 2 * H}


def _rand_cyl(rng):
    d = _rand_onegroup(rng)
    Bm = math.sqrt((d["nuSigma_f"] - d["Sigma_a"]) / d["D"])
    d["H"] = _sig(math.pi / Bm * rng.uniform(1.3, 6.0))
    return d


_add("nuc_critical_cylinder",
     name="Critical radius of a bare cylindrical reactor of given height (one group)", topic="reactor physics / criticality of bare reactors",
     purpose="The critical radius of a bare homogeneous cylinder of stated height in one-group diffusion theory, with its extrapolated radius and the physical volume.",
     inputs=[*_ONEGROUP_IN, _EXTRAP_IN, _phys("H", "length", "physical height of the cylinder")],
     outputs=[_o("R", "length", "critical physical radius"), _o("R_ext", "length", "critical extrapolated radius"),
              _o("V", "volume", "critical physical volume")],
     primary="R", conditions=["one-group diffusion", "the height exceeds the critical slab thickness"],
     method="(j01/R_ext)^2 + (pi/H_ext)^2 = B_m^2, j01 = 2.405 (first zero of J0)",
     latex=r"\left(\frac{2.405}{R_{ext}}\right)^2 + \left(\frac{\pi}{H_{ext}}\right)^2 = B_m^2",
     run=critical_cylinder, reference=_ref_critical_cylinder,
     wrong=lambda D, Sigma_a, nuSigma_f, extrapolation, H: (lambda Bm2: (lambda Re: _out(R=Re, R_ext=Re, V=math.pi * Re * Re * H))(
         math.pi / math.sqrt(max(Bm2 - (math.pi / H) ** 2, 1e-30))))((nuSigma_f - Sigma_a) / D),
     rand=_rand_cyl,
     phrases=("Critical cylinder", "Bare cylinder critical radius", "Critical radius of a bare cylinder"),
     examples=[_ex({"D": 0.009, "Sigma_a": 15.0, "nuSigma_f": 18.0, "extrapolation": "2.13D", "H": 0.5},
                   written={"D": "0.9 cm", "Sigma_a": "0.15 1/cm", "nuSigma_f": "0.18 1/cm", "H": "50 cm"}),
               _ex({"D": 0.01, "Sigma_a": 10.0, "nuSigma_f": 12.5, "extrapolation": "none", "H": 1.0},
                   written={"D": "1 cm", "Sigma_a": "0.1 1/cm", "nuSigma_f": "0.125 1/cm", "H": "1 m"})],
     level="advanced",
     extra_q=[{"q": "Critical cylinder: D = 0.9 cm, Sigma_a = 0.15 1/cm, nuSigma_f = 0.18 1/cm, extrapolation = 2.13D, H = 5 cm", "outcome": "INVALID_INPUT"}])


def critical_cylinder_min(D, Sigma_a, nuSigma_f, extrapolation):
    Bm2 = _group1(D, Sigma_a, nuSigma_f)
    _need(Bm2 > 0, "k-infinity <= 1: the medium cannot be critical at any size")
    Bm = math.sqrt(Bm2)
    d = EXTRAP[extrapolation.lower()] * D
    He = math.sqrt(3.0) * math.pi / Bm
    Re = math.sqrt(1.5) * J01 / Bm
    _need(Re - d > 0, "the extrapolation distance exceeds the critical radius")
    return _out(R=Re - d, H=He - 2 * d, V_ext=math.pi * Re * Re * He, HD=He / (2 * Re))


def _ref_critical_cylinder_min(D, Sigma_a, nuSigma_f, extrapolation):
    Bm2 = (nuSigma_f - Sigma_a) / D
    j, p = _ref_xi0("cylinder"), 2 * _ref_xi0("slab")
    if "dV" not in _REF_XI0:
        import sympy as sp
        Hs, B2s, js, ps = sp.symbols("H B2 j p", positive=True)
        V = sp.pi * js ** 2 / (B2s - (ps / Hs) ** 2) * Hs       # pi R_ext^2 H_ext on the critical curve
        _REF_XI0["dV"] = sp.lambdify((Hs, B2s, js, ps), sp.diff(V, Hs), "math")   # symbolic derivative
    dV = _REF_XI0["dV"]
    Hmin = p / math.sqrt(Bm2)
    He = _brentq(lambda H: dV(H, Bm2, j, p), Hmin * 1.0001, Hmin * 10.0)
    Re = j / math.sqrt(Bm2 - (p / He) ** 2)
    d = EXTRAP[extrapolation.lower()] * D
    return {"R": Re - d, "H": He - 2 * d, "V_ext": math.pi * Re * Re * He, "HD": He / (2 * Re)}


_add("nuc_critical_cylinder_minimum_volume",
     name="Minimum-volume critical bare cylinder (one group)", topic="reactor physics / criticality of bare reactors",
     purpose="The radius and height of the bare cylindrical reactor that is critical with the smallest volume (extrapolated height-to-diameter ratio 0.924), and that volume.",
     inputs=[*_ONEGROUP_IN, _EXTRAP_IN],
     outputs=[_o("R", "length", "physical radius"), _o("H", "length", "physical height"),
              _o("V_ext", "volume", "minimum critical volume (extrapolated dimensions)"), _o("HD", "number", "extrapolated height/diameter")],
     primary="V_ext", conditions=["one-group diffusion", "k_inf > 1"],
     method="minimise pi R^2 H subject to (j01/R)^2 + (pi/H)^2 = B_m^2: H_ext = sqrt(3) pi/B_m, R_ext = sqrt(3/2) j01/B_m",
     latex=r"H_{ext} = \frac{\sqrt3\,\pi}{B_m},\quad R_{ext} = \sqrt{\tfrac32}\,\frac{2.405}{B_m}",
     run=critical_cylinder_min, reference=_ref_critical_cylinder_min,
     wrong=lambda D, Sigma_a, nuSigma_f, extrapolation: (lambda Bm: _out(R=J01 / Bm, H=2 * J01 / Bm, V_ext=2 * math.pi * J01 ** 3 / Bm ** 3, HD=1.0))(
         math.sqrt((nuSigma_f - Sigma_a) / D)),
     rand=_rand_onegroup,
     phrases=("Minimum volume critical cylinder", "Critical cylinder of minimum volume", "Minimum-volume cylinder"),
     examples=[_ex({"D": 0.009, "Sigma_a": 15.0, "nuSigma_f": 18.0, "extrapolation": "none"},
                   written={"D": "0.9 cm", "Sigma_a": "0.15 1/cm", "nuSigma_f": "0.18 1/cm"}),
               _ex({"D": 0.01, "Sigma_a": 10.0, "nuSigma_f": 12.5, "extrapolation": "2.13D"},
                   written={"D": "1 cm", "Sigma_a": "0.1 1/cm", "nuSigma_f": "0.125 1/cm"})],
     level="advanced")


def critical_cuboid(D, Sigma_a, nuSigma_f, extrapolation, a, b):
    Bm2 = _group1(D, Sigma_a, nuSigma_f)
    _need(Bm2 > 0, "k-infinity <= 1: the medium cannot be critical at any size")
    _pos("the sides a and b", a, b)
    d = EXTRAP[extrapolation.lower()] * D
    Bz2 = Bm2 - (math.pi / (a + 2 * d)) ** 2 - (math.pi / (b + 2 * d)) ** 2
    _need(Bz2 > 0, "these two sides are too small: no third side makes the box critical")
    ce = math.pi / math.sqrt(Bz2)
    _need(ce - 2 * d > 0, "the extrapolation distance exceeds the critical size")
    return _out(c=ce - 2 * d, c_ext=ce, V=a * b * (ce - 2 * d))


def _ref_critical_cuboid(D, Sigma_a, nuSigma_f, extrapolation, a, b):
    Bm2 = (nuSigma_f - Sigma_a) / D
    d = EXTRAP[extrapolation.lower()] * D
    p = 2 * _ref_xi0("slab")
    ce = p / math.sqrt(Bm2 - (p / (a + 2 * d)) ** 2 - (p / (b + 2 * d)) ** 2)
    return {"c": ce - 2 * d, "c_ext": ce, "V": a * b * (ce - 2 * d)}


def _rand_cuboid(rng):
    d = _rand_onegroup(rng)
    Bm = math.sqrt((d["nuSigma_f"] - d["Sigma_a"]) / d["D"])
    d["a"] = _sig(math.pi / Bm * rng.uniform(2.0, 6.0))
    d["b"] = _sig(math.pi / Bm * rng.uniform(2.0, 6.0))
    return d


_add("nuc_critical_cuboid",
     name="Critical third side of a bare rectangular reactor (one group)", topic="reactor physics / criticality of bare reactors",
     purpose="The third side that makes a bare rectangular parallelepiped of two stated sides critical in one-group diffusion theory, its extrapolated value and the physical volume.",
     inputs=[*_ONEGROUP_IN, _EXTRAP_IN, _phys("a", "length", "first physical side"), _phys("b", "length", "second physical side")],
     outputs=[_o("c", "length", "critical physical third side"), _o("c_ext", "length", "critical extrapolated third side"),
              _o("V", "volume", "critical physical volume")],
     primary="c", conditions=["one-group diffusion", "the two given sides are large enough"],
     method="(pi/a_ext)^2 + (pi/b_ext)^2 + (pi/c_ext)^2 = B_m^2",
     latex=r"\left(\frac{\pi}{a_{ext}}\right)^2+\left(\frac{\pi}{b_{ext}}\right)^2+\left(\frac{\pi}{c_{ext}}\right)^2 = B_m^2",
     run=critical_cuboid, reference=_ref_critical_cuboid,
     wrong=lambda D, Sigma_a, nuSigma_f, extrapolation, a, b: (lambda Bm2: (lambda ce: _out(c=ce, c_ext=ce, V=a * b * ce))(
         math.pi / math.sqrt(max(Bm2 - (math.pi / a) ** 2, 1e-30))))((nuSigma_f - Sigma_a) / D),
     rand=_rand_cuboid,
     phrases=("Critical cuboid", "Critical rectangular reactor", "Critical parallelepiped"),
     examples=[_ex({"D": 0.009, "Sigma_a": 15.0, "nuSigma_f": 18.0, "extrapolation": "2.13D", "a": 0.6, "b": 0.6},
                   written={"D": "0.9 cm", "Sigma_a": "0.15 1/cm", "nuSigma_f": "0.18 1/cm", "a": "60 cm", "b": "60 cm"}),
               _ex({"D": 0.01, "Sigma_a": 10.0, "nuSigma_f": 12.5, "extrapolation": "none", "a": 1.0, "b": 2.0},
                   written={"D": "1 cm", "Sigma_a": "0.1 1/cm", "nuSigma_f": "0.125 1/cm", "a": "1 m", "b": "2 m"})],
     level="advanced")


_GEOM_IN = _choice("geometry", _GEOMS, "bare reactor shape: slab (a), sphere (R), cylinder (R, H) or cuboid (a, b, c)")
_GDIM_IN = [_phys("a", "length", "slab thickness or first side of a cuboid", required=False),
            _phys("b", "length", "second side of a cuboid", required=False),
            _phys("c", "length", "third side of a cuboid", required=False),
            _phys("R", "length", "radius of a sphere or cylinder", required=False),
            _phys("H", "length", "height of a cylinder", required=False)]


def bare_keff(D, Sigma_a, nuSigma_f, extrapolation, geometry, a=None, b=None, c=None, R=None, H=None):
    _pos("D, Sigma_a and nu Sigma_f", D, Sigma_a, nuSigma_f)
    dims = _dims(geometry, {"a": a, "b": b, "c": c, "R": R, "H": H})
    d = EXTRAP[extrapolation.lower()] * D
    ext = [x + 2 * d for x in dims] if geometry in ("slab", "cuboid") else ([dims[0] + d] if geometry == "sphere" else [dims[0] + d, dims[1] + 2 * d])
    B2 = _bg2(geometry, ext)
    k = nuSigma_f / (Sigma_a + D * B2)
    return _out(B2=B2, k_eff=k, P_NL=Sigma_a / (Sigma_a + D * B2), rho=(k - 1) / k)


def _ref_bare_keff(D, Sigma_a, nuSigma_f, extrapolation, geometry, a=None, b=None, c=None, R=None, H=None):
    dims = [v for v in {"a": a, "b": b, "c": c, "R": R, "H": H}.values() if v is not None]
    dims = [{"a": a, "b": b, "c": c, "R": R, "H": H}[k] for k in _DIMS[geometry]]
    d = EXTRAP[extrapolation.lower()] * D
    ext = [x + 2 * d for x in dims] if geometry in ("slab", "cuboid") else ([dims[0] + d] if geometry == "sphere" else [dims[0] + d, dims[1] + 2 * d])
    B2 = _ref_bg2(geometry, ext)
    # the k-eigenvalue: the flux equation D B^2 phi + Sigma_a phi = (nu Sigma_f/k) phi solved for k by root finding
    k = _brentq(lambda k: (nuSigma_f / k - Sigma_a) / D - B2, 1e-6, 1e3)
    return {"B2": B2, "k_eff": k, "P_NL": k * Sigma_a / nuSigma_f, "rho": (k - 1) / k}


def _rand_bare_keff(rng):
    d = _rand_onegroup(rng)
    g = rng.choice(_GEOMS)
    d["geometry"] = g
    Bm = math.sqrt((d["nuSigma_f"] - d["Sigma_a"]) / d["D"])
    for k in _DIMS[g]:
        d[k] = _sig(math.pi / Bm * rng.uniform(0.5, 4.0))
    return d


_add("nuc_bare_reactor_keff",
     name="k-effective and geometric buckling of a bare reactor (one group)", topic="reactor physics / criticality of bare reactors",
     purpose="The geometric buckling of a bare homogeneous slab, sphere, cylinder or rectangular reactor of stated size and its effective multiplication factor in one-group diffusion theory, with the non-leakage probability and the reactivity.",
     inputs=[*_ONEGROUP_IN, _EXTRAP_IN, _GEOM_IN, *_GDIM_IN],
     outputs=[_o("B2", "buckling", "geometric buckling (extrapolated dimensions)"), _o("k_eff", "number", "effective multiplication factor"),
              _o("P_NL", "number", "non-leakage probability 1/(1 + L^2 B^2)"), _o("rho", "number", "reactivity (k - 1)/k")],
     primary="k_eff", conditions=["one-group diffusion", "bare homogeneous reactor", "the dimensions of the stated geometry only"],
     method="B_g^2 from the shape (pi/a)^2, (pi/R)^2, (2.405/R)^2 + (pi/H)^2, sum (pi/x_i)^2; k = nu Sigma_f/(Sigma_a + D B_g^2)",
     latex=r"k_{eff} = \frac{\nu\Sigma_f}{\Sigma_a + D B_g^2}",
     run=bare_keff, reference=_ref_bare_keff,
     wrong=lambda D, Sigma_a, nuSigma_f, extrapolation, geometry, a=None, b=None, c=None, R=None, H=None: (lambda o: {**o, "k_eff": _r(nuSigma_f / Sigma_a * (1 - D * o["B2"] / Sigma_a))})(
         bare_keff(D, Sigma_a, nuSigma_f, extrapolation, geometry, a, b, c, R, H)),
     rand=_rand_bare_keff,
     phrases=("Bare reactor k-effective", "k-effective of a bare reactor", "Bare reactor keff"),
     examples=[_ex({"D": 0.009, "Sigma_a": 15.0, "nuSigma_f": 18.0, "extrapolation": "2.13D", "geometry": "sphere", "R": 0.12},
                   written={"D": "0.9 cm", "Sigma_a": "0.15 1/cm", "nuSigma_f": "0.18 1/cm", "R": "12 cm"}),
               _ex({"D": 0.01, "Sigma_a": 10.0, "nuSigma_f": 12.5, "extrapolation": "none", "geometry": "cylinder", "R": 0.3, "H": 0.6},
                   written={"D": "1 cm", "Sigma_a": "0.1 1/cm", "nuSigma_f": "0.125 1/cm", "R": "30 cm", "H": "60 cm"})],
     level="intermediate",
     extra_q=[{"q": "Bare reactor k-effective: D = 0.9 cm, Sigma_a = 0.15 1/cm, nuSigma_f = 0.18 1/cm, extrapolation = none, geometry = sphere, R = 12 cm, H = 10 cm", "outcome": "INVALID_INPUT"},
              {"q": "Bare reactor k-effective: D = 0.9 cm, Sigma_a = 0.15 1/cm, nuSigma_f = 0.18 1/cm, extrapolation = none, geometry = cylinder, R = 12 cm", "outcome": "INVALID_INPUT"}])


def two_group_keff(geometry, D1, D2, Sigma_a1, Sigma_12, Sigma_a2, nuSigma_f1, nuSigma_f2, a=None, b=None, c=None, R=None, H=None):
    _pos("D1, D2, Sigma_12 and Sigma_a2", D1, D2, Sigma_12, Sigma_a2)
    _nonneg("Sigma_a1 and the fission cross sections", Sigma_a1, nuSigma_f1, nuSigma_f2)
    _need(nuSigma_f1 + nuSigma_f2 > 0, "there is no fission")
    dims = _dims(geometry, {"a": a, "b": b, "c": c, "R": R, "H": H})
    B2 = _bg2(geometry, dims)
    R1 = D1 * B2 + Sigma_a1 + Sigma_12
    T2 = D2 * B2 + Sigma_a2
    k = (nuSigma_f1 + nuSigma_f2 * Sigma_12 / T2) / R1
    kinf = (nuSigma_f1 + nuSigma_f2 * Sigma_12 / Sigma_a2) / (Sigma_a1 + Sigma_12)
    return _out(B2=B2, k_eff=k, k_inf=kinf, flux_ratio=Sigma_12 / T2)


def _ref_two_group_keff(geometry, D1, D2, Sigma_a1, Sigma_12, Sigma_a2, nuSigma_f1, nuSigma_f2, a=None, b=None, c=None, R=None, H=None):
    import numpy as np
    from scipy.linalg import eig
    dims = [{"a": a, "b": b, "c": c, "R": R, "H": H}[k] for k in _DIMS[geometry]]
    B2 = _ref_bg2(geometry, dims)

    def keig(B2):
        Am = np.array([[D1 * B2 + Sigma_a1 + Sigma_12, 0.0], [-Sigma_12, D2 * B2 + Sigma_a2]])
        Fm = np.array([[nuSigma_f1, nuSigma_f2], [0.0, 0.0]])
        w, v = eig(Fm, Am)                         # F phi = k A phi
        i = int(np.argmax(np.where(np.isfinite(w.real), w.real, -np.inf)))
        return float(w[i].real), v[:, i].real
    k, v = keig(B2)
    kinf, _ = keig(0.0)
    return {"B2": B2, "k_eff": k, "k_inf": kinf, "flux_ratio": float(v[1] / v[0])}


def _rand_two_group(rng):
    g = rng.choice(_GEOMS)
    d = {"geometry": g, "D1": _sig(rng.uniform(0.01, 0.02)), "D2": _sig(rng.uniform(0.002, 0.01)),
         "Sigma_a1": _sig(rng.uniform(0.2, 1.5)), "Sigma_12": _sig(rng.uniform(1.0, 3.0)), "Sigma_a2": _sig(rng.uniform(5.0, 12.0)),
         "nuSigma_f1": _sig(rng.uniform(0.0, 1.0)), "nuSigma_f2": _sig(rng.uniform(8.0, 20.0))}
    for k in _DIMS[g]:
        d[k] = _sig(rng.uniform(0.3, 4.0))
    return d


_add("nuc_two_group_bare_keff",
     name="k-effective of a bare reactor in two-group diffusion theory", topic="reactor physics / two-group theory",
     purpose="The effective and infinite multiplication factors of a bare homogeneous reactor in two-group (fast and thermal) diffusion theory, with fission in both groups and downscatter from fast to thermal, and the thermal-to-fast flux ratio of the fundamental mode.",
     inputs=[_GEOM_IN, _phys("D1", "neutron_diffusion_coefficient", "fast-group diffusion coefficient"),
             _phys("D2", "neutron_diffusion_coefficient", "thermal-group diffusion coefficient"),
             _phys("Sigma_a1", "macroscopic_cross_section", "fast-group absorption cross section"),
             _phys("Sigma_12", "macroscopic_cross_section", "downscatter (removal) cross section from fast to thermal"),
             _phys("Sigma_a2", "macroscopic_cross_section", "thermal-group absorption cross section"),
             _phys("nuSigma_f1", "macroscopic_cross_section", "fast-group nu Sigma_f"),
             _phys("nuSigma_f2", "macroscopic_cross_section", "thermal-group nu Sigma_f"),
             *[dict(i, meaning=i["meaning"] + " (extrapolated)") for i in _GDIM_IN]],
     outputs=[_o("B2", "buckling", "geometric buckling"), _o("k_eff", "number", "effective multiplication factor"),
              _o("k_inf", "number", "infinite multiplication factor"), _o("flux_ratio", "number", "thermal/fast flux ratio of the fundamental mode")],
     primary="k_eff", conditions=["two-group diffusion, no upscatter", "all fission neutrons born fast", "extrapolated dimensions"],
     method="k = [nu Sigma_f1 + nu Sigma_f2 Sigma_12/(D2 B^2 + Sigma_a2)] / (D1 B^2 + Sigma_a1 + Sigma_12)",
     latex=r"k = \frac{\nu\Sigma_{f1} + \nu\Sigma_{f2}\frac{\Sigma_{12}}{D_2B^2+\Sigma_{a2}}}{D_1B^2+\Sigma_{a1}+\Sigma_{12}}",
     run=two_group_keff, reference=_ref_two_group_keff,
     wrong=lambda geometry, D1, D2, Sigma_a1, Sigma_12, Sigma_a2, nuSigma_f1, nuSigma_f2, a=None, b=None, c=None, R=None, H=None: (lambda o: {
         **o, "k_eff": _r((nuSigma_f1 + nuSigma_f2) / (Sigma_a1 + Sigma_a2 + (D1 + D2) * o["B2"]))})(
         two_group_keff(geometry, D1, D2, Sigma_a1, Sigma_12, Sigma_a2, nuSigma_f1, nuSigma_f2, a, b, c, R, H)),
     rand=_rand_two_group,
     phrases=("Two-group k-effective", "Two-group bare reactor", "Two-group keff"),
     examples=[_ex({"geometry": "cylinder", "D1": 0.0143, "D2": 0.0037, "Sigma_a1": 1.0, "Sigma_12": 1.8, "Sigma_a2": 8.0, "nuSigma_f1": 0.5, "nuSigma_f2": 13.5, "R": 1.7, "H": 3.7},
                   written={"D1": "1.43 cm", "D2": "0.37 cm", "Sigma_a1": "0.01 1/cm", "Sigma_12": "0.018 1/cm", "Sigma_a2": "0.08 1/cm",
                            "nuSigma_f1": "0.005 1/cm", "nuSigma_f2": "0.135 1/cm", "R": "170 cm", "H": "370 cm"}),
               _ex({"geometry": "sphere", "D1": 0.012, "D2": 0.004, "Sigma_a1": 0.5, "Sigma_12": 2.0, "Sigma_a2": 10.0, "nuSigma_f1": 0.0, "nuSigma_f2": 14.0, "R": 1.0},
                   written={"D1": "1.2 cm", "D2": "0.4 cm", "Sigma_a1": "0.005 1/cm", "Sigma_12": "0.02 1/cm", "Sigma_a2": "0.1 1/cm",
                            "nuSigma_f1": "0 1/cm", "nuSigma_f2": "0.14 1/cm", "R": "1 m"})],
     level="expert")


def reflected_slab(D_c, Sigma_ac, nuSigma_fc, D_r, Sigma_ar, T_r, extrapolation):
    Bm2 = _group1(D_c, Sigma_ac, nuSigma_fc)
    _need(Bm2 > 0, "k-infinity of the core <= 1: it cannot be critical")
    _pos("D_r, Sigma_ar and the reflector thickness", D_r, Sigma_ar, T_r)
    B = math.sqrt(Bm2)
    Lr = math.sqrt(D_r / Sigma_ar)
    d = EXTRAP[extrapolation.lower()] * D_r
    q = D_r / (D_c * B * Lr) / math.tanh((T_r + d) / Lr)
    a = 2.0 / B * math.atan(q)
    return _out(a=a, delta=(math.pi / B - a) / 2.0)


def _ref_reflected_slab(D_c, Sigma_ac, nuSigma_fc, D_r, Sigma_ar, T_r, extrapolation):
    B2 = (nuSigma_fc - Sigma_ac) / D_c
    d = EXTRAP[extrapolation.lower()] * D_r
    k2 = Sigma_ar / D_r

    # reflector phi'' = phi/L^2 is linear: its two fundamental solutions across T_r + d are integrated once
    Y1 = _ivp(lambda x, y: [y[1], k2 * y[0]], T_r + d, [1.0, 0.0], atol=[1e-16, 1e-16])[0]
    Y2 = _ivp(lambda x, y: [y[1], k2 * y[0]], T_r + d, [0.0, 1.0], atol=[1e-16, 1e-16])[0]

    def outer(a):
        # two-region shooting from the mid-plane: core phi'' = -B^2 phi, then the reflector, with phi and the current
        # D phi' continuous at the interface; returns phi at the outer extrapolated boundary
        ph, dph = _ivp(lambda x, y: [y[1], -B2 * y[0]], a / 2, [1.0, 0.0], atol=[1e-16, 1e-16])
        return ph * Y1 + dph * D_c / D_r * Y2
    abare = 2 * _ref_xi0("slab") / math.sqrt(B2)
    a = _brentq(outer, abare * 1e-6, abare * (1 - 1e-9))
    return {"a": a, "delta": (abare - a) / 2.0}


def _rand_reflected(rng):
    d = _rand_onegroup(rng)
    return {"D_c": d["D"], "Sigma_ac": d["Sigma_a"], "nuSigma_fc": d["nuSigma_f"], "D_r": _sig(_lu(rng, 0.002, 0.02)),
            "Sigma_ar": _sig(_lu(rng, 0.01, 5.0)), "T_r": _sig(_lu(rng, 0.01, 0.5)), "extrapolation": d["extrapolation"]}


_add("nuc_reflected_slab_critical",
     name="Critical thickness of a reflected slab reactor and reflector savings (one group)", topic="reactor physics / reflected reactors",
     purpose="The critical core thickness of a slab reactor with a reflector of stated thickness on both faces (one-group diffusion in core and reflector) and the reflector savings compared with the bare core.",
     inputs=[_phys("D_c", "neutron_diffusion_coefficient", "core diffusion coefficient"),
             _phys("Sigma_ac", "macroscopic_cross_section", "core absorption cross section"),
             _phys("nuSigma_fc", "macroscopic_cross_section", "core nu Sigma_f"),
             _phys("D_r", "neutron_diffusion_coefficient", "reflector diffusion coefficient"),
             _phys("Sigma_ar", "macroscopic_cross_section", "reflector absorption cross section"),
             _phys("T_r", "length", "reflector thickness on each face"), _EXTRAP_IN],
     outputs=[_o("a", "length", "critical core thickness"), _o("delta", "length", "reflector savings per face (bare extrapolated half-thickness minus reflected half-thickness)")],
     primary="a", conditions=["one-group diffusion", "non-multiplying reflector", "extrapolation distance of the reflector at its outer face"],
     method="D_c B tan(B a/2) = (D_r/L_r) coth((T_r + d)/L_r), L_r^2 = D_r/Sigma_ar",
     latex=r"D_cB\tan\frac{Ba}{2} = \frac{D_r}{L_r}\coth\frac{T_r+d}{L_r}",
     run=reflected_slab, reference=_ref_reflected_slab,
     wrong=lambda D_c, Sigma_ac, nuSigma_fc, D_r, Sigma_ar, T_r, extrapolation: (lambda o: {**o, "a": _r(o["a"] + 2 * T_r), "delta": _r(o["delta"] - T_r)})(
         reflected_slab(D_c, Sigma_ac, nuSigma_fc, D_r, Sigma_ar, T_r, extrapolation)),
     rand=_rand_reflected,
     phrases=("Reflected slab critical", "Reflected slab reactor", "Reflector savings"),
     examples=[_ex({"D_c": 0.009, "Sigma_ac": 15.0, "nuSigma_fc": 18.0, "D_r": 0.0016, "Sigma_ar": 1.97, "T_r": 0.2, "extrapolation": "2.13D"},
                   written={"D_c": "0.9 cm", "Sigma_ac": "0.15 1/cm", "nuSigma_fc": "0.18 1/cm", "D_r": "0.16 cm", "Sigma_ar": "0.0197 1/cm", "T_r": "20 cm"}),
               _ex({"D_c": 0.01, "Sigma_ac": 10.0, "nuSigma_fc": 12.5, "D_r": 0.009, "Sigma_ar": 0.03, "T_r": 0.5, "extrapolation": "none"},
                   written={"D_c": "1 cm", "Sigma_ac": "0.1 1/cm", "nuSigma_fc": "0.125 1/cm", "D_r": "0.9 cm", "Sigma_ar": "0.0003 1/cm", "T_r": "50 cm"})],
     level="expert")


def exponential_pile(z, phi, a, b):
    _need(len(z) == len(phi) and len(z) >= 3, "give at least three positions with their fluxes")
    _need(all(_isnum(v) for v in z) and len(set(z)) == len(z), "positions must be distinct numbers")
    _need(all(_isnum(p) and p > 0 for p in phi), "fluxes must be positive")
    _pos("the transverse sides", a, b)
    zs = [v / 100.0 for v in z]
    n = len(zs)
    y = [math.log(p) for p in phi]
    mz, my = math.fsum(zs) / n, math.fsum(y) / n
    s = math.fsum((u - mz) * (w - my) for u, w in zip(zs, y)) / math.fsum((u - mz) ** 2 for u in zs)
    g = -s
    L2inv = g * g - (math.pi / a) ** 2 - (math.pi / b) ** 2
    _need(g > 0 and L2inv > 0, "the measured relaxation is too slow for these transverse dimensions: no diffusion length")
    return _out(gamma=g, L=1.0 / math.sqrt(L2inv), L2=1.0 / L2inv)


def _ref_exponential_pile(z, phi, a, b):
    import numpy as np
    p = np.polyfit(np.array(z, dtype=float) / 100.0, np.log(np.array(phi, dtype=float)), 1)
    g = -float(p[0])
    L2inv = g * g - (math.pi / a) ** 2 - (math.pi / b) ** 2
    return {"gamma": g, "L": 1.0 / math.sqrt(L2inv), "L2": 1.0 / L2inv}


def _rand_pile(rng):
    a, b = _sig(rng.uniform(1.0, 3.0)), _sig(rng.uniform(1.0, 3.0))
    L = rng.uniform(0.02, 0.6)
    g = math.sqrt(1 / L ** 2 + (math.pi / a) ** 2 + (math.pi / b) ** 2)
    n = rng.randint(3, 10)
    z = sorted(rng.sample(range(10, 200), n))
    phi = [_sig(1e5 * math.exp(-g * v / 100.0) * rng.uniform(0.995, 1.005)) for v in z]
    return {"z": [float(v) for v in z], "phi": phi, "a": a, "b": b}


_add("nuc_exponential_pile",
     name="Diffusion length from an exponential pile measurement", topic="neutron physics / diffusion",
     purpose="The thermal diffusion length of a moderator from fluxes measured along the axis of an exponential pile (a source at the bottom of a prism of extrapolated sides a and b): least-squares relaxation constant gamma, then L from gamma^2 = 1/L^2 + (pi/a)^2 + (pi/b)^2.",
     inputs=[_vecin("z", "heights of the measurements along the axis, in cm (far from the source and the top)", 3, 100, unit="cm"),
             _vecin("phi", "measured fluxes or detector readings at those heights (any consistent unit)", 3, 100),
             _phys("a", "length", "first extrapolated transverse side"), _phys("b", "length", "second extrapolated transverse side")],
     outputs=[_o("gamma", "inverse_length", "fitted relaxation constant"), _o("L", "length", "diffusion length"), _o("L2", "area", "diffusion area L^2")],
     primary="L", conditions=["fundamental transverse mode only (points far from the source)", "an effectively infinite pile (points far from the top)"],
     method="least squares of ln phi = c - gamma z; 1/L^2 = gamma^2 - (pi/a)^2 - (pi/b)^2",
     latex=r"\gamma^2 = \frac{1}{L^2} + \left(\frac{\pi}{a}\right)^2 + \left(\frac{\pi}{b}\right)^2",
     run=exponential_pile, reference=_ref_exponential_pile,
     wrong=lambda z, phi, a, b: (lambda g: _out(gamma=g, L=1 / g, L2=1 / g ** 2))(math.log(phi[0] / phi[-1]) / ((z[-1] - z[0]) / 100.0)),
     rand=_rand_pile,
     phrases=("Exponential pile", "Diffusion length from an exponential pile", "Exponential pile diffusion length"),
     examples=[_ex({"z": [20.0, 40.0, 60.0, 80.0, 100.0], "phi": [10000.0, 6200.0, 3850.0, 2390.0, 1480.0], "a": 2.0, "b": 2.0},
                   written={"a": "2 m", "b": "2 m"}),
               _ex({"z": [30.0, 50.0, 70.0], "phi": [500.0, 254.0, 129.0], "a": 1.5, "b": 1.8}, written={"a": "150 cm", "b": "180 cm"})],
     level="advanced")


def plane_source_slab(S, D, Sigma_a, a, x):
    _pos("S, D, Sigma_a and the slab thickness", S, D, Sigma_a, a)
    _need(_isnum(x) and abs(x) <= a / 2, "the position must lie inside the slab (|x| <= a/2)")
    L = math.sqrt(D / Sigma_a)
    h = a / (2 * L)
    with _mp.workdps(30):
        phi = float(_mpf(S) * L / (2 * D) * _mp.sinh((_mpf(a) / 2 - abs(_mpf(x))) / L) / _mp.cosh(h))
        phi0 = float(_mpf(S) * L / (2 * D) * _mp.tanh(h))
        leak = float(1 / _mp.cosh(h))
    return _out(phi=phi, phi0=phi0, leak=leak)


def _ref_plane_source_slab(S, D, Sigma_a, a, x):
    L = math.sqrt(D / Sigma_a)
    h = a / (2 * L)
    # shoot inward from the extrapolated face (phi = 0) in sigma = (a/2 - |x|)/L, where the solution grows (no
    # cancellation): u'' = u, u(0) = 0, u'(0) = 1; phi = K u, with K fixed by the source condition -D phi'(0+) = S/2
    w = lambda s: _ivp(lambda t, y: [y[1], y[0]], s, [0.0, 1.0], atol=[1e-30, 1e-30]) if s > 0 else [0.0, 1.0]
    uh = w(h)
    K = S * L / (2 * D * uh[1])
    phi = K * w(h - abs(x) / L)[0]
    return {"phi": phi, "phi0": K * uh[0], "leak": 2 * D * K / (L * S)}


_add("nuc_plane_source_slab_flux",
     name="Flux from a plane neutron source at the centre of a slab", topic="neutron physics / diffusion",
     purpose="The one-group thermal flux at a position in a non-multiplying slab of stated (extrapolated) thickness with a plane source of neutrons at its mid-plane, the flux at the source plane and the fraction of neutrons that leak from the two faces.",
     inputs=[_phys("S", "surface_source_strength", "source strength per unit area of the plane (neutrons/(m^2 s))"),
             _phys("D", "neutron_diffusion_coefficient", "diffusion coefficient"), _phys("Sigma_a", "macroscopic_cross_section", "absorption cross section"),
             _phys("a", "length", "extrapolated slab thickness (source at the centre)"), _phys("x", "length", "distance from the source plane")],
     outputs=[_o("phi", "neutron_flux", "flux at x"), _o("phi0", "neutron_flux", "flux at the source plane"),
              _o("leak", "number", "fraction of source neutrons leaking from the two faces")],
     primary="phi", conditions=["one-group diffusion in a non-multiplying medium", "flux zero at the extrapolated faces"],
     method="phi(x) = (S L/(2D)) sinh((a/2 - |x|)/L)/cosh(a/(2L)); leakage 1/cosh(a/(2L))",
     latex=r"\phi(x) = \frac{SL}{2D}\,\frac{\sinh\left[(a/2-|x|)/L\right]}{\cosh\left[a/(2L)\right]}",
     run=plane_source_slab, reference=_ref_plane_source_slab,
     wrong=lambda S, D, Sigma_a, a, x: (lambda L: _out(phi=S * L / (2 * D) * math.exp(-abs(x) / L), phi0=S * L / (2 * D), leak=0.0))(math.sqrt(D / Sigma_a)),
     rand=lambda rng: (lambda L, a: {"S": _sig(_lu(rng, 1e8, 1e14)), "D": _sig(L * L * 10.0), "Sigma_a": 10.0, "a": a,
                                     "x": _sig(rng.uniform(-0.5, 0.5) * a)})(rng.uniform(0.01, 0.6), _sig(_lu(rng, 0.05, 4.0))),
     phrases=("Plane source in a slab", "Plane source slab flux", "Flux from a plane source"),
     examples=[_ex({"S": 1e12, "D": 0.0016, "Sigma_a": 1.97, "a": 1.0, "x": 0.1},
                   written={"S": "1e8 1/(cm^2*s)", "D": "0.16 cm", "Sigma_a": "0.0197 1/cm", "a": "1 m", "x": "10 cm"}),
               _ex({"S": 1e10, "D": 0.009, "Sigma_a": 0.03, "a": 2.0, "x": 0.0},
                   written={"S": "1e10 1/(m^2*s)", "D": "0.9 cm", "Sigma_a": "0.03 1/m", "a": "2 m", "x": "0 m"})],
     level="advanced")


def point_source_sphere(S, D, Sigma_a, R, r):
    _pos("S, D, Sigma_a, the radius and the position", S, D, Sigma_a, R, r)
    _need(r < R, "the point must lie inside the sphere (r < R)")
    L = math.sqrt(D / Sigma_a)
    with _mp.workdps(30):
        phi = float(_mpf(S) * _mp.sinh((_mpf(R) - r) / L) / (4 * _mp.pi * D * r * _mp.sinh(_mpf(R) / L)))
        leak = float((_mpf(R) / L) / _mp.sinh(_mpf(R) / L))
    phi_inf = S * math.exp(-r / L) / (4 * math.pi * D * r)
    return _out(phi=phi, leak=leak, phi_inf=phi_inf)


def _ref_point_source_sphere(S, D, Sigma_a, R, r):
    L = math.sqrt(D / Sigma_a)
    # u = r phi solves u'' = u/L^2 with u(R) = 0 and u(0) = S/(4 pi D): shoot inward from the surface in
    # sigma = (R - r)/L (u(0) = 0, u'(0) = 1, growing inward), then scale to the source condition at the centre
    w = lambda s: _ivp(lambda t, y: [y[1], y[0]], s, [0.0, 1.0], atol=[1e-30, 1e-30])
    u0 = S / (4 * math.pi * D)
    K = u0 / w(R / L)[0]
    phi = K * w((R - r) / L)[0] / r
    leak = 4 * math.pi * R * D * K / (L * S)          # J(R) = -D dphi/dr = D K/(L R)
    inf = _ivp(lambda t, y: [-y[0]], r / L, [1.0], atol=[1e-300])[0]
    return {"phi": phi, "leak": leak, "phi_inf": u0 * inf / r}


_add("nuc_point_source_sphere_flux",
     name="Flux from a point neutron source at the centre of a sphere", topic="neutron physics / diffusion",
     purpose="The one-group thermal flux at a radius in a non-multiplying sphere (extrapolated radius R) with an isotropic point source at its centre, the fraction of neutrons that leak out, and the flux the same source would give in an infinite medium.",
     inputs=[_phys("S", "emission_rate", "source strength (neutrons per second)"),
             _phys("D", "neutron_diffusion_coefficient", "diffusion coefficient"), _phys("Sigma_a", "macroscopic_cross_section", "absorption cross section"),
             _phys("R", "length", "extrapolated radius of the sphere"), _phys("r", "length", "distance from the source")],
     outputs=[_o("phi", "neutron_flux", "flux at r"), _o("leak", "number", "fraction of source neutrons leaking out"),
              _o("phi_inf", "neutron_flux", "flux at r in an infinite medium")],
     primary="phi", conditions=["one-group diffusion in a non-multiplying medium", "0 < r < R"],
     method="phi(r) = S sinh((R - r)/L)/(4 pi D r sinh(R/L)); leakage (R/L)/sinh(R/L); infinite medium S e^(-r/L)/(4 pi D r)",
     latex=r"\phi(r) = \frac{S}{4\pi D r}\,\frac{\sinh[(R-r)/L]}{\sinh(R/L)}",
     run=point_source_sphere, reference=_ref_point_source_sphere,
     wrong=lambda S, D, Sigma_a, R, r: (lambda L: _out(phi=S * math.exp(-r / L) / (4 * math.pi * D * r), leak=math.exp(-R / L), phi_inf=S / (4 * math.pi * D * r)))(math.sqrt(D / Sigma_a)),
     rand=lambda rng: (lambda L, R: {"S": _sig(_lu(rng, 1e6, 1e12)), "D": _sig(L * L * 10.0), "Sigma_a": 10.0, "R": R,
                                     "r": _sig(R * rng.uniform(0.02, 0.95))})(rng.uniform(0.01, 0.6), _sig(_lu(rng, 0.05, 4.0))),
     phrases=("Point source in a sphere", "Point source sphere flux", "Flux from a point source in a sphere"),
     examples=[_ex({"S": 1e9, "D": 0.0016, "Sigma_a": 1.97, "R": 0.5, "r": 0.1},
                   written={"S": "1e9 1/s", "D": "0.16 cm", "Sigma_a": "0.0197 1/cm", "R": "50 cm", "r": "10 cm"}),
               _ex({"S": 1e7, "D": 0.009, "Sigma_a": 0.03, "R": 1.0, "r": 0.5},
                   written={"S": "1e7 1/s", "D": "0.9 cm", "Sigma_a": "0.03 1/m", "R": "1 m", "r": "50 cm"})],
     level="advanced")


def flux_shape(geometry, a=None, R=None, H=None, x=None, r=None, z=None):
    _need(geometry in ("slab", "sphere", "cylinder"), "geometry must be slab, sphere or cylinder")
    if geometry == "slab":
        _need(R is None and H is None and r is None and z is None, "a slab takes a and x only")
        _pos("the thickness a", a)
        _need(x is not None and abs(x) <= a / 2, "x must lie inside the slab (|x| <= a/2)")
        return _out(ratio=math.cos(math.pi * x / a) + 0.0, B2=(math.pi / a) ** 2)
    if geometry == "sphere":
        _need(a is None and H is None and x is None and z is None, "a sphere takes R and r only")
        _pos("the radius R", R)
        _need(r is not None and 0 <= r <= R, "r must lie inside the sphere")
        u = math.pi * r / R
        return _out(ratio=1.0 if r == 0 else math.sin(u) / u, B2=(math.pi / R) ** 2)
    _need(a is None and x is None, "a cylinder takes R, H, r and z")
    _pos("the radius and height", R, H)
    _need(r is not None and 0 <= r <= R and z is not None and abs(z) <= H / 2, "r and z must lie inside the cylinder")
    with _mp.workdps(20):
        j0 = float(_mp.besselj(0, J01 * r / R))
    return _out(ratio=j0 * math.cos(math.pi * z / H), B2=(J01 / R) ** 2 + (math.pi / H) ** 2)


def _ref_flux_shape(geometry, a=None, R=None, H=None, x=None, r=None, z=None):
    p = _ref_xi0("slab")
    if geometry == "slab":
        return {"ratio": _ref_shape("slab", p * abs(x) / (a / 2))[0], "B2": (2 * p / a) ** 2}
    if geometry == "sphere":
        q = _ref_xi0("sphere")
        return {"ratio": _ref_shape("sphere", q * r / R)[0], "B2": (q / R) ** 2}
    q = _ref_xi0("cylinder")
    return {"ratio": _ref_shape("cylinder", q * r / R)[0] * _ref_shape("slab", p * abs(z) / (H / 2))[0],
            "B2": (q / R) ** 2 + (2 * p / H) ** 2}


def _rand_flux_shape(rng):
    g = rng.choice(["slab", "sphere", "cylinder"])
    if g == "slab":
        a = _sig(_lu(rng, 0.1, 5.0))
        return {"geometry": g, "a": a, "x": _sig(a * rng.uniform(-0.48, 0.48))}
    R = _sig(_lu(rng, 0.1, 5.0))
    if g == "sphere":
        return {"geometry": g, "R": R, "r": _sig(R * rng.uniform(0.01, 0.97))}
    H = _sig(_lu(rng, 0.1, 5.0))
    return {"geometry": g, "R": R, "H": H, "r": _sig(R * rng.uniform(0.01, 0.97)), "z": _sig(H * rng.uniform(-0.48, 0.48))}


_add("nuc_flux_shape",
     name="Fundamental-mode flux shape of a bare reactor at a point", topic="reactor physics / flux shapes",
     purpose="The neutron flux at a point of a bare slab, sphere or finite cylinder relative to its maximum at the centre (cosine, sin(x)/x, J0 times cosine) and the geometric buckling, for extrapolated dimensions.",
     inputs=[_choice("geometry", ("slab", "sphere", "cylinder"), "shape: slab (a, x), sphere (R, r) or cylinder (R, H, r, z)"),
             _phys("a", "length", "extrapolated slab thickness", required=False), _phys("R", "length", "extrapolated radius", required=False),
             _phys("H", "length", "extrapolated cylinder height", required=False),
             _phys("x", "length", "distance from the slab mid-plane", required=False), _phys("r", "length", "distance from the centre or axis", required=False),
             _phys("z", "length", "height above the cylinder mid-plane", required=False)],
     outputs=[_o("ratio", "number", "phi/phi_max at the point"), _o("B2", "buckling", "geometric buckling")],
     primary="ratio", conditions=["fundamental mode of a bare homogeneous reactor", "extrapolated dimensions", "the point inside the reactor"],
     method="slab cos(pi x/a); sphere sin(pi r/R)/(pi r/R); cylinder J0(2.405 r/R) cos(pi z/H)",
     latex=r"\frac{\phi}{\phi_{max}} = J_0\!\left(\frac{2.405\,r}{R}\right)\cos\frac{\pi z}{H}",
     run=flux_shape, reference=_ref_flux_shape,
     wrong=lambda geometry, a=None, R=None, H=None, x=None, r=None, z=None: (lambda o: {**o, "ratio": _r(1 - o["ratio"] / 2)})(flux_shape(geometry, a, R, H, x, r, z)),
     rand=_rand_flux_shape,
     phrases=("Flux shape", "Fundamental mode flux shape", "Bare reactor flux shape"),
     examples=[_ex({"geometry": "cylinder", "R": 1.5, "H": 3.0, "r": 0.75, "z": 0.5}, written={"R": "1.5 m", "H": "3 m", "r": "75 cm", "z": "50 cm"}),
               _ex({"geometry": "slab", "a": 1.0, "x": 0.25}, written={"a": "1 m", "x": "25 cm"}),
               _ex({"geometry": "sphere", "R": 0.5, "r": 0.25}, written={"R": "50 cm", "r": "25 cm"})],
     level="intermediate")


def core_power(geometry, Sigma_f, E_f, phi_max=None, P=None, R=None, H=None, a=None, b=None, c=None):
    _need(geometry in ("sphere", "cylinder", "cuboid"), "geometry must be sphere, cylinder or cuboid")
    _pos("Sigma_f and the energy per fission", Sigma_f, E_f)
    _need((phi_max is None) != (P is None), "give either the peak flux phi_max or the power P (one of them)")
    dims = _dims(geometry, {"a": a, "b": b, "c": c, "R": R, "H": H})
    if geometry == "sphere":
        V, f = 4.0 / 3.0 * math.pi * dims[0] ** 3, 3.0 / math.pi ** 2
    elif geometry == "cylinder":
        with _mp.workdps(20):
            j1 = float(_mp.besselj(1, J01))
        V, f = math.pi * dims[0] ** 2 * dims[1], 2 * j1 / J01 * 2 / math.pi
    else:
        V, f = dims[0] * dims[1] * dims[2], (2 / math.pi) ** 3
    if P is None:
        _pos("the peak flux", phi_max)
        P = E_f * Sigma_f * phi_max * f * V
    else:
        _pos("the power", P)
        phi_max = P / (E_f * Sigma_f * f * V)
    return _out(P=P, phi_max=phi_max, phi_avg=phi_max * f, peaking=1.0 / f)


def _ref_core_power(geometry, Sigma_f, E_f, phi_max=None, P=None, R=None, H=None, a=None, b=None, c=None):
    from scipy.special import j0, jn_zeros
    if geometry == "sphere":
        V = 4.0 / 3.0 * math.pi * R ** 3
        I = _quad(lambda r: (math.sin(math.pi * r / R) / (math.pi * r / R) if r > 0 else 1.0) * 4 * math.pi * r * r, 0.0, R)
    elif geometry == "cylinder":
        j = float(jn_zeros(0, 1)[0])
        V = math.pi * R * R * H
        I = _quad(lambda r: float(j0(j * r / R)) * 2 * math.pi * r, 0.0, R) * _quad(lambda z: math.cos(math.pi * z / H), -H / 2, H / 2)
    else:
        V = a * b * c
        I = 1.0
        for s in (a, b, c):
            I *= _quad(lambda x, s=s: math.cos(math.pi * x / s), -s / 2, s / 2)
    f = I / V
    if P is None:
        P = E_f * Sigma_f * phi_max * I
    else:
        phi_max = P / (E_f * Sigma_f * I)
    return {"P": P, "phi_max": phi_max, "phi_avg": phi_max * f, "peaking": 1.0 / f}


def _rand_core_power(rng):
    g = rng.choice(["sphere", "cylinder", "cuboid"])
    d = {"geometry": g, "Sigma_f": _sig(_lu(rng, 0.05, 20.0)), "E_f": _sig(rng.uniform(190.0, 210.0)) * MEV}
    for k in _DIMS[g]:
        d[k] = _sig(_lu(rng, 0.2, 6.0))
    if rng.random() < 0.5:
        d["phi_max"] = _sig(_lu(rng, 1e15, 1e19))
    else:
        d["P"] = _sig(_lu(rng, 1e5, 4e9))
    return d


_add("nuc_core_power_from_flux",
     name="Reactor power from the peak flux (or peak flux from power) of a bare core", topic="reactor physics / power and flux shapes",
     purpose="The thermal power of a bare homogeneous sphere, cylinder or rectangular core from its peak thermal flux (or the peak flux from the power), with the average flux, the peak-to-average ratio and the volume, for the fundamental-mode flux shape.",
     inputs=[_choice("geometry", ("sphere", "cylinder", "cuboid"), "core shape: sphere (R), cylinder (R, H) or cuboid (a, b, c)"),
             _phys("Sigma_f", "macroscopic_cross_section", "macroscopic fission cross section"),
             _phys("E_f", "energy", "recoverable energy per fission (e.g. 200 MeV)"),
             _phys("phi_max", "neutron_flux", "peak (centre) thermal flux (give this or P)", required=False),
             _phys("P", "power", "thermal power (give this or phi_max)", required=False),
             _phys("R", "length", "extrapolated radius", required=False), _phys("H", "length", "extrapolated cylinder height", required=False),
             _phys("a", "length", "first extrapolated side of a cuboid", required=False), _phys("b", "length", "second side", required=False),
             _phys("c", "length", "third side", required=False)],
     outputs=[_o("P", "power", "thermal power"), _o("phi_max", "neutron_flux", "peak flux"), _o("phi_avg", "neutron_flux", "volume-average flux"),
              _o("peaking", "number", "peak-to-average flux ratio")],
     primary="P", conditions=["fundamental-mode flux of a bare homogeneous core", "extrapolated dimensions taken as the core size", "uniform Sigma_f"],
     method="P = E_f Sigma_f phi_max <shape> V; <shape> = 3/pi^2 (sphere), (2 J1(2.405)/2.405)(2/pi) (cylinder), (2/pi)^3 (cuboid)",
     latex=r"P = E_f\Sigma_f\int_V\phi\,dV = E_f\Sigma_f\,\phi_{max}\,\frac{\bar\phi}{\phi_{max}}\,V",
     run=core_power, reference=_ref_core_power,
     wrong=lambda geometry, Sigma_f, E_f, phi_max=None, P=None, R=None, H=None, a=None, b=None, c=None: (lambda o: {**o, "P": _r(o["P"] / o["phi_avg"] * o["phi_max"] if phi_max else o["P"]),
                                                                                                            "phi_max": _r(o["phi_avg"] if P else o["phi_max"])})(
         core_power(geometry, Sigma_f, E_f, phi_max, P, R, H, a, b, c)),
     rand=_rand_core_power,
     phrases=("Core power from flux", "Reactor power from peak flux", "Peak flux from reactor power"),
     examples=[_ex({"geometry": "cylinder", "Sigma_f": 0.2, "E_f": 200 * MEV, "phi_max": 1e17, "R": 1.7, "H": 3.7},
                   written={"Sigma_f": "0.002 1/cm", "E_f": "200 MeV", "phi_max": "1e13 1/(cm^2*s)", "R": "170 cm", "H": "370 cm"}),
               _ex({"geometry": "sphere", "Sigma_f": 0.5, "E_f": 200 * MEV, "P": 1e6, "R": 0.5},
                   written={"Sigma_f": "0.005 1/cm", "E_f": "200 MeV", "P": "1 MW", "R": "50 cm"})],
     level="advanced",
     extra_q=[{"q": "Core power from flux: geometry = sphere, Sigma_f = 0.005 1/cm, E_f = 200 MeV, P = 1 MW, phi_max = 1e13 1/(cm^2*s), R = 50 cm", "outcome": "INVALID_INPUT"}])


# ================================================================================================ REACTOR KINETICS
def _kin_check(rho, beta, lam, Lam):
    _need(len(beta) == len(lam) and 1 <= len(beta) <= 8, "give one decay constant per delayed group (1 to 8 groups)")
    _need(all(_isnum(b) and b > 0 for b in beta), "delayed fractions must be positive")
    _need(all(_isnum(x) and x > 0 for x in lam), "decay constants must be positive")
    _need(len(set(lam)) == len(lam), "the groups' decay constants must differ")
    _pos("the generation time", Lam)
    _need(_isnum(rho) and rho != 0, "a step reactivity other than zero is needed")
    _need(rho < math.fsum(beta), "prompt critical or above (rho >= beta) is not supported")
    _need(rho > -0.5, "the reactivity is below -0.5")


def _inhour_roots(rho, beta, lam, Lam):
    """All roots of rho = omega Lam + sum beta_i omega/(omega + lambda_i): f increases between consecutive poles, so
    each root is bracketed; 60 bisection steps, then Newton steps in 40-digit arithmetic kept inside the bracket."""
    with _mp.workdps(40):
        b = [_mpf(x) for x in beta]
        L = sorted(_mpf(x) for x in lam)
        bl = sorted(zip([_mpf(x) for x in lam], b))
        Lm, r = _mpf(Lam), _mpf(rho)
        f = lambda w: w * Lm + sum(bi * w / (w + li) for li, bi in bl) - r
        df = lambda w: Lm + sum(bi * li / (w + li) ** 2 for li, bi in bl)

        def bis(lo, hi):
            flo = f(lo)
            for _ in range(60):
                m = (lo + hi) / 2
                fm = f(m)
                if (fm > 0) == (flo > 0):
                    lo, flo = m, fm
                else:
                    hi = m
            w = (lo + hi) / 2
            for _ in range(60):
                step = f(w) / df(w)
                w2 = w - step
                if not (lo <= w2 <= hi):                  # never leave the bracket: bisect instead
                    m = (lo + hi) / 2
                    if (f(m) > 0) == (flo > 0):
                        lo, flo = m, f(m)
                    else:
                        hi = m
                    w2 = (lo + hi) / 2
                if abs(w2 - w) <= _mpf(10) ** -36 * (1 + abs(w)):
                    return w2
                w = w2
            return w
        eps = _mpf(10) ** -30
        roots = []
        hi = abs(r) / Lm + 10
        roots.append(bis(-L[0] * (1 - eps), hi))
        for i in range(len(L) - 1):
            roots.append(bis(-L[i + 1] * (1 - eps), -L[i] * (1 + eps)))
        lo = -(sum(b) + abs(r)) / Lm - L[-1] - 10
        while f(lo) > 0:
            lo *= 2
        roots.append(bis(lo, -L[-1] * (1 + eps)))
        amps = []
        for w in roots:
            N = Lm + sum(bi / (w + li) for li, bi in bl)
            S2 = sum(bi / (w + li) ** 2 for li, bi in bl)
            amps.append(1 / (1 - w * S2 / N))
        return roots, amps


def _ref_kin_matrix(rho, beta, lam, Lam):
    import mpmath as mp
    n = len(beta)
    A = mp.zeros(n + 1, n + 1)
    A[0, 0] = (mp.mpf(rho) - mp.fsum(beta)) / Lam
    v0 = mp.zeros(n + 1, 1)
    v0[0] = 1
    for i in range(n):
        A[0, i + 1] = lam[i]
        A[i + 1, 0] = mp.mpf(beta[i]) / Lam
        A[i + 1, i + 1] = -mp.mpf(lam[i])
        v0[i + 1] = mp.mpf(beta[i]) / (Lam * lam[i])          # precursors in equilibrium with P = 1
    return A, v0


def _ref_omega1(rho, beta, lam, Lam):
    """Reference: the largest eigenvalue of the point-kinetics matrix (mpmath QR eigenvalue routine, 40 digits)."""
    import mpmath as mp
    with mp.workdps(40):
        A, _ = _ref_kin_matrix(rho, beta, lam, Lam)
        ev = mp.eig(A, left=False, right=False)
        re = sorted(float(mp.re(e)) for e in ev)
    return re[-1], re[0]


def _ref_power(rho, beta, lam, Lam, t):
    """Reference: P(t)/P0 after the step from the matrix exponential of the point-kinetics system (mpmath, 40 digits)."""
    import mpmath as mp
    with mp.workdps(40):
        A, v0 = _ref_kin_matrix(rho, beta, lam, Lam)
        v = mp.expm(A * mp.mpf(t)) * v0
        return float(v[0])


_KIN_IN1 = [_rhoin("rho", "step reactivity"), _numin("beta", "delayed-neutron fraction beta"),
            _phys("lam", "decay_constant", "precursor decay constant of the single delayed group (e.g. 0.08 1/s)"),
            _phys("Lambda", "time", "prompt neutron generation time")]


def period_one_group(rho, beta, lam, Lambda):
    _kin_check(rho, [beta], [lam], Lambda)
    roots, _a = _inhour_roots(rho, [beta], [lam], Lambda)
    w = float(roots[0])
    return _out(omega=w, T=1.0 / w, T_d=LN2 / w)


def _ref_period_one_group(rho, beta, lam, Lambda):
    w, _p = _ref_omega1(rho, [beta], [lam], Lambda)
    return {"omega": w, "T": 1.0 / w, "T_d": LN2 / w}


def _rand_kin1(rng):
    beta = round(rng.uniform(0.002, 0.008), 5)
    rho = _sig(beta * rng.choice([1, -1]) * _lu(rng, 0.02, 0.9 if rng.random() < 0.6 else 3.0))
    if rho >= beta:
        rho = _sig(0.9 * beta)
    return {"rho": rho, "beta": beta, "lam": _sig(rng.uniform(0.05, 0.1)), "Lambda": _sig(_lu(rng, 1e-5, 1e-3))}


_add("nuc_reactor_period_one_group",
     name="Stable reactor period after a reactivity step (one delayed group)", topic="reactor kinetics / point kinetics",
     purpose="The stable (asymptotic) period, inverse period and doubling time of a reactor after a step reactivity change, with one effective delayed-neutron group (the stable root of the inhour equation).",
     inputs=_KIN_IN1,
     outputs=[_o("omega", "inverse_period", "inverse stable period (largest inhour root)"), _o("T", "time", "stable period (negative: falling power)"),
              _o("T_d", "time", "doubling time T ln 2 (negative: halving time)")],
     primary="T", conditions=["point kinetics with one delayed group", "rho below prompt critical (rho < beta)", "rho != 0"],
     method="largest root of rho = omega Lambda + beta omega/(omega + lambda)",
     latex=r"\rho = \omega\Lambda + \frac{\beta\,\omega}{\omega+\lambda}",
     run=period_one_group, reference=_ref_period_one_group,
     wrong=lambda rho, beta, lam, Lambda: (lambda w: _out(omega=w, T=1 / w, T_d=LN2 / w))(lam * rho / beta),
     rand=_rand_kin1, post=_dollars_with("beta"),
     phrases=("Reactor period one group", "Stable period with one delayed group", "One-group reactor period"),
     examples=[_ex({"rho": 0.001, "beta": 0.0065, "lam": 0.08, "Lambda": 1e-4}, written={"rho": "100 pcm", "lam": "0.08 1/s", "Lambda": "1e-4 s"}),
               _ex({"rho": -0.0065, "beta": 0.0065, "lam": 0.0767, "Lambda": 5e-5}, written={"rho": "-1 $", "lam": "0.0767 1/s", "Lambda": "50 us"})],
     level="intermediate",
     extra_q=[{"q": "Reactor period one group: rho = 1.2 $, beta = 0.0065, lam = 0.08 1/s, Lambda = 1e-4 s", "outcome": "INVALID_INPUT"},
              {"q": "Reactor period one group: rho = 0.1 $, lam = 0.08 1/s, Lambda = 1e-4 s", "outcome": "UNRECOGNIZED_FORMAT"}])


def power_after_step_one_group(rho, beta, lam, Lambda, t):
    _kin_check(rho, [beta], [lam], Lambda)
    _nonneg("the time t", t)
    roots, amps = _inhour_roots(rho, [beta], [lam], Lambda)
    with _mp.workdps(40):
        P = float(sum(a * _mp.exp(w * _mpf(t)) for w, a in zip(roots, amps)))
    return _out(P=P, A1=float(amps[0]), omega=float(roots[0]))


def _ref_power_after_step_one_group(rho, beta, lam, Lambda, t):
    w, _p = _ref_omega1(rho, [beta], [lam], Lambda)
    P = _ref_power(rho, [beta], [lam], Lambda, t)
    # the stable-mode amplitude: P(s) e^(-omega s) once the prompt transient has died away
    s = 60.0 * Lambda / (beta - rho) + 1.0
    A1 = _ref_power(rho, [beta], [lam], Lambda, s) * math.exp(-w * s)
    return {"P": P, "A1": A1, "omega": w}


_add("nuc_power_after_step_one_group",
     name="Power after a reactivity step (one delayed group, exact)", topic="reactor kinetics / point kinetics",
     purpose="The relative power P(t)/P0 a time t after a step reactivity change from steady state, from the exact two-mode solution with one delayed group, with the amplitude of the stable period (the exact prompt jump or drop) and the inverse period.",
     inputs=[*_KIN_IN1, _phys("t", "time", "time after the step")],
     outputs=[_o("P", "number", "relative power P(t)/P0"), _o("A1", "number", "amplitude of the stable mode (exact prompt jump or drop)"),
              _o("omega", "inverse_period", "inverse stable period")],
     primary="P", conditions=["point kinetics, one delayed group, steady state before the step", "rho < beta, rho != 0"],
     method="P/P0 = A1 e^(w1 t) + A2 e^(w2 t), w_j the inhour roots, A_j = 1/(1 - w_j S2(w_j)/N(w_j)) (residues)",
     latex=r"\frac{P(t)}{P_0} = \sum_j A_je^{\omega_jt}",
     run=power_after_step_one_group, reference=_ref_power_after_step_one_group,
     wrong=lambda rho, beta, lam, Lambda, t: (lambda w, a: _out(P=a * math.exp(w * t), A1=a, omega=w))(lam * rho / (beta - rho), beta / (beta - rho)),
     rand=lambda rng: dict(_rand_kin1(rng), t=_sig(_lu(rng, 0.01, 100.0))), post=_dollars_with("beta"),
     phrases=("Power after a reactivity step one group", "Power after a step one delayed group", "One-group power transient"),
     examples=[_ex({"rho": 0.001, "beta": 0.0065, "lam": 0.08, "Lambda": 1e-4, "t": 10.0},
                   written={"rho": "100 pcm", "lam": "0.08 1/s", "Lambda": "1e-4 s", "t": "10 s"}),
               _ex({"rho": -0.0065, "beta": 0.0065, "lam": 0.08, "Lambda": 1e-4, "t": 1.0},
                   written={"rho": "-1 $", "lam": "0.08 1/s", "Lambda": "1e-4 s", "t": "1 s"})],
     level="advanced")


_KIN_IN6 = [_rhoin("rho", "step reactivity"), _vecin("beta", "delayed-neutron fraction of each group", 1, 8),
            _vecin("lam", "decay constant of each group, in 1/s", 1, 8, unit="1/s"), _phys("Lambda", "time", "prompt neutron generation time")]


def power_after_step_multi(rho, beta, lam, Lambda, t):
    _kin_check(rho, beta, lam, Lambda)
    _nonneg("the time t", t)
    roots, amps = _inhour_roots(rho, beta, lam, Lambda)
    with _mp.workdps(40):
        P = float(sum(a * _mp.exp(w * _mpf(t)) for w, a in zip(roots, amps)))
    w = float(roots[0])
    return _out(P=P, omega=w, T=1.0 / w)


def _ref_power_after_step_multi(rho, beta, lam, Lambda, t):
    w, _p = _ref_omega1(rho, beta, lam, Lambda)
    return {"P": _ref_power(rho, beta, lam, Lambda, t), "omega": w, "T": 1.0 / w}


_KEEPIN = ([0.000215, 0.001424, 0.001274, 0.002568, 0.000748, 0.000273], [0.0124, 0.0305, 0.111, 0.301, 1.14, 3.01])


def _rand_kin6(rng):
    n = rng.choice([6, 6, 6, 1, 2, 3, 4])
    if n == 6 and rng.random() < 0.5:
        beta, lam = list(_KEEPIN[0]), list(_KEEPIN[1])
    else:
        lam = sorted(_sig(_lu(rng, 0.01, 3.5)) for _ in range(n))
        while len(set(lam)) < n:
            lam = sorted(_sig(_lu(rng, 0.01, 3.5)) for _ in range(n))
        beta = [_sig(_lu(rng, 1e-4, 3e-3)) for _ in range(n)]
    B = sum(beta)
    rho = _sig(B * rng.choice([1, -1]) * _lu(rng, 0.02, 0.9 if rng.random() < 0.6 else 3.0))
    if rho >= B:
        rho = _sig(0.9 * B)
    return {"rho": rho, "beta": beta, "lam": lam, "Lambda": _sig(_lu(rng, 1e-5, 1e-3)), "t": _sig(_lu(rng, 0.01, 60.0))}


_add("nuc_power_after_step_six_group",
     name="Power after a reactivity step (six delayed groups, exact)", topic="reactor kinetics / point kinetics",
     purpose="The relative power P(t)/P0 a time t after a step reactivity change from steady state with six (or any number up to eight) delayed-neutron groups, from all the inhour roots and their residues, and the stable period.",
     inputs=[*_KIN_IN6, _phys("t", "time", "time after the step")],
     outputs=[_o("P", "number", "relative power P(t)/P0"), _o("omega", "inverse_period", "inverse stable period"), _o("T", "time", "stable period")],
     primary="P", conditions=["point kinetics, steady state before the step", "rho < beta (sum of the groups), rho != 0", "distinct decay constants"],
     method="P/P0 = sum_j A_j e^(w_j t) over the n + 1 inhour roots (bisection between the poles, 40 digits), A_j = 1/(1 - w_j sum beta_i/(w_j + lambda_i)^2 / N(w_j))",
     latex=r"\rho = \omega\Lambda + \sum_i\frac{\beta_i\,\omega}{\omega+\lambda_i},\quad \frac{P}{P_0} = \sum_j A_je^{\omega_jt}",
     run=power_after_step_multi, reference=_ref_power_after_step_multi,
     wrong=lambda rho, beta, lam, Lambda, t: (lambda B: (lambda w: _out(P=B / (B - rho) * math.exp(w * t), omega=w, T=1 / w))(
         rho / (B - rho) / math.fsum(b / l for b, l in zip(beta, lam)) * 1.5))(math.fsum(beta)),
     rand=_rand_kin6, post=_dollars_with("beta"),
     phrases=("Power after a reactivity step six groups", "Six-group power transient", "Power after a step six delayed groups"),
     examples=[_ex({"rho": 0.001, "beta": list(_KEEPIN[0]), "lam": list(_KEEPIN[1]), "Lambda": 5e-5, "t": 10.0},
                   written={"rho": "100 pcm", "Lambda": "50 us", "t": "10 s"}),
               _ex({"rho": -sum(_KEEPIN[0]), "beta": list(_KEEPIN[0]), "lam": list(_KEEPIN[1]), "Lambda": 5e-5, "t": 30.0},
                   written={"rho": "-1 $", "Lambda": "50 us", "t": "30 s"})],
     level="expert",
     extra_q=[{"q": "Power after a reactivity step six groups: rho = 100 pcm, beta = [0.000215, 0.001424, 0.001274, 0.002568, 0.000748, 0.000273], "
                    "lam = [0.0124, 0.0305, 0.111, 0.301, 1.14, 3.01] 1/s, Lambda = 50 us, t = 10 s", "like": 0}])


def stable_period_multi(rho, beta, lam, Lambda):
    _kin_check(rho, beta, lam, Lambda)
    roots, _a = _inhour_roots(rho, beta, lam, Lambda)
    w, wp = float(roots[0]), float(roots[-1])
    return _out(omega=w, T=1.0 / w, T_d=LN2 / w, omega_p=wp)


def _ref_stable_period_multi(rho, beta, lam, Lambda):
    w, wp = _ref_omega1(rho, beta, lam, Lambda)
    return {"omega": w, "T": 1.0 / w, "T_d": LN2 / w, "omega_p": wp}


_add("nuc_stable_period_six_group",
     name="Stable period and prompt root of the inhour equation (six delayed groups)", topic="reactor kinetics / inhour equation",
     purpose="The stable reactor period, inverse period and doubling time after a step reactivity change with six (or any number up to eight) delayed groups, and the prompt (most negative) root that sets the prompt jump's time constant.",
     inputs=_KIN_IN6,
     outputs=[_o("omega", "inverse_period", "inverse stable period (largest root)"), _o("T", "time", "stable period"),
              _o("T_d", "time", "doubling time (negative: halving time)"), _o("omega_p", "inverse_period", "prompt root (most negative)")],
     primary="T", conditions=["point kinetics", "rho < beta, rho != 0", "distinct decay constants"],
     method="largest and smallest roots of the inhour equation (bisection between the poles, 40 digits)",
     latex=r"\rho = \omega\Lambda + \sum_i\frac{\beta_i\,\omega}{\omega+\lambda_i}",
     run=stable_period_multi, reference=_ref_stable_period_multi,
     wrong=lambda rho, beta, lam, Lambda: (lambda B: (lambda w: _out(omega=w, T=1 / w, T_d=LN2 / w, omega_p=-(B - rho) / Lambda * 0.5))(
         rho / ((B - rho) * math.fsum(b / l for b, l in zip(beta, lam)) / B) * 2))(math.fsum(beta)),
     rand=lambda rng: {k: v for k, v in _rand_kin6(rng).items() if k != "t"}, post=_dollars_with("beta"),
     phrases=("Stable period six groups", "Inhour equation stable period", "Six-group stable period"),
     examples=[_ex({"rho": 0.001, "beta": list(_KEEPIN[0]), "lam": list(_KEEPIN[1]), "Lambda": 5e-5}, written={"rho": "100 pcm", "Lambda": "50 us"}),
               _ex({"rho": 0.5 * sum(_KEEPIN[0]), "beta": list(_KEEPIN[0]), "lam": list(_KEEPIN[1]), "Lambda": 1e-4},
                   written={"rho": "50 cents", "Lambda": "100 us"})],
     level="advanced")


def reactivity_from_period(beta, lam, Lambda, T=None, T_d=None):
    _need((T is None) != (T_d is None), "give either the stable period T or the doubling time T_d (one of them)")
    _need(len(beta) == len(lam) and 1 <= len(beta) <= 8, "give one decay constant per delayed group")
    _need(all(_isnum(b) and b > 0 for b in beta) and all(_isnum(x) and x > 0 for x in lam), "delayed data must be positive")
    _pos("the generation time", Lambda)
    per = T if T is not None else T_d / LN2
    _need(_isnum(per) and per != 0, "the period must be a non-zero time")
    w = 1.0 / per
    _need(w > -min(lam), "a negative period shorter than 1/lambda_min is not a stable period")
    rho = w * Lambda + math.fsum(b * w / (w + l) for b, l in zip(beta, lam))
    B = math.fsum(beta)
    return _out(rho=rho, rho_dollars=rho / B, rho_pcm=rho * 1e5)


def _ref_reactivity_from_period(beta, lam, Lambda, T=None, T_d=None):
    w = 1.0 / (T if T is not None else T_d / LN2)
    # P = e^(w t) for all past times: each precursor group is the convolution of its source (beta_i/Lambda) P with
    # its decay, by quadrature; the power equation then gives rho
    C = [b / Lambda * _quad(lambda s, l=l: math.exp((w + l) * s), -60.0 / (w + l), 0.0) for b, l in zip(beta, lam)]
    rho = Lambda * w + math.fsum(beta) - Lambda * math.fsum(l * c for l, c in zip(lam, C))
    B = math.fsum(beta)
    return {"rho": rho, "rho_dollars": rho / B, "rho_pcm": rho * 1e5}


def _rand_rho_from_T(rng):
    d = _rand_kin6(rng)
    lam_min = min(d["lam"])
    if rng.random() < 0.7:
        per = _sig(_lu(rng, 0.5, 1e4))
    else:
        per = -_sig(_lu(rng, 1.2 / lam_min, 50.0 / lam_min))
    out = {"beta": d["beta"], "lam": d["lam"], "Lambda": d["Lambda"]}
    if rng.random() < 0.5:
        out["T"] = per
    else:
        out["T_d"] = _sig(per * LN2)
    return out


_add("nuc_reactivity_from_period",
     name="Reactivity from a measured stable period or doubling time (inhour)", topic="reactor kinetics / inhour equation",
     purpose="The reactivity, in absolute units, dollars and pcm, that produces a measured stable period (or doubling time) with stated delayed-neutron data, from the inhour equation.",
     inputs=[_vecin("beta", "delayed-neutron fraction of each group", 1, 8), _vecin("lam", "decay constant of each group, in 1/s", 1, 8, unit="1/s"),
             _phys("Lambda", "time", "prompt neutron generation time"),
             _phys("T", "time", "stable period (give this or T_d; negative for falling power)", required=False),
             _phys("T_d", "time", "doubling time (give this or T)", required=False)],
     outputs=[_o("rho", "number", "reactivity"), _o("rho_dollars", "number", "reactivity in dollars"), _o("rho_pcm", "number", "reactivity in pcm")],
     primary="rho", conditions=["point kinetics, asymptotic period", "for a negative period: 1/T > -lambda_min"],
     method="rho = omega Lambda + sum beta_i omega/(omega + lambda_i), omega = 1/T = ln 2/T_d",
     latex=r"\rho = \frac{\Lambda}{T} + \sum_i\frac{\beta_i}{1+\lambda_iT}",
     run=reactivity_from_period, reference=_ref_reactivity_from_period,
     wrong=lambda beta, lam, Lambda, T=None, T_d=None: (lambda w: (lambda r: _out(rho=r, rho_dollars=r / math.fsum(beta), rho_pcm=r * 1e5))(
         math.fsum(b * l for b, l in zip(beta, lam)) * w / math.fsum(lam)))(1.0 / (T if T is not None else T_d / LN2)),
     rand=_rand_rho_from_T,
     phrases=("Reactivity from period", "Reactivity from doubling time", "Inhour reactivity"),
     examples=[_ex({"beta": list(_KEEPIN[0]), "lam": list(_KEEPIN[1]), "Lambda": 5e-5, "T": 60.0}, written={"Lambda": "50 us", "T": "60 s"}),
               _ex({"beta": [0.0065], "lam": [0.08], "Lambda": 1e-4, "T_d": 30.0}, written={"Lambda": "1e-4 s", "T_d": "30 s"})],
     level="advanced",
     extra_q=[{"q": "Reactivity from period: beta = [0.0065], lam = [0.08] 1/s, Lambda = 1e-4 s, T = 60 s, T_d = 30 s", "outcome": "INVALID_INPUT"}])


def rod_worth(rho_tot, H, x, beta=None):
    _need(_isnum(rho_tot) and rho_tot > 0, "the total rod worth must be positive")
    _pos("the core height", H)
    _need(_isnum(x) and 0 <= x <= H, "the insertion depth must lie between 0 and H")
    u = x / H
    frac = u - math.sin(2 * math.pi * u) / (2 * math.pi)
    if abs(frac) < 1e-15:
        frac = 0.0
    out = dict(rho=rho_tot * frac, frac=frac, drho_dx=rho_tot / H * (1 - math.cos(2 * math.pi * u)))
    if beta is not None:
        _pos("beta", beta)
        out["rho_dollars"] = rho_tot * frac / beta
    return _out(**out)


def _ref_rod_worth(rho_tot, H, x, beta=None):
    w = lambda z: math.sin(math.pi * z / H) ** 2          # first-order perturbation: importance ~ flux^2 ~ sin^2
    tot = _quad(w, 0.0, H)
    frac = _quad(w, 0.0, x) / tot if x > 0 else 0.0
    out = {"rho": rho_tot * frac, "frac": frac, "drho_dx": rho_tot * w(x) / tot}
    if beta is not None:
        out["rho_dollars"] = rho_tot * frac / beta
    return out


_add("nuc_control_rod_worth",
     name="Integral and differential control-rod worth (sine-squared weighting)", topic="reactor kinetics / control rods",
     purpose="The reactivity worth of a control rod inserted to a depth x in a core of height H with a cosine axial flux (first-order perturbation: worth density ~ sin^2), as a fraction of its total worth, and the differential worth at that depth.",
     inputs=[_rhoin("rho_tot", "total worth of the fully inserted rod"), _phys("H", "length", "core height"),
             _phys("x", "length", "insertion depth from the top of the core"),
             _numin("beta", "delayed-neutron fraction (optional: converts a worth in dollars and reports dollars)", required=False)],
     outputs=[_o("rho", "number", "worth inserted (absolute)"), _o("frac", "number", "fraction of the total worth"),
              _o("drho_dx", "inverse_length", "differential worth at x (reactivity per metre)"), _o("rho_dollars", "number", "worth in dollars (when beta is given)")],
     primary="rho", conditions=["cosine axial flux, flux zero at the core ends", "first-order perturbation (small rod)"],
     method="rho(x)/rho_tot = x/H - sin(2 pi x/H)/(2 pi); d rho/dx = (rho_tot/H)(1 - cos(2 pi x/H))",
     latex=r"\frac{\rho(x)}{\rho_{tot}} = \frac{x}{H} - \frac{1}{2\pi}\sin\frac{2\pi x}{H}",
     run=rod_worth, reference=_ref_rod_worth,
     wrong=lambda rho_tot, H, x, beta=None: _out(rho=rho_tot * x / H, frac=x / H, drho_dx=rho_tot / H, **({"rho_dollars": rho_tot * x / H / beta} if beta else {})),
     rand=lambda rng: (lambda H: {"rho_tot": _sig(_lu(rng, 1e-3, 0.05)), "H": H, "x": _sig(H * rng.uniform(0.05, 0.95)),
                                  **({"beta": round(rng.uniform(0.002, 0.008), 5)} if rng.random() < 0.4 else {})})(_sig(rng.uniform(0.5, 5.0))),
     phrases=("Control rod worth", "Rod worth", "Integral rod worth"),
     examples=[_ex({"rho_tot": 0.01, "H": 3.66, "x": 1.83}, written={"rho_tot": "1000 pcm", "H": "3.66 m", "x": "1.83 m"}),
               _ex({"rho_tot": 0.0065 * 2.0, "H": 1.0, "x": 0.25, "beta": 0.0065}, written={"rho_tot": "2 $", "H": "1 m", "x": "25 cm"})],
     level="intermediate", post=_dollars_with("beta"))


# ================================================================================================ POISONS AND FUEL
_LAM_I = LN2 / _ND["I-135"][0]
_LAM_X = LN2 / _ND["Xe-135"][0]
_LAM_PM = LN2 / _ND["Pm-149"][0]

_XE_IN = [_phys("Sigma_f", "macroscopic_cross_section", "macroscopic fission cross section of the fuel"),
          _phys("phi", "neutron_flux", "thermal flux during operation"),
          _numin("gamma_I", "cumulative fission yield of I-135 (e.g. 0.0639)", pct=True),
          _numin("gamma_X", "direct fission yield of Xe-135 (e.g. 0.00237)", pct=True),
          _phys("sigma_X", "microscopic_cross_section", "Xe-135 absorption cross section (e.g. 2.65e6 b)")]


def _xe_eq(Sigma_f, phi, gamma_I, gamma_X, sigma_X):
    _pos("Sigma_f, the flux and sigma_X", Sigma_f, phi, sigma_X)
    _frac("gamma_I", gamma_I)
    _frac("gamma_X", gamma_X)
    _need(gamma_I + gamma_X > 0, "the yields cannot both be zero")
    I = gamma_I * Sigma_f * phi / _LAM_I
    X = (gamma_I + gamma_X) * Sigma_f * phi / (_LAM_X + sigma_X * phi)
    return I, X


def xenon_equilibrium(Sigma_f, phi, gamma_I, gamma_X, sigma_X):
    I, X = _xe_eq(Sigma_f, phi, gamma_I, gamma_X, sigma_X)
    return _out(I_eq=I, X_eq=X, abs_per_fission=sigma_X * X / Sigma_f)


def _ref_xenon_equilibrium(Sigma_f, phi, gamma_I, gamma_X, sigma_X):
    F = Sigma_f * phi
    kX = _LAM_X + sigma_X * phi
    T = 60.0 / min(_LAM_I, kX)
    # the I-135 -> Xe-135 rate equations integrated (scaled by the fission rate) until equilibrium
    i, x = _ivp(lambda t, y: [gamma_I - _LAM_I * y[0], gamma_X + _LAM_I * y[0] - kX * y[1]], T, [0.0, 0.0], atol=[1e-30, 1e-30])
    return {"I_eq": i * F, "X_eq": x * F, "abs_per_fission": sigma_X * phi * x}


def _rand_xe(rng):
    return {"Sigma_f": _sig(_lu(rng, 1.0, 50.0)), "phi": _sig(_lu(rng, 1e15, 1e19)), "gamma_I": round(rng.uniform(0.05, 0.07), 5),
            "gamma_X": round(rng.uniform(0.0, 0.01), 5), "sigma_X": _sig(rng.uniform(1.5e6, 3e6)) * BARN}


_add("nuc_xenon_equilibrium",
     name="Equilibrium iodine-135 and xenon-135 concentrations", topic="reactor physics / fission-product poisons",
     purpose="The equilibrium I-135 and Xe-135 atom densities in fuel operating at a steady flux, the xenon absorptions per fission (its poisoning strength) and the flux at which xenon burn-out equals its decay.",
     inputs=_XE_IN,
     outputs=[_o("I_eq", "number_density", "equilibrium I-135 atoms per volume"), _o("X_eq", "number_density", "equilibrium Xe-135 atoms per volume"),
              _o("abs_per_fission", "number", "Xe-135 absorptions per fission at equilibrium")],
     primary="X_eq", conditions=["constant flux", "half-lives of I-135 (6.57 h) and Xe-135 (9.14 h) of radioactivedecay 0.6.1", "Xe-135m neglected"],
     method="I = gamma_I Sigma_f phi/lambda_I; X = (gamma_I + gamma_X) Sigma_f phi/(lambda_X + sigma_X phi)",
     latex=r"X_\infty = \frac{(\gamma_I+\gamma_X)\Sigma_f\phi}{\lambda_X+\sigma_X\phi}",
     run=xenon_equilibrium, reference=_ref_xenon_equilibrium,
     wrong=lambda Sigma_f, phi, gamma_I, gamma_X, sigma_X: (lambda I: _out(I_eq=I, X_eq=gamma_I * Sigma_f * phi / _LAM_X, abs_per_fission=sigma_X * gamma_I * phi / _LAM_X))(gamma_I * Sigma_f * phi / _LAM_I),
     rand=_rand_xe,
     phrases=("Xenon equilibrium", "Equilibrium xenon", "Equilibrium iodine and xenon"),
     examples=[_ex({"Sigma_f": 10.0, "phi": 3e17, "gamma_I": 0.0639, "gamma_X": 0.00237, "sigma_X": 2.65e6 * BARN},
                   written={"Sigma_f": "0.1 1/cm", "phi": "3e13 1/(cm^2*s)", "sigma_X": "2.65e6 b"}),
               _ex({"Sigma_f": 10.0, "phi": 1e16, "gamma_I": 0.0639, "gamma_X": 0.00237, "sigma_X": 2.65e6 * BARN},
                   written={"Sigma_f": "0.1 1/cm", "phi": "1e12 1/(cm^2*s)", "sigma_X": "2.65 Mb"})],
     level="advanced")


def xenon_after_shutdown(Sigma_f, phi, gamma_I, gamma_X, sigma_X, t):
    I0, X0 = _xe_eq(Sigma_f, phi, gamma_I, gamma_X, sigma_X)
    _nonneg("the time after shutdown", t)
    lI, lX = _LAM_I, _LAM_X
    with _mp.workdps(40):
        a, b = _mpf(lI), _mpf(lX)
        Xt = lambda s: _mpf(X0) * _mp.exp(-b * s) + _mpf(I0) * a / (b - a) * (_mp.exp(-a * s) - _mp.exp(-b * s))
        X = float(Xt(_mpf(t)))
        arg = b * (_mpf(I0) * a - _mpf(X0) * (b - a)) / (a * a * _mpf(I0))
        tp = _mp.log(arg) / (b - a) if arg > 0 else _mpf(-1)
        if lI * I0 > lX * X0 and tp > 0:                    # xenon rises first: a peak after shutdown
            Xp = float(Xt(tp))
            tp = float(tp)
        else:
            tp, Xp = 0.0, X0
    return _out(X=X, ratio=X / X0, t_peak=tp, peak_ratio=Xp / X0, I=I0 * math.exp(-lI * t))


def _ref_xenon_after_shutdown(Sigma_f, phi, gamma_I, gamma_X, sigma_X, t):
    lI, lX = _LAM_I, _LAM_X
    I0 = gamma_I * Sigma_f * phi / lI
    X0 = (gamma_I + gamma_X) * Sigma_f * phi / (lX + sigma_X * phi)
    if t > 0:
        i, x = _ivp(lambda s, y: [-lI * y[0], lI * y[0] - lX * y[1]], t, [I0, X0], atol=[1e-30, 1e-30])
    else:
        i, x = I0, X0
    Xc = lambda s: X0 * math.exp(-lX * s) + lI * I0 * _conv(lI, lX, s)        # xenon from the iodine by convolution
    g = lambda s: lI * I0 * math.exp(-lI * s) - lX * Xc(s)
    if g(0.0) > 0:
        tp = _brentq(g, 0.0, 60.0 / lI)
        Xp = Xc(tp)
    else:
        tp, Xp = 0.0, X0
    return {"X": x, "ratio": x / X0, "t_peak": tp, "peak_ratio": Xp / X0, "I": i}


_add("nuc_xenon_after_shutdown",
     name="Xenon-135 after shutdown: concentration and peak", topic="reactor physics / fission-product poisons",
     purpose="The Xe-135 and I-135 atom densities a time t after shutdown from equilibrium operation, the xenon relative to its equilibrium value, and the time and height of the post-shutdown xenon peak (none at low flux).",
     inputs=[*_XE_IN, _phys("t", "time", "time after shutdown")],
     outputs=[_o("X", "number_density", "Xe-135 atoms per volume at t"), _o("ratio", "number", "X(t)/X_eq"),
              _o("t_peak", "time", "time of the xenon peak after shutdown (0 when xenon only falls)"),
              _o("peak_ratio", "number", "peak xenon / equilibrium xenon"), _o("I", "number_density", "I-135 atoms per volume at t")],
     primary="X", conditions=["equilibrium before shutdown", "no flux after shutdown"],
     method="X(t) = X0 e^(-lambda_X t) + I0 lambda_I/(lambda_X - lambda_I)(e^(-lambda_I t) - e^(-lambda_X t)); dX/dt = 0 for the peak",
     latex=r"X(t) = X_0e^{-\lambda_Xt} + \frac{\lambda_II_0}{\lambda_X-\lambda_I}\left(e^{-\lambda_It}-e^{-\lambda_Xt}\right)",
     run=xenon_after_shutdown, reference=_ref_xenon_after_shutdown,
     wrong=lambda Sigma_f, phi, gamma_I, gamma_X, sigma_X, t: (lambda I0, X0: _out(X=X0 * math.exp(-_LAM_X * t), ratio=math.exp(-_LAM_X * t), t_peak=0.0, peak_ratio=1.0,
                                                                                   I=I0 * math.exp(-_LAM_I * t)))(*_xe_eq(Sigma_f, phi, gamma_I, gamma_X, sigma_X)),
     rand=lambda rng: dict(_rand_xe(rng), t=_sig(_lu(rng, 600.0, 3e5))),
     phrases=("Xenon after shutdown", "Xenon peak after shutdown", "Post-shutdown xenon"),
     examples=[_ex({"Sigma_f": 10.0, "phi": 3e17, "gamma_I": 0.0639, "gamma_X": 0.00237, "sigma_X": 2.65e6 * BARN, "t": 36000.0},
                   written={"Sigma_f": "0.1 1/cm", "phi": "3e13 1/(cm^2*s)", "sigma_X": "2.65e6 b", "t": "10 h"}),
               _ex({"Sigma_f": 10.0, "phi": 1e15, "gamma_I": 0.0639, "gamma_X": 0.00237, "sigma_X": 2.65e6 * BARN, "t": 3600.0},
                   written={"Sigma_f": "0.1 1/cm", "phi": "1e11 1/(cm^2*s)", "sigma_X": "2.65e6 b", "t": "1 h"})],
     level="expert")


def samarium(Sigma_f, phi, gamma_Pm, sigma_Sm, t=None):
    _pos("Sigma_f, the flux and sigma_Sm", Sigma_f, phi, sigma_Sm)
    _frac("gamma_Pm", gamma_Pm, lo_open=True)
    Pm = gamma_Pm * Sigma_f * phi / _LAM_PM
    Sm = gamma_Pm * Sigma_f / sigma_Sm
    out = dict(Pm_eq=Pm, Sm_eq=Sm, Sm_max=Sm + Pm, max_ratio=(Sm + Pm) / Sm)
    if t is not None:
        _nonneg("the time after shutdown", t)
        out["Sm_t"] = Sm + Pm * -math.expm1(-_LAM_PM * t)
    return _out(**out)


def _ref_samarium(Sigma_f, phi, gamma_Pm, sigma_Sm, t=None):
    F = Sigma_f * phi
    kS = sigma_Sm * phi
    T = 60.0 / min(_LAM_PM, kS)
    p, s = _ivp(lambda u, y: [gamma_Pm - _LAM_PM * y[0], _LAM_PM * y[0] - kS * y[1]], T, [0.0, 0.0], atol=[1e-30, 1e-30])
    Pm, Sm = p * F, s * F
    pe, se = _ivp(lambda u, y: [-_LAM_PM * y[0], _LAM_PM * y[0]], 60.0 / _LAM_PM, [Pm, Sm], atol=[1e-30, 1e-30])
    out = {"Pm_eq": Pm, "Sm_eq": Sm, "Sm_max": se, "max_ratio": se / Sm}
    if t is not None:
        out["Sm_t"] = _ivp(lambda u, y: [-_LAM_PM * y[0], _LAM_PM * y[0]], t, [Pm, Sm], atol=[1e-30, 1e-30])[1] if t > 0 else Sm
    return out


_add("nuc_samarium",
     name="Samarium-149 poisoning: equilibrium and after shutdown", topic="reactor physics / fission-product poisons",
     purpose="The equilibrium Pm-149 and Sm-149 atom densities during operation (Sm-149 is independent of the flux), the Sm-149 level reached after shutdown when all the promethium has decayed, and (optionally) the level a time t after shutdown.",
     inputs=[_phys("Sigma_f", "macroscopic_cross_section", "macroscopic fission cross section"), _phys("phi", "neutron_flux", "thermal flux during operation"),
             _numin("gamma_Pm", "fission yield of Pm-149 (e.g. 0.0113)", pct=True),
             _phys("sigma_Sm", "microscopic_cross_section", "Sm-149 absorption cross section (e.g. 40800 b)"),
             _phys("t", "time", "time after shutdown (optional)", required=False)],
     outputs=[_o("Pm_eq", "number_density", "equilibrium Pm-149"), _o("Sm_eq", "number_density", "equilibrium Sm-149"),
              _o("Sm_max", "number_density", "Sm-149 after all Pm-149 has decayed"), _o("max_ratio", "number", "Sm_max/Sm_eq"),
              _o("Sm_t", "number_density", "Sm-149 at t after shutdown (when t is given)")],
     primary="Sm_eq", conditions=["constant flux before shutdown", "Pm-149 half-life of radioactivedecay 0.6.1 (53.08 h)", "Pm-149 absorption neglected"],
     method="Pm = gamma Sigma_f phi/lambda_Pm; Sm = gamma Sigma_f/sigma_Sm; after shutdown Sm + Pm(1 - e^(-lambda_Pm t))",
     latex=r"S_\infty = \frac{\gamma_{Pm}\Sigma_f}{\sigma_{Sm}},\quad P_\infty = \frac{\gamma_{Pm}\Sigma_f\phi}{\lambda_{Pm}}",
     run=samarium, reference=_ref_samarium,
     wrong=lambda Sigma_f, phi, gamma_Pm, sigma_Sm, t=None: (lambda Pm, Sm: _out(Pm_eq=Pm, Sm_eq=Sm, Sm_max=Sm, max_ratio=1.0, **({"Sm_t": Sm} if t is not None else {})))(
         gamma_Pm * Sigma_f * phi / _LAM_PM, gamma_Pm * Sigma_f * phi / (sigma_Sm * phi + _LAM_PM)),
     rand=lambda rng: {"Sigma_f": _sig(_lu(rng, 1.0, 50.0)), "phi": _sig(_lu(rng, 1e16, 1e19)), "gamma_Pm": round(rng.uniform(0.008, 0.015), 5),
                       "sigma_Sm": _sig(rng.uniform(3e4, 7e4)) * BARN, **({"t": _sig(_lu(rng, 3600.0, 3e6))} if rng.random() < 0.5 else {})},
     phrases=("Samarium poisoning", "Samarium equilibrium", "Samarium-149"),
     examples=[_ex({"Sigma_f": 10.0, "phi": 3e17, "gamma_Pm": 0.0113, "sigma_Sm": 40800e-28},
                   written={"Sigma_f": "0.1 1/cm", "phi": "3e13 1/(cm^2*s)", "sigma_Sm": "40800 b"}),
               _ex({"Sigma_f": 10.0, "phi": 3e17, "gamma_Pm": 0.0113, "sigma_Sm": 40800e-28, "t": 172800.0},
                   written={"Sigma_f": "0.1 1/cm", "phi": "3e13 1/(cm^2*s)", "sigma_Sm": "40800 b", "t": "48 h"})],
     level="advanced")


def depletion_constant_flux(m0, sigma_a, sigma_f, phi, t, E_f):
    _pos("m0, the cross sections, the flux and E_f", m0, sigma_a, sigma_f, phi, E_f)
    _need(sigma_f <= sigma_a, "the fission cross section cannot exceed the absorption cross section")
    _nonneg("the time t", t)
    N0 = m0 * 1000.0 / _M5 * NA
    x = sigma_a * phi * t
    f = math.exp(-x)
    E = E_f * sigma_f / sigma_a * N0 * -math.expm1(-x)
    return _out(m=m0 * f, burnt=-math.expm1(-x), E=E, E_MWd=E / (1e6 * DAY), P_t=E_f * sigma_f * phi * N0 * f)


def _ref_depletion_constant_flux(m0, sigma_a, sigma_f, phi, t, E_f):
    N0 = m0 * 1000.0 / _M5 * NA
    s_end = sigma_a * phi * t
    # dN/ds = -N, dE/ds = E_f (sigma_f/sigma_a) N in s = sigma_a phi t
    n, e = _ivp(lambda s, y: [-y[0], y[0]], s_end, [1.0, 0.0], atol=[1e-300, 1e-30]) if s_end > 0 else (1.0, 0.0)
    E = E_f * sigma_f / sigma_a * N0 * e
    return {"m": m0 * n, "burnt": 1 - n if n < 0.5 else e, "E": E, "E_MWd": E / (1e6 * DAY),
            "P_t": E_f * sigma_f * phi * N0 * n}


_add("nuc_depletion_constant_flux",
     name="U-235 depletion and energy released at constant flux", topic="reactor physics / fuel depletion and burnup",
     purpose="How much of an initial mass of U-235 remains after irradiation at a constant flux, the fraction burnt, the fission energy released (J and MWd) and the power at the start and at the end.",
     inputs=[_phys("m0", "mass", "initial mass of U-235"), _phys("sigma_a", "microscopic_cross_section", "U-235 absorption cross section (barns)"),
             _phys("sigma_f", "microscopic_cross_section", "U-235 fission cross section (barns)"), _phys("phi", "neutron_flux", "neutron flux"),
             _phys("t", "time", "irradiation time"), _phys("E_f", "energy", "recoverable energy per fission (e.g. 200 MeV)")],
     outputs=[_o("m", "mass", "U-235 mass remaining"), _o("burnt", "number", "fraction of U-235 consumed"), _o("E", "energy", "fission energy released"),
              _o("E_MWd", "number", "energy released in MWd"), _o("P_t", "power", "fission power at t")],
     primary="m", conditions=["constant flux, one group", "U-235 only (no breeding)", "M(U-235) = 235.043928 g/mol"],
     method="N = N0 e^(-sigma_a phi t); E = E_f (sigma_f/sigma_a) (N0 - N); P = E_f sigma_f phi N",
     latex=r"N(t) = N_0e^{-\sigma_a\phi t},\quad E = E_f\frac{\sigma_f}{\sigma_a}\left(N_0-N\right)",
     run=depletion_constant_flux, reference=_ref_depletion_constant_flux,
     wrong=lambda m0, sigma_a, sigma_f, phi, t, E_f: (lambda N0: _out(m=m0 * math.exp(-sigma_f * phi * t), burnt=-math.expm1(-sigma_f * phi * t), E=E_f * N0 * sigma_f * phi * t,
                                                                    E_MWd=E_f * N0 * sigma_f * phi * t / (1e6 * DAY), P_t=E_f * sigma_f * phi * N0))(m0 * 1000.0 / _M5 * NA),
     rand=lambda rng: (lambda sa: {"m0": _sig(_lu(rng, 1e-3, 1e3)), "sigma_a": sa * BARN, "sigma_f": _sig(sa * rng.uniform(0.8, 0.87)) * BARN,
                                   "phi": _sig(_lu(rng, 1e16, 1e18)), "t": _sig(_lu(rng, 1e5, 1e8)), "E_f": _sig(rng.uniform(190.0, 210.0)) * MEV})(_sig(rng.uniform(400.0, 700.0))),
     phrases=("Depletion at constant flux", "U-235 depletion constant flux", "Fuel depletion constant flux"),
     examples=[_ex({"m0": 1.0, "sigma_a": 680.9e-28, "sigma_f": 582.6e-28, "phi": 3e17, "t": 31557600.0, "E_f": 200 * MEV},
                   written={"m0": "1 kg", "sigma_a": "680.9 b", "sigma_f": "582.6 b", "phi": "3e13 1/(cm^2*s)", "t": "1 y", "E_f": "200 MeV"}),
               _ex({"m0": 0.001, "sigma_a": 680.9e-28, "sigma_f": 582.6e-28, "phi": 1e18, "t": 86400.0, "E_f": 200 * MEV},
                   written={"m0": "1 g", "sigma_a": "680.9 b", "sigma_f": "582.6 b", "phi": "1e14 1/(cm^2*s)", "t": "1 d", "E_f": "200 MeV"})],
     level="intermediate")


def _swu_value(x):
    return (2 * x - 1) * math.log(x / (1 - x))


def swu(x_p, x_f, x_w, P):
    for nm, v in (("x_p", x_p), ("x_f", x_f), ("x_w", x_w)):
        _frac(nm, v, lo_open=True, hi_open=True)
    _need(x_w < x_f < x_p, "the assays must satisfy tails < feed < product")
    _pos("the product mass", P)
    F = P * (x_p - x_w) / (x_f - x_w)
    W = F - P
    S = P * _swu_value(x_p) + W * _swu_value(x_w) - F * _swu_value(x_f)
    return _out(F=F, W=W, SWU=S, F_per_P=F / P, SWU_per_P=S / P)


def _ref_swu(x_p, x_f, x_w, P):
    import numpy as np
    F, W = np.linalg.solve(np.array([[1.0, -1.0], [x_f, -x_w]]), np.array([P, P * x_p]))
    F, W = float(F), float(W)
    # separative work of an ideal cascade: integral of the stage flows, V''(x) = 1/(x^2 (1 - x)^2)
    g = lambda x: 1.0 / (x * x * (1 - x) ** 2)
    S = P * _quad(lambda x: (x_p - x) * g(x), x_f, x_p) + W * _quad(lambda x: (x - x_w) * g(x), x_w, x_f)
    return {"F": F, "W": W, "SWU": S, "F_per_P": F / P, "SWU_per_P": S / P}


_add("nuc_enrichment_swu",
     name="Uranium enrichment: feed, tails and separative work (SWU)", topic="fuel cycle / enrichment",
     purpose="The natural-uranium feed and tails masses and the separative work (kg SWU) needed to produce a mass of enriched uranium from stated product, feed and tails assays, per kg of product too.",
     inputs=[_numin("x_p", "U-235 assay of the product (weight fraction, e.g. 4.5%)", pct=True),
             _numin("x_f", "U-235 assay of the feed (0.711% for natural uranium)", pct=True),
             _numin("x_w", "U-235 assay of the tails (e.g. 0.25%)", pct=True), _phys("P", "mass", "mass of product")],
     outputs=[_o("F", "mass", "feed mass"), _o("W", "mass", "tails mass"), _o("SWU", "separative_work", "separative work (kg SWU)"),
              _o("F_per_P", "number", "feed per unit product"), _o("SWU_per_P", "number", "kg SWU per kg of product")],
     primary="SWU", conditions=["tails < feed < product assays", "value function V(x) = (2x - 1) ln(x/(1 - x))"],
     method="F = P (x_p - x_w)/(x_f - x_w), W = F - P; SWU = P V(x_p) + W V(x_w) - F V(x_f)",
     latex=r"SWU = P\,V(x_p) + W\,V(x_w) - F\,V(x_f),\quad V(x) = (2x-1)\ln\frac{x}{1-x}",
     run=swu, reference=_ref_swu,
     wrong=lambda x_p, x_f, x_w, P: (lambda F: _out(F=F, W=F - P, SWU=P * _swu_value(x_p) - F * _swu_value(x_f), F_per_P=F / P, SWU_per_P=0.0))(P * (x_p - x_w) / (x_f - x_w)),
     rand=lambda rng: (lambda xw, xf: {"x_w": xw, "x_f": xf, "x_p": round(rng.uniform(xf * 1.5, 0.9), 5), "P": _sig(_lu(rng, 0.1, 1e5))})(
         round(rng.uniform(0.0005, 0.004), 5), round(rng.uniform(0.005, 0.02), 5)),
     phrases=("Enrichment SWU", "Separative work", "Uranium enrichment"),
     examples=[_ex({"x_p": 0.045, "x_f": 0.00711, "x_w": 0.0025, "P": 1.0}, written={"x_p": "4.5%", "x_f": "0.711%", "x_w": "0.25%", "P": "1 kg"}),
               _ex({"x_p": 0.2, "x_f": 0.00711, "x_w": 0.003, "P": 100.0}, written={"x_p": "20%", "x_f": "0.711%", "x_w": "0.3%", "P": "100 kg"})],
     level="intermediate",
     extra_q=[{"q": "Enrichment SWU: x_p = 4.5%, x_f = 0.711%, x_w = 1%, P = 1 kg", "outcome": "INVALID_INPUT"}])


# ------------------------------------------------------------------------------------------------ fuel-rod heat transfer
def fuel_rod_temperatures(q_lin, R_f, k_f, h_g, t_c, k_c, h, T_cool):
    _pos("q', the radii, the conductivities, the conductances and the coolant temperature", q_lin, R_f, k_f, h_g, t_c, k_c, h, T_cool)
    Ro = R_f + t_c
    T_co = T_cool + q_lin / (2 * math.pi * Ro * h)
    T_ci = T_co + q_lin * math.log(Ro / R_f) / (2 * math.pi * k_c)
    T_fs = T_ci + q_lin / (2 * math.pi * R_f * h_g)
    T_0 = T_fs + q_lin / (4 * math.pi * k_f)
    return _out(T_0=T_0, T_fs=T_fs, T_ci=T_ci)


def _ref_fuel_rod_temperatures(q_lin, R_f, k_f, h_g, t_c, k_c, h, T_cool):
    Ro = R_f + t_c
    T_co = T_cool + q_lin / (2 * math.pi * Ro * h)              # film (surface heat flux q'/(2 pi Ro) over h)
    # cladding: conduction ODE dT/dr = -q'/(2 pi r k_c), integrated inward from Ro to R_f
    T_ci = _ivp(lambda r, y: [-q_lin / (2 * math.pi * r * k_c)], R_f, [T_co], t0=Ro)[0]
    T_fs = T_ci + q_lin / (2 * math.pi * R_f * h_g)             # gap conductance
    # fuel: uniform source q''' = q'/(pi R_f^2); heat balance gives dT/dr = -q''' r/(2 k_f); integrated inward
    qv = q_lin / (math.pi * R_f * R_f)
    T_0 = _ivp(lambda r, y: [-qv * r / (2 * k_f)], 0.0, [T_fs], t0=R_f)[0]
    return {"T_0": T_0, "T_fs": T_fs, "T_ci": T_ci}


_add("nuc_fuel_rod_temperatures",
     name="Fuel-rod radial temperatures: centreline, pellet surface and cladding", topic="thermal hydraulics / fuel rods",
     purpose="The temperatures at the fuel centreline, the pellet surface and the cladding inner and outer surfaces of a cylindrical fuel rod at a stated linear heat rate, from the coolant temperature through the film, cladding, gap and pellet (constant conductivities).",
     inputs=[_phys("q_lin", "linear_heat_rate", "linear heat rate q' (W/m)"), _phys("R_f", "length", "pellet radius"),
             _phys("k_f", "thermal_conductivity", "fuel thermal conductivity"), _phys("h_g", "heat_transfer_coefficient", "gap conductance"),
             _phys("t_c", "length", "cladding thickness"), _phys("k_c", "thermal_conductivity", "cladding thermal conductivity"),
             _phys("h", "heat_transfer_coefficient", "coolant heat transfer coefficient"), _phys("T_cool", "temperature", "bulk coolant temperature")],
     outputs=[_o("T_0", "temperature", "fuel centreline temperature"), _o("T_fs", "temperature", "pellet surface temperature"),
              _o("T_ci", "temperature", "cladding inner surface temperature")],
     primary="T_0", conditions=["steady state, radial conduction only", "uniform heat generation in the pellet", "thin gap of conductance h_g at the pellet radius"],
     method="T_co = T_b + q'/(2 pi R_o h); T_ci = T_co + q' ln(R_o/R_f)/(2 pi k_c); T_fs = T_ci + q'/(2 pi R_f h_g); T_0 = T_fs + q'/(4 pi k_f)",
     latex=r"T_0 - T_b = q'\left[\frac{1}{4\pi k_f} + \frac{1}{2\pi R_fh_g} + \frac{\ln(R_o/R_f)}{2\pi k_c} + \frac{1}{2\pi R_oh}\right]",
     run=fuel_rod_temperatures, reference=_ref_fuel_rod_temperatures,
     wrong=lambda q_lin, R_f, k_f, h_g, t_c, k_c, h, T_cool: (lambda o: {**o, "T_0": _r(o["T_fs"] + q_lin / (2 * math.pi * k_f))})(fuel_rod_temperatures(q_lin, R_f, k_f, h_g, t_c, k_c, h, T_cool)),
     rand=lambda rng: {"q_lin": _sig(_lu(rng, 5e3, 6e4)), "R_f": _sig(rng.uniform(0.003, 0.007)), "k_f": _sig(rng.uniform(2.0, 6.0)),
                       "h_g": _sig(_lu(rng, 2e3, 2e4)), "t_c": _sig(rng.uniform(0.0004, 0.001)), "k_c": _sig(rng.uniform(10.0, 25.0)),
                       "h": _sig(_lu(rng, 1e4, 1e5)), "T_cool": _sig(rng.uniform(300.0, 620.0))},
     phrases=("Fuel rod temperatures", "Fuel centreline temperature", "Fuel rod radial temperature profile"),
     examples=[_ex({"q_lin": 20000.0, "R_f": 0.0041, "k_f": 3.0, "h_g": 5000.0, "t_c": 0.00057, "k_c": 15.0, "h": 34000.0, "T_cool": 580.0},
                   written={"q_lin": "200 W/cm", "R_f": "4.1 mm", "k_f": "3 W/(m*K)", "h_g": "5000 W/(m^2*K)", "t_c": "0.57 mm", "k_c": "15 W/(m*K)",
                            "h": "34000 W/(m^2*K)", "T_cool": "580 K"}),
               _ex({"q_lin": 40000.0, "R_f": 0.005, "k_f": 2.5, "h_g": 10000.0, "t_c": 0.0007, "k_c": 17.0, "h": 50000.0, "T_cool": 600.0},
                   written={"q_lin": "40 kW/m", "R_f": "5 mm", "k_f": "2.5 W/(m*K)", "h_g": "10000 W/(m^2*K)", "t_c": "0.7 mm", "k_c": "17 W/(m*K)",
                            "h": "50000 W/(m^2*K)", "T_cool": "600 K"})],
     level="intermediate")


def centreline_variable_k(q_lin, T_s, A, B):
    _pos("q', the surface temperature, A and B", q_lin, T_s, A, B)
    with _mp.workdps(30):
        T0 = float(((A + B * _mpf(T_s)) * _mp.exp(B * _mpf(q_lin) / (4 * _mp.pi)) - A) / B)
    return _out(T_0=T0, dT=T0 - T_s, k_s=1.0 / (A + B * T_s), k_0=1.0 / (A + B * T0))


def _ref_centreline_variable_k(q_lin, T_s, A, B):
    k = lambda T: 1.0 / (A + B * T)
    Rp = 1.0                                         # the result does not depend on the pellet radius
    qv = q_lin / (math.pi * Rp * Rp)

    def surface(T0):
        # shoot from the centre: k(T) dT/dr = -q''' r/2 (heat balance on a cylinder of radius r)
        return _ivp(lambda r, y: [-qv * r / (2 * k(y[0]))], Rp, [T0])[0]
    hi = T_s + 1.0
    while surface(hi) < T_s:
        hi = T_s + 2 * (hi - T_s)
    T0 = _brentq(lambda T0: surface(T0) - T_s, T_s, hi)
    return {"T_0": T0, "dT": T0 - T_s, "k_s": k(T_s), "k_0": k(T0)}


_add("nuc_centreline_temperature_variable_k",
     name="Fuel centreline temperature with temperature-dependent conductivity", topic="thermal hydraulics / fuel rods",
     purpose="The centreline temperature of a cylindrical fuel pellet at a stated linear heat rate and surface temperature when its conductivity varies as k = 1/(A + B T) (the usual form for UO2), through the conductivity integral, with the conductivities at the surface and the centre.",
     inputs=[_phys("q_lin", "linear_heat_rate", "linear heat rate q'"), _phys("T_s", "temperature", "pellet surface temperature"),
             _phys("A", "thermal_resistivity", "A in k = 1/(A + B T) (m*K/W)"), _phys("B", "thermal_resistivity_slope", "B in k = 1/(A + B T) (m/W)")],
     outputs=[_o("T_0", "temperature", "centreline temperature"), _o("dT", "temperature_difference", "centre-to-surface temperature rise"),
              _o("k_s", "thermal_conductivity", "conductivity at the surface"), _o("k_0", "thermal_conductivity", "conductivity at the centre")],
     primary="T_0", conditions=["uniform heat generation", "radial conduction, steady state", "k(T) = 1/(A + B T) with T in K"],
     method="int_{Ts}^{T0} k dT = q'/(4 pi): T0 = [(A + B Ts) e^(B q'/(4 pi)) - A]/B",
     latex=r"\int_{T_s}^{T_0}k(T)\,dT = \frac{q'}{4\pi}",
     run=centreline_variable_k, reference=_ref_centreline_variable_k,
     wrong=lambda q_lin, T_s, A, B: (lambda T0: _out(T_0=T0, dT=T0 - T_s, k_s=1 / (A + B * T_s), k_0=1 / (A + B * T0)))(T_s + q_lin / (4 * math.pi) * (A + B * T_s)),
     rand=lambda rng: {"q_lin": _sig(_lu(rng, 5e3, 5e4)), "T_s": _sig(rng.uniform(500.0, 1100.0)), "A": _sig(rng.uniform(0.03, 0.06)),
                       "B": _sig(rng.uniform(1.5e-4, 3e-4))},
     phrases=("Centreline temperature variable conductivity", "Centerline temperature with variable conductivity", "Fuel centreline temperature with k(T)"),
     examples=[_ex({"q_lin": 30000.0, "T_s": 800.0, "A": 0.0452, "B": 2.46e-4}, written={"q_lin": "300 W/cm", "T_s": "800 K", "A": "0.0452 m*K/W", "B": "2.46e-4 m/W"}),
               _ex({"q_lin": 15000.0, "T_s": 700.0, "A": 0.04, "B": 2e-4}, written={"q_lin": "15 kW/m", "T_s": "700 K", "A": "0.04 m*K/W", "B": "2e-4 m/W"})],
     level="advanced")


_CH_IN = [_phys("q0", "linear_heat_rate", "peak linear heat rate (at the core mid-plane)"), _phys("H", "length", "heated (core) height"),
          _phys("H_e", "length", "extrapolated height of the chopped-cosine power shape (H_e >= H)"),
          _phys("m_dot", "mass_flow_rate", "coolant mass flow rate in the channel"), _phys("c_p", "specific_heat", "coolant specific heat"),
          _phys("T_in", "temperature", "coolant inlet temperature")]


def _ch_check(q0, H, H_e, m_dot, c_p, T_in):
    _pos("q0, H, H_e, the flow, c_p and T_in", q0, H, H_e, m_dot, c_p, T_in)
    _need(H_e >= H, "the extrapolated height must be at least the heated height")


def channel_temperature(q0, H, H_e, m_dot, c_p, T_in, z):
    _ch_check(q0, H, H_e, m_dot, c_p, T_in)
    _need(_isnum(z) and abs(z) <= H / 2, "z must lie within the core (|z| <= H/2), measured from the mid-plane")
    k = q0 * H_e / (math.pi * m_dot * c_p)
    s0 = math.sin(math.pi * H / (2 * H_e))
    return _out(T=T_in + k * (math.sin(math.pi * z / H_e) + s0), T_out=T_in + 2 * k * s0, Q=2 * q0 * H_e / math.pi * s0)


def _ref_channel_temperature(q0, H, H_e, m_dot, c_p, T_in, z):
    q = lambda s: q0 * math.cos(math.pi * s / H_e)
    Qz = _quad(q, -H / 2, z) if z > -H / 2 else 0.0
    Q = _quad(q, -H / 2, H / 2)
    return {"T": T_in + Qz / (m_dot * c_p), "T_out": T_in + Q / (m_dot * c_p), "Q": Q}


def _rand_channel(rng):
    H = _sig(rng.uniform(1.0, 4.5))
    return {"q0": _sig(_lu(rng, 1e4, 6e4)), "H": H, "H_e": _sig(H * rng.uniform(1.0, 1.3)), "m_dot": _sig(rng.uniform(0.1, 0.5)),
            "c_p": _sig(rng.uniform(4000.0, 6000.0)), "T_in": _sig(rng.uniform(500.0, 570.0)), "z": _sig(H * rng.uniform(-0.5, 0.5))}


_add("nuc_channel_coolant_temperature",
     name="Coolant temperature along a channel with a chopped-cosine power shape", topic="thermal hydraulics / coolant channels",
     purpose="The bulk coolant temperature at a height z (from the core mid-plane) in a heated channel whose linear heat rate follows a chopped cosine, the outlet temperature and the channel's total power.",
     inputs=[*_CH_IN, _phys("z", "length", "height above the core mid-plane (negative below)")],
     outputs=[_o("T", "temperature", "coolant temperature at z"), _o("T_out", "temperature", "coolant outlet temperature"),
              _o("Q", "power", "total heat added to the channel")],
     primary="T", conditions=["steady state, single-phase coolant, constant c_p", "q'(z) = q0 cos(pi z/H_e)"],
     method="T(z) = T_in + (q0 H_e/(pi m c_p)) [sin(pi z/H_e) + sin(pi H/(2 H_e))]",
     latex=r"T(z) = T_{in} + \frac{q_0H_e}{\pi\dot mc_p}\left[\sin\frac{\pi z}{H_e} + \sin\frac{\pi H}{2H_e}\right]",
     run=channel_temperature, reference=_ref_channel_temperature,
     wrong=lambda q0, H, H_e, m_dot, c_p, T_in, z: (lambda k: _out(T=T_in + k * (math.sin(math.pi * z / H_e) + 1), T_out=T_in + 2 * k, Q=2 * q0 * H_e / math.pi))(
         q0 * H_e / (math.pi * m_dot * c_p)),
     rand=_rand_channel,
     phrases=("Channel coolant temperature", "Coolant temperature along a channel", "Coolant channel temperature"),
     examples=[_ex({"q0": 43000.0, "H": 3.66, "H_e": 4.0, "m_dot": 0.3, "c_p": 5500.0, "T_in": 565.0, "z": 0.0},
                   written={"q0": "43 kW/m", "H": "3.66 m", "H_e": "4 m", "m_dot": "0.3 kg/s", "c_p": "5500 J/(kg*K)", "T_in": "565 K", "z": "0 m"}),
               _ex({"q0": 30000.0, "H": 2.0, "H_e": 2.0, "m_dot": 0.2, "c_p": 4200.0, "T_in": 520.0, "z": 0.5},
                   written={"q0": "30 kW/m", "H": "2 m", "H_e": "2 m", "m_dot": "0.2 kg/s", "c_p": "4200 J/(kg*K)", "T_in": "520 K", "z": "50 cm"})],
     level="advanced")


def max_clad_temperature(q0, H, H_e, m_dot, c_p, T_in, R_o, h):
    _ch_check(q0, H, H_e, m_dot, c_p, T_in)
    _pos("the cladding radius and the heat transfer coefficient", R_o, h)
    k = q0 * H_e / (math.pi * m_dot * c_p)
    s0 = math.sin(math.pi * H / (2 * H_e))
    Tc = lambda z: T_in + k * (math.sin(math.pi * z / H_e) + s0) + q0 * math.cos(math.pi * z / H_e) / (2 * math.pi * R_o * h)
    z = H_e / math.pi * math.atan(2 * R_o * h * H_e / (m_dot * c_p))
    z = min(z, H / 2)
    return _out(z_max=z, T_max=Tc(z), T_out=T_in + 2 * k * s0)


def _ref_max_clad_temperature(q0, H, H_e, m_dot, c_p, T_in, R_o, h):
    import sympy as sp
    zs = sp.Symbol("z", real=True)
    k = q0 * H_e / (sp.pi * m_dot * c_p)
    Tc = T_in + k * (sp.sin(sp.pi * zs / H_e) + sp.sin(sp.pi * H / (2 * H_e))) + q0 * sp.cos(sp.pi * zs / H_e) / (2 * sp.pi * R_o * h)
    d = sp.lambdify(zs, sp.diff(Tc, zs), "math")                    # symbolic derivative of the clad temperature
    f = sp.lambdify(zs, Tc, "math")
    z = _brentq(d, -H / 2, H / 2) if d(H / 2) < 0 else H / 2
    Q = _quad(lambda s: q0 * math.cos(math.pi * s / H_e), -H / 2, H / 2)
    return {"z_max": z, "T_max": f(z), "T_out": T_in + Q / (m_dot * c_p)}


_add("nuc_max_clad_temperature",
     name="Location and value of the maximum cladding surface temperature", topic="thermal hydraulics / coolant channels",
     purpose="Where along a channel with a chopped-cosine power shape the cladding surface is hottest (above the mid-plane, because the coolant warms up), its temperature there and the coolant outlet temperature.",
     inputs=[*_CH_IN, _phys("R_o", "length", "cladding outer radius"), _phys("h", "heat_transfer_coefficient", "coolant heat transfer coefficient")],
     outputs=[_o("z_max", "length", "height of the maximum cladding temperature above the mid-plane"),
              _o("T_max", "temperature", "maximum cladding surface temperature"), _o("T_out", "temperature", "coolant outlet temperature")],
     primary="z_max", conditions=["single-phase coolant, constant h and c_p", "q'(z) = q0 cos(pi z/H_e)", "the maximum may sit at the core exit"],
     method="dT_c/dz = 0: tan(pi z/H_e) = 2 R_o h H_e/(m c_p); T_c = T(z) + q'(z)/(2 pi R_o h)",
     latex=r"\tan\frac{\pi z_{max}}{H_e} = \frac{2R_ohH_e}{\dot mc_p}",
     run=max_clad_temperature, reference=_ref_max_clad_temperature,
     wrong=lambda q0, H, H_e, m_dot, c_p, T_in, R_o, h: (lambda o: {**o, "z_max": 0.0})(max_clad_temperature(q0, H, H_e, m_dot, c_p, T_in, R_o, h)),
     rand=lambda rng: dict({k: v for k, v in _rand_channel(rng).items() if k != "z"}, R_o=_sig(rng.uniform(0.004, 0.0075)), h=_sig(_lu(rng, 1e4, 6e4))),
     phrases=("Maximum cladding temperature", "Maximum clad temperature", "Hottest cladding point"),
     examples=[_ex({"q0": 43000.0, "H": 3.66, "H_e": 4.0, "m_dot": 0.3, "c_p": 5500.0, "T_in": 565.0, "R_o": 0.00475, "h": 34000.0},
                   written={"q0": "43 kW/m", "H": "3.66 m", "H_e": "4 m", "m_dot": "0.3 kg/s", "c_p": "5500 J/(kg*K)", "T_in": "565 K", "R_o": "4.75 mm",
                            "h": "34000 W/(m^2*K)"}),
               _ex({"q0": 30000.0, "H": 2.0, "H_e": 2.2, "m_dot": 0.2, "c_p": 4200.0, "T_in": 520.0, "R_o": 0.005, "h": 20000.0},
                   written={"q0": "30 kW/m", "H": "2 m", "H_e": "2.2 m", "m_dot": "0.2 kg/s", "c_p": "4200 J/(kg*K)", "T_in": "520 K", "R_o": "5 mm",
                            "h": "20000 W/(m^2*K)"})],
     level="expert")


# ================================================================================================ MEASUREMENTS AND THERMAL NEUTRONS
def thermal_speeds(T):
    _pos("the temperature", T)
    vp = math.sqrt(2 * KB * T / MN)
    return _out(v_p=vp, v_mean=2 * vp / math.sqrt(math.pi), v_rms=vp * math.sqrt(1.5), E_mean=1.5 * KB * T, E_mean_eV=1.5 * KB * T / QE)


def _ref_thermal_speeds(T):
    a = MN / (2 * KB * T)
    s = 1.0 / math.sqrt(a)                                          # speed scale: v = s u
    n = lambda u: u * u * math.exp(-u * u)                          # Maxwell speed density in u (unnormalised)
    norm = _quad(n, 0.0, 12.0)
    vmean = s * _quad(lambda u: u * n(u), 0.0, 12.0) / norm
    v2 = s * s * _quad(lambda u: u * u * n(u), 0.0, 12.0) / norm
    vp = s * _brentq(lambda u: 2 * u * math.exp(-u * u) * (1 - u * u), 0.5, 2.0)    # d n/du = 0 (u > 0)
    E = 0.5 * MN * v2
    return {"v_p": vp, "v_mean": vmean, "v_rms": math.sqrt(v2), "E_mean": E, "E_mean_eV": E / QE}


_add("nuc_thermal_neutron_speeds",
     name="Speeds and mean energy of thermal neutrons (Maxwell distribution)", topic="neutron physics / thermal neutrons",
     purpose="The most probable, mean and root-mean-square speeds and the mean kinetic energy of neutrons in thermal equilibrium (Maxwell-Boltzmann distribution) at a stated temperature.",
     inputs=[_phys("T", "temperature", "neutron (moderator) temperature")],
     outputs=[_o("v_p", "speed", "most probable speed sqrt(2kT/m)"), _o("v_mean", "speed", "mean speed"), _o("v_rms", "speed", "root-mean-square speed"),
              _o("E_mean", "energy", "mean kinetic energy 3kT/2"), _o("E_mean_eV", "number", "mean kinetic energy in eV")],
     primary="v_p", conditions=["Maxwell-Boltzmann distribution of the neutron density", "neutron mass 1.67492750056e-27 kg"],
     method="v_p = sqrt(2kT/m_n), v_mean = 2 v_p/sqrt(pi), v_rms = sqrt(3/2) v_p, E_mean = 3kT/2",
     latex=r"v_p = \sqrt{\frac{2kT}{m_n}},\quad \bar v = \frac{2}{\sqrt\pi}v_p",
     run=thermal_speeds, reference=_ref_thermal_speeds,
     wrong=lambda T: (lambda vp: _out(v_p=vp, v_mean=vp, v_rms=vp, E_mean=KB * T, E_mean_eV=KB * T / QE))(math.sqrt(KB * T / MN)),
     rand=lambda rng: {"T": _sig(rng.uniform(20.0, 2000.0))},
     phrases=("Thermal neutron speeds", "Thermal neutron speed", "Maxwellian neutron speeds"),
     examples=[_ex({"T": 293.6}, written={"T": "293.6 K"}), _ex({"T": 600.0}, written={"T": "600 K"})],
     level="beginner")


def attenuation_fit(x, I):
    _need(len(x) == len(I) and len(x) >= 3, "give at least three thicknesses with their intensities")
    _need(all(_isnum(v) and v >= 0 for v in x) and len(set(x)) == len(x), "thicknesses must be distinct and >= 0")
    _need(all(_isnum(v) and v > 0 for v in I), "intensities must be positive")
    xs = [v / 100.0 for v in x]
    n = len(xs)
    y = [math.log(v) for v in I]
    mx, my = math.fsum(xs) / n, math.fsum(y) / n
    s = math.fsum((u - mx) * (w - my) for u, w in zip(xs, y)) / math.fsum((u - mx) ** 2 for u in xs)
    mu = -s
    _need(mu > 0, "the intensity does not fall with thickness: no attenuation coefficient")
    return _out(mu=mu, HVL=LN2 / mu, TVL=math.log(10.0) / mu, I0=math.exp(my - s * mx))


def _ref_attenuation_fit(x, I):
    import numpy as np
    p = np.polyfit(np.array(x, dtype=float) / 100.0, np.log(np.array(I, dtype=float)), 1)
    mu = -float(p[0])
    return {"mu": mu, "HVL": LN2 / mu, "TVL": math.log(10.0) / mu, "I0": float(np.exp(p[1]))}


def _rand_att_fit(rng):
    mu = _lu(rng, 0.05, 2.0)                                # 1/cm
    n = rng.randint(3, 8)
    xs = sorted(rng.sample([round(0.5 * k, 1) for k in range(0, 40)], n))
    I0 = _lu(rng, 1e2, 1e6)
    return {"x": xs, "I": [_sig(I0 * math.exp(-mu * v) * rng.uniform(0.98, 1.02)) for v in xs]}


_add("nuc_attenuation_coefficient_fit",
     name="Linear attenuation coefficient from narrow-beam transmission measurements", topic="shielding and dose / attenuation",
     purpose="The linear attenuation coefficient of an absorber from intensities measured behind several thicknesses (least-squares fit of ln I against x), with the half- and tenth-value layers and the fitted unattenuated intensity.",
     inputs=[_vecin("x", "absorber thicknesses, in cm", 3, 100, unit="cm"), _vecin("I", "measured intensities or count rates (any consistent unit)", 3, 100)],
     outputs=[_o("mu", "linear_attenuation_coefficient", "fitted linear attenuation coefficient"), _o("HVL", "length", "half-value layer"),
              _o("TVL", "length", "tenth-value layer"), _o("I0", "number", "fitted intensity with no absorber")],
     primary="mu", conditions=["narrow-beam geometry (no buildup)", "background already subtracted", "unweighted least squares of ln I"],
     method="least squares of ln I = ln I0 - mu x; HVL = ln 2/mu, TVL = ln 10/mu",
     latex=r"\ln I = \ln I_0 - \mu x",
     run=attenuation_fit, reference=_ref_attenuation_fit,
     wrong=lambda x, I: (lambda mu: _out(mu=mu, HVL=LN2 / mu, TVL=math.log(10) / mu, I0=I[0]))(math.log(I[0] / I[-1]) / ((x[-1] - x[0]) / 100.0) / 2),
     rand=_rand_att_fit,
     phrases=("Attenuation coefficient fit", "Attenuation coefficient from transmission", "Fit the attenuation coefficient"),
     examples=[_ex({"x": [0.0, 1.0, 2.0, 3.0, 4.0], "I": [1000.0, 520.0, 270.0, 141.0, 73.0]}),
               _ex({"x": [0.0, 5.0, 10.0], "I": [5000.0, 2450.0, 1210.0]})],
     level="beginner",
     extra_q=[{"q": "Attenuation coefficient fit: x = [0, 1, 2, 3, 4] cm, I = [1000, 520, 270, 141, 73]", "like": 0}])


def count_rate_point_source(A, y, d, a, eps_int):
    _pos("the activity, the distance and the radius", A, d, a)
    _frac("the emission probability", y, lo_open=True)
    _frac("the intrinsic efficiency", eps_int, lo_open=True)
    g = (1 - d / math.hypot(d, a)) / 2
    return _out(R=A * y * g * eps_int, g=g, R_cpm=A * y * g * eps_int * 60.0)


def _ref_count_rate_point_source(A, y, d, a, eps_int):
    g = _quad(lambda rho: d * rho / (2 * (d * d + rho * rho) ** 1.5), 0.0, a)      # fraction of 4 pi through the face
    return {"R": A * y * g * eps_int, "g": g, "R_cpm": A * y * g * eps_int * 60.0}


_add("nuc_count_rate_point_source",
     name="Expected count rate from a point source on a detector's axis", topic="detectors and counting / efficiency",
     purpose="The count rate a detector of stated intrinsic efficiency records from a point source on its axis, from the activity, the emission probability per decay and the geometric efficiency of the detector face.",
     inputs=[_phys("A", "radioactivity", "source activity"), _numin("y", "emission probability of the counted radiation per decay", pct=True),
             _phys("d", "length", "source-to-face distance"), _phys("a", "length", "radius of the detector face"),
             _numin("eps_int", "intrinsic efficiency (counts per radiation reaching the face)", pct=True)],
     outputs=[_o("R", "count_rate", "expected count rate"), _o("g", "number", "geometric efficiency"), _o("R_cpm", "number", "expected count rate in counts per minute")],
     primary="R", conditions=["point source on the axis", "no dead time, no attenuation between source and detector"],
     method="R = A y g eps_int, g = (1 - d/sqrt(d^2 + a^2))/2",
     latex=r"R = A\,y\,\varepsilon_{int}\,\frac{1}{2}\left(1-\frac{d}{\sqrt{d^2+a^2}}\right)",
     run=count_rate_point_source, reference=_ref_count_rate_point_source,
     wrong=lambda A, y, d, a, eps_int: (lambda g: _out(R=A * y * g * eps_int, g=g, R_cpm=A * y * g * eps_int * 60))(a * a / (4 * d * d)),
     rand=lambda rng: (lambda a: {"A": _sig(_lu(rng, 1e2, 1e9)), "y": round(rng.uniform(0.05, 1.0), 4), "a": a, "d": _sig(a * _lu(rng, 0.05, 50.0)),
                                  "eps_int": round(rng.uniform(0.05, 1.0), 4)})(_sig(_lu(rng, 0.005, 0.1))),
     phrases=("Count rate from a point source", "Expected count rate", "Detector count rate"),
     examples=[_ex({"A": 37000.0, "y": 0.851, "d": 0.1, "a": 0.0254, "eps_int": 0.6}, written={"A": "1 uCi", "y": "85.1%", "d": "10 cm", "a": "2.54 cm", "eps_int": "60%"}),
               _ex({"A": 1e5, "y": 1.0, "d": 0.01, "a": 0.01, "eps_int": 1.0}, written={"A": "100 kBq", "d": "1 cm", "a": "1 cm"})],
     level="beginner")


def flux_from_foil(A_d, m, M, theta, sigma, T_half, t_irr, t_d):
    _pos("the measured activity, mass, molar mass, cross section, half-life and irradiation time", A_d, m, M, sigma, T_half, t_irr)
    _frac("the isotopic abundance theta", theta, lo_open=True)
    _nonneg("the decay time t_d", t_d)
    lam = LN2 / T_half
    N = m / M * NA * theta
    A_sat = A_d * math.exp(lam * t_d) / -math.expm1(-lam * t_irr)
    return _out(phi=A_sat / (N * sigma), A_sat=A_sat, R_atom=A_sat / N)


def _ref_flux_from_foil(A_d, m, M, theta, sigma, T_half, t_irr, t_d):
    lam = LN2 / T_half
    N = m / M * NA * theta
    # activity per unit flux: the activation equation integrated (scaled by 1/lambda), then free decay
    n = _ivp(lambda s, y: [1.0 - y[0]], lam * t_irr, [0.0], atol=[1e-30])[0]
    nd = _ivp(lambda s, y: [-y[0]], lam * t_d, [n], atol=[1e-300])[0] if t_d > 0 else n
    unit = N * sigma * nd                               # activity at the count for a flux of 1
    phi = A_d / unit
    return {"phi": phi, "A_sat": N * sigma * phi, "R_atom": sigma * phi}


_add("nuc_flux_from_foil_activation",
     name="Neutron flux from a measured foil activation", topic="detectors and counting / activation foils",
     purpose="The neutron flux that irradiated an activation foil, from the foil's measured activity a decay time after an irradiation of known length, its mass, molar mass, isotopic abundance and activation cross section; with the saturation activity and the reaction rate per atom.",
     inputs=[_phys("A_d", "radioactivity", "foil activity measured at the count"), _phys("m", "mass", "foil mass (target element)"),
             _phys("M", "molar_mass", "molar mass of the target element"), _numin("theta", "isotopic abundance of the target nuclide", pct=True),
             _phys("sigma", "microscopic_cross_section", "activation cross section (barns)"), _phys("T_half", "time", "half-life of the product"),
             _phys("t_irr", "time", "irradiation time"), _phys("t_d", "time", "time from the end of irradiation to the count")],
     outputs=[_o("phi", "neutron_flux", "neutron flux"), _o("A_sat", "radioactivity", "saturation activity"),
              _o("R_atom", "decay_constant", "reaction rate per target atom sigma phi (1/s)")],
     primary="phi", conditions=["constant flux during the irradiation", "thin foil (no self-shielding), no target depletion"],
     method="A_sat = A_d e^(lambda t_d)/(1 - e^(-lambda t_irr)); phi = A_sat/(N sigma), N = m N_A theta/M",
     latex=r"\phi = \frac{A_d\,e^{\lambda t_d}}{N\sigma\left(1-e^{-\lambda t_{irr}}\right)}",
     run=flux_from_foil, reference=_ref_flux_from_foil,
     wrong=lambda A_d, m, M, theta, sigma, T_half, t_irr, t_d: (lambda N: _out(phi=A_d / (N * sigma), A_sat=A_d, R_atom=A_d / N))(m / M * NA * theta),
     rand=lambda rng: (lambda T: {"A_d": _sig(_lu(rng, 10.0, 1e8)), "m": _sig(_lu(rng, 1e-6, 1e-2)), "M": _sig(rng.uniform(0.02, 0.2)),
                                  "theta": round(rng.uniform(0.01, 1.0), 4), "sigma": _sig(_lu(rng, 0.1, 100.0)) * BARN, "T_half": _sig(T),
                                  "t_irr": _sig(T * _lu(rng, 0.02, 5.0)), "t_d": _sig(T * _lu(rng, 0.01, 3.0))})(_lu(rng, 60.0, 1e7)),
     phrases=("Flux from foil activation", "Neutron flux from an activation foil", "Foil activation flux"),
     examples=[_ex({"A_d": 2.0e4, "m": 1e-4, "M": 0.19697, "theta": 1.0, "sigma": 98.65e-28, "T_half": 232848.0, "t_irr": 3600.0, "t_d": 1800.0},
                   written={"A_d": "20 kBq", "m": "0.1 g", "M": "196.97 g/mol", "sigma": "98.65 b", "T_half": "2.695 d", "t_irr": "1 h", "t_d": "30 min"}),
               _ex({"A_d": 500.0, "m": 5e-4, "M": 0.11482, "theta": 0.957, "sigma": 157.0e-28, "T_half": 3257.4, "t_irr": 7200.0, "t_d": 600.0},
                   written={"A_d": "500 Bq", "m": "0.5 g", "M": "114.82 g/mol", "theta": "95.7%", "sigma": "157 b", "T_half": "54.29 min", "t_irr": "2 h", "t_d": "10 min"})],
     level="intermediate")


# ================================================================================================ DEFINITIONAL ADDITIONS
# (owner, 2026-10-23: a capability that passes every gate ships; a weaker reference is documented). Effective dose,
# committed dose from a dose coefficient, power from the reaction rate, four- and six-factor formula, diffusion length,
# Currie detection limits, escape peaks, half- and tenth-value layers, unit conversions, constant-power depletion.
_KU["dose_coefficient"] = "J*s/kg"                 # Sv/Bq


def effective_dose(H, w):
    n = len(H)
    _need(1 <= n <= 30 and len(w) == n, "give one tissue weighting factor per equivalent dose (1 to 30 tissues)")
    for x in H:
        _need(_isnum(x) and x >= 0, "every equivalent dose must be zero or positive")
    for x in w:
        _need(_isnum(x) and 0 < x <= 1, "every tissue weighting factor must lie in (0, 1]")
    sw = math.fsum(w)
    _need(sw <= 1 + 1e-9, "the tissue weighting factors must not sum to more than 1")
    E = math.fsum(a * b for a, b in zip(H, w))
    return _out(E_mSv=E, E=E * 1e-3, w_sum=sw)


def _ref_effective_dose(H, w):
    """The weighted sum as a numpy dot product of the tissue weighting factors and the equivalent doses."""
    import numpy as np
    E = float(np.dot(np.asarray(w, float), np.asarray(H, float)))
    return {"E_mSv": E, "E": E * 1e-3, "w_sum": float(np.sum(w))}


_add("nuc_effective_dose",
     name="Effective dose from organ equivalent doses and tissue weighting factors", topic="dosimetry / effective dose",
     purpose="The effective dose E = sum w_T H_T from the equivalent doses of the irradiated tissues and their tissue weighting factors (stated; ICRP 103: 0.12 for red marrow, colon, lung, stomach, breast and the remainder, 0.08 gonads, 0.04 bladder, oesophagus, liver, thyroid, 0.01 bone surface, brain, salivary glands, skin).",
     inputs=[_vecin("H", "equivalent dose of each tissue in mSv", 1, 30, unit="mSv"), _vecin("w", "tissue weighting factor of each tissue (as stated)", 1, 30)],
     outputs=[_o("E_mSv", "number", "effective dose in mSv"), _o("E", "equivalent_dose", "effective dose (Sv)"), _o("w_sum", "number", "sum of the weighting factors given")],
     primary="E_mSv", conditions=["one weighting factor per tissue, 0 < w_T <= 1, sum at most 1", "tissues not listed received no dose"],
     method="E = sum_T w_T H_T", latex=r"E = \sum_T w_T\,H_T",
     run=effective_dose, reference=_ref_effective_dose,
     wrong=lambda H, w: _out(E_mSv=math.fsum(H) / len(H), E=math.fsum(H) / len(H) * 1e-3, w_sum=math.fsum(w)),
     rand=lambda rng: (lambda n: {"H": [_sig(_lu(rng, 0.01, 100.0)) for _ in range(n)], "w": [rng.choice([0.12, 0.08, 0.04, 0.01]) for _ in range(n)]})(rng.randint(2, 8)),
     phrases=("Effective dose", "Effective dose from organ doses"),
     examples=[_ex({"H": [10.0, 5.0, 2.0], "w": [0.12, 0.04, 0.01]}), _ex({"H": [3.0, 3.0], "w": [0.12, 0.12]})],
     level="intermediate",
     extra_q=[{"q": "Effective dose: H = [10, 5], w = [0.6, 0.7]", "outcome": "INVALID_INPUT"}])


def committed_dose(I, e):
    _pos("the intake and the dose coefficient", I, e)
    _need(e <= 1e-3, "the dose coefficient must be at most 1e-3 Sv/Bq")
    E = I * e
    return _out(E=E, E_mSv=E * 1e3, I_Bq=I)


def _ref_committed_dose(I, e):
    """The intake in becquerel times the dose coefficient in sieverts per becquerel, in exact rational arithmetic
    (sympy)."""
    import sympy as sp
    E = sp.Rational(repr(I)) * sp.Rational(repr(e))
    return {"E": float(E), "E_mSv": float(E * 1000), "I_Bq": I}


_add("nuc_committed_dose_coefficient",
     name="Committed effective dose from a dose coefficient", topic="dosimetry / internal dose",
     purpose="The committed effective dose E(50) = e(50) I of an intake I of a radionuclide with the dose coefficient e(50) (Sv/Bq, from ICRP tables for the route and chemical form).",
     inputs=[_phys("I", "radioactivity", "activity taken in (Bq, kBq, MBq, Ci ...)"), _phys("e", "dose_coefficient", "dose coefficient e(50) (Sv/Bq)")],
     outputs=[_o("E", "equivalent_dose", "committed effective dose (Sv)"), _o("E_mSv", "number", "committed effective dose in mSv"),
              _o("I_Bq", "radioactivity", "intake in Bq")],
     primary="E_mSv", conditions=["dose coefficient for the route, particle size and chemical form of the intake", "e <= 1e-3 Sv/Bq"],
     method="E(50) = e(50) I", latex=r"E(50) = e(50)\,I",
     run=committed_dose, reference=_ref_committed_dose,
     wrong=lambda I, e: _out(E=I * e / 1000, E_mSv=I * e, I_Bq=I),
     rand=lambda rng: {"I": _sig(_lu(rng, 1.0, 1e9)), "e": _sig(_lu(rng, 1e-11, 1e-6))},
     phrases=("Committed dose from a dose coefficient", "Committed effective dose", "Dose coefficient dose"),
     examples=[_ex({"I": 1e6, "e": 2.2e-8}, written={"I": "1 MBq", "e": "2.2e-8 Sv/Bq"}),
               _ex({"I": 3.7e4, "e": 1.3e-8}, written={"I": "1 uCi", "e": "1.3e-8 Sv/Bq"})],
     level="beginner",
     extra_q=[{"q": "Committed dose from a dose coefficient: I = 1 MBq, e = -2.2e-8 Sv/Bq", "outcome": "INVALID_INPUT"}])


def reaction_power(E_f, N, sigma, phi, V=None):
    _pos("E_f, N, sigma and phi", E_f, N, sigma, phi)
    R = N * sigma * phi
    q = E_f * R
    out = dict(R=R, q=q, q_MW_m3=q / 1e6)
    if V is not None:
        _pos("the volume", V)
        out.update(P=q * V, P_MW=q * V / 1e6)
    return _out(**out)


def _ref_reaction_power(E_f, N, sigma, phi, V=None):
    """The reaction rate as the macroscopic cross section times the flux, built from scipy's barn and electron-volt
    constants in MeV-barn units and converted back; the power as the energy released per second in the volume."""
    from scipy import constants as sc
    Nb = N * 1e-28                                   # atoms per (m * barn)
    R = Nb * (sigma / 1e-28) * phi
    q = (E_f / (1e6 * sc.e)) * R * 1e6 * sc.e
    out = {"R": R, "q": q, "q_MW_m3": q / 1e6}
    if V is not None:
        out.update(P=q * V, P_MW=q * V / 1e6)
    return out


_add("nuc_power_from_reaction_rate",
     name="Power density and power from the fission reaction rate", topic="reactor physics / power and flux",
     purpose="The fission reaction rate density R = N sigma_f phi, the power density q''' = E_f N sigma_f phi and, with a volume, the power, from the fissile number density, the fission cross section, the flux and the energy per fission.",
     inputs=[_phys("E_f", "energy", "recoverable energy per fission (e.g. 200 MeV)"), _phys("N", "number_density", "fissile atoms per unit volume"),
             _phys("sigma", "microscopic_cross_section", "fission cross section (barns)"), _phys("phi", "neutron_flux", "neutron flux"),
             _phys("V", "volume", "volume (optional, for the power)", required=False)],
     outputs=[_o("R", "volumetric_reaction_rate", "fission rate density (1/(m^3 s))"), _o("q", "volumetric_heat_generation", "power density"),
              _o("q_MW_m3", "number", "power density in MW/m^3"), _o("P", "power", "power E_f N sigma phi V"), _o("P_MW", "number", "power in MW")],
     primary="q", conditions=["one-group cross section and flux", "uniform over the volume"],
     method="R = N sigma phi, q''' = E_f R, P = q''' V", latex=r"q''' = E_f N \sigma_f \phi,\quad P = q''' V",
     run=reaction_power, reference=_ref_reaction_power,
     wrong=lambda E_f, N, sigma, phi, V=None: (lambda q: _out(R=N * sigma * phi, q=q, q_MW_m3=q / 1e6, **({"P": q * V, "P_MW": q * V / 1e6} if V is not None else {})))(E_f * N * sigma * phi / 1.0e4),
     rand=lambda rng: {"E_f": _sig(rng.uniform(190, 210)) * MEV, "N": _sig(_lu(rng, 1e24, 1e28)), "sigma": _sig(_lu(rng, 1.0, 1000.0)) * BARN,
                       "phi": _sig(_lu(rng, 1e15, 1e19)), **({"V": _sig(_lu(rng, 1e-6, 100.0))} if rng.random() < 0.5 else {})},
     phrases=("Power from reaction rate", "Fission power density", "Power from fission rate"),
     examples=[_ex({"E_f": 200 * MEV, "N": 1e27, "sigma": 582.6e-28, "phi": 3e17}, written={"E_f": "200 MeV", "N": "1e21 1/cm^3", "sigma": "582.6 b", "phi": "3e13 1/(cm^2*s)"}),
               _ex({"E_f": 200 * MEV, "N": 4.8e26, "sigma": 582.6e-28, "phi": 1e17, "V": 30.0}, written={"E_f": "200 MeV", "N": "4.8e26 1/m^3", "sigma": "582.6 b", "phi": "1e13 1/(cm^2*s)", "V": "30 m^3"})],
     level="intermediate",
     extra_q=[{"q": "Power from reaction rate: E_f = 200 MeV, N = 1e21 1/cm^3, sigma = -582.6 b, phi = 3e13 1/(cm^2*s)", "outcome": "INVALID_INPUT"}])


def six_factor(eta, f, p, eps, P_FNL=None, P_TNL=None):
    for nm, v in (("eta", eta), ("eps", eps)):
        _need(_isnum(v) and 0 < v <= 3, f"{nm} must lie in (0, 3]")
    for nm, v in (("f", f), ("p", p)):
        _need(_isnum(v) and 0 < v <= 1, f"{nm} must lie in (0, 1]")
    for nm, v in (("P_FNL", P_FNL), ("P_TNL", P_TNL)):
        _need(v is None or (_isnum(v) and 0 < v <= 1), f"{nm} must lie in (0, 1]")
    kinf = eta * f * p * eps
    PN = (1.0 if P_FNL is None else P_FNL) * (1.0 if P_TNL is None else P_TNL)
    k = kinf * PN
    return _out(k_inf=kinf, k_eff=k, P_NL=PN, rho=(k - 1) / k)


def _ref_six_factor(eta, f, p, eps, P_FNL=None, P_TNL=None):
    """The neutron cycle followed generation by generation: n0 fast neutrons, times eps (fast fission), times P_FNL,
    p (escape resonances), P_TNL, f (absorbed in fuel), eta (new fast neutrons) -- the ratio of successive generations,
    with numpy.prod."""
    import numpy as np
    stages = [eps, 1.0 if P_FNL is None else P_FNL, p, 1.0 if P_TNL is None else P_TNL, f, eta]
    n1 = float(np.prod([1000.0] + stages))
    k = n1 / 1000.0
    kinf = float(np.prod([eta, f, p, eps]))
    return {"k_inf": kinf, "k_eff": k, "P_NL": k / kinf, "rho": (k - 1) / k}


_add("nuc_six_factor_formula",
     name="Multiplication factor by the four- and six-factor formulas", topic="reactor physics / neutron life cycle",
     purpose="k_inf = eta f p eps (four-factor formula) and, with the fast and thermal non-leakage probabilities, k_eff = k_inf P_FNL P_TNL (six-factor formula), with the reactivity (k - 1)/k.",
     inputs=[_numin("eta", "reproduction factor eta"), _numin("f", "thermal utilization f"), _numin("p", "resonance escape probability p"),
             _numin("eps", "fast fission factor epsilon"), _numin("P_FNL", "fast non-leakage probability (optional)", required=False, pct=True),
             _numin("P_TNL", "thermal non-leakage probability (optional)", required=False, pct=True)],
     outputs=[_o("k_inf", "number", "infinite multiplication factor"), _o("k_eff", "number", "effective multiplication factor (k_inf without leakage factors)"),
              _o("P_NL", "number", "total non-leakage probability"), _o("rho", "number", "reactivity (k_eff - 1)/k_eff")],
     primary="k_eff", conditions=["0 < f, p, P_FNL, P_TNL <= 1", "missing non-leakage factors are taken as 1 (an infinite medium)"],
     method="k_inf = eta f p eps; k_eff = k_inf P_FNL P_TNL", latex=r"k_{eff} = \eta f p \varepsilon\,P_{FNL}P_{TNL}",
     run=six_factor, reference=_ref_six_factor,
     wrong=lambda eta, f, p, eps, P_FNL=None, P_TNL=None: (lambda kinf: (lambda k: _out(k_inf=kinf, k_eff=k, P_NL=k / kinf, rho=(k - 1) / k))(
         kinf * (1.0 if P_FNL is None else P_FNL)))(eta * f * eps),
     rand=lambda rng: {"eta": round(rng.uniform(1.6, 2.1), 4), "f": round(rng.uniform(0.6, 0.95), 4), "p": round(rng.uniform(0.6, 0.9), 4), "eps": round(rng.uniform(1.0, 1.1), 4),
                       **({"P_FNL": round(rng.uniform(0.9, 0.99), 4), "P_TNL": round(rng.uniform(0.95, 0.995), 4)} if rng.random() < 0.6 else {})},
     phrases=("Six-factor formula", "Four-factor formula", "Multiplication factor from the factors"),
     examples=[_ex({"eta": 2.02, "f": 0.71, "p": 0.87, "eps": 1.03}), _ex({"eta": 1.65, "f": 0.9, "p": 0.75, "eps": 1.02, "P_FNL": 0.97, "P_TNL": 0.99})],
     level="beginner",
     extra_q=[{"q": "Six-factor formula: eta = 2.02, f = 1.3, p = 0.87, eps = 1.03", "outcome": "INVALID_INPUT"}])


def diffusion_length(D, Sigma_a, tau=None):
    _pos("D and Sigma_a", D, Sigma_a)
    L2 = D / Sigma_a
    out = dict(L=math.sqrt(L2), L2=L2, L_cm=math.sqrt(L2) * 100)
    if tau is not None:
        _pos("the Fermi age", tau)
        out.update(M2=L2 + tau, M=math.sqrt(L2 + tau))
    return _out(**out)


def _ref_diffusion_length(D, Sigma_a, tau=None):
    """The flux of a plane source in a thick absorber, D phi'' = Sigma_a phi with phi = 0 far away (x = X), integrated
    from X back to the source (scipy DOP853; the backward direction follows the growing, physical mode); the diffusion
    length is the relaxation length phi/(-phi') at the source (X is many lengths, so tanh(X/L) = 1 to double precision)."""
    X = 40.0 * math.sqrt(D / Sigma_a)                  # only the size of the domain, any thick absorber will do
    phi, dphi = _ivp(lambda x, s: [s[1], Sigma_a / D * s[0]], 0.0, [0.0, -1.0], t0=X)
    Lr = phi / -dphi
    out = {"L": Lr, "L2": Lr * Lr, "L_cm": Lr * 100}
    if tau is not None:
        out.update(M2=Lr * Lr + tau, M=math.sqrt(Lr * Lr + tau))
    return out


_add("nuc_diffusion_length",
     name="Thermal diffusion length and migration length", topic="neutron physics / diffusion",
     purpose="The thermal diffusion length L = sqrt(D/Sigma_a) of a medium and, with the Fermi age tau, the migration area M^2 = L^2 + tau and migration length.",
     inputs=[_phys("D", "neutron_diffusion_coefficient", "diffusion coefficient (cm)"), _phys("Sigma_a", "macroscopic_cross_section", "macroscopic absorption cross section"),
             _phys("tau", "area", "Fermi age to thermal (cm^2, optional)", required=False)],
     outputs=[_o("L", "length", "diffusion length"), _o("L2", "area", "diffusion area L^2"), _o("L_cm", "number", "diffusion length in cm"),
              _o("M2", "area", "migration area L^2 + tau"), _o("M", "length", "migration length")],
     primary="L", conditions=["D, Sigma_a > 0", "one-group (thermal) diffusion theory"],
     method="L^2 = D/Sigma_a; M^2 = L^2 + tau", latex=r"L = \sqrt{\frac{D}{\Sigma_a}},\quad M^2 = L^2 + \tau",
     run=diffusion_length, reference=_ref_diffusion_length,
     wrong=lambda D, Sigma_a, tau=None: (lambda L2: _out(L=L2, L2=L2, L_cm=L2 * 100, **({"M2": L2 + tau, "M": math.sqrt(L2 + tau)} if tau is not None else {})))(D / Sigma_a),
     rand=lambda rng: {"D": _sig(_lu(rng, 0.1, 2.0)) * 0.01, "Sigma_a": _sig(_lu(rng, 1e-4, 0.5)) * 100, **({"tau": _sig(_lu(rng, 25, 400)) * 1e-4} if rng.random() < 0.5 else {})},
     phrases=("Diffusion length", "Thermal diffusion length", "Migration length"),
     examples=[_ex({"D": 0.0016, "Sigma_a": 1.97}, written={"D": "0.16 cm", "Sigma_a": "0.0197 1/cm"}),
               _ex({"D": 0.0084, "Sigma_a": 0.032, "tau": 0.0368}, written={"D": "0.84 cm", "Sigma_a": "0.00032 1/cm", "tau": "368 cm^2"})],
     level="beginner",
     extra_q=[{"q": "Diffusion length: D = 0.16 cm, Sigma_a = 0 1/cm", "outcome": "INVALID_INPUT"}])

_K95 = 1.6448536269514722                           # standard normal quantile at 0.95 (alpha = beta = 0.05)


def currie(B, t=None, eff=None, y=1.0):
    _need(_isnum(B) and B >= 0, "the background counts must be zero or positive")
    k = _K95
    LC = k * math.sqrt(2.0 * B)
    LD = k * k + 2.0 * LC
    out = dict(L_C=LC, L_D=LD)
    if (t is None) != (eff is None):
        raise ValueError("the minimum detectable activity needs both the counting time t and the efficiency eff")
    if t is not None:
        _pos("t", t)
        _need(0 < eff <= 1 and 0 < y <= 1, "the efficiency and the emission yield must lie in (0, 1]")
        out["MDA"] = LD / (eff * y * t)
    return _out(**out)


def _ref_currie(B, t=None, eff=None, y=1.0):
    """Currie's limits with k from scipy's normal quantile (alpha = beta = 0.05): the critical level as the one-sided
    95% point of the net count of a paired blank, the detection limit as the net mean whose 5% point equals L_C
    (brentq on the normal tail with variance mu + 2B)."""
    from scipy.stats import norm
    from scipy.optimize import brentq
    k = float(norm.ppf(0.95))
    LC = k * math.sqrt(2.0 * B)
    # L_D: mu - k sqrt(mu + 2B) = L_C (the variance of the net count at mean mu, with a paired blank)
    LD = brentq(lambda mu: mu - k * math.sqrt(mu + 2.0 * B) - LC, 0.0, 10 * LC + 10 * k * k + 10, xtol=1e-14, rtol=8.9e-16)
    out = {"L_C": LC, "L_D": LD}
    if t is not None:
        out["MDA"] = LD / (eff * y * t)
    return out


_add("nuc_currie_detection_limits",
     name="Currie critical level, detection limit and minimum detectable activity", topic="detectors and counting / detection limits",
     purpose="Currie's (1968) decision threshold L_C = k sqrt(2B) and detection limit L_D = k^2 + 2 L_C (net counts, paired blank, alpha = beta = 0.05, k = 1.645) from the background counts B, and with the counting time, efficiency and emission yield the minimum detectable activity L_D/(eff y t).",
     inputs=[_numin("B", "background counts in the counting time"), _phys("t", "time", "counting time (optional, for the MDA)", required=False),
             _numin("eff", "counting efficiency (optional, with t)", required=False, pct=True), _numin("y", "emission yield per decay (default 1)", required=False, pct=True)],
     outputs=[_o("L_C", "number", "critical level (net counts)"), _o("L_D", "number", "detection limit (net counts)"),
              _o("MDA", "radioactivity", "minimum detectable activity (Bq)")],
     primary="L_D", conditions=["paired blank (same counting time)", "alpha = beta = 0.05", "Gaussian approximation of the counts"],
     method="L_C = k sqrt(2B), L_D = k^2 + 2 k sqrt(2B), k = z(0.95) = 1.645; MDA = L_D/(eff y t)",
     latex=r"L_C = k\sqrt{2B},\quad L_D = k^2 + 2k\sqrt{2B},\quad MDA = \frac{L_D}{\varepsilon\,y\,t}",
     run=currie, reference=_ref_currie,
     wrong=lambda B, t=None, eff=None, y=1.0: (lambda LC: _out(L_C=LC, L_D=_K95 ** 2 + LC, **({"MDA": (_K95 ** 2 + LC) / (eff * y * t)} if t is not None else {})))(_K95 * math.sqrt(2.0 * B)),
     rand=lambda rng: {"B": float(rng.randint(1, 100000)), **({"t": _sig(_lu(rng, 60, 86400)), "eff": round(rng.uniform(0.01, 0.6), 4),
                                                              **({"y": round(rng.uniform(0.1, 1.0), 4)} if rng.random() < 0.5 else {})} if rng.random() < 0.6 else {})},
     phrases=("Currie detection limits", "Detection limit", "Minimum detectable activity"),
     examples=[_ex({"B": 100.0}), _ex({"B": 400.0, "t": 600.0, "eff": 0.25, "y": 0.851}, written={"t": "10 min", "eff": "25%", "y": "85.1%"})],
     level="intermediate",
     extra_q=[{"q": "Currie detection limits: B = 100, t = 10 min", "outcome": "INVALID_INPUT"}])


def escape_peaks(E):
    _pos("the gamma-ray energy", E)
    Em = E / MEV
    _need(Em > 2 * ME_MEV, "escape peaks need pair production: the energy must exceed 1.022 MeV")
    return _out(single_escape_keV=(Em - ME_MEV) * 1e3, double_escape_keV=(Em - 2 * ME_MEV) * 1e3, annihilation_keV=ME_MEV * 1e3, E_keV=Em * 1e3)


def _ref_escape_peaks(E):
    """The positron annihilation photon energy from scipy's electron mass energy equivalent (CODATA); each escaping
    photon carries that energy out of the detector."""
    from scipy import constants as sc
    me = sc.physical_constants["electron mass energy equivalent in MeV"][0]
    Em = E / (1e6 * sc.e)
    return {"single_escape_keV": (Em - me) * 1e3, "double_escape_keV": (Em - 2 * me) * 1e3, "annihilation_keV": me * 1e3, "E_keV": Em * 1e3}


_add("nuc_escape_peaks",
     name="Single and double escape peaks of a gamma ray", topic="detectors and counting / gamma spectroscopy",
     purpose="The energies of the single- and double-escape peaks (E - m_e c^2 and E - 2 m_e c^2) that pair production by a gamma ray above 1.022 MeV produces in a detector.",
     inputs=[_phys("E", "energy", "gamma-ray energy (MeV, keV)")],
     outputs=[_o("single_escape_keV", "number", "single-escape peak in keV"), _o("double_escape_keV", "number", "double-escape peak in keV"),
              _o("annihilation_keV", "number", "annihilation photon energy in keV"), _o("E_keV", "number", "full-energy peak in keV")],
     primary="double_escape_keV", conditions=["E > 2 m_e c^2 = 1.022 MeV"],
     method="E_SE = E - m_e c^2, E_DE = E - 2 m_e c^2 (m_e c^2 = 0.51099895069 MeV)", latex=r"E_{SE} = E_\gamma - m_ec^2,\quad E_{DE} = E_\gamma - 2m_ec^2",
     run=escape_peaks, reference=_ref_escape_peaks,
     wrong=lambda E: (lambda Em: _out(single_escape_keV=(Em - 2 * ME_MEV) * 1e3, double_escape_keV=(Em - 4 * ME_MEV) * 1e3, annihilation_keV=ME_MEV * 1e3, E_keV=Em * 1e3))(E / MEV),
     rand=lambda rng: {"E": _sig(rng.uniform(1.1, 10.0)) * MEV},
     phrases=("Escape peaks", "Single and double escape peaks"),
     examples=[_ex({"E": 2.614 * MEV}, written={"E": "2614 keV"}), _ex({"E": 1.3325 * MEV}, written={"E": "1.3325 MeV"})],
     level="beginner",
     extra_q=[{"q": "Escape peaks: E = 662 keV", "outcome": "INVALID_INPUT"}])


def hvl_tvl(mu, x=None):
    _pos("the attenuation coefficient", mu)
    out = dict(HVL=math.log(2) / mu, TVL=math.log(10) / mu, mfp=1.0 / mu, HVL_cm=math.log(2) / mu * 100, TVL_cm=math.log(10) / mu * 100)
    if x is not None:
        _need(_isnum(x) and x >= 0, "the thickness must be zero or positive")
        out.update(transmission=math.exp(-mu * x), n_HVL=mu * x / math.log(2))
    return _out(**out)


def _ref_hvl_tvl(mu, x=None):
    """The narrow beam attenuated through the absorber as dI/dx = -mu I (scipy DOP853) until the intensity falls to one
    half and one tenth (terminal events)."""
    from scipy.integrate import solve_ivp
    ev2 = lambda s, y: y[0] - 0.5                                     # noqa: E731
    ev10 = lambda s, y: y[0] - 0.1                                    # noqa: E731
    ev2.terminal = False
    ev10.terminal = True
    sol = solve_ivp(lambda s, y: [-mu * y[0]], (0, 10.0 / mu), [1.0], events=[ev2, ev10], method="DOP853", rtol=1e-13, atol=1e-16)
    h, t = float(sol.t_events[0][0]), float(sol.t_events[1][0])
    out = {"HVL": h, "TVL": t, "HVL_cm": h * 100, "TVL_cm": t * 100}
    if x is not None:
        out["transmission"] = float(solve_ivp(lambda s, y: [-mu * y[0]], (0, x), [1.0], method="DOP853", rtol=1e-13, atol=1e-300).y[0, -1]) if x > 0 else 1.0
    return out


_add("nuc_half_value_layer",
     name="Half-value and tenth-value layers", topic="shielding / attenuation",
     purpose="The half-value layer ln 2/mu, tenth-value layer ln 10/mu and mean free path 1/mu of a narrow beam in an absorber of linear attenuation coefficient mu, and with a thickness the transmission and number of half-value layers.",
     inputs=[_phys("mu", "linear_attenuation_coefficient", "linear attenuation coefficient"), _phys("x", "length", "absorber thickness (optional)", required=False)],
     outputs=[_o("HVL", "length", "half-value layer"), _o("TVL", "length", "tenth-value layer"), _o("mfp", "length", "mean free path 1/mu"),
              _o("HVL_cm", "number", "half-value layer in cm"), _o("TVL_cm", "number", "tenth-value layer in cm"),
              _o("transmission", "number", "narrow-beam transmission exp(-mu x)"), _o("n_HVL", "number", "thickness in half-value layers")],
     primary="HVL", conditions=["narrow beam (no buildup)", "mu > 0"],
     method="HVL = ln 2/mu, TVL = ln 10/mu, T = e^(-mu x)", latex=r"HVL = \frac{\ln 2}{\mu},\quad TVL = \frac{\ln 10}{\mu}",
     run=hvl_tvl, reference=_ref_hvl_tvl,
     wrong=lambda mu, x=None: _out(HVL=0.5 / mu, TVL=0.1 / mu, mfp=1.0 / mu, HVL_cm=50 / mu, TVL_cm=10 / mu, **({"transmission": math.exp(-mu * x), "n_HVL": 2 * mu * x} if x is not None else {})),
     rand=lambda rng: (lambda mu: {"mu": mu, **({"x": _sig(_lu(rng, 0.05, 5.0) / mu)} if rng.random() < 0.5 else {})})(_sig(_lu(rng, 0.01, 50.0)) * 100),
     phrases=("Half-value layer", "Half value layer", "Tenth-value layer", "HVL"),
     examples=[_ex({"mu": 65.6}, written={"mu": "0.656 1/cm"}), _ex({"mu": 14.9, "x": 0.3}, written={"mu": "0.149 1/cm", "x": "30 cm"})],
     level="beginner",
     extra_q=[{"q": "Half-value layer: mu = -0.656 1/cm", "outcome": "INVALID_INPUT"}])


def unit_conversion(A=None, D=None, H=None):
    given = [v for v in (A, D, H) if v is not None]
    _need(len(given) == 1, "give exactly one quantity: an activity A, an absorbed dose D or an equivalent dose H")
    if A is not None:
        _pos("the activity", A)
        return _out(SI_value=A, traditional_value=A / CI, si_unit="Bq", traditional_unit="Ci", value_mCi=A / CI * 1e3, value_MBq=A / 1e6)
    v = D if D is not None else H
    _need(_isnum(v) and v >= 0, "the dose must be zero or positive")
    si, tr = ("Gy", "rad") if D is not None else ("Sv", "rem")
    return _out(SI_value=v, traditional_value=v * 100.0, si_unit=si, traditional_unit=tr, value_mSv_or_mGy=v * 1e3, value_mrem_or_mrad=v * 1e5)


def _ref_unit_conversion(A=None, D=None, H=None):
    """The defining relations 1 Ci = 3.7e10 Bq and 1 rad = 1 rem = 0.01 Gy (Sv) in exact decimal arithmetic."""
    from decimal import Decimal
    if A is not None:
        a = Decimal(repr(A))
        return {"SI_value": A, "traditional_value": float(a / Decimal("3.7e10")), "value_mCi": float(a / Decimal("3.7e7")), "value_MBq": float(a / Decimal("1e6"))}
    v = Decimal(repr(D if D is not None else H))
    return {"SI_value": float(v), "traditional_value": float(v * 100), "value_mSv_or_mGy": float(v * 1000), "value_mrem_or_mrad": float(v * 100000)}


_add("nuc_radiation_unit_conversion",
     name="Radiation unit conversion (Bq and Ci, Gy and rad, Sv and rem)", topic="dosimetry / units",
     purpose="An activity, absorbed dose or equivalent dose written in any of the SI or traditional units, expressed in both systems (1 Ci = 3.7e10 Bq, 1 rad = 0.01 Gy, 1 rem = 0.01 Sv).",
     inputs=[_phys("A", "radioactivity", "an activity (Bq, kBq, MBq, Ci, mCi, uCi ...)", required=False),
             _phys("D", "absorbed_dose", "an absorbed dose (Gy, mGy, rad, mrad)", required=False),
             _phys("H", "equivalent_dose", "an equivalent or effective dose (Sv, mSv, rem, mrem)", required=False)],
     outputs=[_o("SI_value", "number", "value in the SI unit (Bq, Gy or Sv)"), _o("traditional_value", "number", "value in the traditional unit (Ci, rad or rem)"),
              {"symbol": "si_unit", "kind": "number", "meaning": "the SI unit"}, {"symbol": "traditional_unit", "kind": "number", "meaning": "the traditional unit"},
              _o("value_mCi", "number", "activity in mCi"), _o("value_MBq", "number", "activity in MBq"),
              _o("value_mSv_or_mGy", "number", "dose in mSv or mGy"), _o("value_mrem_or_mrad", "number", "dose in mrem or mrad")],
     primary="traditional_value", conditions=["exactly one of A, D, H"],
     method="Ci = A/3.7e10; rad = 100 Gy-value; rem = 100 Sv-value", latex=r"1\ \mathrm{Ci} = 3.7\times10^{10}\ \mathrm{Bq},\quad 1\ \mathrm{rad} = 0.01\ \mathrm{Gy},\quad 1\ \mathrm{rem} = 0.01\ \mathrm{Sv}",
     run=unit_conversion, reference=_ref_unit_conversion,
     wrong=lambda A=None, D=None, H=None: (lambda o: o | _out(traditional_value=o["SI_value"] / 3.7e7 if A is not None else o["SI_value"] * 10.0))(unit_conversion(A, D, H)),
     rand=lambda rng: rng.choice([{"A": _sig(_lu(rng, 1.0, 1e15))}, {"D": _sig(_lu(rng, 1e-6, 10.0))}, {"H": _sig(_lu(rng, 1e-6, 10.0))}]),
     phrases=("Radiation unit conversion", "Convert radiation units", "Unit conversion"),
     examples=[_ex({"A": 3.7e8}, written={"A": "10 mCi"}), _ex({"H": 0.05}, written={"H": "5 rem"})],
     level="beginner",
     extra_q=[{"q": "Radiation unit conversion: D = 250 mrad", "outputs": {"SI_value": 0.0025}},
              {"q": "Radiation unit conversion: A = 1 Ci, H = 1 rem", "outcome": "INVALID_INPUT"}])


def constant_power_depletion(m0, P, t, E_f, sigma_a, sigma_f):
    _pos("m0, P, E_f and the cross sections", m0, P, E_f, sigma_a, sigma_f)
    _need(sigma_f <= sigma_a, "the fission cross section cannot exceed the absorption cross section")
    _nonneg("the time t", t)
    N0 = m0 * 1000.0 / _M5 * NA
    dN = P * t / E_f * sigma_a / sigma_f                 # atoms absorbed: fissions P t/E_f, captures with them
    _need(dN < N0, "the fuel would be used up before t at this power")
    N = N0 - dN
    return _out(m=m0 * N / N0, burnt=dN / N0, consumed=m0 * dN / N0, flux_ratio=N0 / N, fissions=P * t / E_f)


def _ref_constant_power_depletion(m0, P, t, E_f, sigma_a, sigma_f):
    """The coupled depletion at constant power integrated in time (scipy DOP853): dN/dt = -sigma_a phi N with the flux
    adjusted at every instant to keep E_f sigma_f phi N = P."""
    N0 = m0 * 1000.0 / _M5 * NA
    if t == 0:
        return {"m": m0, "burnt": 0.0, "consumed": 0.0, "flux_ratio": 1.0, "fissions": 0.0}
    n = _ivp(lambda s, y: [-sigma_a * (P / (E_f * sigma_f * y[0] * N0)) * y[0]], t, [1.0], atol=[1e-16])[0]
    return {"m": m0 * n, "burnt": 1.0 - n, "consumed": m0 * (1.0 - n), "flux_ratio": 1.0 / n, "fissions": P * t / E_f}


_add("nuc_constant_power_depletion",
     name="U-235 depletion at constant power", topic="reactor physics / fuel depletion and burnup",
     purpose="How much of an initial mass of U-235 remains after operation at a constant power (the flux rising as the fuel is consumed), the fraction burnt, the mass consumed and the factor by which the flux has risen.",
     inputs=[_phys("m0", "mass", "initial mass of U-235"), _phys("P", "power", "constant thermal power"), _phys("t", "time", "operating time"),
             _phys("E_f", "energy", "recoverable energy per fission (e.g. 200 MeV)"), _phys("sigma_a", "microscopic_cross_section", "U-235 absorption cross section (barns)"),
             _phys("sigma_f", "microscopic_cross_section", "U-235 fission cross section (barns)")],
     outputs=[_o("m", "mass", "U-235 mass remaining"), _o("burnt", "number", "fraction of U-235 consumed"), _o("consumed", "mass", "U-235 mass consumed"),
              _o("flux_ratio", "number", "flux at t over the initial flux (N0/N)"), _o("fissions", "number", "number of fissions P t/E_f")],
     primary="m", conditions=["constant power, one group", "U-235 only (no breeding)", "M(U-235) = 235.043928 g/mol"],
     method="fissions = P t/E_f; atoms absorbed = fissions sigma_a/sigma_f; N = N0 - P t sigma_a/(E_f sigma_f); phi/phi0 = N0/N",
     latex=r"N(t) = N_0 - \frac{P\,t}{E_f}\frac{\sigma_a}{\sigma_f},\quad \frac{\phi(t)}{\phi_0} = \frac{N_0}{N(t)}",
     run=constant_power_depletion, reference=_ref_constant_power_depletion,
     wrong=lambda m0, P, t, E_f, sigma_a, sigma_f: (lambda N0, dN: _out(m=m0 * (N0 - dN) / N0, burnt=dN / N0, consumed=m0 * dN / N0, flux_ratio=N0 / (N0 - dN), fissions=P * t / E_f))(
         m0 * 1000.0 / _M5 * NA, P * t / E_f),
     rand=lambda rng: (lambda m0, sa, Ef: (lambda P_full: {"m0": m0, "sigma_a": sa * BARN, "sigma_f": _sig(sa * rng.uniform(0.8, 0.87)) * BARN, "E_f": Ef,
                                                          "P": _sig(P_full * rng.uniform(0.05, 0.8)), "t": 1.0})(
         m0 * 1000.0 / _M5 * NA * Ef / sa * sa))(_sig(_lu(rng, 1e-2, 1e3)), _sig(rng.uniform(600.0, 700.0)), _sig(rng.uniform(190.0, 210.0)) * MEV),
     phrases=("Depletion at constant power", "U-235 depletion constant power", "Fuel depletion constant power"),
     examples=[_ex({"m0": 100.0, "P": 3e7, "t": 31557600.0, "E_f": 200 * MEV, "sigma_a": 680.9e-28, "sigma_f": 582.6e-28},
                   written={"m0": "100 kg", "P": "30 MW", "t": "1 y", "E_f": "200 MeV", "sigma_a": "680.9 b", "sigma_f": "582.6 b"}),
               _ex({"m0": 1.0, "P": 1e5, "t": 86400.0 * 30, "E_f": 200 * MEV, "sigma_a": 680.9e-28, "sigma_f": 582.6e-28},
                   written={"m0": "1 kg", "P": "100 kW", "t": "30 d", "E_f": "200 MeV", "sigma_a": "680.9 b", "sigma_f": "582.6 b"})],
     level="advanced",
     extra_q=[{"q": "Depletion at constant power: m0 = 1 kg, P = 3000 MW, t = 1 y, E_f = 200 MeV, sigma_a = 680.9 b, sigma_f = 582.6 b", "outcome": "INVALID_INPUT"}])


# ================================================================================================ worked example outputs
# computed by `run` and checked against every procedure's independent reference on the box (scratch/fill_examples.py:
# 151 examples, 0 disagreements); certification recomputes them.
# @@EXV-START@@
_EXV = {
    'nuc_accumulated_dose_decaying_source': [
        {'D': 0.00140226015359, 'D_mSv': 1.40226015359, 'D_inf': 0.00263941057731, 'H_mean': 1.94758354665e-07},
        {'D': 0.00578081326993, 'D_mSv': 5.78081326993, 'D_inf': 0.0127360988542, 'H_mean': 9.55822299922e-09},
    ],
    'nuc_activation_activity': [
        {'A_eoi': 40813620612.8, 'A_d': 40813620612.8, 'A_sat': 3799271250380.0, 'S': 0.0107424866305},
        {'A_eoi': 492089922.878, 'A_d': 361372738.322, 'A_sat': 515609531.342, 'S': 0.954384845443},
    ],
    'nuc_activation_with_burnup': [
        {'A_P': 4303591318310.0, 'f_T': 0.88929064975, 't_max': 247907988.5, 'A_max': 14114011895000.0},
        {'A_P': 2803284011020.0, 'f_T': 0.606530659713, 't_max': 761002.08524, 'A_max': 2891313644290.0},
    ],
    'nuc_attenuation_coefficient_fit': [
        {'mu': 65.3966059686, 'HVL': 0.0105991308003, 'TVL': 0.035209550387, 'I0': 1000.02191685},
        {'mu': 14.1881755283, 'HVL': 0.0488538627944, 'TVL': 0.16228901936, 'I0': 4993.43579355},
    ],
    'nuc_bare_reactor_keff': [
        {'B2': 509.496363352, 'k_eff': 0.919048790192, 'P_NL': 0.765873991827, 'rho': -0.0880815150096},
        {'B2': 91.6731895913, 'k_eff': 1.14503132615, 'P_NL': 0.91602506092, 'rho': 0.126661448327},
    ],
    'nuc_centreline_temperature_variable_k': [
        {'T_0': 1586.09800909, 'dT': 786.098009086, 'k_s': 4.13223140496, 'k_0': 2.29684355461},
        {'T_0': 942.674878942, 'dT': 242.674878942, 'k_s': 5.55555555556, 'k_0': 4.37569784034},
    ],
    'nuc_chain_activity': [
        {'A_parent': 28749921717.7, 'A_daughter': 25506124877.7, 'A_total': 54256046806.5, 'ratio': 0.887171976608},
        {'A_parent': 999341.038995, 'A_daughter': 924948.519033, 'A_total': 1924289.55803, 'ratio': 0.925558425943},
        {'A_parent': 999.997378806, 'A_daughter': 943.987598484, 'A_total': 1943.98497729, 'ratio': 0.943990072865},
    ],
    'nuc_channel_coolant_temperature': [
        {'T': 597.886071405, 'T_out': 630.77214281, 'Q': 108524.035637},
        {'T': 558.813497516, 'T_out': 565.472840883, 'Q': 38197.1863421},
    ],
    'nuc_collisions_to_thermalize': [
        {'n': 115.342927739, 'xi': 0.157768989823},
        {'n': 18.1975371926, 'xi': 1.0},
    ],
    'nuc_committed_dose_retention': [
        {'U': 272600706922.0, 'H': 0.393079034738, 'H_mSv': 393.079034738},
        {'U': 797566508818.0, 'H': 2.65791224294, 'H_mSv': 2657.91224294},
    ],
    'nuc_compton_edge': [
        {'T_max': 0.477650409375, 'E_bs': 0.184349590625},
        {'T_max': 1.11762178669, 'E_bs': 0.214378213313},
    ],
    'nuc_control_rod_worth': [
        {'rho': 0.005, 'frac': 0.5, 'drho_dx': 0.00546448087432},
        {'rho': 0.00118098573981, 'frac': 0.0908450569081, 'drho_dx': 0.013, 'rho_dollars': 0.181690113816},
    ],
    'nuc_core_power_from_flux': [
        {'P': 5917484.50112, 'phi_max': 1e+17, 'phi_avg': 2.74863646963e+16, 'peaking': 3.63816754615},
        {'P': 1000000.0, 'phi_max': 3.92165581113e+17, 'phi_avg': 1.19204042586e+17, 'peaking': 3.2898681337},
    ],
    'nuc_count_rate_point_source': [
        {'R': 290.718577374, 'g': 0.015388286032, 'R_cpm': 17443.1146425},
        {'R': 14644.6609407, 'g': 0.146446609407, 'R_cpm': 878679.65644},
    ],
    'nuc_count_time_for_precision': [
        {'T': 400.0, 'tg': 266.666666667, 'tb': 133.333333333},
        {'T': 261.803398875, 'tg': 180.901699437, 'tb': 80.9016994375},
    ],
    'nuc_count_time_split': [
        {'tg': 2400.0, 'tb': 1200.0, 'sigma': 0.25, 'rel': 0.00333333333333},
        {'tg': 490.353601421, 'tb': 109.646398579, 'sigma': 0.157966958333, 'rel': 0.0166281008772},
    ],
    'nuc_counts_during_decay': [
        {'N': 57953848.461, 'A_mean': 83238.3207816, 'F': 1.20136974246},
        {'N': 1132354.78572, 'A_mean': 18872.579762, 'F': 1.38629436112},
    ],
    'nuc_critical_cuboid': [
        {'c': 0.147764834339, 'c_ext': 0.186126434339, 'V': 0.0531953403619},
        {'c': 0.203783534377, 'c_ext': 0.203783534377, 'V': 0.407567068754},
    ],
    'nuc_critical_cylinder': [
        {'R': 0.119828633705, 'R_ext': 0.139009433705, 'V': 0.0225549096632},
        {'R': 0.155188669817, 'R_ext': 0.155188669817, 'V': 0.0756606196822},
    ],
    'nuc_critical_cylinder_minimum_volume': [
        {'R': 0.161320602631, 'H': 0.298037647974, 'V_ext': 0.0243669219101, 'HD': 0.92374328856},
        {'R': 0.16496498671, 'H': 0.301520232573, 'V_ext': 0.0375153304644, 'HD': 0.92374328856},
    ],
    'nuc_critical_slab': [
        {'a': 0.133710516286, 'a_ext': 0.172072116286},
        {'a': 0.172072116286, 'a_ext': 0.172072116286},
    ],
    'nuc_critical_sphere': [
        {'R': 0.152891316286, 'R_ext': 0.172072116286, 'V': 0.0149705262888},
        {'R': 0.178691765316, 'R_ext': 0.198691765316, 'V': 0.0239002380823},
    ],
    'nuc_daughter_max_activity_time': [
        {'t_max': 82312.6147055, 'ratio_max': 0.786354204679, 't_max_half_lives': 3.80126603424},
        {'t_max': 14.4269504089, 'ratio_max': 0.367879441171, 't_max_half_lives': 1.44269504089, 'A_D_max': 367.879441171},
    ],
    'nuc_dead_time_nonparalyzable': [
        {'n': 66666.6666667, 'loss': 0.25, 'corr': 1.33333333333},
        {'n': 20000.0, 'loss': 0.166666666667, 'corr': 1.2},
    ],
    'nuc_dead_time_paralyzable': [
        {'n': 71480.5912363, 'loss': 0.300509423114, 'm_max': 73575.8882343},
        {'n': 430658.472822, 'loss': 0.883898719855, 'm_max': 73575.8882343},
    ],
    'nuc_decay_curve_fit': [
        {'lambda_': 9.62845226723e-05, 'T_half': 7198.94705112, 'C0': 10012.5780133, 'sigma_T': 94.4047167396},
        {'lambda_': 0.00115524530093, 'T_half': 600.0, 'C0': 5120.0, 'sigma_T': 9.51438162019},
    ],
    'nuc_decay_heat_energy': [
        {'E': 1563755897410.0, 'E_MWh': 434.376638168, 'P_mean': 18101121.6276},
        {'E': 131822482477.0, 'E_MWh': 36.6173562437, 'P_mean': 219265.60625},
    ],
    'nuc_decay_heat_power': [
        {'P_d': 30375540.9564, 'ratio': 0.0101251803188},
        {'P_d': 318186.948233, 'ratio': 0.00318186948233},
    ],
    'nuc_depletion_constant_flux': [
        {'m': 0.524858227771, 'burnt': 0.475141772229, 'E': 33377408542800.0, 'E_MWd': 386.312598875, 'P_t': 753140.49579},
        {'m': 0.000994134294819, 'burnt': 0.00586570518123, 'E': 412049728.456, 'E_MWd': 0.00476909407936, 'P_t': 4755.07960097},
    ],
    'nuc_detector_efficiency_point_source': [
        {'eff': 0.0291541727637, 'g': 0.0327635347053, 'intrinsic': 0.889836002919},
        {'eff': 0.0503627156283, 'g': 0.187652476223, 'intrinsic': 0.268382899294},
    ],
    'nuc_disk_source_flux': [
        {'phi': 1895736.59669, 'phi0': 4023594.78109, 'T': 0.471154949699},
        {'phi': 402359.478109, 'phi0': 402359.478109, 'T': 1.0},
    ],
    'nuc_effective_half_life': [
        {'T_eff': 629841.371405, 'lambda_eff': 1.10051071909e-06},
        {'T_eff': 17314.5427286, 'lambda_eff': 4.00326587553e-05, 'N_decays': 9242453824050.0},
    ],
    'nuc_elastic_scattering': [
        {'E_out': 2.7492971826e-13, 'ratio': 0.85798816568, 'psi': 1.48765509491, 'du': 0.153164972502},
        {'E_out': 1.2016324755e-13, 'ratio': 0.75, 'psi': 0.523598775598, 'du': 0.287682072452},
    ],
    'nuc_enrichment_swu': [
        {'F': 9.21908893709, 'W': 8.21908893709, 'SWU': 6.87111298932, 'F_per_P': 9.21908893709, 'SWU_per_P': 6.87111298932},
        {'F': 4793.18734793, 'W': 4693.18734793, 'SWU': 3831.50730487, 'F_per_P': 47.9318734793, 'SWU_per_P': 38.3150730487},
    ],
    'nuc_equilibrium_ratio': [
        {'ratio_A': 1.10037546934, 'ratio_N': 0.100375469337, 't_99': 158306.685422},
        {'ratio_A': 1.00025405895, 'ratio_N': 0.000254058947898, 't_99': 1533525.76134},
    ],
    'nuc_exponential_pile': [
        {'gamma': 2.38717096827, 'L': 1.14423439231, 'L2': 1.30927234454},
        {'gamma': 3.38698923515, 'L': 0.497578268895, 'L2': 0.247584133677},
    ],
    'nuc_flux_from_foil_activation': [
        {'phi': 625429992521.0, 'A_sat': 1886368.82769, 'R_atom': 6.16986687622e-15},
        {'phi': 18392158.9906, 'A_sat': 724.682321488, 'R_atom': 2.88756896152e-19},
    ],
    'nuc_flux_shape': [
        {'ratio': 0.580176172711, 'B2': 3.66692758365},
        {'ratio': 0.707106781187, 'B2': 9.86960440109},
        {'ratio': 0.636619772368, 'B2': 39.4784176044},
    ],
    'nuc_fuel_cross_sections': [
        {'N5': 7.04634822371e+26, 'N8': 2.24954143203e+28, 'NO': 4.64000982853e+28, 'Sigma_a': 54.0082376949, 'Sigma_f': 41.0520247513, 'eta': 1.84705934508, 'mfp_a': 0.0185156939511},
        {'N5': 1.76157364557e+26, 'N8': 2.42891185917e+28, 'NO': 4.89305519125e+28, 'Sigma_a': 18.5049684157, 'Sigma_f': 10.2629280591, 'eta': 1.34768753036, 'mfp_a': 0.0540395410321},
    ],
    'nuc_fuel_rod_temperatures': [
        {'T_0': 1313.46019731, 'T_fs': 782.943720338, 'T_ci': 627.670605126},
        {'T_0': 2071.96878954, 'T_fs': 798.729244809, 'T_ci': 671.405290335},
    ],
    'nuc_generator_daughter_activity': [
        {'A_D': 25506124877.7, 'A_P': 28749921717.7, 'ratio': 0.887171976608, 't_max': 82312.6147055, 'A_D_max': 25525136119.3},
        {'A_D': 458910250.547, 'A_P': 999893413.57, 'ratio': 0.458959169367, 't_max': 50760.5913776, 'A_D_max': 998498162.611},
    ],
    'nuc_inverse_multiplication': [
        {'x_crit': 99.9760479042, 'slope': -0.01, 'x_crit_last2': 100.0, 'inv_M_last': 0.2},
        {'x_crit': 67.5757575758, 'slope': -0.0171875, 'x_crit_last2': 70.0, 'inv_M_last': 0.3125},
    ],
    'nuc_irradiation_time_for_saturation': [
        {'t_irr': 179384.117124, 'n_half': 3.32192809489},
        {'t_irr': 134.7, 'n_half': 1.0},
    ],
    'nuc_klein_nishina': [
        {'sigma': 2.56140352183e-29, 'sigma_b': 0.256140352183, 'sigma_tr': 9.7749373874e-30, 'T_mean': 0.252635264038},
        {'sigma': 1.88819414552e-29, 'sigma_b': 0.188819414552, 'sigma_tr': 8.88541561229e-30, 'T_mean': 0.588221796032},
    ],
    'nuc_line_source_flux': [
        {'phi': 5012706.8213, 'phi0': 17620819.1175, 'T': 0.284476379213},
        {'phi': 195520598.868, 'phi0': 195520598.868, 'T': 1.0},
    ],
    'nuc_logarithmic_energy_decrement': [
        {'alpha': 0.715976331361, 'xi': 0.157768989823, 'mu_bar': 0.0555555555556},
        {'alpha': 0.0, 'xi': 1.0, 'mu_bar': 0.666666666667},
        {'alpha': 0.111111111111, 'xi': 0.725346927833, 'mu_bar': 0.333333333333},
    ],
    'nuc_mass_for_activity': [
        {'m': 1.8984007654e-12, 'm_ug': 0.0018984007654, 'N': 11558843813700.0},
        {'m': 2.91423286786e-10, 'm_ug': 0.291423286786, 'N': 728040799803000.0},
    ],
    'nuc_max_clad_temperature': [
        {'z_max': 0.845822674485, 'T_max': 651.707052916, 'T_out': 630.77214281},
        {'z_max': 0.337895254446, 'T_max': 598.655680318, 'T_out': 569.510992223},
    ],
    'nuc_maxwellian_average_cross_section': [
        {'sigma_avg': 6.03412493012e-26, 'sigma_avg_b': 603.412493012, 'g': 0.8861984036},
        {'sigma_avg': 1.66137686225e-28, 'sigma_avg_b': 1.66137686225, 'g': 0.619916739646},
    ],
    'nuc_moderator_mixture': [
        {'xi': 0.923648339594, 'MP': 135.237582983, 'MR': 60.9099764739},
        {'xi': 0.157768989823, 'MP': 6.07511635472, 'MR': 222.732691515},
    ],
    'nuc_multilayer_transmission': [
        {'T': 0.000430742540576, 'tau': 7.75, 'n_hvl': 11.1808865669, 'attenuation': 2321.57241461},
        {'T': 0.00302755474538, 'tau': 5.8, 'n_hvl': 8.36763123716, 'attenuation': 330.29955991, 'H': 8.40987429271e-10},
    ],
    'nuc_plane_source_slab_flux': [
        {'phi': 266546827205.0, 'phi0': 8905881248490.0, 'leak': 4.8030054865e-08},
        {'phi': 288895596785.0, 'phi0': 288895596785.0, 'leak': 0.314045870704},
    ],
    'nuc_point_source_dose_rate': [
        {'H_dot': 7.75013916132e-06, 'H_uSv_h': 27900.5009807, 'B': 1.656, 'T': 0.859336289543},
        {'H_dot': 8.50486799004e-06, 'H_uSv_h': 30617.5247641, 'B': 1.8172656127, 'T': 0.943020705756},
        {'H_dot': 2.575e-08, 'H_uSv_h': 92.7, 'B': 1.0, 'T': 1.0},
    ],
    'nuc_point_source_sphere_flux': [
        {'phi': 14885614605.2, 'leak': 8.42667431023e-07, 'phi_inf': 14885614605.2},
        {'phi': 61129930.9371, 'leak': 0.603920343925, 'phi_inf': 70977845.9201},
    ],
    'nuc_power_after_step_one_group': [
        {'P': 1.36578805499, 'A1': 1.18119494222, 'omega': 0.0145205002736},
        {'P': 0.480696263265, 'A1': 0.500307692249, 'omega': -0.0399876923089},
    ],
    'nuc_power_after_step_six_group': [
        {'P': 1.61698696853, 'omega': 0.0181937197626, 'T': 54.9640212694},
        {'P': 0.126560969787, 'omega': -0.0119183324181, 'T': -83.9043554854},
    ],
    'nuc_reactivity_from_period': [
        {'rho': 0.000940625725592, 'rho_dollars': 0.144667137126, 'rho_pcm': 94.0625725592},
        {'rho': 0.00145890353666, 'rho_dollars': 0.224446697948, 'rho_pcm': 145.890353666},
    ],
    'nuc_reactor_period_one_group': [
        {'omega': 0.0145205002736, 'T': 68.8681506253, 'T_d': 47.7357644363},
        {'omega': -0.0383443433751, 'T': -26.0794660171, 'T_d': -18.0769083403},
    ],
    'nuc_reflected_slab_critical': [
        {'a': 0.0360662339368, 'delta': 0.0680029411748},
        {'a': 0.0180701215003, 'delta': 0.0903108219078},
    ],
    'nuc_samarium': [
        {'Pm_eq': 9.3456099681e+21, 'Sm_eq': 2.76960784314e+22, 'Sm_max': 3.70416883995e+22, 'max_ratio': 1.33743441301},
        {'Pm_eq': 9.3456099681e+21, 'Sm_eq': 2.76960784314e+22, 'Sm_max': 3.70416883995e+22, 'max_ratio': 1.33743441301, 'Sm_t': 3.20483889588e+22},
    ],
    'nuc_shield_thickness_for_dose_rate': [
        {'x': 0.183532056545, 'n_mfp': 12.0397029093, 'B': 13.0397029093},
        {'x': 0.144385733782, 'n_mfp': 9.47170413611, 'B': 1.0},
    ],
    'nuc_solid_angle_off_axis': [
        {'Omega': 0.637048888348, 'g': 0.0506947397859},
        {'Omega': 0.282708299275, 'g': 0.0224972116413},
    ],
    'nuc_solid_angle_on_axis': [
        {'Omega': 0.193374905398, 'g': 0.015388286032},
        {'Omega': 1.84030236902, 'g': 0.146446609407},
    ],
    'nuc_specific_activity': [
        {'A': 41868944303400.0, 'A_Ci': 1131.59308928, 'a': 4.18689443034e+16, 'a_Ci_per_g': 1131.59308928, 'N': 1.00479849416e+22},
        {'A': 36576692063.3, 'A_Ci': 0.988559244954, 'a': 36576692063300.0, 'a_Ci_per_g': 0.988559244954, 'N': 2.66436451031e+21},
        {'A': 3202743.59036, 'A_Ci': 8.65606375774e-05, 'a': 3202743590360000.0, 'a_Ci_per_g': 86.5606375774, 'N': 4398706298530000.0},
    ],
    'nuc_stable_period_six_group': [
        {'omega': 0.0181937197626, 'T': 54.9640212694, 'T_d': 38.0981563751, 'omega_p': -110.522992371},
        {'omega': 0.172398026653, 'T': 5.80053043191, 'T_d': 4.02062131463, 'omega_p': -33.3362006913},
    ],
    'nuc_stay_time_decaying_source': [
        {'t_stay': 1995.63447212, 'frac': 0.189436234097},
        {'t_stay': 8184.25748311, 'frac': 0.230472877992},
    ],
    'nuc_thermal_neutron_speeds': [
        {'v_p': 2200.0708059, 'v_mean': 2482.51406351, 'v_rms': 2694.52543622, 'E_mean': 6.080378196e-21, 'E_mean_eV': 0.0379507356865},
        {'v_p': 3145.09854519, 'v_mean': 3548.86367685, 'v_rms': 3851.94331324, 'E_mean': 1.2425841e-20, 'E_mean_eV': 0.0775559993593},
    ],
    'nuc_three_member_chain': [
        {'A1': 707.106781187, 'A2': 750.951979096, 'A3': 755.030057448, 'A_total': 2213.08881773},
        {'A1': 707106.781187, 'A2': 554237.841694, 'A3': 472938.7239, 'A_total': 1734283.34678},
    ],
    'nuc_transit_dose': [
        {'D': 0.000495454326675, 'D_uSv': 495.454326675, 'frac': 0.874334083622},
        {'D': 1.60762055991e-06, 'D_uSv': 1.60762055991, 'frac': 0.99363401447},
    ],
    'nuc_two_component_decay_fit': [
        {'C1': 8973.69922743, 'C2': 1696.92097822, 'frac1': 0.840972600888},
        {'C1': 1364.46047901, 'C2': 568.416945478, 'frac1': 0.705921887092},
    ],
    'nuc_two_group_bare_keff': [
        {'B2': 2.72203771985, 'k_eff': 1.24472488601, 'k_inf': 1.26339285714, 'flux_ratio': 0.224717094112},
        {'B2': 9.86960440109, 'k_eff': 1.06513594627, 'k_inf': 1.12, 'flux_ratio': 0.199213536481},
    ],
    'nuc_xenon_after_shutdown': [
        {'X': 3.72338221456e+21, 'ratio': 1.88342988689, 't_peak': 30167.0150657, 'peak_ratio': 1.90154919697, 'I': 2.27759128356e+21},
        {'X': 3.08999549507e+19, 'ratio': 0.99459630648, 't_peak': 0.0, 'peak_ratio': 1.0, 'I': 1.96211471332e+19},
    ],
    'nuc_xenon_equilibrium': [
        {'I_eq': 6.54130684963e+21, 'X_eq': 1.9769157538e+21, 'abs_per_fission': 0.0523882674756},
        {'I_eq': 2.18043561654e+20, 'X_eq': 2.79434645174e+20, 'abs_per_fission': 0.00740501809712},
    ],
}
# @@EXV-END@@

# the worked examples of the definitional additions (computed by their references on the build machine)
_EXV.update({
    'nuc_effective_dose': [{"E_mSv": 1.42, "E": 0.00142, "w_sum": 0.17}, {"E_mSv": 0.72, "E": 0.00072, "w_sum": 0.24}],
    'nuc_committed_dose_coefficient': [{"E": 0.022, "E_mSv": 22.0, "I_Bq": 1000000.0}, {"E": 0.000481, "E_mSv": 0.481, "I_Bq": 37000.0}],
    'nuc_power_from_reaction_rate': [{"R": 1.7478e+19, "q": 560056864.181, "q_MW_m3": 560.056864181}, {"R": 2.79648e+18, "q": 89609098.269, "q_MW_m3": 89.609098269, "P": 2688272948.07, "P_MW": 2688.27294807}],
    'nuc_six_factor_formula': [{"k_inf": 1.28518662, "k_eff": 1.28518662, "P_NL": 1.0, "rho": 0.221902885979}, {"k_inf": 1.136025, "k_eff": 1.0909248075, "P_NL": 0.9603, "rho": 0.0833465394452}],
    'nuc_diffusion_length': [{"L": 0.0284988199952, "L2": 0.000812182741117, "L_cm": 2.84988199952}, {"L": 0.512347538298, "L2": 0.2625, "L_cm": 51.2347538298, "M2": 0.2993, "M": 0.547083174664}],
    'nuc_currie_detection_limits': [{"L_C": 23.2617430735, "L_D": 49.2290296012}, {"L_C": 46.5234861471, "L_D": 95.7525157482, "MDA": 0.750117632184}],
    'nuc_escape_peaks': [{"single_escape_keV": 2103.00104931, "double_escape_keV": 1592.00209862, "annihilation_keV": 510.99895069, "E_keV": 2614.0}, {"single_escape_keV": 821.50104931, "double_escape_keV": 310.50209862, "annihilation_keV": 510.99895069, "E_keV": 1332.5}],
    'nuc_half_value_layer': [{"HVL": 0.0105662679963, "TVL": 0.0351003825152, "HVL_cm": 1.05662679963, "TVL_cm": 3.51003825152}, {"HVL": 0.046519945004, "TVL": 0.154535912281, "HVL_cm": 4.6519945004, "TVL_cm": 15.4535912281, "transmission": 0.0114473158505}],
    'nuc_radiation_unit_conversion': [{"SI_value": 370000000.0, "traditional_value": 0.01, "value_mCi": 10.0, "value_MBq": 370.0}, {"SI_value": 0.05, "traditional_value": 5.0, "value_mSv_or_mGy": 50.0, "value_mrem_or_mrad": 5000.0}],
    'nuc_constant_power_depletion': [{"m": 86.5229195621, "burnt": 0.134770804379, "consumed": 13.4770804379, "flux_ratio": 1.15576312619, "fissions": 2.95450570152e+25}, {"m": 0.996310176471, "burnt": 0.00368982352851, "consumed": 0.00368982352851, "flux_ratio": 1.00370348875, "fissions": 8.0889957605e+21}],
})

for _k, _vals in _EXV.items():
    if _k in PROCEDURES:
        for _e, _v in zip(PROCEDURES[_k]["examples"], _vals):
            _e["outputs"].update(_v)
