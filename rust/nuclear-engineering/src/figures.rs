//! Nuclear Engineering's own live figures (ax_engine::figkit), drawn for the visitor's inputs from what the model
//! computes: reactor cores to scale over their flux shapes, detector and source geometries, decay chains, fits of
//! measured data, shielding layers, temperature profiles, spectra lines, and maps over two inputs. Every number drawn
//! is the model's answer or a further solve of it (docs/FIGURE_PARITY.md); a figure that cannot be drawn is left out.

use crate::figs_common as c;
use ax_engine::figkit::{self as k, Canvas, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;
use std::f64::consts::{LN_2, PI};

pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        "nuc_committed_dose_coefficient" => vec![c::field_map(s, "I", "e", "E_mSv")],
        "nuc_dead_time_nonparalyzable" => vec![c::field_map_k(s, "field", "m", "tau", "n", (0.5, 1.2), (0.5, 1.2))],
        "nuc_dead_time_paralyzable" => vec![c::field_map_k(s, "field", "m", "tau", "n", (0.5, 1.2), (0.5, 1.2))],
        "nuc_diffusion_length" => vec![c::field_map(s, "D", "Sigma_a", "L")],
        "nuc_effective_half_life" => vec![c::field_map(s, "T_p", "T_b", "T_eff")],
        "nuc_irradiation_time_for_saturation" => vec![c::field_map_k(s, "field", "T_half", "f", "t_irr", (0.5, 1.5), (0.5, 1.08))],
        "nuc_maxwellian_average_cross_section" => vec![c::field_map(s, "sigma0", "T", "sigma_avg")],
        "nuc_reactivity_from_period" => vec![reactivity_field(s)],
        "nuc_activation_activity" => vec![c::family_std(s, "curve-2", "M", "A_eoi", "m", 0.5, 1.5, false)],
        "nuc_control_rod_worth" => vec![c::family_std(s, "curve-2", "H", "rho", "rho_tot", 0.5, 1.5, false)],
        "nuc_generator_daughter_activity" => vec![chain_pair(s, "A_P", "A_D", Some("t"), "parent", "daughter"),
                                                 c::family_std(s, "curve-2", "t", "A_D", "A_P0", 0.1, 10.0, true)],
        "nuc_shield_thickness_for_dose_rate" => vec![c::family_std(s, "curve-2", "mu", "x", "r", 0.5, 1.5, false)],
        "nuc_xenon_after_shutdown" => vec![xenon(s)],
        "nuc_chain_activity" => vec![chain_pair(s, "A_parent", "A_daughter", Some("t"), "parent", "daughter")],
        "nuc_daughter_max_activity_time" => vec![ingrowth(s, false)],
        "nuc_equilibrium_ratio" => vec![ingrowth(s, true)],
        "nuc_three_member_chain" => vec![three_bars(s), three_chain(s), three_diagram(s)],
        "nuc_critical_sphere" => vec![critical(s, "sphere")],
        "nuc_critical_slab" => vec![critical(s, "slab")],
        "nuc_critical_cylinder" => vec![critical(s, "cylinder")],
        "nuc_critical_cylinder_minimum_volume" => vec![critical(s, "cylinder-min")],
        "nuc_critical_cuboid" => vec![cuboid(s)],
        "nuc_bare_reactor_keff" => vec![bare_keff(s)],
        "nuc_core_power_from_flux" => vec![flux_section(s, true)],
        "nuc_flux_shape" => vec![flux_section(s, false)],
        "nuc_reflected_slab_critical" => vec![reflected(s)],
        "nuc_count_rate_point_source" | "nuc_detector_efficiency_point_source" | "nuc_solid_angle_on_axis"
        | "nuc_solid_angle_off_axis" => vec![detector(s)],
        "nuc_disk_source_flux" => vec![extended_source(s, true)],
        "nuc_line_source_flux" => vec![extended_source(s, false)],
        "nuc_attenuation_coefficient_fit" => vec![attenuation_fit(s), data_xy(s, "x", "I", "absorber thickness (cm)", "intensity")],
        "nuc_decay_curve_fit" => vec![decay_fit(s)],
        "nuc_two_component_decay_fit" => vec![two_fit(s)],
        "nuc_inverse_multiplication" => vec![inverse_m(s)],
        "nuc_effective_dose" => vec![data_xy(s, "H", "w", "equivalent dose (mSv)", "tissue weighting factor")],
        "nuc_exponential_pile" => vec![data_xy(s, "z", "phi", "height (cm)", "flux (as measured)")],
        "nuc_half_value_layer" => vec![hvl_layers(s)],
        "nuc_multilayer_transmission" => vec![multilayer(s)],
        "nuc_point_source_dose_rate" => vec![dose_layers(s)],
        "nuc_channel_coolant_temperature" => vec![coolant_profile(s, false)],
        "nuc_max_clad_temperature" => vec![coolant_profile(s, true)],
        "nuc_fuel_rod_temperatures" => vec![rod_profile(s)],
        "nuc_compton_edge" => vec![lines_spectrum(s, false)],
        "nuc_escape_peaks" => vec![lines_spectrum(s, true)],
        "nuc_collisions_to_thermalize" => vec![staircase(s)],
        "nuc_six_factor_formula" => vec![six_factor(s)],
        "nuc_enrichment_swu" => vec![enrichment(s)],
        "nuc_power_after_step_six_group" => vec![power_heat(s)],
        "nuc_moderator_mixture" => vec![moderator(s)],
        _ => vec![],
    };
    v.into_iter().flatten().collect()
}

// ------------------------------------------------------------------------------------------------ helpers
fn f(x: f64) -> Input {
    Input::Float(x)
}

fn txt(t: &str) -> Input {
    Input::Text(t.to_string())
}

fn list(v: &[f64]) -> Input {
    Input::Json(serde_json::Value::Array(v.iter().map(|x| serde_json::json!(x)).collect()))
}

fn wi(s: &Solved, set: &[(&str, Input)], y: &str) -> Option<f64> {
    let a = s.with_input(set)?;
    k::num_of(&a, y)
}

fn oi(s: &Solved, key: &str, set: &[(&str, Input)], y: &str) -> Option<f64> {
    let a = s.solve_other(key, set)?;
    k::num_of(&a, y)
}

fn fig(s: &Solved, kind: &str, title: &str, cap: &str, cv: &Canvas) -> Option<Figure> {
    Some(s.figure(kind, title, cap, cv.body(), cv.h))
}

#[allow(clippy::too_many_arguments)]
fn chart(s: &Solved, kind: &str, title: &str, cap: &str, series: Vec<(String, Vec<(f64, f64)>)>, xlab: &str,
         ylab: &str, header: &str, marker: Option<(f64, f64)>, mlabel: &str, log_x: bool) -> Option<Figure> {
    let series: Vec<(String, Vec<(f64, f64)>)> = series.into_iter().filter(|x| x.1.len() >= 2).collect();
    if series.is_empty() {
        return None;
    }
    let body = c::fit_right(&k::lines(&series, xlab, ylab, header, marker, mlabel, log_x));
    Some(s.chart(kind, title, cap, &body))
}

/// A chart drawn below a sketch: the sketch's canvas grows by the chart's 340 px.
fn below(cv: &mut Canvas, chart: &str, top: f64) {
    cv.raw(&format!("<g transform=\"translate(0,{top})\">{}</g>", c::fit_right(chart)));
}

fn q(x: f64, u: &str) -> String {
    c::qv(x, u)
}

// ------------------------------------------------------------------------------------------------ maps and families
/// Xenon after shutdown for three fission cross sections, 21 times on a logarithmic axis each (a slower calculation).
fn xenon(s: &Solved) -> Option<Figure> {
    let (t0, sf) = (c::inp(s, "t")?, c::inp(s, "Sigma_f")?);
    let xs = k::span(t0 * 0.1, t0 * 10.0, 21, true);
    c::family_fig(s, "curve-2", "t", &xs, "X", "Sigma_f", &[sf, sf / 2.0, sf * 2.0], true,
                  "Xenon-135 after shutdown: Xe-135 atoms per volume against time after shutdown", "")
}

fn reactivity_field(s: &Solved) -> Option<Figure> {
    // the period and the generation time are the two number inputs; the delayed groups stay as given
    // the inhour equation is solved by iteration: a coarser grid
    c::field_map_n(s, "field", "T", "Lambda", "rho", (0.5, 1.5), (0.5, 1.5), 11)
}

// ------------------------------------------------------------------------------------------------ decay chains
/// A parent and its daughter: the pair with the model's activities at the given time (top), both activities against
/// time below (60 times from the model).
fn chain_pair(s: &Solved, yp: &str, yd: &str, t: Option<&str>, np: &str, nd: &str) -> Option<Figure> {
    let tsym = t?;
    let t0 = c::inp(s, tsym)?;
    let (ap, ad) = (s.out(yp)?, s.out(yd)?);
    if !(t0 > 0.0) {
        return None;
    }
    let ts = k::span(0.0, t0 * 3.0, 60, false);
    let mut pp = vec![];
    let mut dd = vec![];
    for tt in &ts {
        let Some(b) = s.with(&[(tsym, *tt)]) else { continue };
        if let (Some(x), Some(y)) = (k::num_of(&b, yp), k::num_of(&b, yd)) {
            pp.push((*tt, x));
            dd.push((*tt, y));
        }
    }
    let mut cv = Canvas::new(414.0);
    let names = s.txt("nuclide").map(|n| (n, String::new()));
    let pname = names.as_ref().map(|n| n.0.clone()).unwrap_or_else(|| np.to_string());
    cv.block(70.0, 14.0, 140.0, 30.0, &pname);
    c::varrow(&mut cv, (214.0, 29.0), (300.0, 29.0), "axp-k");
    cv.block(304.0, 14.0, 140.0, 30.0, nd);
    cv.value(140.0, 62.0, "middle", &format!("{} Bq", k::num(ap)));
    cv.value(374.0, 62.0, "middle", &format!("{} Bq", k::num(ad)));
    let ch = k::lines(&[(np.to_string(), pp), (nd.to_string(), dd)], "t, time (s)", "activity (Bq)",
                      &format!("at t = {}: ratio {}", k::q(t0, "s"), k::num(ad / ap)), Some((t0, ad)), "", false);
    below(&mut cv, &ch, 70.0);
    let extra = s.out("A_D_max").map(|m| format!(", the daughter's maximum {} Bq at t = {}", k::num(m),
                                                   k::q(s.out("t_max").unwrap_or(f64::NAN), "s"))).unwrap_or_default();
    fig(s, "chain", "Parent and daughter activities", &format!(
        "Parent-daughter decay{}: the pair with the model's activities at t = {} (top) and both activities from it at 60 \
         times (bottom). Marked: {np} {} Bq, {nd} {} Bq, ratio {}{extra}.",
        s.txt("nuclide").map(|n| format!(" of {n}")).unwrap_or_default(), k::q(t0, "s"), k::num(ap), k::num(ad),
        k::num(ad / ap)), &cv)
}

/// Daughter ingrowth from the generator capability (every decay feeding the daughter, stated): the activities
/// relative to the parent's initial one, or their ratio.
fn ingrowth(s: &Solved, ratio: bool) -> Option<Figure> {
    let (tp, td) = (c::inp(s, "T_P")?, c::inp(s, "T_D")?);
    let mark_t = if ratio { s.out("t_99")? } else { s.out("t_max")? };
    let ts = k::span(0.0, mark_t * 2.5, 60, false);
    let mut pp = vec![];
    let mut dd = vec![];
    let mut rr = vec![];
    for t in &ts {
        let Some(b) = s.solve_other("nuc_generator_daughter_activity", &[("A_P0", f(1.0)), ("T_P", f(tp)), ("T_D", f(td)),
                                                                          ("b", f(1.0)), ("t", f(*t))]) else { continue };
        let (Some(ap), Some(ad)) = (k::num_of(&b, "A_P"), k::num_of(&b, "A_D")) else { continue };
        pp.push((*t, ap));
        dd.push((*t, ad));
        if *t > 0.0 {
            rr.push((*t, ad / ap));
        }
    }
    if ratio {
        let (ra, rn) = (s.out("ratio_A")?, s.out("ratio_N")?);
        let eq_line = vec![(0.0, ra), (ts[59], ra)];
        chart(s, "chain", "Approach to transient equilibrium", &format!(
            "Approach to transient equilibrium for half-lives {} (parent) and {} (daughter): the daughter-to-parent \
             activity ratio from the model's generator capability at 60 times (every decay feeding the daughter, \
             stated). Marked: the model's equilibrium activity ratio {} (atom ratio {}) and the time to reach 99 % of \
             it, {}.", k::q(tp, "s"), k::q(td, "s"), k::num(ra), k::num(rn), k::q(mark_t, "s")),
              vec![("activity ratio A_D / A_P".into(), rr), ("equilibrium".into(), eq_line)], "t, time (s)",
              "A_D / A_P", &format!("equilibrium {}", k::num(ra)), Some((mark_t, ra * 0.99)), "99 %", false)
    } else {
        let rm = s.out("ratio_max")?;
        chart(s, "chain", "Daughter ingrowth and its maximum", &format!(
            "Daughter ingrowth for half-lives {} (parent) and {} (daughter): both activities relative to the parent's \
             initial activity, from the model's generator capability (every decay feeding the daughter, stated) at 60 \
             times. Marked: the model's time of the daughter's maximum, {} ({} daughter half-lives), where daughter \
             and parent activities are equal at {} of the initial parent activity.", k::q(tp, "s"), k::q(td, "s"),
            k::q(mark_t, "s"), k::num(s.out("t_max_half_lives")?), k::num(rm)),
              vec![("parent".into(), pp), ("daughter".into(), dd)], "t, time (s)", "activity / initial parent activity",
              &format!("maximum at {}", k::q(mark_t, "s")), Some((mark_t, rm)), &k::num(rm), false)
    }
}

fn three_bars(s: &Solved) -> Option<Figure> {
    let t0 = c::inp(s, "t")?;
    let times = [t0 / 5.0, t0, 2.0 * t0, 4.0 * t0, 8.0 * t0];
    let mut labels = vec![];
    let mut g = [vec![], vec![], vec![]];
    let mut notes = vec![];
    for t in times {
        let b = s.with(&[("t", t)])?;
        let v: Vec<f64> = ["A1", "A2", "A3"].iter().map(|y| k::num_of(&b, y)).collect::<Option<Vec<f64>>>()?;
        labels.push(k::q(t, "s"));
        for i in 0..3 {
            g[i].push(Some(v[i]));
        }
        notes.push(format!("t = {}: {} / {} / {} Bq", k::q(t, "s"), k::num(v[0]), k::num(v[1]), k::num(v[2])));
    }
    let groups = vec![("parent".to_string(), g[0].clone()), ("daughter".to_string(), g[1].clone()),
                      ("granddaughter".to_string(), g[2].clone())];
    let body = c::fit_right(&k::bar_groups(&labels, &groups, "time", "activity (Bq)", &format!("t = {}", k::q(t0, "s"))));
    Some(s.chart("bars", "Activities of a three-member chain at five times", &format!(
        "Activities of the three members (parent / daughter / granddaughter) at five times, each a solve of the model: {}.",
        notes.join("; ")), &body))
}

fn three_boxes(s: &Solved, cv: &mut Canvas) -> Option<[f64; 3]> {
    let (t1, t2, t3) = (c::inp(s, "T1")?, c::inp(s, "T2")?, c::inp(s, "T3")?);
    let a = [s.out("A1")?, s.out("A2")?, s.out("A3")?];
    for (i, (n, t)) in [("member 1", t1), ("member 2", t2), ("member 3", t3)].iter().enumerate() {
        let x = 30.0 + 170.0 * i as f64;
        cv.block(x, 40.0, 120.0, 34.0, n);
        cv.name(x + 60.0, 92.0, "middle", &format!("T½ = {}", k::q(*t, "s")));
        cv.value(x + 60.0, 110.0, "middle", &format!("{} Bq", k::num(a[i])));
        if i < 2 {
            c::varrow(cv, (x + 124.0, 57.0), (x + 166.0, 57.0), "axp-k");
        }
    }
    Some(a)
}

fn three_chain(s: &Solved) -> Option<Figure> {
    let t0 = c::inp(s, "t")?;
    let mut cv = Canvas::new(470.0);
    let a = three_boxes(s, &mut cv)?;
    let ts = k::span(0.0, t0 * 4.0, 60, false);
    let mut ser = vec![vec![], vec![], vec![]];
    for t in &ts {
        let Some(b) = s.with(&[("t", *t)]) else { continue };
        for (i, y) in ["A1", "A2", "A3"].iter().enumerate() {
            if let Some(v) = k::num_of(&b, y) {
                ser[i].push((*t, v));
            }
        }
    }
    let ch = k::lines(&[("member 1".into(), ser[0].clone()), ("member 2".into(), ser[1].clone()),
                        ("member 3".into(), ser[2].clone())], "t, time (s)", "activity (Bq)",
                      &format!("total {} Bq at t = {}", k::num(s.out("A_total")?), k::q(t0, "s")), Some((t0, a[1])), "", false);
    below(&mut cv, &ch, 124.0);
    fig(s, "chain", "A three-member decay chain", &format!(
        "A three-member decay chain: the chain with the model's activities at t = {} (top) and each member's activity \
         from it at 60 times (bottom). Marked: A1 = {}, A2 = {}, A3 = {} Bq, total {} Bq.", k::q(t0, "s"), k::num(a[0]),
        k::num(a[1]), k::num(a[2]), k::num(s.out("A_total")?)), &cv)
}

fn three_diagram(s: &Solved) -> Option<Figure> {
    let t0 = c::inp(s, "t")?;
    let mut cv = Canvas::new(260.0);
    let a = three_boxes(s, &mut cv)?;
    let tot = s.out("A_total")?;
    let sc = 120.0 / a.iter().cloned().fold(0.0, f64::max).max(1e-300);
    for (i, v) in a.iter().enumerate() {
        let x = 70.0 + 170.0 * i as f64;
        cv.raw(&format!("<rect class=\"axp-b{}\" x=\"{x:.1}\" y=\"{:.1}\" width=\"40\" height=\"{:.1}\"/>",
                        if i == 0 { "" } else { "2" }, 250.0 - v * sc, v * sc));
    }
    cv.bold(16.0, 24.0, "start", &format!("at t = {}: total {} Bq", k::q(t0, "s"), k::num(tot)));
    fig(s, "diagram", "Activities along a decay chain", &format!(
        "A three-member decay chain with the activities the model computes at t = {} (bars to scale): member 1 {} Bq, \
         member 2 {} Bq, member 3 {} Bq, total {} Bq.", k::q(t0, "s"), k::num(a[0]), k::num(a[1]), k::num(a[2]),
        k::num(tot)), &cv)
}

// ------------------------------------------------------------------------------------------------ reactor cores
fn flux_profile(s: &Solved, geometry: &str, ext: f64, h_ext: Option<f64>) -> Vec<(f64, f64)> {
    let mut p = vec![];
    for u in k::span(-ext / 2.0, ext / 2.0, 41, false) {
        let set: Vec<(&str, Input)> = match geometry {
            "slab" => vec![("geometry", txt("slab")), ("a", f(ext)), ("x", f(u))],
            "sphere" => vec![("geometry", txt("sphere")), ("R", f(ext / 2.0)), ("r", f(u.abs()))],
            _ => vec![("geometry", txt("cylinder")), ("R", f(ext / 2.0)), ("H", f(h_ext.unwrap_or(ext))), ("r", f(u.abs())),
                      ("z", f(0.0))],
        };
        if let Some(v) = oi(s, "nuc_flux_shape", &set, "ratio") {
            p.push((u, v));
        }
    }
    p
}

/// A critical core to scale with its extrapolated boundary dashed, and the flux across it below.
fn critical(s: &Solved, shape: &str) -> Option<Figure> {
    let (w, w_ext, h, h_ext, geo, label) = match shape {
        "sphere" => {
            let (r, re) = (s.out("R")?, s.out("R_ext")?);
            (2.0 * r, 2.0 * re, 2.0 * r, 2.0 * re, "sphere", format!("R = {}, extrapolated {}", k::q(r, "m"), k::q(re, "m")))
        }
        "slab" => {
            let (a, ae) = (s.out("a")?, s.out("a_ext")?);
            (a, ae, a * 2.5, a * 2.5, "slab", format!("thickness a = {}, extrapolated {}", k::q(a, "m"), k::q(ae, "m")))
        }
        "cylinder" => {
            let (r, re, hh) = (s.out("R")?, s.out("R_ext")?, c::inp(s, "H")?);
            (2.0 * r, 2.0 * re, hh, hh + 2.0 * (re - r), "cylinder", format!("R = {} for H = {}", k::q(r, "m"), k::q(hh, "m")))
        }
        _ => {
            let (r, hh) = (s.out("R")?, s.out("H")?);
            let hd = s.out("HD")?;
            let v = s.out("V_ext")?;
            (2.0 * r, 2.0 * r, hh, hh, "cylinder", format!("R = {}, H = {}, H/D = {}, V = {}", k::q(r, "m"), k::q(hh, "m"),
                                                         k::num(hd), q(v, "m^3")))
        }
    };
    if !(w > 0.0 && h > 0.0) {
        return None;
    }
    let mut cv = Canvas::new(464.0);
    let sc = 100.0 / w_ext.max(h_ext);
    let ctr = (430.0, 70.0);
    if geo == "sphere" {
        c::disc(&mut cv, ctr, w / 2.0 * sc, true);
        cv.raw(&format!("<circle class=\"axp-c4\" stroke-dasharray=\"5 4\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\"/>",
                        ctr.0, ctr.1, w_ext / 2.0 * sc));
    } else {
        c::rect(&mut cv, ctr.0 - w / 2.0 * sc, ctr.1 - h / 2.0 * sc, w * sc, h * sc);
        cv.raw(&format!("<rect class=\"axp-c4\" stroke-dasharray=\"5 4\" fill=\"none\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>",
                        ctr.0 - w_ext / 2.0 * sc, ctr.1 - h_ext / 2.0 * sc, w_ext * sc, h_ext * sc));
    }
    cv.bold(16.0, 24.0, "start", &format!("critical {}", if geo == "cylinder" { "cylinder" } else { geo }));
    cv.value(16.0, 44.0, "start", &label);
    if let Some(v) = s.out("V") {
        cv.value(16.0, 64.0, "start", &format!("V = {}", q(v, "m^3")));
    }
    let p = flux_profile(s, geo, w_ext, Some(h_ext));
    if p.len() < 2 {
        return None;
    }
    let band = vec![(-w / 2.0, 1.0), (w / 2.0, 1.0)];
    let ch = k::lines_y(&[("flux φ/φmax".into(), p), ("physical core".into(), band)], "position across the core (m)",
                      "φ / φmax", "", None, "", false, true);
    below(&mut cv, &ch, 124.0);
    fig(s, "geometry", &format!("The critical {} of a bare reactor", if geo == "cylinder" { "cylinder" } else { geo }), &format!(
        "The critical {} of a bare homogeneous reactor, drawn to scale: the core with its extrapolated boundary dashed. \
         Marked, from the model: {label}. Bottom: the flux across the core from the model's flux-shape capability on the \
         extrapolated dimensions; the second line spans the physical core.", if geo == "cylinder" { "cylinder" } else { geo }),
        &cv)
}

fn cuboid(s: &Solved) -> Option<Figure> {
    let (a, b) = (c::inp(s, "a")?, c::inp(s, "b")?);
    let (cc, ce, v) = (s.out("c")?, s.out("c_ext")?, s.out("V")?);
    let d = (ce - cc) / 2.0;
    let mut cv = Canvas::new(230.0);
    let sc = 160.0 / (a + 2.0 * d).max(b + 2.0 * d).max(ce);
    // front view a x c and side view b x c
    for (i, w) in [a, b].iter().enumerate() {
        let x0 = 130.0 + 220.0 * i as f64;
        let (x, y) = (x0 - w / 2.0 * sc, 120.0 - cc / 2.0 * sc);
        c::rect(&mut cv, x, y, w * sc, cc * sc);
        cv.raw(&format!("<rect class=\"axp-c4\" stroke-dasharray=\"5 4\" fill=\"none\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>",
                        x - d * sc, 120.0 - ce / 2.0 * sc, (w + 2.0 * d) * sc, ce * sc));
        cv.name(x0, 215.0, "middle", &format!("{} × c", if i == 0 { "a" } else { "b" }));
    }
    cv.bold(16.0, 24.0, "start", &format!("a = {}, b = {}: critical c = {}", k::q(a, "m"), k::q(b, "m"), k::q(cc, "m")));
    cv.value(16.0, 44.0, "start", &format!("extrapolated {}, V = {}", k::q(ce, "m"), q(v, "m^3")));
    fig(s, "geometry", "The critical cuboid of a bare reactor", &format!(
        "The critical cuboid of a bare homogeneous reactor, drawn to scale in two views: the core with its extrapolated \
         boundary dashed. Marked, from the model: a = {}, b = {}, critical c = {} (extrapolated {}), V = {}.",
        k::q(a, "m"), k::q(b, "m"), k::q(cc, "m"), k::q(ce, "m"), q(v, "m^3")), &cv)
}

fn bare_keff(s: &Solved) -> Option<Figure> {
    let geo = s.txt("geometry")?;
    let (kf, rho, pnl, b2) = (s.out("k_eff")?, s.out("rho")?, s.out("P_NL")?, s.out("B2")?);
    let mut cv = Canvas::new(464.0);
    let (w, h) = match geo.as_str() {
        "sphere" => (2.0 * c::inp(s, "R")?, 2.0 * c::inp(s, "R")?),
        "cylinder" => (2.0 * c::inp(s, "R")?, c::inp(s, "H")?),
        "slab" => (c::inp(s, "a")?, c::inp(s, "a")? * 2.5),
        _ => (c::inp(s, "a")?, c::inp(s, "c")?),
    };
    let sc = 100.0 / w.max(h);
    let ctr = (430.0, 70.0);
    if geo == "sphere" {
        c::disc(&mut cv, ctr, w / 2.0 * sc, true);
    } else {
        c::rect(&mut cv, ctr.0 - w / 2.0 * sc, ctr.1 - h / 2.0 * sc, w * sc, h * sc);
    }
    cv.bold(16.0, 24.0, "start", &format!("bare {geo}: k_eff = {}", k::num(kf)));
    cv.value(16.0, 44.0, "start", &format!("ρ = {}, P_NL = {}", k::num(rho), k::num(pnl)));
    cv.value(16.0, 64.0, "start", &format!("B² = {} 1/m²", k::num(b2)));
    let flux_geo = if geo == "cuboid" { "slab" } else { geo.as_str() };
    let p = flux_profile(s, flux_geo, w, Some(h));
    if p.len() >= 2 {
        let ch = k::lines_y(&[("flux φ/φmax (physical dimensions)".into(), p)], "position across the core (m)", "φ / φmax",
                          "", None, "", false, true);
        below(&mut cv, &ch, 124.0);
    }
    fig(s, "geometry", "A bare reactor and its multiplication factor", &format!(
        "A bare {geo} reactor to scale (the given dimensions) over its flux shape from the model's flux-shape capability \
         (on the physical dimensions). Marked: the model's k_eff = {}, reactivity {}, non-leakage probability {}, \
         buckling {} 1/m².", k::num(kf), k::num(rho), k::num(pnl), k::num(b2)), &cv)
}

/// A cylinder's half axial section shaded by φ/φmax on a 10 x 14 grid from the flux-shape capability.
fn flux_section(s: &Solved, power: bool) -> Option<Figure> {
    let geo = s.txt("geometry")?;
    if geo != "cylinder" {
        return slab_or_sphere_flux(s, &geo, power);
    }
    let (r, h) = (c::inp(s, "R")?, c::inp(s, "H")?);
    let mut cv = Canvas::new(420.0);
    let sc = (330.0 / h).min(200.0 / r);
    let (x0, y0) = (150.0, 225.0);
    let (nr, nz) = (10, 14);
    for i in 0..nr {
        for j in 0..nz {
            let rr = r * (i as f64 + 0.5) / nr as f64;
            let zz = -h / 2.0 + h * (j as f64 + 0.5) / nz as f64;
            let v = oi(s, "nuc_flux_shape", &[("geometry", txt("cylinder")), ("R", f(r)), ("H", f(h)), ("r", f(rr)), ("z", f(zz))], "ratio")?;
            cv.raw(&format!("<rect class=\"axp-hp\" fill-opacity=\"{:.2}\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>",
                            0.08 + 0.82 * v.clamp(0.0, 1.0), x0 + r * sc * i as f64 / nr as f64, y0 - h / 2.0 * sc + h * sc * j as f64 / nz as f64,
                            r * sc / nr as f64 + 0.5, h * sc / nz as f64 + 0.5));
        }
    }
    cv.raw(&format!("<rect class=\"axp-k\" fill=\"none\" x=\"{x0}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>",
                    y0 - h / 2.0 * sc, r * sc, h * sc));
    cv.dashed((x0, y0 - h / 2.0 * sc - 10.0), (x0, y0 + h / 2.0 * sc + 10.0));
    cv.name(x0 - 6.0, y0 + 4.0, "end", "axis");
    cv.name(x0 + r * sc / 2.0, y0 + h / 2.0 * sc + 18.0, "middle", &format!("R = {}", k::q(r, "m")));
    let mut notes = vec![];
    if power {
        let (p, pmax, pavg, peak) = (s.out("P")?, s.out("phi_max")?, s.out("phi_avg")?, s.out("peaking")?);
        for (i, t) in [format!("P = {}", k::q(p, "W")), format!("peak flux {} /(m²·s)", k::num(pmax)),
                       format!("average {} /(m²·s)", k::num(pavg)), format!("peaking {}", k::num(peak))].iter().enumerate() {
            cv.value(370.0, 80.0 + 22.0 * i as f64, "start", t);
        }
        notes.push(format!("power P = {}, peak flux {} /(m²·s), volume-average flux {} /(m²·s), peaking {}", k::q(p, "W"),
                           k::num(pmax), k::num(pavg), k::num(peak)));
    } else {
        let (rq, zq, ratio) = (c::inp(s, "r")?, c::inp(s, "z")?, s.out("ratio")?);
        let p = (x0 + rq * sc, y0 - zq * sc);
        cv.raw(&format!("<circle class=\"axp-m\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"6\"/>", p.0, p.1));
        cv.value(370.0, 80.0, "start", &format!("φ/φmax = {}", k::num(ratio)));
        cv.value(370.0, 102.0, "start", &format!("B² = {} 1/m²", k::num(s.out("B2")?)));
        notes.push(format!("the given point r = {}, z = {}, where the model returns φ/φmax = {}", k::q(rq, "m"), k::q(zq, "m"),
                           k::num(ratio)));
    }
    cv.bold(16.0, 24.0, "start", &format!("cylinder R = {}, H = {}: φ/φmax", k::q(r, "m"), k::q(h, "m")));
    fig(s, "geometry", "Flux shape of a bare cylindrical core", &format!(
        "A bare cylindrical core of extrapolated radius {} and height {} in a half axial section (axis dashed), shaded by \
         φ/φmax from the model's flux-shape capability on a 10 × 14 grid of (r, z). Marked: {}.", k::q(r, "m"),
        k::q(h, "m"), notes.join("; ")), &cv)
}

fn slab_or_sphere_flux(s: &Solved, geo: &str, power: bool) -> Option<Figure> {
    let ext = match geo {
        "slab" => c::inp(s, "a")?,
        "sphere" => 2.0 * c::inp(s, "R")?,
        _ => return None,
    };
    let p = flux_profile(s, geo, ext, None);
    let head = if power { format!("P = {}", k::q(s.out("P")?, "W")) } else { format!("φ/φmax = {}", k::num(s.out("ratio")?)) };
    chart(s, "geometry", &format!("Flux shape across a bare {geo}"), &format!(
        "The flux shape across a bare {geo} of extrapolated size {} from the model's flux-shape capability at 41 points. \
         Marked: {head}.", k::q(ext, "m")),
          vec![("φ / φmax".into(), p)], "position (m)", "φ / φmax", &head, None, "", false)
}

fn reflected(s: &Solved) -> Option<Figure> {
    let (a, delta, tr) = (s.out("a")?, s.out("delta")?, c::inp(s, "T_r")?);
    let bare = oi(s, "nuc_critical_slab", &[("D", f(c::inp(s, "D_c")?)), ("Sigma_a", f(c::inp(s, "Sigma_ac")?)),
                                            ("nuSigma_f", f(c::inp(s, "nuSigma_fc")?)),
                                            ("extrapolation", txt(&s.txt("extrapolation")?))], "a")?;
    let total = a + 2.0 * tr;
    let sc = 440.0 / total.max(bare);
    let mut cv = Canvas::new(260.0);
    let x0 = 260.0 - total / 2.0 * sc;
    cv.raw(&format!("<rect class=\"axp-s3\" x=\"{x0:.1}\" y=\"80\" width=\"{:.1}\" height=\"100\"/>", tr * sc));
    cv.raw(&format!("<rect class=\"axp-b\" x=\"{:.1}\" y=\"80\" width=\"{:.1}\" height=\"100\"/>", x0 + tr * sc, a * sc));
    cv.raw(&format!("<rect class=\"axp-s3\" x=\"{:.1}\" y=\"80\" width=\"{:.1}\" height=\"100\"/>", x0 + (tr + a) * sc, tr * sc));
    cv.name(x0 + tr * sc / 2.0, 200.0, "middle", "reflector");
    cv.name(x0 + (total - tr / 2.0) * sc, 200.0, "middle", "reflector");
    cv.value(260.0, 200.0, "middle", &format!("core a = {}", k::q(a, "m")));
    cv.dashed((260.0 - bare / 2.0 * sc, 70.0), (260.0 + bare / 2.0 * sc, 70.0));
    cv.name(260.0, 60.0, "middle", &format!("bare critical slab: {}", k::q(bare, "m")));
    cv.bold(16.0, 24.0, "start", &format!("reflector savings δ = {} per face", k::q(delta, "m")));
    fig(s, "geometry", "A reflected critical slab", &format!(
        "A reflected critical slab drawn to scale across its thickness, reflector {} thick on each face. Marked: the \
         model's critical core thickness a = {} and reflector savings δ = {} per face; the bare critical slab of the same \
         core, from its bare-slab capability, is {} thick (dashed).", k::q(tr, "m"), k::q(a, "m"), k::q(delta, "m"),
        k::q(bare, "m")), &cv)
}

// ------------------------------------------------------------------------------------------------ geometry of sources and detectors
fn detector(s: &Solved) -> Option<Figure> {
    let (d, a) = (c::inp(s, "d")?, c::inp(s, "a")?);
    let off = c::inp(s, "rho0").unwrap_or(0.0);
    let thick = c::inp(s, "t").filter(|_| s.key == "nuc_detector_efficiency_point_source");
    let mut cv = Canvas::new(260.0);
    let sc = 380.0 / (d + thick.unwrap_or(0.0)).max(2.4 * a.max(off + a)).max(1e-30);
    let sc = sc.min(160.0 / (a + off).max(1e-30));
    let src = (60.0, 140.0 - off * sc);
    let fx = 60.0 + d * sc;
    let (top, bot) = (140.0 - a * sc, 140.0 + a * sc);
    c::area(&mut cv, &[src, (fx, top), (fx, bot)], "axp-s");
    cv.raw(&format!("<rect class=\"axp-ka\" x=\"{fx:.1}\" y=\"{top:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>",
                    thick.map(|t| t * sc).unwrap_or(10.0).max(4.0), bot - top));
    cv.dashed((40.0, 140.0), (500.0, 140.0));
    cv.dot(src);
    cv.name(src.0, src.1 - 10.0, "middle", "source");
    cv.name((src.0 + fx) / 2.0, 240.0, "middle", &format!("d = {}", k::q(d, "m")));
    cv.name(fx + 14.0, top - 6.0, "start", &format!("radius {}", k::q(a, "m")));
    let lines: Vec<String> = match s.key {
        "nuc_count_rate_point_source" => vec![format!("geometric {}", k::num(s.out("g")?)),
                                              format!("count rate {} /s", k::num(s.out("R")?)),
                                              format!("{} counts/min", k::num(s.out("R_cpm")?))],
        "nuc_detector_efficiency_point_source" => vec![format!("geometric {}", k::num(s.out("g")?)),
                                                       format!("intrinsic {}", k::num(s.out("intrinsic")?)),
                                                       format!("total {}", k::num(s.out("eff")?))],
        _ => vec![format!("Ω = {} sr", k::num(s.out("Omega")?)), format!("g = Ω/4π = {}", k::num(s.out("g")?))],
    };
    for (i, l) in lines.iter().enumerate() {
        cv.value(16.0, 24.0 + 20.0 * i as f64, "start", l);
    }
    fig(s, "geometry", "A point source in front of a disk detector", &format!(
        "A point source in front of a disk detector, drawn to scale (distance {}, face radius {}{}{}); the shaded cone is \
         the solid angle the detector subtends. Marked, from the model: {}.", k::q(d, "m"), k::q(a, "m"),
        if off > 0.0 { format!(", source {} off the axis", k::q(off, "m")) } else { String::new() },
        thick.map(|t| format!(", thickness {}", k::q(t, "m"))).unwrap_or_default(), lines.join(", ")), &cv)
}

fn extended_source(s: &Solved, disk: bool) -> Option<Figure> {
    let (h, mu, t) = (c::inp(s, "h")?, c::inp(s, "mu")?, c::inp(s, "t")?);
    let (phi, phi0, tr) = (s.out("phi")?, s.out("phi0")?, s.out("T")?);
    let (y1, y2) = if disk { let r = c::inp(s, "R")?; (-r, r) } else { (c::inp(s, "y1")?, c::inp(s, "y2")?) };
    let mut cv = Canvas::new(280.0);
    let span = (y2 - y1).abs().max(h);
    let sc = 360.0 / span.max(1e-30);
    let sc = sc.min(180.0 / h.max(1e-30));
    let src_y = 220.0;
    let py = src_y - h * sc;
    let xm = 260.0;
    let x = |y: f64| xm + y * sc;
    cv.raw(&format!("<path class=\"axp-c\" style=\"stroke-width:5\" d=\"M{:.1},{src_y} L{:.1},{src_y}\"/>", x(y1), x(y2)));
    // the shield between source and point
    let sh = (t * sc).max(3.0);
    cv.raw(&format!("<rect class=\"axp-s4\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{sh:.1}\"/>", x(y1) - 20.0,
                    src_y - 20.0 - sh, (y2 - y1) * sc + 40.0));
    for i in 0..=6 {
        let y = y1 + (y2 - y1) * i as f64 / 6.0;
        cv.raw(&format!("<path class=\"axp-c4\" stroke-width=\"1\" d=\"M{:.1},{src_y} L{xm:.1},{py:.1}\"/>", x(y)));
    }
    cv.raw(&format!("<circle class=\"axp-m\" cx=\"{xm:.1}\" cy=\"{py:.1}\" r=\"6\"/>"));
    cv.name(xm + 10.0, py - 6.0, "start", &format!("point at h = {}", k::q(h, "m")));
    cv.name(x(y2) + 6.0, src_y + 16.0, "start", if disk { "disk source" } else { "line source" });
    cv.value(16.0, 24.0, "start", &format!("φ = {} /(m²·s)", k::num(phi)));
    cv.value(16.0, 44.0, "start", &format!("unshielded {} /(m²·s), transmission {}", k::num(phi0), k::num(tr)));
    cv.name(16.0, 64.0, "start", &format!("shield {} thick, μ = {} 1/m", k::q(t, "m"), k::num(mu)));
    fig(s, "geometry", &format!("Flux from a shielded {} source", if disk { "disk" } else { "line" }), &format!(
        "A {} source from {} to {} seen from a point at h = {} on its perpendicular through a shield of thickness {} and \
         μ = {} 1/m (the lines are sample rays). Marked: the model's uncollided flux φ = {} /(m²·s), the flux without the \
         shield {} /(m²·s), transmission {}.", if disk { "disk" } else { "line" }, k::q(y1, "m"), k::q(y2, "m"),
        k::q(h, "m"), k::q(t, "m"), k::num(mu), k::num(phi), k::num(phi0), k::num(tr)), &cv)
}

// ------------------------------------------------------------------------------------------------ measured data and fits
fn data_xy(s: &Solved, xs: &str, ys: &str, xlab: &str, ylab: &str) -> Option<Figure> {
    let (x, y) = (s.vec(xs)?, s.vec(ys)?);
    if x.len() != y.len() || x.len() < 2 {
        return None;
    }
    let pts: Vec<(f64, f64)> = x.iter().cloned().zip(y.iter().cloned()).collect();
    let outs: Vec<String> = s.a.values.iter().filter_map(|v| v.1.as_f64().map(|n| format!("{} = {}", v.0, k::num(n)))).collect();
    chart(s, "xy", &format!("The data: {ys} against {xs}"), &format!(
        "The data {ys} against {xs} as given ({} points). Also returned by the model: {}.", pts.len(), outs.join(", ")),
          vec![(String::new(), pts.clone())], xlab, ylab, outs.first().cloned().unwrap_or_default().as_str(),
          pts.last().copied(), "", false)
}

fn attenuation_fit(s: &Solved) -> Option<Figure> {
    let (x, i) = (s.vec("x")?, s.vec("I")?);
    let (mu, i0, hvl, tvl) = (s.out("mu")?, s.out("I0")?, s.out("HVL")?, s.out("TVL")?);
    let xmax = x.iter().cloned().fold(0.0, f64::max);
    let mut fit = vec![];
    for xc in k::span(0.0, xmax.max(1e-9), 51, false) {
        let t = oi(s, "nuc_half_value_layer", &[("mu", f(mu)), ("x", f(xc / 100.0))], "transmission")?;
        fit.push((xc, i0 * t));
    }
    let data: Vec<(f64, f64)> = x.iter().cloned().zip(i.iter().cloned()).collect();
    let body = c::fit_right(&k::lines_y(&[("fit".into(), fit), ("measured".into(), data)], "absorber thickness (cm)",
                                        "intensity", &format!("μ = {} 1/m", k::num(mu)), None, "", false, false));
    Some(s.chart("fit", "Narrow-beam attenuation and its fit", &format!(
        "Narrow-beam attenuation: the measured intensities and the fit by the model, μ = {} 1/m with I0 = {}, drawn as I0 \
         times the transmission it computes at 51 thicknesses; half-value layer {}, tenth-value layer {}.", k::num(mu),
        k::num(i0), k::q(hvl, "m"), k::q(tvl, "m")), &body))
}

fn unit_s(u: &str) -> f64 {
    match u {
        "min" => 60.0,
        "h" => 3600.0,
        "d" => 86400.0,
        "y" => 365.25 * 86400.0,
        _ => 1.0,
    }
}

fn decay_fit(s: &Solved) -> Option<Figure> {
    let (t, cn) = (s.vec("t")?, s.vec("C")?);
    let u = s.txt("t_unit").unwrap_or_else(|| "s".into());
    let us = unit_s(&u);
    let (lam, th, c0, st) = (s.out("lambda_")?, s.out("T_half")?, s.out("C0")?, s.out("sigma_T")?);
    let tmax = t.iter().cloned().fold(0.0, f64::max);
    let fit: Vec<(f64, f64)> = k::span(0.0, tmax, 41, false).iter().map(|x| (*x, c0 * (-lam * x * us).exp())).collect();
    let data: Vec<(f64, f64)> = t.iter().cloned().zip(cn.iter().cloned()).collect();
    let body = c::fit_right(&k::lines(&[("fit C0 e^(-λt)".into(), fit), ("measured".into(), data)],
                                      &format!("t, time ({u})"), "counts", &format!("T½ = {}", k::q(th, "s")), None, "", false));
    Some(s.chart("fit", "A decay curve and its fitted half-life", &format!(
        "Decay-counting data ({} counts over {} {u}) with the model's fit: decay constant {} 1/s, half-life {} ± {} and C0 \
         = {}; the line is C0 e^(-λt) drawn from them.", t.len(), k::num(tmax), k::num(lam), k::q(th, "s"), k::q(st, "s"),
        k::num(c0)), &body))
}

fn two_fit(s: &Solved) -> Option<Figure> {
    let (t, cn) = (s.vec("t")?, s.vec("C")?);
    let (t1, t2) = (c::inp(s, "T1")?, c::inp(s, "T2")?);
    let u = s.txt("t_unit").unwrap_or_else(|| "s".into());
    let us = unit_s(&u);
    let (c1, c2, fr) = (s.out("C1")?, s.out("C2")?, s.out("frac1")?);
    let tmax = t.iter().cloned().fold(0.0, f64::max);
    let ts = k::span(0.0, tmax, 41, false);
    let comp = |cc: f64, th: f64| -> Vec<(f64, f64)> { ts.iter().map(|x| (*x, cc * (-LN_2 / th * x * us).exp())).collect() };
    let sum: Vec<(f64, f64)> = ts.iter().map(|x| (*x, c1 * (-LN_2 / t1 * x * us).exp() + c2 * (-LN_2 / t2 * x * us).exp())).collect();
    let data: Vec<(f64, f64)> = t.iter().cloned().zip(cn.iter().cloned()).collect();
    let body = c::fit_right(&k::lines(&[("sum".into(), sum), ("component 1".into(), comp(c1, t1)),
                                        ("component 2".into(), comp(c2, t2)), ("measured".into(), data)],
                                      &format!("t, time ({u})"), "counts", &format!("C1 = {}, C2 = {}", k::num(c1), k::num(c2)),
                                      None, "", false));
    Some(s.chart("fit", "A two-component decay curve", &format!(
        "Decay-counting data ({} counts over {} {u}) with the model's fit: the initial counts C1 = {} and C2 = {} of the two \
         components with the stated half-lives {} and {} (short-lived fraction {}); the curves are drawn from them.",
        t.len(), k::num(tmax), k::num(c1), k::num(c2), k::q(t1, "s"), k::q(t2, "s"), k::num(fr)), &body))
}

fn inverse_m(s: &Solved) -> Option<Figure> {
    let (x, cn) = (s.vec("x")?, s.vec("C")?);
    let (xc, slope, xc2) = (s.out("x_crit")?, s.out("slope")?, s.out("x_crit_last2")?);
    if cn.is_empty() || cn[0] == 0.0 {
        return None;
    }
    let inv: Vec<(f64, f64)> = x.iter().cloned().zip(cn.iter().map(|cc| cn[0] / cc)).collect();
    let line = vec![(x[0], slope * (x[0] - xc)), (xc, 0.0)];
    chart(s, "fit", "Approach to criticality: inverse multiplication", &format!(
        "Approach to criticality: inverse multiplication 1/M = C0/C (derived from the given counts) against loading, with \
         the line the model fits (slope {}). Marked: its predicted critical loading {}, where 1/M reaches zero (from the \
         last two points: {}).", k::num(slope), k::num(xc), k::num(xc2)),
          vec![("1/M".into(), inv), ("fitted line".into(), line)], "loading", "1/M", &format!("critical at {}", k::num(xc)),
          Some((xc, 0.0)), &k::num(xc), false)
}

// ------------------------------------------------------------------------------------------------ shielding
fn hvl_layers(s: &Solved) -> Option<Figure> {
    let mu = c::inp(s, "mu")?;
    let (hvl, tvl, mfp) = (s.out("HVL")?, s.out("TVL")?, s.out("mfp")?);
    let xs = k::span(0.0, tvl * 1.5, 50, false);
    let p: Vec<(f64, f64)> = xs.iter().filter_map(|x| wi(s, &[("x", f(*x))], "transmission").map(|y| (*x, y))).collect();
    let marks = vec![(hvl, 0.5), (mfp, (-1.0f64).exp()), (tvl, 0.1)];
    let body = c::fit_right(&k::lines(&[("transmission".into(), p), ("HVL, 1/μ, TVL".into(), marks)],
                                      "absorber thickness (m)", "transmission", &format!("HVL = {}", k::q(hvl, "m")),
                                      Some((hvl, 0.5)), "0.5", false));
    Some(s.chart("layers", "Transmission through an absorber", &format!(
        "Narrow-beam transmission against absorber thickness for μ = {} 1/m, 50 solves of the model. Marked: its \
         half-value layer {} (transmission 0.5), mean free path {} (1/e) and tenth-value layer {} (0.1).", k::num(mu),
        k::q(hvl, "m"), k::q(mfp, "m"), k::q(tvl, "m")), &body))
}

fn multilayer(s: &Solved) -> Option<Figure> {
    let (mu, x) = (s.vec("mu")?, s.vec("x")?);
    if mu.len() != x.len() || mu.is_empty() || mu.len() > 8 {
        return None;
    }
    let tt = s.out("T")?;
    let total: f64 = x.iter().sum();
    let mut pts = vec![(0.0, 1.0)];
    let mut acc = 0.0;
    for i in 0..mu.len() {
        for j in 1..=10 {
            let part = x[i] * j as f64 / 10.0;
            let mut xs: Vec<f64> = x[..i].to_vec();
            xs.push(part);
            let t = wi(s, &[("mu", list(&mu[..=i])), ("x", list(&xs))], "T")?;
            pts.push((acc + part, t));
        }
        acc += x[i];
    }
    let mut cv = Canvas::new(474.0);
    let sc = 400.0 / total.max(1e-30);
    let mut x0 = 96.0;
    let mut notes = vec![];
    for i in 0..mu.len() {
        cv.raw(&format!("<rect class=\"axp-b{}\" x=\"{x0:.1}\" y=\"40\" width=\"{:.1}\" height=\"60\"/>",
                        ["", "2", "3", "4"][i % 4], x[i] * sc));
        cv.name(x0 + x[i] * sc / 2.0, 120.0, "middle", &format!("{} cm", k::num(x[i])));
        notes.push(format!("{} cm at {} 1/cm", k::num(x[i]), k::num(mu[i])));
        x0 += x[i] * sc;
    }
    cv.name(16.0, 24.0, "start", &format!("layers: {}", notes.join(", ")));
    let ch = k::lines(&[("transmission".into(), pts)], "depth into the shield (cm)", "uncollided transmission",
                      &format!("T = {}", k::num(tt)), None, "", false);
    below(&mut cv, &ch, 134.0);
    fig(s, "layers", "Transmission through a multilayer shield", &format!(
        "A shield of {} layers drawn to scale ({}) over the uncollided transmission through it, each point the model's \
         transmission of the layers crossed so far. Marked: total T = {} (optical thickness {}, {} half-value layers).",
        mu.len(), notes.join("; "), k::num(tt), k::num(s.out("tau")?), k::num(s.out("n_hvl")?)), &cv)
}

fn dose_layers(s: &Solved) -> Option<Figure> {
    let (r, x0) = (c::inp(s, "r")?, c::inp(s, "x")?);
    let (hd, b) = (s.out("H_uSv_h")?, s.out("B")?);
    let xs = k::span(0.0, (x0 * 2.0).max(r * 0.1).min(r), 41, false);
    let p: Vec<(f64, f64)> = xs.iter().filter_map(|x| wi(s, &[("x", f(*x))], "H_uSv_h").map(|y| (*x, y))).collect();
    let mut cv = Canvas::new(394.0);
    cv.dot((60.0, 30.0));
    cv.name(60.0, 20.0, "middle", "source");
    cv.raw("<rect class=\"axp-s4\" x=\"200\" y=\"12\" width=\"30\" height=\"36\"/>");
    cv.name(215.0, 62.0, "middle", &format!("shield {}", k::q(x0, "m")));
    c::varrow(&mut cv, (66.0, 30.0), (440.0, 30.0), "axp-c4");
    cv.name(450.0, 34.0, "start", &format!("r = {}", k::q(r, "m")));
    let ch = k::lines(&[("dose rate".into(), p)], "shield thickness x (m)", "dose rate (µSv/h)",
                      &format!("{} µSv/h at x = {}", k::num(hd), k::q(x0, "m")), Some((x0, hd)), "", false);
    below(&mut cv, &ch, 54.0);
    fig(s, "layers", "Dose rate behind a shield", &format!(
        "A point source seen at r = {} through a shield (the geometry not to scale across), and the model's dose rate for \
         41 shield thicknesses. Marked: at x = {} the dose rate {} µSv/h, buildup factor {}, transmission {}.",
        k::q(r, "m"), k::q(x0, "m"), k::num(hd), k::num(b), k::num(s.out("T")?)), &cv)
}

// ------------------------------------------------------------------------------------------------ temperatures
fn coolant_profile(s: &Solved, clad: bool) -> Option<Figure> {
    let h = c::inp(s, "H")?;
    let zs = k::span(-h / 2.0, h / 2.0, 41, false);
    let base: Vec<(&str, Input)> = ["q0", "H", "H_e", "m_dot", "c_p", "T_in"].iter()
        .filter_map(|x| c::inp(s, x).map(|v| (*x, f(v)))).collect();
    let mut p = vec![];
    for z in &zs {
        let mut set = base.clone();
        set.push(("z", f(*z)));
        if let Some(t) = if clad { oi(s, "nuc_channel_coolant_temperature", &set, "T") } else { wi(s, &[("z", f(*z))], "T") } {
            p.push((*z, t));
        }
    }
    if clad {
        let (zm, tm, to) = (s.out("z_max")?, s.out("T_max")?, s.out("T_out")?);
        chart(s, "profile", "Coolant temperature and the hottest cladding point", &format!(
            "A coolant channel with a chopped-cosine power shape: the model's coolant temperature at 41 heights (from its \
             coolant-temperature capability). Marked: the model's hottest cladding point T_max = {} at z = {} above the \
             mid-plane; coolant outlet {}.", q(tm, "K"), k::q(zm, "m"), q(to, "K")),
              vec![("coolant".into(), p), ("hottest cladding".into(), vec![(zm, tm), (zm, tm)])], "z, height from the mid-plane (m)",
              "temperature (K)", &format!("T_max = {}", q(tm, "K")), Some((zm, tm)), &q(tm, "K"), false)
    } else {
        let (z0, t0, to, qq) = (c::inp(s, "z")?, s.out("T")?, s.out("T_out")?, s.out("Q")?);
        chart(s, "profile", "Coolant temperature along a channel", &format!(
            "A coolant channel with a chopped-cosine power shape: the model's coolant temperature at 41 heights. Marked: \
             T = {} at z = {}; outlet {}, heat added {}.", q(t0, "K"), k::q(z0, "m"), q(to, "K"), k::q(qq, "W")),
              vec![("coolant".into(), p)], "z, height from the mid-plane (m)", "temperature (K)", &format!("T = {}", q(t0, "K")),
              Some((z0, t0)), &q(t0, "K"), false)
    }
}

fn rod_profile(s: &Solved) -> Option<Figure> {
    let (q_lin, rf, tc, h, tcool) = (c::inp(s, "q_lin")?, c::inp(s, "R_f")?, c::inp(s, "t_c")?, c::inp(s, "h")?, c::inp(s, "T_cool")?);
    let (t0, tfs, tci) = (s.out("T_0")?, s.out("T_fs")?, s.out("T_ci")?);
    // derived for the drawing: the parabola in the fuel, the gap step, the cladding to its outer surface
    let rco = rf + tc;
    let tco = tcool + q_lin / (2.0 * PI * rco * h);
    let fuel: Vec<(f64, f64)> = k::span(0.0, rf, 21, false).iter().map(|r| (r * 1000.0, t0 - (t0 - tfs) * (r / rf).powi(2))).collect();
    let clad = vec![(rf * 1000.0, tci), (rco * 1000.0, tco)];
    let gap = vec![(rf * 1000.0, tfs), (rf * 1000.0, tci)];
    let cool = vec![(rco * 1000.0, tco), (rco * 1000.0 * 1.3, tcool)];
    chart(s, "profile", "Radial temperatures in a fuel rod", &format!(
        "Radial temperatures in a fuel rod (linear heat rate {} W/m, pellet radius {}, cladding {}, coolant at {}). Marked: \
         the model's centreline {}, pellet surface {} and cladding inner surface {}. Derived for the drawing: the parabola \
         in the fuel, the step across the gap, the cladding to its outer surface {} and the film to the coolant.",
        k::num(q_lin), k::q(rf, "m"), k::q(tc, "m"), q(tcool, "K"), q(t0, "K"), q(tfs, "K"), q(tci, "K"), q(tco, "K")),
          vec![("fuel".into(), fuel), ("gap".into(), gap), ("cladding".into(), clad), ("coolant film".into(), cool)],
          "radius (mm)", "temperature (K)", &format!("centreline {}", q(t0, "K")), Some((0.0, t0)), &q(t0, "K"), false)
}

// ------------------------------------------------------------------------------------------------ spectra, counts, processes
fn lines_spectrum(s: &Solved, escape: bool) -> Option<Figure> {
    let mut lines: Vec<(String, f64, f64)> = vec![];
    let e = if escape { s.out("E_keV")? } else { c::inp(s, "E")? / 1.602176634e-13 * 1000.0 };
    if escape {
        lines.push(("full energy".into(), e, 1.0));
        lines.push(("single escape".into(), s.out("single_escape_keV")?, 0.55));
        lines.push(("double escape".into(), s.out("double_escape_keV")?, 0.35));
        lines.push(("annihilation".into(), s.out("annihilation_keV")?, 0.45));
    } else {
        lines.push(("photopeak".into(), e, 1.0));
        lines.push(("Compton edge".into(), s.out("T_max")? * 1000.0, 0.45));
        lines.push(("backscatter".into(), s.out("E_bs")? * 1000.0, 0.3));
    }
    let emax = e * 1.15;
    let mut cv = Canvas::new(300.0);
    let px = c::lin(0.0, emax, 50.0, 490.0);
    cv.line((50.0, 250.0), (490.0, 250.0));
    for t in c::nice_levels(0.0, emax, 6) {
        cv.line((px(t), 250.0), (px(t), 256.0));
        cv.name(px(t), 272.0, "middle", &k::num(t));
    }
    cv.name(270.0, 292.0, "middle", "energy (keV); heights schematic");
    for (i, (n, x, hgt)) in lines.iter().enumerate() {
        let y = 250.0 - 190.0 * hgt;
        cv.raw(&format!("<path class=\"{}\" d=\"M{:.1},250 L{:.1},{y:.1}\"/>", if i == 0 { "axp-c" } else { "axp-c3" }, px(*x), px(*x)));
        cv.value(px(*x), y - 20.0, "middle", &format!("{} keV", k::num(*x)));
        cv.name(px(*x), y - 6.0, "middle", n);
    }
    fig(s, "spectrum", if escape { "Escape peaks in a detector spectrum" } else { "Compton edge and backscatter peak" }, &format!(
        "Where the lines of a {} keV gamma ray fall in a detector spectrum; heights are schematic, the energies are the \
         model's: {}.", k::num(e), lines.iter().map(|(n, x, _)| format!("{n} {} keV", k::num(*x))).collect::<Vec<_>>().join(", ")), &cv)
}

fn staircase(s: &Solved) -> Option<Figure> {
    let (a, e0, ef) = (c::inp(s, "A")?, c::inp(s, "E0")?, c::inp(s, "Ef")?);
    let (n, xi) = (s.out("n")?, s.out("xi")?);
    if !(xi > 0.0 && n > 0.0 && n < 5000.0) {
        return None;
    }
    let mut p = vec![];
    let mut e = e0;
    let mut i = 0usize;
    while e > ef && i < 5000 {
        p.push((i as f64, e / 1.602176634e-19));
        e *= (-xi).exp();
        i += 1;
        p.push((i as f64, p[p.len() - 1].1));
    }
    p.push((i as f64, e / 1.602176634e-19));
    chart(s, "chart", "Slowing down by elastic collisions", &format!(
        "Slowing down by elastic collisions with nuclei of mass number {} from {} eV to {} eV: on average each collision \
         divides the energy by e^ξ, so the staircase (drawn from the model's ξ = {}, derived steps) falls evenly on a \
         logarithmic axis. Marked: the model's number of collisions n = {}.", k::num(a), k::num(e0 / 1.602176634e-19),
        k::num(ef / 1.602176634e-19), k::num(xi), k::num(n)),
          vec![(String::new(), p)], "collisions", "neutron energy (eV)", &format!("n = {}", k::num(n)),
          Some((n, ef / 1.602176634e-19)), &k::num(n), false)
}

fn six_factor(s: &Solved) -> Option<Figure> {
    let (eta, ff, p, eps) = (c::inp(s, "eta")?, c::inp(s, "f")?, c::inp(s, "p")?, c::inp(s, "eps")?);
    let (pf, pt) = (c::inp(s, "P_FNL").unwrap_or(1.0), c::inp(s, "P_TNL").unwrap_or(1.0));
    let (ki, ke, rho) = (s.out("k_inf")?, s.out("k_eff")?, s.out("rho")?);
    let steps = [("start", 1.0), ("× ε", eps), ("× P_FNL", pf), ("× p", p), ("× P_TNL", pt), ("× f", ff), ("× η", eta)];
    let mut labels = vec![];
    let mut vals = vec![];
    let mut n = 1000.0;
    for (l, fct) in steps {
        n *= fct;
        labels.push(l.to_string());
        vals.push(n);
    }
    let body = c::fit_right(&k::bars(&labels, &vals, &[6], "", "neutrons", &format!("k_eff = {}", k::num(ke))));
    Some(s.chart("cycle", "One neutron generation through the six factors", &format!(
        "One neutron generation through the six-factor formula, starting from 1000 fast neutrons (a stated number): each \
         bar is the previous times one factor (ε = {}, non-leakage {}, p = {}, thermal non-leakage {}, f = {}, η = {}; the \
         products are derived). The last bar matches the model's k_eff = {} (k∞ = {}, reactivity {}).", k::num(eps),
        k::num(pf), k::num(p), k::num(pt), k::num(ff), k::num(eta), k::num(ke), k::num(ki), k::num(rho)), &body))
}

fn enrichment(s: &Solved) -> Option<Figure> {
    let (xp, xf, xw, p) = (c::inp(s, "x_p")?, c::inp(s, "x_f")?, c::inp(s, "x_w")?, c::inp(s, "P")?);
    let (fm, w, swu) = (s.out("F")?, s.out("W")?, s.out("SWU")?);
    let mut cv = Canvas::new(260.0);
    cv.block(40.0, 110.0, 110.0, 40.0, "feed");
    cv.block(200.0, 100.0, 120.0, 60.0, "cascade");
    cv.block(380.0, 50.0, 110.0, 40.0, "product");
    cv.block(380.0, 170.0, 110.0, 40.0, "tails");
    c::varrow(&mut cv, (150.0, 130.0), (198.0, 130.0), "axp-c");
    c::varrow(&mut cv, (320.0, 115.0), (378.0, 70.0), "axp-c2");
    c::varrow(&mut cv, (320.0, 145.0), (378.0, 190.0), "axp-c3");
    cv.value(95.0, 170.0, "middle", &format!("F = {} at {} %", k::q(fm, "kg").replace(" kg", " kg"), k::num(xf * 100.0)));
    cv.value(435.0, 106.0, "middle", &format!("P = {} at {} %", k::num(p), k::num(xp * 100.0)));
    cv.value(435.0, 226.0, "middle", &format!("W = {} at {} %", k::num(w), k::num(xw * 100.0)));
    cv.bold(260.0, 186.0, "middle", &format!("{} SWU", k::num(swu)));
    cv.bold(16.0, 24.0, "start", "masses in kg, assays in % U-235");
    fig(s, "process", "Uranium enrichment balance", &format!(
        "Uranium enrichment balance: product {} kg at {} % U-235 from feed at {} %, tails at {} %. Marked, from the model: \
         feed F = {} kg, tails W = {} kg and the separative work {} SWU.", k::num(p), k::num(xp * 100.0), k::num(xf * 100.0),
        k::num(xw * 100.0), k::num(fm), k::num(w), k::num(swu)), &cv)
}

fn power_heat(s: &Solved) -> Option<Figure> {
    let rho0 = c::inp(s, "rho")?;
    let t0 = c::inp(s, "t")?;
    // a slow calculation (six delayed groups, about a tenth of a second a solve in a browser): a 2 x 3 grid, the
    // given cell from the answer itself, the stable period read from the same solves
    let rhos = [rho0, rho0 * 2.0];
    let ts = [t0 / 10.0, t0 / 2.0, t0];
    let mut m = vec![];
    let mut notes = vec![];
    for r in rhos {
        let mut row = vec![];
        let mut period = None;
        for t in ts {
            let b = if r == rho0 && t == t0 { Some(s.a.clone()) } else { s.with(&[("rho", r), ("t", t)]) };
            row.push(b.as_ref().and_then(|b| k::num_of(b, "P")).unwrap_or(f64::NAN));
            period = period.or_else(|| b.as_ref().and_then(|b| k::num_of(b, "T")));
        }
        if let Some(tp) = period {
            notes.push(format!("ρ = {}: stable period {}", k::num(r), k::q(tp, "s")));
        }
        m.push(row);
    }
    let rl: Vec<String> = rhos.iter().map(|r| format!("ρ = {}", k::num(*r))).collect();
    let cl: Vec<String> = ts.iter().map(|t| k::q(*t, "s")).collect();
    let body = k::heat(&m, &rl, &cl, "P/P0");
    if body.is_empty() {
        return None;
    }
    Some(s.chart("heatmap", "Reactor power after a reactivity step", &format!(
        "Relative reactor power P/P0 after a step of reactivity (the given delayed-neutron groups and generation time): \
         rows are the reactivities, columns the times, every cell a solve of the model; {}. \
         Given: ρ = {}, P = {} at t = {}.", notes.join("; "), k::num(rho0), k::num(s.out("P")?), k::q(t0, "s")),
                 &c::fit_right(&body)))
}

fn moderator(s: &Solved) -> Option<Figure> {
    let n = s.vec("n")?;
    if n.len() < 2 {
        return None;
    }
    let mr = s.out("MR")?;
    let j = 1;
    let n0 = n[j];
    let mut p = vec![];
    for v in k::span((n0 * 0.5).max(0.05), n0 * 2.5, 41, false) {
        let mut nn = n.clone();
        nn[j] = v;
        if let Some(y) = wi(s, &[("n", list(&nn))], "MR") {
            p.push((v, y));
        }
    }
    chart(s, "sensitivity", "Moderating ratio against the second element's atom count", &format!(
        "MR (moderating ratio) as the second value of n (atoms of each element per molecule) moves from {} to {}, the \
         other values as given; 41 solves of the model. Marked: the given n[2] = {}, MR = {}.", k::num((n0 * 0.5).max(0.05)),
        k::num(n0 * 2.5), k::num(n0), k::num(mr)),
          vec![(String::new(), p)], "n[2], atoms per molecule", "MR, moderating ratio", &format!("MR = {}", k::num(mr)),
          Some((n0, mr)), &k::num(mr), false)
}
