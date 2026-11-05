//! nuclear_engineering: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/nuclear_engineering/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "nuc_accumulated_dose_decaying_source",
    "nuc_activation_activity",
    "nuc_activation_with_burnup",
    "nuc_attenuation_coefficient_fit",
    "nuc_bare_reactor_keff",
    "nuc_centreline_temperature_variable_k",
    "nuc_chain_activity",
    "nuc_channel_coolant_temperature",
    "nuc_collisions_to_thermalize",
    "nuc_committed_dose_coefficient",
    "nuc_committed_dose_retention",
    "nuc_compton_edge",
    "nuc_constant_power_depletion",
    "nuc_control_rod_worth",
    "nuc_core_power_from_flux",
    "nuc_count_rate_point_source",
    "nuc_count_time_for_precision",
    "nuc_count_time_split",
    "nuc_counts_during_decay",
    "nuc_critical_cuboid",
    "nuc_critical_cylinder",
    "nuc_critical_cylinder_minimum_volume",
    "nuc_critical_slab",
    "nuc_critical_sphere",
    "nuc_currie_detection_limits",
    "nuc_daughter_max_activity_time",
    "nuc_dead_time_nonparalyzable",
    "nuc_dead_time_paralyzable",
    "nuc_decay_curve_fit",
    "nuc_decay_heat_energy",
    "nuc_decay_heat_power",
    "nuc_depletion_constant_flux",
    "nuc_detector_efficiency_point_source",
    "nuc_diffusion_length",
    "nuc_disk_source_flux",
    "nuc_effective_dose",
    "nuc_effective_half_life",
    "nuc_elastic_scattering",
    "nuc_enrichment_swu",
    "nuc_equilibrium_ratio",
    "nuc_escape_peaks",
    "nuc_exponential_pile",
    "nuc_flux_from_foil_activation",
    "nuc_flux_shape",
    "nuc_fuel_cross_sections",
    "nuc_fuel_rod_temperatures",
    "nuc_generator_daughter_activity",
    "nuc_half_value_layer",
    "nuc_inverse_multiplication",
    "nuc_irradiation_time_for_saturation",
    "nuc_klein_nishina",
    "nuc_line_source_flux",
    "nuc_logarithmic_energy_decrement",
    "nuc_mass_for_activity",
    "nuc_max_clad_temperature",
    "nuc_maxwellian_average_cross_section",
    "nuc_moderator_mixture",
    "nuc_multilayer_transmission",
    "nuc_plane_source_slab_flux",
    "nuc_point_source_dose_rate",
    "nuc_point_source_sphere_flux",
    "nuc_power_after_step_one_group",
    "nuc_power_after_step_six_group",
    "nuc_power_from_reaction_rate",
    "nuc_radiation_unit_conversion",
    "nuc_reactivity_from_period",
    "nuc_reactor_period_one_group",
    "nuc_reflected_slab_critical",
    "nuc_samarium",
    "nuc_shield_thickness_for_dose_rate",
    "nuc_six_factor_formula",
    "nuc_solid_angle_off_axis",
    "nuc_solid_angle_on_axis",
    "nuc_specific_activity",
    "nuc_stable_period_six_group",
    "nuc_stay_time_decaying_source",
    "nuc_thermal_neutron_speeds",
    "nuc_three_member_chain",
    "nuc_transit_dose",
    "nuc_two_component_decay_fit",
    "nuc_two_group_bare_keff",
    "nuc_xenon_after_shutdown",
    "nuc_xenon_equilibrium",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "nuc_accumulated_dose_decaying_source" => {
            let r = f_accumulated_dose(match v.get("H0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_half") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("td") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_23(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_activation_activity" => {
            let r = f_activation(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_half") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_irr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_d") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_80(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_activation_with_burnup" => {
            let r = f_activation_burnup(match v.get("N_T0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma_T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma_P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_half") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_132(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_attenuation_coefficient_fit" => {
            let r = f_attenuation_fit(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_196(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_bare_reactor_keff" => {
            let r = f_bare_keff(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sigma_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nuSigma_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("extrapolation") { Some(x) => py::from_val(x), None => { return None } }, match v.get("geometry") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("b") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("c") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("R") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("H") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_263(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_centreline_temperature_variable_k" => {
            let r = f_centreline_variable_k(match v.get("q_lin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_307(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_chain_activity" => {
            let r = f_chain_activity(match v.get("nuclide") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_370(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_channel_coolant_temperature" => {
            let r = f_channel_temperature(match v.get("q0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H_e") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_dot") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c_p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_in") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_419(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_collisions_to_thermalize" => {
            let r = f_collisions_to_thermalize(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ef") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_470(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_committed_dose_coefficient" => {
            let r = f_committed_dose(match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_516(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_committed_dose_retention" => {
            let r = s_542(match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tb") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_unit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w_R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_582(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_compton_edge" => {
            let r = f_compton_edge(match v.get("E") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_626(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_constant_power_depletion" => {
            let r = f_constant_power_depletion(match v.get("m0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma_f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_678(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_control_rod_worth" => {
            let r = f_rod_worth(match v.get("rho_tot") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_734(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_core_power_from_flux" => {
            let r = f_core_power(match v.get("geometry") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sigma_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi_max") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("P") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("R") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("H") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("a") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("b") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("c") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_806(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_count_rate_point_source" => {
            let r = f_count_rate_point_source(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps_int") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_859(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_count_time_for_precision" => {
            let r = f_count_time_for_precision(match v.get("Rg") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rb") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_915(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_count_time_split" => {
            let r = f_count_time_split(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rg") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rb") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_962(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_counts_during_decay" => {
            let r = f_counts_during_decay(match v.get("A0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_half") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eff") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("td") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1019(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_critical_cuboid" => {
            let r = f_critical_cuboid(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sigma_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nuSigma_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("extrapolation") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1069(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_critical_cylinder" => {
            let r = f_critical_cylinder(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sigma_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nuSigma_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("extrapolation") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1119(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_critical_cylinder_minimum_volume" => {
            let r = f_critical_cylinder_min(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sigma_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nuSigma_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("extrapolation") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1167(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_critical_slab" => {
            let r = f_critical_slab(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sigma_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nuSigma_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("extrapolation") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1215(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_critical_sphere" => {
            let r = f_critical_sphere(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sigma_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nuSigma_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("extrapolation") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1263(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_currie_detection_limits" => {
            let r = f_currie(match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("eff") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("y") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1326(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_daughter_max_activity_time" => {
            let r = f_daughter_max_time(match v.get("T_P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A_P0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1376(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_dead_time_nonparalyzable" => {
            let r = f_dead_time_nonparalyzable(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1422(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_dead_time_paralyzable" => {
            let r = f_dead_time_paralyzable(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("branch") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1498(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_decay_curve_fit" => {
            let r = f_decay_fit(match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_unit") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1563(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_decay_heat_energy" => {
            let r = f_decay_heat_energy(match v.get("P0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_op") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1614(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_decay_heat_power" => {
            let r = f_decay_heat(match v.get("P0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_op") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1663(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_depletion_constant_flux" => {
            let r = f_depletion_constant_flux(match v.get("m0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E_f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1713(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_detector_efficiency_point_source" => {
            let r = f_detector_efficiency(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1757(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_diffusion_length" => {
            let r = f_diffusion_length(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sigma_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1805(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_disk_source_flux" => {
            let r = f_disk_source(match v.get("S_A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1857(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_effective_dose" => {
            let r = f_effective_dose(match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1916(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_effective_half_life" => {
            let r = f_effective_half_life(match v.get("T_p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1963(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_elastic_scattering" => {
            let r = f_elastic_scattering(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2019(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_enrichment_swu" => {
            let r = f_swu(match v.get("x_p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x_w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2077(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_equilibrium_ratio" => {
            let r = f_equilibrium_ratio(match v.get("T_P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_D") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2124(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_escape_peaks" => {
            let r = f_escape_peaks(match v.get("E") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2170(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_exponential_pile" => {
            let r = f_exponential_pile(match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2238(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_flux_from_foil_activation" => {
            let r = f_flux_from_foil(match v.get("A_d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_half") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_irr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2295(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_flux_shape" => {
            let r = f_flux_shape(match v.get("geometry") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("R") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("H") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("x") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("r") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("z") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2397(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_fuel_cross_sections" => {
            let r = f_fuel_cross_sections(match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma_a5") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma_f5") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma_a8") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma_aO") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nu") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2457(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_fuel_rod_temperatures" => {
            let r = f_fuel_rod_temperatures(match v.get("q_lin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h_g") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k_c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_cool") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2501(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_generator_daughter_activity" => {
            let r = f_generator(match v.get("A_P0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2564(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_half_value_layer" => {
            let r = f_hvl_tvl(match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2615(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_inverse_multiplication" => {
            let r = f_inverse_multiplication(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2677(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_irradiation_time_for_saturation" => {
            let r = f_irradiation_time(match v.get("T_half") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2730(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_klein_nishina" => {
            let r = f_klein_nishina(match v.get("E") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2778(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_line_source_flux" => {
            let r = f_line_source(match v.get("S_L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2837(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_logarithmic_energy_decrement" => {
            let r = f_log_decrement(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2882(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_mass_for_activity" => {
            let r = f_mass_for_activity(match v.get("nuclide") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2929(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_max_clad_temperature" => {
            let r = f_max_clad_temperature(match v.get("q0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H_e") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_dot") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c_p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_in") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R_o") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2975(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_maxwellian_average_cross_section" => {
            let r = f_maxwellian_average(match v.get("sigma0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3019(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_moderator_mixture" => {
            let r = f_moderator_mixture(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma_s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3100(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_multilayer_transmission" => {
            let r = s_3126(match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3166(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_plane_source_slab_flux" => {
            let r = f_plane_source_slab(match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sigma_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3213(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_point_source_dose_rate" => {
            let r = f_point_dose_rate(match v.get("Gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("buildup") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A1") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("alpha1") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("alpha2") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3292(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_point_source_sphere_flux" => {
            let r = f_point_source_sphere(match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sigma_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3338(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_power_after_step_one_group" => {
            let r = f_power_after_step_one_group(match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Lambda") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3449(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_power_after_step_six_group" => {
            let r = f_power_after_step_multi(match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Lambda") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3560(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_power_from_reaction_rate" => {
            let r = f_reaction_power(match v.get("E_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3608(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_radiation_unit_conversion" => {
            let r = f_unit_conversion(match v.get("A") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("D") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("H") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3667(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_reactivity_from_period" => {
            let r = f_reactivity_from_period(match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Lambda") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("T_d") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3741(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_reactor_period_one_group" => {
            let r = f_period_one_group(match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Lambda") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3846(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_reflected_slab_critical" => {
            let r = f_reflected_slab(match v.get("D_c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sigma_ac") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nuSigma_fc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D_r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sigma_ar") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("extrapolation") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3892(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_samarium" => {
            let r = f_samarium(match v.get("Sigma_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma_Pm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma_Sm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3952(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_shield_thickness_for_dose_rate" => {
            let r = f_shield_thickness(match v.get("Gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H_target") { Some(x) => py::from_val(x), None => { return None } }, match v.get("buildup") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4032(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_six_factor_formula" => {
            let r = f_six_factor(match v.get("eta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P_FNL") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("P_TNL") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4097(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_solid_angle_off_axis" => {
            let r = f_solid_angle_off_axis(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4149(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_solid_angle_on_axis" => {
            let r = f_solid_angle_axis(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4193(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_specific_activity" => {
            let r = f_specific_activity(match v.get("nuclide") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4240(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_stable_period_six_group" => {
            let r = f_stable_period_multi(match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Lambda") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4346(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_stay_time_decaying_source" => {
            let r = f_stay_time(match v.get("H0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_half") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D_lim") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4392(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_thermal_neutron_speeds" => {
            let r = f_thermal_speeds(match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4436(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_three_member_chain" => {
            let r = f_three_member_chain(match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T3") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A10") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A20") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A30") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4501(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_transit_dose" => {
            let r = f_transit_dose(match v.get("Gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4545(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_two_component_decay_fit" => {
            let r = f_two_component_fit(match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_unit") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4621(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_two_group_bare_keff" => {
            let r = f_two_group_keff(match v.get("geometry") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sigma_a1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sigma_12") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sigma_a2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nuSigma_f1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nuSigma_f2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("b") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("c") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("R") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("H") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4689(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_xenon_after_shutdown" => {
            let r = f_xenon_after_shutdown(match v.get("Sigma_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma_I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma_X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma_X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4759(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuc_xenon_equilibrium" => {
            let r = f_xenon_equilibrium(match v.get("Sigma_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma_I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma_X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma_X") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4815(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c_BARN() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1e-28)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_CI() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(37000000000.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_DAY() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(86400.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_EXTRAP() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("none"), Py::Float(0.0)), (Py::str("2d"), Py::Float(2.0)), (Py::str("2.13d"), Py::Float(2.1312))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_J01() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(2.404825557695773)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_KB() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.380649e-23)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_LN2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.6931471805599453)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_MEV() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.602176634e-13)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_ME_MEV() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.51099895069)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_MN() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.67492750056e-27)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_NA() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(6.02214076e+23)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_QE() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.602176634e-19)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_RE() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(2.8179403205e-15)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__CHAINS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("Sr-90"), Py::dict(vec![(Py::str("daughter"), Py::str("Y-90")), (Py::str("order"), Py::list(vec![Py::str("Sr-90"), Py::str("Y-90")])), (Py::str("lam"), Py::dict(vec![(Py::str("Sr-90"), Py::Float(7.629377489724798e-10)), (Py::str("Y-90"), Py::Float(3.0037579327437396e-06))])), (Py::str("edges"), Py::list(vec![Py::tuple(vec![Py::str("Sr-90"), Py::str("Y-90"), Py::Float(1.0)])]))])), (Py::str("Cs-137"), Py::dict(vec![(Py::str("daughter"), Py::str("Ba-137m")), (Py::str("order"), Py::list(vec![Py::str("Cs-137"), Py::str("Ba-137m")])), (Py::str("lam"), Py::dict(vec![(Py::str("Cs-137"), Py::Float(7.28110351771224e-10)), (Py::str("Ba-137m"), Py::Float(0.004526823279518974))])), (Py::str("edges"), Py::list(vec![Py::tuple(vec![Py::str("Cs-137"), Py::str("Ba-137m"), Py::Float(0.94399)])]))])), (Py::str("Mo-99"), Py::dict(vec![(Py::str("daughter"), Py::str("Tc-99m")), (Py::str("order"), Py::list(vec![Py::str("Mo-99"), Py::str("Tc-99m"), Py::str("Tc-99")])), (Py::str("lam"), Py::dict(vec![(Py::str("Mo-99"), Py::Float(2.9199406049268075e-06)), (Py::str("Tc-99m"), Py::Float(3.20101219432874e-05)), (Py::str("Tc-99"), Py::Float(1.0405010797213499e-13))])), (Py::str("edges"), Py::list(vec![Py::tuple(vec![Py::str("Mo-99"), Py::str("Tc-99m"), Py::Float(0.8773)]), Py::tuple(vec![Py::str("Mo-99"), Py::str("Tc-99"), Py::Float(0.1227)]), Py::tuple(vec![Py::str("Tc-99m"), Py::str("Tc-99"), Py::Float(0.99996)])]))])), (Py::str("Ge-68"), Py::dict(vec![(Py::str("daughter"), Py::str("Ga-68")), (Py::str("order"), Py::list(vec![Py::str("Ge-68"), Py::str("Ga-68")])), (Py::str("lam"), Py::dict(vec![(Py::str("Ge-68"), Py::Float(2.960891977130985e-08)), (Py::str("Ga-68"), Py::Float(0.00017061664465119514))])), (Py::str("edges"), Py::list(vec![Py::tuple(vec![Py::str("Ge-68"), Py::str("Ga-68"), Py::Float(1.0)])]))])), (Py::str("W-188"), Py::dict(vec![(Py::str("daughter"), Py::str("Re-188")), (Py::str("order"), Py::list(vec![Py::str("W-188"), Py::str("Re-188")])), (Py::str("lam"), Py::dict(vec![(Py::str("W-188"), Py::Float(1.1496899988587567e-07)), (Py::str("Re-188"), Py::Float(1.1323270024045736e-05))])), (Py::str("edges"), Py::list(vec![Py::tuple(vec![Py::str("W-188"), Py::str("Re-188"), Py::Float(1.0)])]))])), (Py::str("Ru-106"), Py::dict(vec![(Py::str("daughter"), Py::str("Rh-106")), (Py::str("order"), Py::list(vec![Py::str("Ru-106"), Py::str("Rh-106")])), (Py::str("lam"), Py::dict(vec![(Py::str("Ru-106"), Py::Float(2.1474174394487017e-08)), (Py::str("Rh-106"), Py::Float(0.023259972502011585))])), (Py::str("edges"), Py::list(vec![Py::tuple(vec![Py::str("Ru-106"), Py::str("Rh-106"), Py::Float(1.0)])]))])), (Py::str("Ce-144"), Py::dict(vec![(Py::str("daughter"), Py::str("Pr-144")), (Py::str("order"), Py::list(vec![Py::str("Ce-144"), Py::str("Pr-144m"), Py::str("Pr-144"), Py::str("Nd-144")])), (Py::str("lam"), Py::dict(vec![(Py::str("Ce-144"), Py::Float(2.8158144017536774e-08)), (Py::str("Pr-144"), Py::Float(0.0006685447343363669)), (Py::str("Pr-144m"), Py::Float(0.0016045073624072809)), (Py::str("Nd-144"), Py::Float(9.591693359352705e-24))])), (Py::str("edges"), Py::list(vec![Py::tuple(vec![Py::str("Ce-144"), Py::str("Pr-144"), Py::Float(0.99023)]), Py::tuple(vec![Py::str("Ce-144"), Py::str("Pr-144m"), Py::Float(0.0097699)]), Py::tuple(vec![Py::str("Pr-144"), Py::str("Nd-144"), Py::Float(1.0)]), Py::tuple(vec![Py::str("Pr-144m"), Py::str("Pr-144"), Py::Float(0.9993)]), Py::tuple(vec![Py::str("Pr-144m"), Py::str("Nd-144"), Py::Float(0.0007)])]))])), (Py::str("Ba-140"), Py::dict(vec![(Py::str("daughter"), Py::str("La-140")), (Py::str("order"), Py::list(vec![Py::str("Ba-140"), Py::str("La-140")])), (Py::str("lam"), Py::dict(vec![(Py::str("Ba-140"), Py::Float(6.291198880204206e-07)), (Py::str("La-140"), Py::Float(4.780726304771113e-06))])), (Py::str("edges"), Py::list(vec![Py::tuple(vec![Py::str("Ba-140"), Py::str("La-140"), Py::Float(1.0)])]))])), (Py::str("Te-132"), Py::dict(vec![(Py::str("daughter"), Py::str("I-132")), (Py::str("order"), Py::list(vec![Py::str("Te-132"), Py::str("I-132")])), (Py::str("lam"), Py::dict(vec![(Py::str("Te-132"), Py::Float(2.503912862683022e-06)), (Py::str("I-132"), Py::Float(8.389580979907351e-05))])), (Py::str("edges"), Py::list(vec![Py::tuple(vec![Py::str("Te-132"), Py::str("I-132"), Py::Float(1.0)])]))])), (Py::str("Sn-113"), Py::dict(vec![(Py::str("daughter"), Py::str("In-113m")), (Py::str("order"), Py::list(vec![Py::str("Sn-113"), Py::str("In-113m")])), (Py::str("lam"), Py::dict(vec![(Py::str("Sn-113"), Py::Float(6.97066366498949e-08)), (Py::str("In-113m"), Py::Float(0.00011613540230947204))])), (Py::str("edges"), Py::list(vec![Py::tuple(vec![Py::str("Sn-113"), Py::str("In-113m"), Py::Float(0.99998)])]))])), (Py::str("Sr-82"), Py::dict(vec![(Py::str("daughter"), Py::str("Rb-82")), (Py::str("order"), Py::list(vec![Py::str("Sr-82"), Py::str("Rb-82")])), (Py::str("lam"), Py::dict(vec![(Py::str("Sr-82"), Py::Float(3.1634608880269733e-07)), (Py::str("Rb-82"), Py::Float(0.00907498272532005))])), (Py::str("edges"), Py::list(vec![Py::tuple(vec![Py::str("Sr-82"), Py::str("Rb-82"), Py::Float(1.0)])]))])), (Py::str("Zr-95"), Py::dict(vec![(Py::str("daughter"), Py::str("Nb-95")), (Py::str("order"), Py::list(vec![Py::str("Zr-95"), Py::str("Nb-95m"), Py::str("Nb-95")])), (Py::str("lam"), Py::dict(vec![(Py::str("Zr-95"), Py::Float(1.2528949294159802e-07)), (Py::str("Nb-95"), Py::Float(2.292742937337145e-07)), (Py::str("Nb-95m"), Py::Float(2.2223093662150703e-06))])), (Py::str("edges"), Py::list(vec![Py::tuple(vec![Py::str("Zr-95"), Py::str("Nb-95"), Py::Float(0.9892)]), Py::tuple(vec![Py::str("Zr-95"), Py::str("Nb-95m"), Py::Float(0.010802)]), Py::tuple(vec![Py::str("Nb-95m"), Py::str("Nb-95"), Py::Float(0.944)])]))])), (Py::str("Sb-125"), Py::dict(vec![(Py::str("daughter"), Py::str("Te-125m")), (Py::str("order"), Py::list(vec![Py::str("Sb-125"), Py::str("Te-125m")])), (Py::str("lam"), Py::dict(vec![(Py::str("Sb-125"), Py::Float(7.962479624484403e-09)), (Py::str("Te-125m"), Py::Float(1.3976544968704536e-07))])), (Py::str("edges"), Py::list(vec![Py::tuple(vec![Py::str("Sb-125"), Py::str("Te-125m"), Py::Float(0.23136)])]))])), (Py::str("Pb-212"), Py::dict(vec![(Py::str("daughter"), Py::str("Bi-212")), (Py::str("order"), Py::list(vec![Py::str("Pb-212"), Py::str("Bi-212"), Py::str("Po-212"), Py::str("Tl-208")])), (Py::str("lam"), Py::dict(vec![(Py::str("Pb-212"), Py::Float(1.8095947696322715e-05)), (Py::str("Bi-212"), Py::Float(0.0001907919572144083)), (Py::str("Po-212"), Py::Float(2318217.9951837636)), (Py::str("Tl-208"), Py::Float(0.0037839675759359388))])), (Py::str("edges"), Py::list(vec![Py::tuple(vec![Py::str("Pb-212"), Py::str("Bi-212"), Py::Float(1.0)]), Py::tuple(vec![Py::str("Bi-212"), Py::str("Po-212"), Py::Float(0.6406)]), Py::tuple(vec![Py::str("Bi-212"), Py::str("Tl-208"), Py::Float(0.3594)])]))])), (Py::str("Te-131m"), Py::dict(vec![(Py::str("daughter"), Py::str("I-131")), (Py::str("order"), Py::list(vec![Py::str("Te-131m"), Py::str("Te-131"), Py::str("I-131"), Py::str("Xe-131m")])), (Py::str("lam"), Py::dict(vec![(Py::str("Te-131m"), Py::Float(6.418029449629123e-06)), (Py::str("I-131"), Py::Float(1.000229008943908e-06)), (Py::str("Te-131"), Py::Float(0.00046209812037329687)), (Py::str("Xe-131m"), Py::Float(6.775791226382097e-07))])), (Py::str("edges"), Py::list(vec![Py::tuple(vec![Py::str("Te-131m"), Py::str("I-131"), Py::Float(0.778)]), Py::tuple(vec![Py::str("Te-131m"), Py::str("Te-131"), Py::Float(0.222)]), Py::tuple(vec![Py::str("I-131"), Py::str("Xe-131m"), Py::Float(0.011759)]), Py::tuple(vec![Py::str("Te-131"), Py::str("I-131"), Py::Float(1.0)])]))])), (Py::str("I-133"), Py::dict(vec![(Py::str("daughter"), Py::str("Xe-133")), (Py::str("order"), Py::list(vec![Py::str("I-133"), Py::str("Xe-133m"), Py::str("Xe-133")])), (Py::str("lam"), Py::dict(vec![(Py::str("I-133"), Py::Float(9.256773244657388e-06)), (Py::str("Xe-133"), Py::Float(1.5301424398314711e-06)), (Py::str("Xe-133m"), Py::Float(3.6632588182814627e-06))])), (Py::str("edges"), Py::list(vec![Py::tuple(vec![Py::str("I-133"), Py::str("Xe-133"), Py::Float(0.97115)]), Py::tuple(vec![Py::str("I-133"), Py::str("Xe-133m"), Py::Float(0.028846)]), Py::tuple(vec![Py::str("Xe-133m"), Py::str("Xe-133"), Py::Float(1.0)])]))])), (Py::str("Pd-103"), Py::dict(vec![(Py::str("daughter"), Py::str("Rh-103m")), (Py::str("order"), Py::list(vec![Py::str("Pd-103"), Py::str("Rh-103m")])), (Py::str("lam"), Py::dict(vec![(Py::str("Pd-103"), Py::Float(4.721638992429171e-07)), (Py::str("Rh-103m"), Py::Float(0.00020587470166682863))])), (Py::str("edges"), Py::list(vec![Py::tuple(vec![Py::str("Pd-103"), Py::str("Rh-103m"), Py::Float(0.99875)])]))])), (Py::str("Ce-134"), Py::dict(vec![(Py::str("daughter"), Py::str("La-134")), (Py::str("order"), Py::list(vec![Py::str("Ce-134"), Py::str("La-134")])), (Py::str("lam"), Py::dict(vec![(Py::str("Ce-134"), Py::Float(2.538777472163419e-06)), (Py::str("La-134"), Py::Float(0.0017910779859430111))])), (Py::str("edges"), Py::list(vec![Py::tuple(vec![Py::str("Ce-134"), Py::str("La-134"), Py::Float(1.0)])]))])), (Py::str("Ba-131"), Py::dict(vec![(Py::str("daughter"), Py::str("Cs-131")), (Py::str("order"), Py::list(vec![Py::str("Ba-131"), Py::str("Cs-131")])), (Py::str("lam"), Py::dict(vec![(Py::str("Ba-131"), Py::Float(6.976118966988177e-07)), (Py::str("Cs-131"), Py::Float(8.280046250424609e-07))])), (Py::str("edges"), Py::list(vec![Py::tuple(vec![Py::str("Ba-131"), Py::str("Cs-131"), Py::Float(1.0)])]))])), (Py::str("Hg-195m"), Py::dict(vec![(Py::str("daughter"), Py::str("Hg-195")), (Py::str("order"), Py::list(vec![Py::str("Hg-195m"), Py::str("Hg-195"), Py::str("Au-195")])), (Py::str("lam"), Py::dict(vec![(Py::str("Hg-195m"), Py::Float(4.628386622328694e-06)), (Py::str("Hg-195"), Py::Float(1.8284984186977558e-05)), (Py::str("Au-195"), Py::Float(4.310920489224174e-08))])), (Py::str("edges"), Py::list(vec![Py::tuple(vec![Py::str("Hg-195m"), Py::str("Hg-195"), Py::Float(0.542)]), Py::tuple(vec![Py::str("Hg-195m"), Py::str("Au-195"), Py::Float(0.458)]), Py::tuple(vec![Py::str("Hg-195"), Py::str("Au-195"), Py::Float(1.0)])]))])), (Py::str("Sr-91"), Py::dict(vec![(Py::str("daughter"), Py::str("Y-91m")), (Py::str("order"), Py::list(vec![Py::str("Sr-91"), Py::str("Y-91m"), Py::str("Y-91")])), (Py::str("lam"), Py::dict(vec![(Py::str("Sr-91"), Py::Float(1.999386121379789e-05)), (Py::str("Y-91m"), Py::Float(0.00023239696256955185)), (Py::str("Y-91"), Py::Float(1.3711394312145624e-07))])), (Py::str("edges"), Py::list(vec![Py::tuple(vec![Py::str("Sr-91"), Py::str("Y-91m"), Py::Float(0.58247)]), Py::tuple(vec![Py::str("Sr-91"), Py::str("Y-91"), Py::Float(0.41753)]), Py::tuple(vec![Py::str("Y-91m"), Py::str("Y-91"), Py::Float(1.0)])]))]))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__DIMS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("slab"), Py::tuple(vec![Py::str("a")])), (Py::str("sphere"), Py::tuple(vec![Py::str("R")])), (Py::str("cylinder"), Py::tuple(vec![Py::str("R"), Py::str("H")])), (Py::str("cuboid"), Py::tuple(vec![Py::str("a"), Py::str("b"), Py::str("c")]))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__E0_2200() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(4.0533245513552e-21)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__GEOMS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::tuple(vec![Py::str("slab"), Py::str("sphere"), Py::str("cylinder"), Py::str("cuboid")])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__K95() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.6448536269514722)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__LAM_I() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(2.9306070546251702e-05)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__LAM_PM() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(3.627371580423393e-06)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__LAM_X() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(2.1065742176025568e-05)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__M5() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(235.043928117)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__M8() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(238.050786936)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__MO() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(15.999)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__ND() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("H-3"), Py::tuple(vec![Py::Float(388781329.30560005), Py::Float(3.01604928132)])), (Py::str("C-14"), Py::tuple(vec![Py::Float(179874478656.0), Py::Float(14.00324198862)])), (Py::str("Na-22"), Py::tuple(vec![Py::Float(82107965.967552), Py::Float(21.994437547)])), (Py::str("P-32"), Py::tuple(vec![Py::Float(1232323.2), Py::Float(31.973907643)])), (Py::str("K-40"), Py::tuple(vec![Py::Float(3.947771452608e+16), Py::Float(39.963998165)])), (Py::str("Co-57"), Py::tuple(vec![Py::Float(23478336.0), Py::Float(56.936289819)])), (Py::str("Co-60"), Py::tuple(vec![Py::Float(166346024.445504), Py::Float(59.933815536)])), (Py::str("Kr-85"), Py::tuple(vec![Py::Float(339426296.91648), Py::Float(84.91252726)])), (Py::str("Sr-89"), Py::tuple(vec![Py::Float(4365792.0), Py::Float(88.907450808)])), (Py::str("Sr-90"), Py::tuple(vec![Py::Float(908523901.8432001), Py::Float(89.90772787)])), (Py::str("Y-90"), Py::tuple(vec![Py::Float(230759.99999999997), Py::Float(89.907141749)])), (Py::str("Tc-99m"), Py::tuple(vec![Py::Float(21654.0), Py::Float(98.9064028581)])), (Py::str("Mo-99"), Py::tuple(vec![Py::Float(237384.0), Py::Float(98.907707299)])), (Py::str("I-125"), Py::tuple(vec![Py::Float(5132160.0), Py::Float(124.90463061)])), (Py::str("I-131"), Py::tuple(vec![Py::Float(692988.48), Py::Float(130.906126375)])), (Py::str("I-135"), Py::tuple(vec![Py::Float(23652.0), Py::Float(134.910059355)])), (Py::str("Xe-133"), Py::tuple(vec![Py::Float(452995.2), Py::Float(132.905910748)])), (Py::str("Xe-135"), Py::tuple(vec![Py::Float(32904.0), Py::Float(134.907231441)])), (Py::str("Cs-134"), Py::tuple(vec![Py::Float(65158740.969984), Py::Float(133.906718501)])), (Py::str("Cs-137"), Py::tuple(vec![Py::Float(951980944.7479681), Py::Float(136.907089296)])), (Py::str("Ir-192"), Py::tuple(vec![Py::Float(6378652.8), Py::Float(191.962602414)])), (Py::str("Tl-201"), Py::tuple(vec![Py::Float(262483.2), Py::Float(200.970820235)])), (Py::str("Po-210"), Py::tuple(vec![Py::Float(11955686.4), Py::Float(209.982873686)])), (Py::str("Rn-222"), Py::tuple(vec![Py::Float(330350.4), Py::Float(222.017576017)])), (Py::str("Ra-223"), Py::tuple(vec![Py::Float(987552.0), Py::Float(223.018500648)])), (Py::str("Ra-226"), Py::tuple(vec![Py::Float(50491081728.0), Py::Float(226.025408186)])), (Py::str("Th-230"), Py::tuple(vec![Py::Float(2378761087910.4), Py::Float(230.033132267)])), (Py::str("Th-232"), Py::tuple(vec![Py::Float(4.43374811424e+17), Py::Float(232.038053606)])), (Py::str("U-234"), Py::tuple(vec![Py::Float(7747225352640.0), Py::Float(234.040950296)])), (Py::str("U-235"), Py::tuple(vec![Py::Float(2.221607596032e+16), Py::Float(235.043928117)])), (Py::str("U-238"), Py::tuple(vec![Py::Float(1.4099634572544002e+17), Py::Float(238.050786936)])), (Py::str("Np-237"), Py::tuple(vec![Py::Float(67658049515520.01), Py::Float(237.04817164)])), (Py::str("Pu-238"), Py::tuple(vec![Py::Float(2767542417.216), Py::Float(238.049558175)])), (Py::str("Pu-239"), Py::tuple(vec![Py::Float(760837487788.8), Py::Float(239.052161596)])), (Py::str("Pu-240"), Py::tuple(vec![Py::Float(207139662789.12003), Py::Float(240.05381174)])), (Py::str("Pu-241"), Py::tuple(vec![Py::Float(452841889.248), Py::Float(241.056849651)])), (Py::str("Am-241"), Py::tuple(vec![Py::Float(13638903451.776001), Py::Float(241.056827343)])), (Py::str("Cm-244"), Py::tuple(vec![Py::Float(571180362.0480001), Py::Float(244.062750622)])), (Py::str("Cf-252"), Py::tuple(vec![Py::Float(83468069.4816), Py::Float(252.081626507)])), (Py::str("F-18"), Py::tuple(vec![Py::Float(6586.2), Py::Float(18.000937324)])), (Py::str("Ga-67"), Py::tuple(vec![Py::Float(281767.68), Py::Float(66.928202276)])), (Py::str("In-111"), Py::tuple(vec![Py::Float(242326.08), Py::Float(110.905107236)])), (Py::str("Fe-55"), Py::tuple(vec![Py::Float(86371306.68096001), Py::Float(54.938291158)])), (Py::str("Ni-63"), Py::tuple(vec![Py::Float(3158848300.608), Py::Float(62.929669021)])), (Py::str("Se-75"), Py::tuple(vec![Py::Float(10348905.6), Py::Float(74.92252287)])), (Py::str("Mn-54"), Py::tuple(vec![Py::Float(26967168.0), Py::Float(53.940355772)])), (Py::str("Zn-65"), Py::tuple(vec![Py::Float(21086784.0), Py::Float(64.929240534)])), (Py::str("Ba-133"), Py::tuple(vec![Py::Float(331978862.3616), Py::Float(132.906007443)])), (Py::str("Eu-152"), Py::tuple(vec![Py::Float(427186108.34496003), Py::Float(151.92175098)])), (Py::str("Lu-177"), Py::tuple(vec![Py::Float(574300.8), Py::Float(176.94376357)])), (Py::str("Sm-153"), Py::tuple(vec![Py::Float(167400.0), Py::Float(152.922103576)])), (Py::str("Ho-166"), Py::tuple(vec![Py::Float(96480.0), Py::Float(165.932291209)])), (Py::str("Re-188"), Py::tuple(vec![Py::Float(61214.4), Py::Float(187.958113658)])), (Py::str("Pm-149"), Py::tuple(vec![Py::Float(191088.0), Py::Float(148.918341507)]))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__TU() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("s"), Py::Float(1.0)), (Py::str("min"), Py::Float(60.0)), (Py::str("h"), Py::Float(3600.0)), (Py::str("d"), Py::Float(86400.0)), (Py::str("y"), Py::Float(31557600.0))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__WW() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.0622)) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ad_build(mut v_lp: Py, mut v_ld: Py, mut v_t: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_tt: Py = Py::None;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(40i64))?;
        let t2547 = (Py::tuple(vec![f__mpf(v_lp.clone())?, f__mpf(v_ld.clone())?, f__mpf(v_t.clone())?])).items()?;
        if t2547.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_a = t2547[0].clone();
        v_b = t2547[1].clone();
        v_tt = t2547[2].clone();
        if ({ let t2548 = v_lp.clone(); Py::Bool({ let t2549 = v_ld.clone(); if py::eq(&t2548, &t2549) { true } else { false } }) }).truthy() {
            return Ok(py::float(&py::mul(&py::mul(&v_a.clone(), &v_tt.clone())?, &ax_engine::pymp::call("exp", &[py::mul(&py::neg(&v_a.clone())?, &v_tt.clone())?], &[])?)?)?);
        }
        return Ok(py::float(&py::mul(&py::truediv(&v_a.clone(), &py::sub(&v_b.clone(), &v_a.clone())?)?, &py::sub(&ax_engine::pymp::call("exp", &[py::mul(&py::neg(&v_a.clone())?, &v_tt.clone())?], &[])?, &ax_engine::pymp::call("exp", &[py::mul(&py::neg(&v_b.clone())?, &v_tt.clone())?], &[])?)?)?)?);
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__alpha(mut v_A: Py) -> R {
    return Ok(py::pow(&py::truediv(&py::sub(&v_A.clone(), &Py::Float(1.0))?, &py::add(&v_A.clone(), &Py::Float(1.0))?)?, &Py::Int(2i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bateman_dag(mut v_order: Py, mut v_lam: Py, mut v_edges: Py, mut v_n0: Py, mut v_t: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cj: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_row: Py = Py::None;
    let mut v_tt: Py = Py::None;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(40i64))?;
        v_L = { let mut out = vec![]; for t4477 in py::iter(&(v_order.clone()))? { let mut v_m: Py; 
v_m = t4477;
out.push((v_m.clone(), f__mpf(py::getitem(&v_lam.clone(), &v_m.clone())?)?)); } Py::dict(out) };
        v_c = Py::dict(vec![]);
        for t4478 in py::iter(&(v_order.clone()))? {
            v_m = t4478;
            v_row = Py::dict(vec![]);
            for t4479 in py::iter(&(v_edges.clone()))? {
                let t4484 = (t4479).items()?;
                if t4484.len() != 3 { return Err(PyErr::Value("unpack".into())); }
                v_a = t4484[0].clone();
                v_b = t4484[1].clone();
                v_f = t4484[2].clone();
                if ({ let t4480 = v_b.clone(); Py::Bool({ let t4481 = v_m.clone(); if py::ne(&t4480, &t4481) { true } else { false } }) }).truthy() {
                    continue;
                }
                for t4482 in py::iter(&(py::method(&py::getitem(&v_c.clone(), &v_a.clone())?, "items", &[])?))? {
                    let t4483 = (t4482).items()?;
                    if t4483.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_j = t4483[0].clone();
                    v_cj = t4483[1].clone();
                    py::setitem(&v_row.clone(), &v_j.clone(), py::add(&py::method(&v_row.clone(), "get", &[v_j.clone(), Py::Int(0i64)])?, &py::truediv(&py::mul(&py::mul(&f__mpf(v_f.clone())?, &py::getitem(&v_L.clone(), &v_a.clone())?)?, &v_cj.clone())?, &py::sub(&py::getitem(&v_L.clone(), &v_m.clone())?, &py::getitem(&v_L.clone(), &v_j.clone())?)?)?)?)?;
                }
            }
            py::setitem(&v_row.clone(), &v_m.clone(), py::sub(&f__mpf(py::method(&v_n0.clone(), "get", &[v_m.clone(), Py::Float(0.0)])?)?, &py::sum(&py::method(&v_row.clone(), "values", &[])?, Some(f__mpf(Py::Int(0i64))?))?)?)?;
            py::setitem(&v_c.clone(), &v_m.clone(), v_row.clone())?;
        }
        v_tt = f__mpf(v_t.clone())?;
        return Ok({ let mut out = vec![]; for t4485 in py::iter(&(v_order.clone()))? { let mut v_m: Py; 
v_m = t4485;
out.push((v_m.clone(), py::float(&py::sum(&{ let mut out = vec![]; for t4486 in py::iter(&(py::method(&py::getitem(&v_c.clone(), &v_m.clone())?, "items", &[])?))? { let mut v_cj: Py; let mut v_j: Py; 
let t4487 = (t4486).items()?;
if t4487.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_j = t4487[0].clone();
v_cj = t4487[1].clone();
out.push(py::mul(&v_cj.clone(), &ax_engine::pymp::call("exp", &[py::mul(&py::neg(&py::getitem(&v_L.clone(), &v_j.clone())?)?, &v_tt.clone())?], &[])?)?); } Py::list(out) }, Some(f__mpf(Py::Int(0i64))?))?)?)); } Py::dict(out) });
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bg2(mut v_geometry: Py, mut v_dims: Py) -> R {
    if ({ let t4670 = v_geometry.clone(); Py::Bool({ let t4671 = Py::str("slab"); if py::eq(&t4670, &t4671) { true } else { false } }) }).truthy() {
        return Ok(py::pow(&py::truediv(&Py::Float(std::f64::consts::PI), &py::getitem(&v_dims.clone(), &Py::Int(0i64))?)?, &Py::Int(2i64))?);
    }
    if ({ let t4672 = v_geometry.clone(); Py::Bool({ let t4673 = Py::str("sphere"); if py::eq(&t4672, &t4673) { true } else { false } }) }).truthy() {
        return Ok(py::pow(&py::truediv(&Py::Float(std::f64::consts::PI), &py::getitem(&v_dims.clone(), &Py::Int(0i64))?)?, &Py::Int(2i64))?);
    }
    if ({ let t4674 = v_geometry.clone(); Py::Bool({ let t4675 = Py::str("cylinder"); if py::eq(&t4674, &t4675) { true } else { false } }) }).truthy() {
        return Ok(py::add(&py::pow(&py::truediv(&c_J01()?, &py::getitem(&v_dims.clone(), &Py::Int(0i64))?)?, &Py::Int(2i64))?, &py::pow(&py::truediv(&Py::Float(std::f64::consts::PI), &py::getitem(&v_dims.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?)?);
    }
    return Ok(py::sum(&{ let mut out = vec![]; for t4676 in py::iter(&(v_dims.clone()))? { let mut v_x: Py; 
v_x = t4676;
out.push(py::pow(&py::truediv(&Py::Float(std::f64::consts::PI), &v_x.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bisect(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_n: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_fa: Py = Py::None;
    let mut v_fb: Py = Py::None;
    let mut v_fm: Py = Py::None;
    let mut v_m: Py = Py::None;
    let t3990 = (Py::tuple(vec![py::call(&v_f.clone(), &[v_a.clone()])?, py::call(&v_f.clone(), &[v_b.clone()])?])).items()?;
    if t3990.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fa = t3990[0].clone();
    v_fb = t3990[1].clone();
    if ({ let t3991 = v_fa.clone(); Py::Bool({ let t3992 = Py::Int(0i64); if py::eq(&t3991, &t3992) { true } else { false } }) }).truthy() {
        return Ok(v_a.clone());
    }
    if ({ let t3993 = v_fb.clone(); Py::Bool({ let t3994 = Py::Int(0i64); if py::eq(&t3993, &t3994) { true } else { false } }) }).truthy() {
        return Ok(v_b.clone());
    }
    let _ = f__need({ let t3995 = { let t3997 = v_fa.clone(); Py::Bool({ let t3998 = Py::Int(0i64); if py::gt(&t3997, &t3998)? { true } else { false } }) }; Py::Bool({ let t3996 = { let t3999 = v_fb.clone(); Py::Bool({ let t4000 = Py::Int(0i64); if py::gt(&t3999, &t4000)? { true } else { false } }) }; if py::ne(&t3995, &t3996) { true } else { false } }) }, Py::str("no root in the interval"))?;
    for t4001 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t4001;
        v_m = py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?;
        if ({ let t4006 = { let t4002 = v_m.clone(); Py::Bool({ let t4003 = v_a.clone(); if py::eq(&t4002, &t4003) { true } else { false } }) }; if t4006.truthy() { t4006 } else { { let t4004 = v_m.clone(); Py::Bool({ let t4005 = v_b.clone(); if py::eq(&t4004, &t4005) { true } else { false } }) } } }).truthy() {
            break;
        }
        v_fm = py::call(&v_f.clone(), &[v_m.clone()])?;
        if ({ let t4007 = v_fm.clone(); Py::Bool({ let t4008 = Py::Int(0i64); if py::eq(&t4007, &t4008) { true } else { false } }) }).truthy() {
            return Ok(v_m.clone());
        }
        if ({ let t4009 = { let t4011 = v_fm.clone(); Py::Bool({ let t4012 = Py::Int(0i64); if py::gt(&t4011, &t4012)? { true } else { false } }) }; Py::Bool({ let t4010 = { let t4013 = v_fa.clone(); Py::Bool({ let t4014 = Py::Int(0i64); if py::gt(&t4013, &t4014)? { true } else { false } }) }; if py::eq(&t4009, &t4010) { true } else { false } }) }).truthy() {
            let t4015 = (Py::tuple(vec![v_m.clone(), v_fm.clone()])).items()?;
            if t4015.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t4015[0].clone();
            v_fa = t4015[1].clone();
        } else {
            v_b = v_m.clone();
        }
    }
    return Ok(py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__buildup(mut v_form: Py, mut v_b: Py, mut v_A1: Py, mut v_alpha1: Py, mut v_alpha2: Py) -> R {
    let mut v_B: Py = Py::None;
    if ({ let t3251 = v_form.clone(); Py::Bool({ let t3252 = Py::str("taylor"); if py::eq(&t3251, &t3252) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t3260 = { let t3253 = v_A1.clone(); Py::Bool({ let t3254 = Py::None; if !py::is(&t3253, &t3254) { true } else { false } }) }; if !t3260.truthy() { t3260 } else { { let t3259 = { let t3255 = v_alpha1.clone(); Py::Bool({ let t3256 = Py::None; if !py::is(&t3255, &t3256) { true } else { false } }) }; if !t3259.truthy() { t3259 } else { { let t3257 = v_alpha2.clone(); Py::Bool({ let t3258 = Py::None; if !py::is(&t3257, &t3258) { true } else { false } }) } } } } }, Py::str("the Taylor form needs A1, alpha1 and alpha2"))?;
        let _ = f__need({ let t3265 = { let t3261 = py::add(&Py::Int(1i64), &v_alpha1.clone())?; Py::Bool({ let t3262 = Py::Int(0i64); if py::gt(&t3261, &t3262)? { true } else { false } }) }; if !t3265.truthy() { t3265 } else { { let t3263 = py::add(&Py::Int(1i64), &v_alpha2.clone())?; Py::Bool({ let t3264 = Py::Int(0i64); if py::gt(&t3263, &t3264)? { true } else { false } }) } } }, Py::str("the Taylor exponents need 1 + alpha > 0"))?;
        v_B = py::add(&py::mul(&v_A1.clone(), &py::math1("exp", &py::mul(&py::neg(&v_alpha1.clone())?, &v_b.clone())?)?)?, &py::mul(&py::sub(&Py::Float(1.0), &v_A1.clone())?, &py::math1("exp", &py::mul(&py::neg(&v_alpha2.clone())?, &v_b.clone())?)?)?)?;
        let _ = f__need({ let t3266 = v_B.clone(); Py::Bool({ let t3267 = Py::Int(0i64); if py::gt(&t3266, &t3267)? { true } else { false } }) }, Py::str("these Taylor parameters give a buildup factor <= 0 here"))?;
        return Ok(v_B.clone());
    }
    let _ = f__need({ let t3275 = { let t3268 = v_A1.clone(); Py::Bool({ let t3269 = Py::None; if py::is(&t3268, &t3269) { true } else { false } }) }; if !t3275.truthy() { t3275 } else { { let t3274 = { let t3270 = v_alpha1.clone(); Py::Bool({ let t3271 = Py::None; if py::is(&t3270, &t3271) { true } else { false } }) }; if !t3274.truthy() { t3274 } else { { let t3272 = v_alpha2.clone(); Py::Bool({ let t3273 = Py::None; if py::is(&t3272, &t3273) { true } else { false } }) } } } } }, Py::str("Taylor parameters are given but the buildup form is not taylor"))?;
    let _ = f__need({ let t3276 = v_form.clone(); Py::Bool({ let t3277 = Py::tuple(vec![Py::str("none"), Py::str("linear")]); if py::contains(&t3277, &t3276)? { true } else { false } }) }, Py::str("buildup must be none, linear or taylor"))?;
    return Ok((if ({ let t3278 = v_form.clone(); Py::Bool({ let t3279 = Py::str("none"); if py::eq(&t3278, &t3279) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { py::add(&Py::Float(1.0), &v_b.clone())? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ch_check(mut v_q0: Py, mut v_H: Py, mut v_H_e: Py, mut v_m_dot: Py, mut v_c_p: Py, mut v_T_in: Py) -> R {
    let _ = f__pos(Py::str("q0, H, H_e, the flow, c_p and T_in"), Py::tuple(vec![v_q0.clone(), v_H.clone(), v_H_e.clone(), v_m_dot.clone(), v_c_p.clone(), v_T_in.clone()]))?;
    let _ = f__need({ let t2961 = v_H_e.clone(); Py::Bool({ let t2962 = v_H.clone(); if py::ge(&t2961, &t2962)? { true } else { false } }) }, Py::str("the extrapolated height must be at least the heated height"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__dims(mut v_geometry: Py, mut v_given: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_need: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__need({ let t4659 = v_geometry.clone(); Py::Bool({ let t4660 = c__GEOMS()?; if py::contains(&t4660, &t4659)? { true } else { false } }) }, Py::str("geometry must be slab, sphere, cylinder or cuboid"))?;
    v_need = py::getitem(&c__DIMS()?, &v_geometry.clone())?;
    for t4661 in py::iter(&(py::method(&v_given.clone(), "items", &[])?))? {
        let t4668 = (t4661).items()?;
        if t4668.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4668[0].clone();
        v_v = t4668[1].clone();
        if ({ let t4662 = v_k.clone(); Py::Bool({ let t4663 = v_need.clone(); if py::contains(&t4663, &t4662)? { true } else { false } }) }).truthy() {
            let _ = f__need({ let t4664 = v_v.clone(); Py::Bool({ let t4665 = Py::None; if !py::is(&t4664, &t4665) { true } else { false } }) }, Py::str(&["the ".to_string(), py::format(&v_geometry.clone(), "")?, " needs ".to_string(), py::format(&py::method(&Py::str(", "), "join", &[v_need.clone()])?, "")?].concat()))?;
            let _ = f__pos(Py::str(&["the dimension ".to_string(), py::format(&v_k.clone(), "")?].concat()), Py::tuple(vec![v_v.clone()]))?;
        } else {
            let _ = f__need({ let t4666 = v_v.clone(); Py::Bool({ let t4667 = Py::None; if py::is(&t4666, &t4667) { true } else { false } }) }, Py::str(&[py::format(&v_k.clone(), "")?, " is not a dimension of a ".to_string(), py::format(&v_geometry.clone(), "")?, " (it needs ".to_string(), py::format(&py::method(&Py::str(", "), "join", &[v_need.clone()])?, "")?, ")".to_string()].concat()))?;
        }
    }
    return Ok({ let mut out = vec![]; for t4669 in py::iter(&(v_need.clone()))? { let mut v_k: Py; 
v_k = t4669;
out.push(py::getitem(&v_given.clone(), &v_k.clone())?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__frac(mut v_name: Py, mut v_v: Py, mut v_lo_open: Py, mut v_hi_open: Py) -> R {
    let _ = f__need(f__isnum(v_v.clone())?, Py::str(&[py::format(&v_name.clone(), "")?, " must be a number".to_string()].concat()))?;
    let _ = f__need({ let t4799 = (if (v_lo_open.clone()).truthy() { { let t4791 = v_v.clone(); Py::Bool({ let t4792 = Py::Int(0i64); if py::gt(&t4791, &t4792)? { true } else { false } }) } } else { { let t4793 = v_v.clone(); Py::Bool({ let t4794 = Py::Int(0i64); if py::ge(&t4793, &t4794)? { true } else { false } }) } }); if !t4799.truthy() { t4799 } else { (if (v_hi_open.clone()).truthy() { { let t4795 = v_v.clone(); Py::Bool({ let t4796 = Py::Int(1i64); if py::lt(&t4795, &t4796)? { true } else { false } }) } } else { { let t4797 = v_v.clone(); Py::Bool({ let t4798 = Py::Int(1i64); if py::le(&t4797, &t4798)? { true } else { false } }) } }) } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a fraction between 0 and 1".to_string()].concat()))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__group1(mut v_D: Py, mut v_Sigma_a: Py, mut v_nuSigma_f: Py) -> R {
    let _ = f__pos(Py::str("D, Sigma_a and nu Sigma_f"), Py::tuple(vec![v_D.clone(), v_Sigma_a.clone(), v_nuSigma_f.clone()]))?;
    return Ok(py::truediv(&py::sub(&v_nuSigma_f.clone(), &v_Sigma_a.clone())?, &v_D.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__inhour_roots(mut v_rho: Py, mut v_beta: Py, mut v_lam: Py, mut v_Lam: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_Lm: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_S2: Py = Py::None;
    let mut v_amps: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_bis: Py = Py::None;
    let mut v_bl: Py = Py::None;
    let mut v_df: Py = Py::None;
    let mut v_eps: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_roots: Py = Py::None;
    let mut v_w: Py = Py::None;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(40i64))?;
        v_b = { let mut out = vec![]; for t4295 in py::iter(&(v_beta.clone()))? { let mut v_x: Py; 
v_x = t4295;
out.push(f__mpf(v_x.clone())?); } Py::list(out) };
        v_L = py::sorted(&{ let mut out = vec![]; for t4296 in py::iter(&(v_lam.clone()))? { let mut v_x: Py; 
v_x = t4296;
out.push(f__mpf(v_x.clone())?); } Py::list(out) }, (Py::Bool(false)).truthy())?;
        v_bl = py::sorted(&py::zip(&[{ let mut out = vec![]; for t4297 in py::iter(&(v_lam.clone()))? { let mut v_x: Py; 
v_x = t4297;
out.push(f__mpf(v_x.clone())?); } Py::list(out) }, v_b.clone()])?, (Py::Bool(false)).truthy())?;
        let t4298 = (Py::tuple(vec![f__mpf(v_Lam.clone())?, f__mpf(v_rho.clone())?])).items()?;
        if t4298.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_Lm = t4298[0].clone();
        v_r = t4298[1].clone();
        v_f = { let v_L = v_L.clone(); let v_Lam = v_Lam.clone(); let v_Lm = v_Lm.clone(); let v_b = v_b.clone(); let v_beta = v_beta.clone(); let v_bl = v_bl.clone(); let v_lam = v_lam.clone(); let v_r = v_r.clone(); let v_rho = v_rho.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_w: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::add(&py::mul(&v_w.clone(), &v_Lm.clone())?, &py::sum(&{ let mut out = vec![]; for t4299 in py::iter(&(v_bl.clone()))? { let mut v_bi: Py; let mut v_li: Py; 
let t4300 = (t4299).items()?;
if t4300.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_li = t4300[0].clone();
v_bi = t4300[1].clone();
out.push(py::truediv(&py::mul(&v_bi.clone(), &v_w.clone())?, &py::add(&v_w.clone(), &v_li.clone())?)?); } Py::list(out) }, None)?)?, &v_r.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
        v_df = { let v_L = v_L.clone(); let v_Lam = v_Lam.clone(); let v_Lm = v_Lm.clone(); let v_b = v_b.clone(); let v_beta = v_beta.clone(); let v_bl = v_bl.clone(); let v_f = v_f.clone(); let v_lam = v_lam.clone(); let v_r = v_r.clone(); let v_rho = v_rho.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_w: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&v_Lm.clone(), &py::sum(&{ let mut out = vec![]; for t4301 in py::iter(&(v_bl.clone()))? { let mut v_bi: Py; let mut v_li: Py; 
let t4302 = (t4301).items()?;
if t4302.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_li = t4302[0].clone();
v_bi = t4302[1].clone();
out.push(py::truediv(&py::mul(&v_bi.clone(), &v_li.clone())?, &py::pow(&py::add(&v_w.clone(), &v_li.clone())?, &Py::Int(2i64))?)?); } Py::list(out) }, None)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
        v_bis = { let v_L = v_L.clone(); let v_Lam = v_Lam.clone(); let v_Lm = v_Lm.clone(); let v_b = v_b.clone(); let v_beta = v_beta.clone(); let v_bis = v_bis.clone(); let v_bl = v_bl.clone(); let v_df = v_df.clone(); let v_f = v_f.clone(); let v_lam = v_lam.clone(); let v_r = v_r.clone(); let v_rho = v_rho.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_lo: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_hi: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v__: Py = Py::None;
            let mut v_flo: Py = Py::None;
            let mut v_fm: Py = Py::None;
            let mut v_m: Py = Py::None;
            let mut v_step: Py = Py::None;
            let mut v_w: Py = Py::None;
            let mut v_w2: Py = Py::None;
            v_flo = py::call(&v_f.clone(), &[v_lo.clone()])?;
            for t4303 in py::range_iter(&[(Py::Int(60i64)).i()?]) {
                v__ = t4303;
                v_m = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
                v_fm = py::call(&v_f.clone(), &[v_m.clone()])?;
                if ({ let t4304 = { let t4306 = v_fm.clone(); Py::Bool({ let t4307 = Py::Int(0i64); if py::gt(&t4306, &t4307)? { true } else { false } }) }; Py::Bool({ let t4305 = { let t4308 = v_flo.clone(); Py::Bool({ let t4309 = Py::Int(0i64); if py::gt(&t4308, &t4309)? { true } else { false } }) }; if py::eq(&t4304, &t4305) { true } else { false } }) }).truthy() {
                    let t4310 = (Py::tuple(vec![v_m.clone(), v_fm.clone()])).items()?;
                    if t4310.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_lo = t4310[0].clone();
                    v_flo = t4310[1].clone();
                } else {
                    v_hi = v_m.clone();
                }
            }
            v_w = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
            for t4311 in py::range_iter(&[(Py::Int(60i64)).i()?]) {
                v__ = t4311;
                v_step = py::truediv(&py::call(&v_f.clone(), &[v_w.clone()])?, &py::call(&v_df.clone(), &[v_w.clone()])?)?;
                v_w2 = py::sub(&v_w.clone(), &v_step.clone())?;
                if (py::not(&{ let t4312 = v_lo.clone(); Py::Bool({ let t4313 = v_w2.clone(); if py::le(&t4312, &t4313)? { { let t4314 = v_hi.clone(); if py::le(&t4313, &t4314)? { true } else { false } } } else { false } }) })).truthy() {
                    v_m = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
                    if ({ let t4315 = { let t4317 = py::call(&v_f.clone(), &[v_m.clone()])?; Py::Bool({ let t4318 = Py::Int(0i64); if py::gt(&t4317, &t4318)? { true } else { false } }) }; Py::Bool({ let t4316 = { let t4319 = v_flo.clone(); Py::Bool({ let t4320 = Py::Int(0i64); if py::gt(&t4319, &t4320)? { true } else { false } }) }; if py::eq(&t4315, &t4316) { true } else { false } }) }).truthy() {
                        let t4321 = (Py::tuple(vec![v_m.clone(), py::call(&v_f.clone(), &[v_m.clone()])?])).items()?;
                        if t4321.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                        v_lo = t4321[0].clone();
                        v_flo = t4321[1].clone();
                    } else {
                        v_hi = v_m.clone();
                    }
                    v_w2 = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
                }
                if ({ let t4322 = py::abs(&py::sub(&v_w2.clone(), &v_w.clone())?)?; Py::Bool({ let t4323 = py::mul(&py::pow(&f__mpf(Py::Int(10i64))?, &py::neg(&Py::Int(36i64))?)?, &py::add(&Py::Int(1i64), &py::abs(&v_w.clone())?)?)?; if py::le(&t4322, &t4323)? { true } else { false } }) }).truthy() {
                    return Ok(v_w2.clone());
                }
                v_w = v_w2.clone();
            }
            return Ok(v_w.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
        v_eps = py::pow(&f__mpf(Py::Int(10i64))?, &py::neg(&Py::Int(30i64))?)?;
        v_roots = Py::list(vec![]);
        v_hi = py::add(&py::truediv(&py::abs(&v_r.clone())?, &v_Lm.clone())?, &Py::Int(10i64))?;
        let _ = py::method(&v_roots.clone(), "append", &[py::call(&v_bis.clone(), &[py::mul(&py::neg(&py::getitem(&v_L.clone(), &Py::Int(0i64))?)?, &py::sub(&Py::Int(1i64), &v_eps.clone())?)?, v_hi.clone()])?])?;
        for t4324 in py::range_iter(&[(py::sub(&py::len(&v_L.clone())?, &Py::Int(1i64))?).i()?]) {
            v_i = t4324;
            let _ = py::method(&v_roots.clone(), "append", &[py::call(&v_bis.clone(), &[py::mul(&py::neg(&py::getitem(&v_L.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?)?, &py::sub(&Py::Int(1i64), &v_eps.clone())?)?, py::mul(&py::neg(&py::getitem(&v_L.clone(), &v_i.clone())?)?, &py::add(&Py::Int(1i64), &v_eps.clone())?)?])?])?;
        }
        v_lo = py::sub(&py::sub(&py::truediv(&py::neg(&py::add(&py::sum(&v_b.clone(), None)?, &py::abs(&v_r.clone())?)?)?, &v_Lm.clone())?, &py::getitem(&v_L.clone(), &py::neg(&Py::Int(1i64))?)?)?, &Py::Int(10i64))?;
        while ({ let t4325 = py::call(&v_f.clone(), &[v_lo.clone()])?; Py::Bool({ let t4326 = Py::Int(0i64); if py::gt(&t4325, &t4326)? { true } else { false } }) }).truthy() {
            v_lo = py::mul(&v_lo, &Py::Int(2i64))?;
        }
        let _ = py::method(&v_roots.clone(), "append", &[py::call(&v_bis.clone(), &[v_lo.clone(), py::mul(&py::neg(&py::getitem(&v_L.clone(), &py::neg(&Py::Int(1i64))?)?)?, &py::add(&Py::Int(1i64), &v_eps.clone())?)?])?])?;
        v_amps = Py::list(vec![]);
        for t4327 in py::iter(&(v_roots.clone()))? {
            v_w = t4327;
            v_N = py::add(&v_Lm.clone(), &py::sum(&{ let mut out = vec![]; for t4328 in py::iter(&(v_bl.clone()))? { let mut v_bi: Py; let mut v_li: Py; 
let t4329 = (t4328).items()?;
if t4329.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_li = t4329[0].clone();
v_bi = t4329[1].clone();
out.push(py::truediv(&v_bi.clone(), &py::add(&v_w.clone(), &v_li.clone())?)?); } Py::list(out) }, None)?)?;
            v_S2 = py::sum(&{ let mut out = vec![]; for t4330 in py::iter(&(v_bl.clone()))? { let mut v_bi: Py; let mut v_li: Py; 
let t4331 = (t4330).items()?;
if t4331.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_li = t4331[0].clone();
v_bi = t4331[1].clone();
out.push(py::truediv(&v_bi.clone(), &py::pow(&py::add(&v_w.clone(), &v_li.clone())?, &Py::Int(2i64))?)?); } Py::list(out) }, None)?;
            let _ = py::method(&v_amps.clone(), "append", &[py::truediv(&Py::Int(1i64), &py::sub(&Py::Int(1i64), &py::truediv(&py::mul(&v_w.clone(), &v_S2.clone())?, &v_N.clone())?)?)?])?;
        }
        return Ok(Py::tuple(vec![v_roots.clone(), v_amps.clone()]));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__isnum(mut v_v: Py) -> R {
    return Ok({ let t4787 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4787.truthy() { t4787 } else { { let t4786 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4786.truthy() { t4786 } else { py::math1("isfinite", &v_v.clone())? } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__kin_check(mut v_rho: Py, mut v_beta: Py, mut v_lam: Py, mut v_Lam: Py) -> R {
    let _ = f__need({ let t4271 = { let t4266 = py::len(&v_beta.clone())?; Py::Bool({ let t4267 = py::len(&v_lam.clone())?; if py::eq(&t4266, &t4267) { true } else { false } }) }; if !t4271.truthy() { t4271 } else { { let t4268 = Py::Int(1i64); Py::Bool({ let t4269 = py::len(&v_beta.clone())?; if py::le(&t4268, &t4269)? { { let t4270 = Py::Int(8i64); if py::le(&t4269, &t4270)? { true } else { false } } } else { false } }) } } }, Py::str("give one decay constant per delayed group (1 to 8 groups)"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t4272 in py::iter(&(v_beta.clone()))? { let mut v_b: Py; 
v_b = t4272;
out.push({ let t4277 = f__isnum(v_b.clone())?; if !t4277.truthy() { t4277 } else { { let t4275 = v_b.clone(); Py::Bool({ let t4276 = Py::Int(0i64); if py::gt(&t4275, &t4276)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("delayed fractions must be positive"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t4278 in py::iter(&(v_lam.clone()))? { let mut v_x: Py; 
v_x = t4278;
out.push({ let t4281 = f__isnum(v_x.clone())?; if !t4281.truthy() { t4281 } else { { let t4279 = v_x.clone(); Py::Bool({ let t4280 = Py::Int(0i64); if py::gt(&t4279, &t4280)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("decay constants must be positive"))?;
    let _ = f__need({ let t4282 = py::len(&py::set(&v_lam.clone())?)?; Py::Bool({ let t4283 = py::len(&v_lam.clone())?; if py::eq(&t4282, &t4283) { true } else { false } }) }, Py::str("the groups' decay constants must differ"))?;
    let _ = f__pos(Py::str("the generation time"), Py::tuple(vec![v_Lam.clone()]))?;
    let _ = f__need({ let t4290 = f__isnum(v_rho.clone())?; if !t4290.truthy() { t4290 } else { { let t4288 = v_rho.clone(); Py::Bool({ let t4289 = Py::Int(0i64); if py::ne(&t4288, &t4289) { true } else { false } }) } } }, Py::str("a step reactivity other than zero is needed"))?;
    let _ = f__need({ let t4291 = v_rho.clone(); Py::Bool({ let t4292 = py::fsum(&v_beta.clone())?; if py::lt(&t4291, &t4292)? { true } else { false } }) }, Py::str("prompt critical or above (rho >= beta) is not supported"))?;
    let _ = f__need({ let t4293 = v_rho.clone(); Py::Bool({ let t4294 = py::neg(&Py::Float(0.5))?; if py::gt(&t4293, &t4294)? { true } else { false } }) }, Py::str("the reactivity is below -0.5"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lists(mut v_t: Py, mut v_C: Py, mut v_n_min: Py) -> R {
    let _ = f__need({ let t4573 = py::len(&v_t.clone())?; Py::Bool({ let t4574 = py::len(&v_C.clone())?; if py::eq(&t4573, &t4574) { true } else { false } }) }, Py::str("give as many times as counts"))?;
    let _ = f__need({ let t4575 = py::len(&v_t.clone())?; Py::Bool({ let t4576 = v_n_min.clone(); if py::ge(&t4575, &t4576)? { true } else { false } }) }, Py::str(&["give at least ".to_string(), py::format(&v_n_min.clone(), "")?, " measurements".to_string()].concat()))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t4577 in py::iter(&(v_t.clone()))? { let mut v_x: Py; 
v_x = t4577;
out.push(py::math1("isfinite", &v_x.clone())?); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("the times must be numbers"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t4578 in py::iter(&(v_C.clone()))? { let mut v_c: Py; 
v_c = t4578;
out.push({ let t4581 = py::math1("isfinite", &v_c.clone())?; if !t4581.truthy() { t4581 } else { { let t4579 = v_c.clone(); Py::Bool({ let t4580 = Py::Int(0i64); if py::gt(&t4579, &t4580)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("every count must be positive"))?;
    let _ = f__need({ let t4582 = py::len(&py::set(&v_t.clone())?)?; Py::Bool({ let t4583 = py::len(&v_t.clone())?; if py::eq(&t4582, &t4583) { true } else { false } }) }, Py::str("the times must differ"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mpf(mut v_x: Py) -> R {
    return Ok(ax_engine::pymp::call("mpf", &[v_x.clone()], &[])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__need(mut v_ok: Py, mut v_msg: Py) -> R {
    if (py::not(&v_ok.clone())).truthy() {
        return Err(py::value_error(py::to_str(&v_msg.clone())));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nonneg(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t4733 in py::iter(&(v_vals.clone()))? {
        v_v = t4733;
        let _ = f__need({ let t4736 = f__isnum(v_v.clone())?; if !t4736.truthy() { t4736 } else { { let t4734 = v_v.clone(); Py::Bool({ let t4735 = Py::Int(0i64); if py::ge(&t4734, &t4735)? { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be zero or positive".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__out(mut v_kw: Py) -> R {
    return Ok({ let mut out = vec![]; for t4803 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t4804 = (t4803).items()?;
if t4804.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t4804[0].clone();
v_v = t4804[1].clone();
out.push((v_k.clone(), f__r(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t4785 in py::iter(&(v_vals.clone()))? {
        v_v = t4785;
        let _ = f__need({ let t4790 = f__isnum(v_v.clone())?; if !t4790.truthy() { t4790 } else { { let t4788 = v_v.clone(); Py::Bool({ let t4789 = Py::Int(0i64); if py::gt(&t4788, &t4789)? { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a positive number".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r(mut v_x: Py) -> R {
    if ({ let t4808 = Py::Bool(matches!(v_x.clone(), Py::Bool(_))); if t4808.truthy() { t4808 } else { { let t4807 = Py::Bool(matches!(v_x.clone(), Py::Str(_))); if t4807.truthy() { t4807 } else { { let t4805 = v_x.clone(); Py::Bool({ let t4806 = Py::None; if py::is(&t4805, &t4806) { true } else { false } }) } } } } }).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_)))).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t4809 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t4809;
out.push(f__r(v_v.clone())?); } Py::list(out) });
    }
    v_x = py::float(&v_x.clone())?;
    if ({ let t4812 = { let t4810 = v_x.clone(); Py::Bool({ let t4811 = Py::Int(0i64); if py::eq(&t4810, &t4811) { true } else { false } }) }; if t4812.truthy() { t4812 } else { py::not(&py::math1("isfinite", &v_x.clone())?) } }).truthy() {
        return Ok((if ({ let t4813 = v_x.clone(); Py::Bool({ let t4814 = Py::Int(0i64); if py::eq(&t4813, &t4814) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    }
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    if ({ let t4821 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t4821.truthy() { t4821 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t4825 = py::not(&{ let t4822 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t4822.truthy() { t4822 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t4825.truthy() { t4825 } else { { let t4823 = py::len(&v_a.clone())?; Py::Bool({ let t4824 = py::len(&v_b.clone())?; if py::ne(&t4823, &t4824) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        return Ok(py::minmax(&[py::add(&{ let mut out = vec![]; for t4826 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t4827 = (t4826).items()?;
if t4827.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t4827[0].clone();
v_y = t4827[1].clone();
out.push(f__rel(v_x.clone(), v_y.clone())?); } Py::list(out) }, &Py::list(vec![Py::Float(0.0)]))?], true)?);
    }
    if ({ let t4828 = Py::Bool(matches!(v_a.clone(), Py::Str(_) | Py::Bool(_))); if t4828.truthy() { t4828 } else { Py::Bool(matches!(v_b.clone(), Py::Str(_) | Py::Bool(_))) } }).truthy() {
        return Ok((if ({ let t4829 = v_a.clone(); Py::Bool({ let t4830 = v_b.clone(); if py::eq(&t4829, &t4830) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(f64::INFINITY) }));
    }
    let t4832: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t4831 = (Py::tuple(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])).items()?;
            if t4831.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t4831[0].clone();
            v_b = t4831[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4832 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t4833 = v_d.clone(); Py::Bool({ let t4834 = Py::Int(0i64); if py::eq(&t4833, &t4834) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4839 = { let t4835 = py::abs(&v_a.clone())?; Py::Bool({ let t4836 = Py::Float(1e-300); if py::lt(&t4835, &t4836)? { true } else { false } }) }; if !t4839.truthy() { t4839 } else { { let t4837 = py::abs(&v_b.clone())?; Py::Bool({ let t4838 = Py::Float(1e-300); if py::lt(&t4837, &t4838)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(0.0));
    }
    return Ok(py::truediv(&v_d.clone(), &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sievert(mut v_theta: Py, mut v_b: Py) -> R {
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    if ({ let t2821 = v_b.clone(); Py::Bool({ let t2822 = Py::Int(0i64); if py::eq(&t2821, &t2822) { true } else { false } }) }).truthy() {
        return Ok(v_theta.clone());
    }
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(25i64))?;
        v_s = (if ({ let t2823 = v_theta.clone(); Py::Bool({ let t2824 = Py::Int(0i64); if py::ge(&t2823, &t2824)? { true } else { false } }) }).truthy() { Py::Float(1.0) } else { py::neg(&Py::Float(1.0))? });
        v_v = ax_engine::pymp::call("quad", &[{ let v_b = v_b.clone(); let v_s = v_s.clone(); let v_theta = v_theta.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(ax_engine::pymp::call("exp", &[py::truediv(&py::neg(&v_b.clone())?, &ax_engine::pymp::call("cos", &[v_u.clone()], &[])?)?], &[])?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::list(vec![Py::Int(0i64), py::abs(&v_theta.clone())?])], &[])?;
        return Ok(py::mul(&v_s.clone(), &py::float(&v_v.clone())?)?);
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__swu_value(mut v_x: Py) -> R {
    return Ok(py::mul(&py::sub(&py::mul(&Py::Int(2i64), &v_x.clone())?, &Py::Int(1i64))?, &py::math1("log", &py::truediv(&v_x.clone(), &py::sub(&Py::Int(1i64), &v_x.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__tmax(mut v_lp: Py, mut v_ld: Py) -> R {
    let mut v_r: Py = Py::None;
    v_r = py::truediv(&v_ld.clone(), &v_lp.clone())?;
    if ({ let t2550 = v_r.clone(); Py::Bool({ let t2551 = Py::Float(1.0); if py::eq(&t2550, &t2551) { true } else { false } }) }).truthy() {
        return Ok(py::truediv(&Py::Float(1.0), &v_lp.clone())?);
    }
    return Ok(py::truediv(&py::math1("log1p", &py::sub(&v_r.clone(), &Py::Float(1.0))?)?, &py::mul(&v_lp.clone(), &py::sub(&v_r.clone(), &Py::Float(1.0))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ww_check(mut v_P0: Py, mut v_T_op: Py, mut v_ts: Py) -> R {
    let mut v_t: Py = Py::None;
    let _ = f__pos(Py::str("the power before shutdown and the operating time"), Py::tuple(vec![v_P0.clone(), v_T_op.clone()]))?;
    for t1646 in py::iter(&(v_ts.clone()))? {
        v_t = t1646;
        let _ = f__need({ let t1650 = f__isnum(v_t.clone())?; if !t1650.truthy() { t1650 } else { { let t1647 = Py::Float(10.0); Py::Bool({ let t1648 = v_t.clone(); if py::le(&t1647, &t1648)? { { let t1649 = Py::Float(100000000.0); if py::le(&t1648, &t1649)? { true } else { false } } } else { false } }) } } }, Py::str("the Way-Wigner correlation holds from 10 s to about 1e8 s after shutdown"))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__xe_eq(mut v_Sigma_f: Py, mut v_phi: Py, mut v_gamma_I: Py, mut v_gamma_X: Py, mut v_sigma_X: Py) -> R {
    let mut v_I: Py = Py::None;
    let mut v_X: Py = Py::None;
    let _ = f__pos(Py::str("Sigma_f, the flux and sigma_X"), Py::tuple(vec![v_Sigma_f.clone(), v_phi.clone(), v_sigma_X.clone()]))?;
    let _ = f__frac(Py::str("gamma_I"), v_gamma_I.clone(), Py::Bool(false), Py::Bool(false))?;
    let _ = f__frac(Py::str("gamma_X"), v_gamma_X.clone(), Py::Bool(false), Py::Bool(false))?;
    let _ = f__need({ let t4800 = py::add(&v_gamma_I.clone(), &v_gamma_X.clone())?; Py::Bool({ let t4801 = Py::Int(0i64); if py::gt(&t4800, &t4801)? { true } else { false } }) }, Py::str("the yields cannot both be zero"))?;
    v_I = py::truediv(&py::mul(&py::mul(&v_gamma_I.clone(), &v_Sigma_f.clone())?, &v_phi.clone())?, &c__LAM_I()?)?;
    v_X = py::truediv(&py::mul(&py::mul(&py::add(&v_gamma_I.clone(), &v_gamma_X.clone())?, &v_Sigma_f.clone())?, &v_phi.clone())?, &py::add(&c__LAM_X()?, &py::mul(&v_sigma_X.clone(), &v_phi.clone())?)?)?;
    return Ok(Py::tuple(vec![v_I.clone(), v_X.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__xi(mut v_A: Py) -> R {
    let mut v_a: Py = Py::None;
    v_a = f__alpha(v_A.clone())?;
    if ({ let t3083 = v_a.clone(); Py::Bool({ let t3084 = Py::Int(0i64); if py::eq(&t3083, &t3084) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::add(&Py::Float(1.0), &py::truediv(&py::mul(&v_a.clone(), &py::math1("log", &v_a.clone())?)?, &py::sub(&Py::Float(1.0), &v_a.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_accumulated_dose(mut v_H0: Py, mut v_T_half: Py, mut v_t: Py, mut v_td: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_Hs: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let _ = f__pos(Py::str("the initial dose rate, the half-life and the exposure time"), Py::tuple(vec![v_H0.clone(), v_T_half.clone(), v_t.clone()]))?;
    let _ = f__nonneg(Py::str("the delay td"), Py::tuple(vec![v_td.clone()]))?;
    v_lam = py::truediv(&c_LN2()?, &v_T_half.clone())?;
    v_Hs = py::mul(&v_H0.clone(), &py::math1("exp", &py::mul(&py::neg(&v_lam.clone())?, &v_td.clone())?)?)?;
    v_D = py::truediv(&py::mul(&v_Hs.clone(), &py::neg(&py::math1("expm1", &py::mul(&py::neg(&v_lam.clone())?, &v_t.clone())?)?)?)?, &v_lam.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("D"), v_D.clone()), (Py::str("D_mSv"), py::mul(&v_D.clone(), &Py::Float(1000.0))?), (Py::str("D_inf"), py::truediv(&v_Hs.clone(), &v_lam.clone())?), (Py::str("H_mean"), py::truediv(&v_D.clone(), &v_t.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_activation(mut v_m: Py, mut v_M: Py, mut v_theta: Py, mut v_sigma: Py, mut v_phi: Py, mut v_T_half: Py, mut v_t_irr: Py, mut v_t_d: Py) -> R {
    let mut v_A_eoi: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let _ = f__pos(Py::str("the target mass, molar mass, cross section, flux, half-life and irradiation time"), Py::tuple(vec![v_m.clone(), v_M.clone(), v_sigma.clone(), v_phi.clone(), v_T_half.clone(), v_t_irr.clone()]))?;
    let _ = f__frac(Py::str("the isotopic abundance theta"), v_theta.clone(), Py::Bool(true), Py::Bool(false))?;
    let _ = f__nonneg(Py::str("the decay time t_d"), Py::tuple(vec![v_t_d.clone()]))?;
    v_lam = py::truediv(&c_LN2()?, &v_T_half.clone())?;
    v_R = py::mul(&py::mul(&py::mul(&py::mul(&py::truediv(&v_m.clone(), &v_M.clone())?, &c_NA()?)?, &v_theta.clone())?, &v_sigma.clone())?, &v_phi.clone())?;
    v_S = py::neg(&py::math1("expm1", &py::mul(&py::neg(&v_lam.clone())?, &v_t_irr.clone())?)?)?;
    v_A_eoi = py::mul(&v_R.clone(), &v_S.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("A_eoi"), v_A_eoi.clone()), (Py::str("A_d"), py::mul(&v_A_eoi.clone(), &py::math1("exp", &py::mul(&py::neg(&v_lam.clone())?, &v_t_d.clone())?)?)?), (Py::str("A_sat"), v_R.clone()), (Py::str("S"), v_S.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_activation_burnup(mut v_N_T0: Py, mut v_sigma_T: Py, mut v_sigma_P: Py, mut v_phi: Py, mut v_T_half: Py, mut v_t: Py) -> R {
    let mut v_LP: Py = Py::None;
    let mut v_LT: Py = Py::None;
    let mut v_NP: Py = Py::None;
    let mut v_NPm: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_tm: Py = Py::None;
    let mut v_tt: Py = Py::None;
    let _ = f__pos(Py::str("N_T0, the cross sections, the flux and the half-life"), Py::tuple(vec![v_N_T0.clone(), v_sigma_T.clone(), v_sigma_P.clone(), v_phi.clone(), v_T_half.clone()]))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    v_lam = py::truediv(&c_LN2()?, &v_T_half.clone())?;
    let t116 = (Py::tuple(vec![py::mul(&v_sigma_T.clone(), &v_phi.clone())?, py::add(&v_lam.clone(), &py::mul(&v_sigma_P.clone(), &v_phi.clone())?)?])).items()?;
    if t116.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_LT = t116[0].clone();
    v_LP = t116[1].clone();
    let _ = f__need({ let t117 = py::abs(&py::sub(&py::truediv(&v_LP.clone(), &v_LT.clone())?, &Py::Float(1.0))?)?; Py::Bool({ let t118 = Py::Float(1e-06); if py::gt(&t117, &t118)? { true } else { false } }) }, Py::str("the product's removal rate equals the target's burn-up rate: not supported"))?;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(40i64))?;
        let t119 = (Py::tuple(vec![f__mpf(v_LT.clone())?, f__mpf(v_LP.clone())?, f__mpf(v_t.clone())?])).items()?;
        if t119.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_a = t119[0].clone();
        v_b = t119[1].clone();
        v_tt = t119[2].clone();
        v_NP = py::float(&py::mul(&py::truediv(&py::mul(&f__mpf(v_N_T0.clone())?, &v_a.clone())?, &py::sub(&v_b.clone(), &v_a.clone())?)?, &py::sub(&ax_engine::pymp::call("exp", &[py::mul(&py::neg(&v_a.clone())?, &v_tt.clone())?], &[])?, &ax_engine::pymp::call("exp", &[py::mul(&py::neg(&v_b.clone())?, &v_tt.clone())?], &[])?)?)?)?;
        v_tm = py::float(&py::truediv(&ax_engine::pymp::call("log", &[py::truediv(&v_b.clone(), &v_a.clone())?], &[])?, &py::sub(&v_b.clone(), &v_a.clone())?)?)?;
        v_NPm = py::float(&py::mul(&py::truediv(&py::mul(&f__mpf(v_N_T0.clone())?, &v_a.clone())?, &py::sub(&v_b.clone(), &v_a.clone())?)?, &py::sub(&ax_engine::pymp::call("exp", &[py::mul(&py::neg(&v_a.clone())?, &v_tm.clone())?], &[])?, &ax_engine::pymp::call("exp", &[py::mul(&py::neg(&v_b.clone())?, &v_tm.clone())?], &[])?)?)?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("A_P"), py::mul(&v_lam.clone(), &v_NP.clone())?), (Py::str("f_T"), py::math1("exp", &py::mul(&py::neg(&v_LT.clone())?, &v_t.clone())?)?), (Py::str("t_max"), v_tm.clone()), (Py::str("A_max"), py::mul(&v_lam.clone(), &v_NPm.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_attenuation_fit(mut v_x: Py, mut v_I: Py) -> R {
    let mut v_mu: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_xs: Py = Py::None;
    let mut v_y: Py = Py::None;
    let _ = f__need({ let t162 = { let t158 = py::len(&v_x.clone())?; Py::Bool({ let t159 = py::len(&v_I.clone())?; if py::eq(&t158, &t159) { true } else { false } }) }; if !t162.truthy() { t162 } else { { let t160 = py::len(&v_x.clone())?; Py::Bool({ let t161 = Py::Int(3i64); if py::ge(&t160, &t161)? { true } else { false } }) } } }, Py::str("give at least three thicknesses with their intensities"))?;
    let _ = f__need({ let t171 = Py::Bool(({ let mut out = vec![]; for t163 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t163;
out.push({ let t168 = f__isnum(v_v.clone())?; if !t168.truthy() { t168 } else { { let t166 = v_v.clone(); Py::Bool({ let t167 = Py::Int(0i64); if py::ge(&t166, &t167)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t171.truthy() { t171 } else { { let t169 = py::len(&py::set(&v_x.clone())?)?; Py::Bool({ let t170 = py::len(&v_x.clone())?; if py::eq(&t169, &t170) { true } else { false } }) } } }, Py::str("thicknesses must be distinct and >= 0"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t172 in py::iter(&(v_I.clone()))? { let mut v_v: Py; 
v_v = t172;
out.push({ let t175 = f__isnum(v_v.clone())?; if !t175.truthy() { t175 } else { { let t173 = v_v.clone(); Py::Bool({ let t174 = Py::Int(0i64); if py::gt(&t173, &t174)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("intensities must be positive"))?;
    v_xs = { let mut out = vec![]; for t176 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t176;
out.push(py::truediv(&v_v.clone(), &Py::Float(100.0))?); } Py::list(out) };
    v_n = py::len(&v_xs.clone())?;
    v_y = { let mut out = vec![]; for t177 in py::iter(&(v_I.clone()))? { let mut v_v: Py; 
v_v = t177;
out.push(py::math1("log", &v_v.clone())?); } Py::list(out) };
    let t178 = (Py::tuple(vec![py::truediv(&py::fsum(&v_xs.clone())?, &v_n.clone())?, py::truediv(&py::fsum(&v_y.clone())?, &v_n.clone())?])).items()?;
    if t178.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t178[0].clone();
    v_my = t178[1].clone();
    v_s = py::truediv(&py::fsum(&{ let mut out = vec![]; for t179 in py::iter(&(py::zip(&[v_xs.clone(), v_y.clone()])?))? { let mut v_u: Py; let mut v_w: Py; 
let t180 = (t179).items()?;
if t180.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t180[0].clone();
v_w = t180[1].clone();
out.push(py::mul(&py::sub(&v_u.clone(), &v_mx.clone())?, &py::sub(&v_w.clone(), &v_my.clone())?)?); } Py::list(out) })?, &py::fsum(&{ let mut out = vec![]; for t181 in py::iter(&(v_xs.clone()))? { let mut v_u: Py; 
v_u = t181;
out.push(py::pow(&py::sub(&v_u.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) })?)?;
    v_mu = py::neg(&v_s.clone())?;
    let _ = f__need({ let t182 = v_mu.clone(); Py::Bool({ let t183 = Py::Int(0i64); if py::gt(&t182, &t183)? { true } else { false } }) }, Py::str("the intensity does not fall with thickness: no attenuation coefficient"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("mu"), v_mu.clone()), (Py::str("HVL"), py::truediv(&c_LN2()?, &v_mu.clone())?), (Py::str("TVL"), py::truediv(&py::math1("log", &Py::Float(10.0))?, &v_mu.clone())?), (Py::str("I0"), py::math1("exp", &py::sub(&v_my.clone(), &py::mul(&v_s.clone(), &v_mx.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bare_keff(mut v_D: Py, mut v_Sigma_a: Py, mut v_nuSigma_f: Py, mut v_extrapolation: Py, mut v_geometry: Py, mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_R: Py, mut v_H: Py) -> R {
    let mut v_B2: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_dims: Py = Py::None;
    let mut v_ext: Py = Py::None;
    let mut v_k: Py = Py::None;
    let _ = f__pos(Py::str("D, Sigma_a and nu Sigma_f"), Py::tuple(vec![v_D.clone(), v_Sigma_a.clone(), v_nuSigma_f.clone()]))?;
    v_dims = f__dims(v_geometry.clone(), Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone()), (Py::str("c"), v_c.clone()), (Py::str("R"), v_R.clone()), (Py::str("H"), v_H.clone())]))?;
    v_d = py::mul(&py::getitem(&c_EXTRAP()?, &py::method(&v_extrapolation.clone(), "lower", &[])?)?, &v_D.clone())?;
    v_ext = (if ({ let t239 = v_geometry.clone(); Py::Bool({ let t240 = Py::tuple(vec![Py::str("slab"), Py::str("cuboid")]); if py::contains(&t240, &t239)? { true } else { false } }) }).truthy() { { let mut out = vec![]; for t241 in py::iter(&(v_dims.clone()))? { let mut v_x: Py; 
v_x = t241;
out.push(py::add(&v_x.clone(), &py::mul(&Py::Int(2i64), &v_d.clone())?)?); } Py::list(out) } } else { (if ({ let t242 = v_geometry.clone(); Py::Bool({ let t243 = Py::str("sphere"); if py::eq(&t242, &t243) { true } else { false } }) }).truthy() { Py::list(vec![py::add(&py::getitem(&v_dims.clone(), &Py::Int(0i64))?, &v_d.clone())?]) } else { Py::list(vec![py::add(&py::getitem(&v_dims.clone(), &Py::Int(0i64))?, &v_d.clone())?, py::add(&py::getitem(&v_dims.clone(), &Py::Int(1i64))?, &py::mul(&Py::Int(2i64), &v_d.clone())?)?]) }) });
    v_B2 = f__bg2(v_geometry.clone(), v_ext.clone())?;
    v_k = py::truediv(&v_nuSigma_f.clone(), &py::add(&v_Sigma_a.clone(), &py::mul(&v_D.clone(), &v_B2.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("B2"), v_B2.clone()), (Py::str("k_eff"), v_k.clone()), (Py::str("P_NL"), py::truediv(&v_Sigma_a.clone(), &py::add(&v_Sigma_a.clone(), &py::mul(&v_D.clone(), &v_B2.clone())?)?)?), (Py::str("rho"), py::truediv(&py::sub(&v_k.clone(), &Py::Int(1i64))?, &v_k.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_centreline_variable_k(mut v_q_lin: Py, mut v_T_s: Py, mut v_A: Py, mut v_B: Py) -> R {
    let mut v_T0: Py = Py::None;
    let _ = f__pos(Py::str("q', the surface temperature, A and B"), Py::tuple(vec![v_q_lin.clone(), v_T_s.clone(), v_A.clone(), v_B.clone()]))?;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        v_T0 = py::float(&py::truediv(&py::sub(&py::mul(&py::add(&v_A.clone(), &py::mul(&v_B.clone(), &f__mpf(v_T_s.clone())?)?)?, &ax_engine::pymp::call("exp", &[py::truediv(&py::mul(&v_B.clone(), &f__mpf(v_q_lin.clone())?)?, &py::mul(&Py::Int(4i64), &ax_engine::pymp::constant("pi")?)?)?], &[])?)?, &v_A.clone())?, &v_B.clone())?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("T_0"), v_T0.clone()), (Py::str("dT"), py::sub(&v_T0.clone(), &v_T_s.clone())?), (Py::str("k_s"), py::truediv(&Py::Float(1.0), &py::add(&v_A.clone(), &py::mul(&v_B.clone(), &v_T_s.clone())?)?)?), (Py::str("k_0"), py::truediv(&Py::Float(1.0), &py::add(&v_A.clone(), &py::mul(&v_B.clone(), &v_T0.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_chain_activity(mut v_nuclide: Py, mut v_A0: Py, mut v_t: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_Ad: Py = Py::None;
    let mut v_Ap: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_ch: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let _ = f__need({ let t333 = v_nuclide.clone(); Py::Bool({ let t334 = c__CHAINS()?; if py::contains(&t334, &t333)? { true } else { false } }) }, Py::str("the parent must be one of the tabulated chain parents"))?;
    let _ = f__pos(Py::str("the initial activity A0"), Py::tuple(vec![v_A0.clone()]))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    v_ch = py::getitem(&c__CHAINS()?, &v_nuclide.clone())?;
    v_lam = py::getitem(&v_ch.clone(), &Py::str("lam"))?;
    v_N = f__bateman_dag(py::getitem(&v_ch.clone(), &Py::str("order"))?, v_lam.clone(), py::getitem(&v_ch.clone(), &Py::str("edges"))?, Py::dict(vec![(v_nuclide.clone(), py::truediv(&v_A0.clone(), &py::getitem(&v_lam.clone(), &v_nuclide.clone())?)?)]), v_t.clone())?;
    v_A = { let mut out = vec![]; for t356 in py::iter(&(py::getitem(&v_ch.clone(), &Py::str("order"))?))? { let mut v_m: Py; 
v_m = t356;
out.push((v_m.clone(), py::mul(&py::getitem(&v_lam.clone(), &v_m.clone())?, &py::minmax(&[py::getitem(&v_N.clone(), &v_m.clone())?, Py::Float(0.0)], true)?)?)); } Py::dict(out) };
    let t357 = (Py::tuple(vec![py::getitem(&v_A.clone(), &v_nuclide.clone())?, py::getitem(&v_A.clone(), &py::getitem(&v_ch.clone(), &Py::str("daughter"))?)?])).items()?;
    if t357.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Ap = t357[0].clone();
    v_Ad = t357[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("A_parent"), v_Ap.clone()), (Py::str("A_daughter"), v_Ad.clone()), (Py::str("A_total"), py::fsum(&py::method(&v_A.clone(), "values", &[])?)?), (Py::str("ratio"), py::truediv(&v_Ad.clone(), &v_Ap.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_channel_temperature(mut v_q0: Py, mut v_H: Py, mut v_H_e: Py, mut v_m_dot: Py, mut v_c_p: Py, mut v_T_in: Py, mut v_z: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_s0: Py = Py::None;
    let _ = f__ch_check(v_q0.clone(), v_H.clone(), v_H_e.clone(), v_m_dot.clone(), v_c_p.clone(), v_T_in.clone())?;
    let _ = f__need({ let t406 = f__isnum(v_z.clone())?; if !t406.truthy() { t406 } else { { let t404 = py::abs(&v_z.clone())?; Py::Bool({ let t405 = py::truediv(&v_H.clone(), &Py::Int(2i64))?; if py::le(&t404, &t405)? { true } else { false } }) } } }, Py::str("z must lie within the core (|z| <= H/2), measured from the mid-plane"))?;
    v_k = py::truediv(&py::mul(&v_q0.clone(), &v_H_e.clone())?, &py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_m_dot.clone())?, &v_c_p.clone())?)?;
    v_s0 = py::math1("sin", &py::truediv(&py::mul(&Py::Float(std::f64::consts::PI), &v_H.clone())?, &py::mul(&Py::Int(2i64), &v_H_e.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("T"), py::add(&v_T_in.clone(), &py::mul(&v_k.clone(), &py::add(&py::math1("sin", &py::truediv(&py::mul(&Py::Float(std::f64::consts::PI), &v_z.clone())?, &v_H_e.clone())?)?, &v_s0.clone())?)?)?), (Py::str("T_out"), py::add(&v_T_in.clone(), &py::mul(&py::mul(&Py::Int(2i64), &v_k.clone())?, &v_s0.clone())?)?), (Py::str("Q"), py::mul(&py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_q0.clone())?, &v_H_e.clone())?, &Py::Float(std::f64::consts::PI))?, &v_s0.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_collisions_to_thermalize(mut v_A: Py, mut v_E0: Py, mut v_Ef: Py) -> R {
    let mut v_du: Py = Py::None;
    let mut v_xi: Py = Py::None;
    let _ = f__need({ let t449 = f__isnum(v_A.clone())?; if !t449.truthy() { t449 } else { { let t447 = v_A.clone(); Py::Bool({ let t448 = Py::Int(1i64); if py::ge(&t447, &t448)? { true } else { false } }) } } }, Py::str("the mass ratio A must be at least 1"))?;
    let _ = f__pos(Py::str("the energies"), Py::tuple(vec![v_E0.clone(), v_Ef.clone()]))?;
    let _ = f__need({ let t454 = v_E0.clone(); Py::Bool({ let t455 = v_Ef.clone(); if py::gt(&t454, &t455)? { true } else { false } }) }, Py::str("the initial energy must exceed the final energy"))?;
    v_xi = f__xi(v_A.clone())?;
    v_du = py::math1("log", &py::truediv(&v_E0.clone(), &v_Ef.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("n"), py::truediv(&v_du.clone(), &v_xi.clone())?), (Py::str("xi"), v_xi.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_committed_dose(mut v_I: Py, mut v_e: Py) -> R {
    let mut v_E: Py = Py::None;
    let _ = f__pos(Py::str("the intake and the dose coefficient"), Py::tuple(vec![v_I.clone(), v_e.clone()]))?;
    let _ = f__need({ let t502 = v_e.clone(); Py::Bool({ let t503 = Py::Float(0.001); if py::le(&t502, &t503)? { true } else { false } }) }, Py::str("the dose coefficient must be at most 1e-3 Sv/Bq"))?;
    v_E = py::mul(&v_I.clone(), &v_e.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("E"), v_E.clone()), (Py::str("E_mSv"), py::mul(&v_E.clone(), &Py::Float(1000.0))?), (Py::str("I_Bq"), v_I.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_compton_edge(mut v_E: Py) -> R {
    let mut v_Ebs: Py = Py::None;
    let mut v_Emev: Py = Py::None;
    let mut v_k: Py = Py::None;
    let _ = f__pos(Py::str("the photon energy"), Py::tuple(vec![v_E.clone()]))?;
    v_Emev = py::truediv(&v_E.clone(), &c_MEV()?)?;
    v_k = py::truediv(&v_Emev.clone(), &c_ME_MEV()?)?;
    v_Ebs = py::truediv(&v_Emev.clone(), &py::add(&Py::Int(1i64), &py::mul(&Py::Int(2i64), &v_k.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("T_max"), py::sub(&v_Emev.clone(), &v_Ebs.clone())?), (Py::str("E_bs"), v_Ebs.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_constant_power_depletion(mut v_m0: Py, mut v_P: Py, mut v_t: Py, mut v_E_f: Py, mut v_sigma_a: Py, mut v_sigma_f: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_N0: Py = Py::None;
    let mut v_dN: Py = Py::None;
    let _ = f__pos(Py::str("m0, P, E_f and the cross sections"), Py::tuple(vec![v_m0.clone(), v_P.clone(), v_E_f.clone(), v_sigma_a.clone(), v_sigma_f.clone()]))?;
    let _ = f__need({ let t658 = v_sigma_f.clone(); Py::Bool({ let t659 = v_sigma_a.clone(); if py::le(&t658, &t659)? { true } else { false } }) }, Py::str("the fission cross section cannot exceed the absorption cross section"))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    v_N0 = py::mul(&py::truediv(&py::mul(&v_m0.clone(), &Py::Float(1000.0))?, &c__M5()?)?, &c_NA()?)?;
    v_dN = py::truediv(&py::mul(&py::truediv(&py::mul(&v_P.clone(), &v_t.clone())?, &v_E_f.clone())?, &v_sigma_a.clone())?, &v_sigma_f.clone())?;
    let _ = f__need({ let t664 = v_dN.clone(); Py::Bool({ let t665 = v_N0.clone(); if py::lt(&t664, &t665)? { true } else { false } }) }, Py::str("the fuel would be used up before t at this power"))?;
    v_N = py::sub(&v_N0.clone(), &v_dN.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("m"), py::truediv(&py::mul(&v_m0.clone(), &v_N.clone())?, &v_N0.clone())?), (Py::str("burnt"), py::truediv(&v_dN.clone(), &v_N0.clone())?), (Py::str("consumed"), py::truediv(&py::mul(&v_m0.clone(), &v_dN.clone())?, &v_N0.clone())?), (Py::str("flux_ratio"), py::truediv(&v_N0.clone(), &v_N.clone())?), (Py::str("fissions"), py::truediv(&py::mul(&v_P.clone(), &v_t.clone())?, &v_E_f.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_core_power(mut v_geometry: Py, mut v_Sigma_f: Py, mut v_E_f: Py, mut v_phi_max: Py, mut v_P: Py, mut v_R: Py, mut v_H: Py, mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_dims: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_j1: Py = Py::None;
    let _ = f__need({ let t760 = v_geometry.clone(); Py::Bool({ let t761 = Py::tuple(vec![Py::str("sphere"), Py::str("cylinder"), Py::str("cuboid")]); if py::contains(&t761, &t760)? { true } else { false } }) }, Py::str("geometry must be sphere, cylinder or cuboid"))?;
    let _ = f__pos(Py::str("Sigma_f and the energy per fission"), Py::tuple(vec![v_Sigma_f.clone(), v_E_f.clone()]))?;
    let _ = f__need({ let t768 = { let t770 = v_phi_max.clone(); Py::Bool({ let t771 = Py::None; if py::is(&t770, &t771) { true } else { false } }) }; Py::Bool({ let t769 = { let t772 = v_P.clone(); Py::Bool({ let t773 = Py::None; if py::is(&t772, &t773) { true } else { false } }) }; if py::ne(&t768, &t769) { true } else { false } }) }, Py::str("give either the peak flux phi_max or the power P (one of them)"))?;
    v_dims = f__dims(v_geometry.clone(), Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone()), (Py::str("c"), v_c.clone()), (Py::str("R"), v_R.clone()), (Py::str("H"), v_H.clone())]))?;
    if ({ let t785 = v_geometry.clone(); Py::Bool({ let t786 = Py::str("sphere"); if py::eq(&t785, &t786) { true } else { false } }) }).truthy() {
        let t787 = (Py::tuple(vec![py::mul(&py::mul(&py::truediv(&Py::Float(4.0), &Py::Float(3.0))?, &Py::Float(std::f64::consts::PI))?, &py::pow(&py::getitem(&v_dims.clone(), &Py::Int(0i64))?, &Py::Int(3i64))?)?, py::truediv(&Py::Float(3.0), &py::pow(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?)?])).items()?;
        if t787.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_V = t787[0].clone();
        v_f = t787[1].clone();
    } else {
        if ({ let t788 = v_geometry.clone(); Py::Bool({ let t789 = Py::str("cylinder"); if py::eq(&t788, &t789) { true } else { false } }) }).truthy() {
            {
                let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(20i64))?;
                v_j1 = py::float(&ax_engine::pymp::call("besselj", &[Py::Int(1i64), c_J01()?], &[])?)?;
            }
            let t790 = (Py::tuple(vec![py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &py::pow(&py::getitem(&v_dims.clone(), &Py::Int(0i64))?, &Py::Int(2i64))?)?, &py::getitem(&v_dims.clone(), &Py::Int(1i64))?)?, py::truediv(&py::mul(&py::truediv(&py::mul(&Py::Int(2i64), &v_j1.clone())?, &c_J01()?)?, &Py::Int(2i64))?, &Py::Float(std::f64::consts::PI))?])).items()?;
            if t790.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_V = t790[0].clone();
            v_f = t790[1].clone();
        } else {
            let t791 = (Py::tuple(vec![py::mul(&py::mul(&py::getitem(&v_dims.clone(), &Py::Int(0i64))?, &py::getitem(&v_dims.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_dims.clone(), &Py::Int(2i64))?)?, py::pow(&py::truediv(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &Py::Int(3i64))?])).items()?;
            if t791.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_V = t791[0].clone();
            v_f = t791[1].clone();
        }
    }
    if ({ let t792 = v_P.clone(); Py::Bool({ let t793 = Py::None; if py::is(&t792, &t793) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the peak flux"), Py::tuple(vec![v_phi_max.clone()]))?;
        v_P = py::mul(&py::mul(&py::mul(&py::mul(&v_E_f.clone(), &v_Sigma_f.clone())?, &v_phi_max.clone())?, &v_f.clone())?, &v_V.clone())?;
    } else {
        let _ = f__pos(Py::str("the power"), Py::tuple(vec![v_P.clone()]))?;
        v_phi_max = py::truediv(&v_P.clone(), &py::mul(&py::mul(&py::mul(&v_E_f.clone(), &v_Sigma_f.clone())?, &v_f.clone())?, &v_V.clone())?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("phi_max"), v_phi_max.clone()), (Py::str("phi_avg"), py::mul(&v_phi_max.clone(), &v_f.clone())?), (Py::str("peaking"), py::truediv(&Py::Float(1.0), &v_f.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_count_rate_point_source(mut v_A: Py, mut v_y: Py, mut v_d: Py, mut v_a: Py, mut v_eps_int: Py) -> R {
    let mut v_g: Py = Py::None;
    let _ = f__pos(Py::str("the activity, the distance and the radius"), Py::tuple(vec![v_A.clone(), v_d.clone(), v_a.clone()]))?;
    let _ = f__frac(Py::str("the emission probability"), v_y.clone(), Py::Bool(true), Py::Bool(false))?;
    let _ = f__frac(Py::str("the intrinsic efficiency"), v_eps_int.clone(), Py::Bool(true), Py::Bool(false))?;
    v_g = py::truediv(&py::sub(&Py::Int(1i64), &py::truediv(&v_d.clone(), &py::hypot(&[v_d.clone(), v_a.clone()])?)?)?, &Py::Int(2i64))?;
    return Ok(f__out(Py::dict(vec![(Py::str("R"), py::mul(&py::mul(&py::mul(&v_A.clone(), &v_y.clone())?, &v_g.clone())?, &v_eps_int.clone())?), (Py::str("g"), v_g.clone()), (Py::str("R_cpm"), py::mul(&py::mul(&py::mul(&py::mul(&v_A.clone(), &v_y.clone())?, &v_g.clone())?, &v_eps_int.clone())?, &Py::Float(60.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_count_time_for_precision(mut v_Rg: Py, mut v_Rb: Py, mut v_r: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_sb: Py = Py::None;
    let mut v_sg: Py = Py::None;
    let mut v_tg: Py = Py::None;
    let _ = f__pos(Py::str("the count rates"), Py::tuple(vec![v_Rg.clone(), v_Rb.clone()]))?;
    let _ = f__frac(Py::str("the target relative standard deviation r"), v_r.clone(), Py::Bool(true), Py::Bool(true))?;
    let _ = f__need({ let t900 = v_Rg.clone(); Py::Bool({ let t901 = v_Rb.clone(); if py::gt(&t900, &t901)? { true } else { false } }) }, Py::str("the gross count rate must exceed the background rate"))?;
    let t902 = (Py::tuple(vec![py::math1("sqrt", &v_Rg.clone())?, py::math1("sqrt", &v_Rb.clone())?])).items()?;
    if t902.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sg = t902[0].clone();
    v_sb = t902[1].clone();
    v_T = py::truediv(&py::pow(&py::add(&v_sg.clone(), &v_sb.clone())?, &Py::Int(2i64))?, &py::mul(&py::mul(&py::pow(&py::sub(&v_Rg.clone(), &v_Rb.clone())?, &Py::Int(2i64))?, &v_r.clone())?, &v_r.clone())?)?;
    v_tg = py::truediv(&py::mul(&v_T.clone(), &v_sg.clone())?, &py::add(&v_sg.clone(), &v_sb.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("T"), v_T.clone()), (Py::str("tg"), v_tg.clone()), (Py::str("tb"), py::sub(&v_T.clone(), &v_tg.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_count_time_split(mut v_T: Py, mut v_Rg: Py, mut v_Rb: Py) -> R {
    let mut v_sb: Py = Py::None;
    let mut v_sg: Py = Py::None;
    let mut v_sig: Py = Py::None;
    let mut v_tb: Py = Py::None;
    let mut v_tg: Py = Py::None;
    let _ = f__pos(Py::str("the total time and the count rates"), Py::tuple(vec![v_T.clone(), v_Rg.clone(), v_Rb.clone()]))?;
    let _ = f__need({ let t947 = v_Rg.clone(); Py::Bool({ let t948 = v_Rb.clone(); if py::gt(&t947, &t948)? { true } else { false } }) }, Py::str("the gross count rate must exceed the background rate"))?;
    let t949 = (Py::tuple(vec![py::math1("sqrt", &v_Rg.clone())?, py::math1("sqrt", &v_Rb.clone())?])).items()?;
    if t949.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sg = t949[0].clone();
    v_sb = t949[1].clone();
    v_tg = py::truediv(&py::mul(&v_T.clone(), &v_sg.clone())?, &py::add(&v_sg.clone(), &v_sb.clone())?)?;
    v_tb = py::sub(&v_T.clone(), &v_tg.clone())?;
    v_sig = py::truediv(&py::add(&v_sg.clone(), &v_sb.clone())?, &py::math1("sqrt", &v_T.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("tg"), v_tg.clone()), (Py::str("tb"), v_tb.clone()), (Py::str("sigma"), v_sig.clone()), (Py::str("rel"), py::truediv(&v_sig.clone(), &py::sub(&v_Rg.clone(), &v_Rb.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_counts_during_decay(mut v_A0: Py, mut v_T_half: Py, mut v_tc: Py, mut v_eff: Py, mut v_y: Py, mut v_td: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_decays: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__pos(Py::str("the activity A0"), Py::tuple(vec![v_A0.clone()]))?;
    let _ = f__pos(Py::str("the half-life and the counting time"), Py::tuple(vec![v_T_half.clone(), v_tc.clone()]))?;
    let _ = f__nonneg(Py::str("the delay td"), Py::tuple(vec![v_td.clone()]))?;
    let _ = f__frac(Py::str("the efficiency"), v_eff.clone(), Py::Bool(true), Py::Bool(false))?;
    let _ = f__frac(Py::str("the emission probability"), v_y.clone(), Py::Bool(true), Py::Bool(false))?;
    v_lam = py::truediv(&c_LN2()?, &v_T_half.clone())?;
    v_x = py::mul(&v_lam.clone(), &v_tc.clone())?;
    v_decays = py::truediv(&py::mul(&py::mul(&v_A0.clone(), &py::math1("exp", &py::mul(&py::neg(&v_lam.clone())?, &v_td.clone())?)?)?, &py::neg(&py::math1("expm1", &py::neg(&v_x.clone())?)?)?)?, &v_lam.clone())?;
    v_N = py::mul(&py::mul(&v_eff.clone(), &v_y.clone())?, &v_decays.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("N"), v_N.clone()), (Py::str("A_mean"), py::truediv(&v_decays.clone(), &v_tc.clone())?), (Py::str("F"), py::truediv(&v_x.clone(), &py::neg(&py::math1("expm1", &py::neg(&v_x.clone())?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_critical_cuboid(mut v_D: Py, mut v_Sigma_a: Py, mut v_nuSigma_f: Py, mut v_extrapolation: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_Bm2: Py = Py::None;
    let mut v_Bz2: Py = Py::None;
    let mut v_ce: Py = Py::None;
    let mut v_d: Py = Py::None;
    v_Bm2 = f__group1(v_D.clone(), v_Sigma_a.clone(), v_nuSigma_f.clone())?;
    let _ = f__need({ let t1051 = v_Bm2.clone(); Py::Bool({ let t1052 = Py::Int(0i64); if py::gt(&t1051, &t1052)? { true } else { false } }) }, Py::str("k-infinity <= 1: the medium cannot be critical at any size"))?;
    let _ = f__pos(Py::str("the sides a and b"), Py::tuple(vec![v_a.clone(), v_b.clone()]))?;
    v_d = py::mul(&py::getitem(&c_EXTRAP()?, &py::method(&v_extrapolation.clone(), "lower", &[])?)?, &v_D.clone())?;
    v_Bz2 = py::sub(&py::sub(&v_Bm2.clone(), &py::pow(&py::truediv(&Py::Float(std::f64::consts::PI), &py::add(&v_a.clone(), &py::mul(&Py::Int(2i64), &v_d.clone())?)?)?, &Py::Int(2i64))?)?, &py::pow(&py::truediv(&Py::Float(std::f64::consts::PI), &py::add(&v_b.clone(), &py::mul(&Py::Int(2i64), &v_d.clone())?)?)?, &Py::Int(2i64))?)?;
    let _ = f__need({ let t1053 = v_Bz2.clone(); Py::Bool({ let t1054 = Py::Int(0i64); if py::gt(&t1053, &t1054)? { true } else { false } }) }, Py::str("these two sides are too small: no third side makes the box critical"))?;
    v_ce = py::truediv(&Py::Float(std::f64::consts::PI), &py::math1("sqrt", &v_Bz2.clone())?)?;
    let _ = f__need({ let t1055 = py::sub(&v_ce.clone(), &py::mul(&Py::Int(2i64), &v_d.clone())?)?; Py::Bool({ let t1056 = Py::Int(0i64); if py::gt(&t1055, &t1056)? { true } else { false } }) }, Py::str("the extrapolation distance exceeds the critical size"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("c"), py::sub(&v_ce.clone(), &py::mul(&Py::Int(2i64), &v_d.clone())?)?), (Py::str("c_ext"), v_ce.clone()), (Py::str("V"), py::mul(&py::mul(&v_a.clone(), &v_b.clone())?, &py::sub(&v_ce.clone(), &py::mul(&Py::Int(2i64), &v_d.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_critical_cylinder(mut v_D: Py, mut v_Sigma_a: Py, mut v_nuSigma_f: Py, mut v_extrapolation: Py, mut v_H: Py) -> R {
    let mut v_Bm2: Py = Py::None;
    let mut v_Br2: Py = Py::None;
    let mut v_He: Py = Py::None;
    let mut v_Re: Py = Py::None;
    let mut v_d: Py = Py::None;
    v_Bm2 = f__group1(v_D.clone(), v_Sigma_a.clone(), v_nuSigma_f.clone())?;
    let _ = f__need({ let t1101 = v_Bm2.clone(); Py::Bool({ let t1102 = Py::Int(0i64); if py::gt(&t1101, &t1102)? { true } else { false } }) }, Py::str("k-infinity <= 1: the medium cannot be critical at any size"))?;
    let _ = f__pos(Py::str("the height H"), Py::tuple(vec![v_H.clone()]))?;
    v_d = py::mul(&py::getitem(&c_EXTRAP()?, &py::method(&v_extrapolation.clone(), "lower", &[])?)?, &v_D.clone())?;
    v_He = py::add(&v_H.clone(), &py::mul(&Py::Int(2i64), &v_d.clone())?)?;
    v_Br2 = py::sub(&v_Bm2.clone(), &py::pow(&py::truediv(&Py::Float(std::f64::consts::PI), &v_He.clone())?, &Py::Int(2i64))?)?;
    let _ = f__need({ let t1103 = v_Br2.clone(); Py::Bool({ let t1104 = Py::Int(0i64); if py::gt(&t1103, &t1104)? { true } else { false } }) }, Py::str("this height is too small: no radius makes the cylinder critical"))?;
    v_Re = py::truediv(&c_J01()?, &py::math1("sqrt", &v_Br2.clone())?)?;
    let _ = f__need({ let t1105 = py::sub(&v_Re.clone(), &v_d.clone())?; Py::Bool({ let t1106 = Py::Int(0i64); if py::gt(&t1105, &t1106)? { true } else { false } }) }, Py::str("the extrapolation distance exceeds the critical radius"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("R"), py::sub(&v_Re.clone(), &v_d.clone())?), (Py::str("R_ext"), v_Re.clone()), (Py::str("V"), py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &py::pow(&py::sub(&v_Re.clone(), &v_d.clone())?, &Py::Int(2i64))?)?, &v_H.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_critical_cylinder_min(mut v_D: Py, mut v_Sigma_a: Py, mut v_nuSigma_f: Py, mut v_extrapolation: Py) -> R {
    let mut v_Bm: Py = Py::None;
    let mut v_Bm2: Py = Py::None;
    let mut v_He: Py = Py::None;
    let mut v_Re: Py = Py::None;
    let mut v_d: Py = Py::None;
    v_Bm2 = f__group1(v_D.clone(), v_Sigma_a.clone(), v_nuSigma_f.clone())?;
    let _ = f__need({ let t1151 = v_Bm2.clone(); Py::Bool({ let t1152 = Py::Int(0i64); if py::gt(&t1151, &t1152)? { true } else { false } }) }, Py::str("k-infinity <= 1: the medium cannot be critical at any size"))?;
    v_Bm = py::math1("sqrt", &v_Bm2.clone())?;
    v_d = py::mul(&py::getitem(&c_EXTRAP()?, &py::method(&v_extrapolation.clone(), "lower", &[])?)?, &v_D.clone())?;
    v_He = py::truediv(&py::mul(&py::math1("sqrt", &Py::Float(3.0))?, &Py::Float(std::f64::consts::PI))?, &v_Bm.clone())?;
    v_Re = py::truediv(&py::mul(&py::math1("sqrt", &Py::Float(1.5))?, &c_J01()?)?, &v_Bm.clone())?;
    let _ = f__need({ let t1153 = py::sub(&v_Re.clone(), &v_d.clone())?; Py::Bool({ let t1154 = Py::Int(0i64); if py::gt(&t1153, &t1154)? { true } else { false } }) }, Py::str("the extrapolation distance exceeds the critical radius"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("R"), py::sub(&v_Re.clone(), &v_d.clone())?), (Py::str("H"), py::sub(&v_He.clone(), &py::mul(&Py::Int(2i64), &v_d.clone())?)?), (Py::str("V_ext"), py::mul(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_Re.clone())?, &v_Re.clone())?, &v_He.clone())?), (Py::str("HD"), py::truediv(&v_He.clone(), &py::mul(&Py::Int(2i64), &v_Re.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_critical_slab(mut v_D: Py, mut v_Sigma_a: Py, mut v_nuSigma_f: Py, mut v_extrapolation: Py) -> R {
    let mut v_Bm2: Py = Py::None;
    let mut v_ae: Py = Py::None;
    let mut v_d: Py = Py::None;
    v_Bm2 = f__group1(v_D.clone(), v_Sigma_a.clone(), v_nuSigma_f.clone())?;
    let _ = f__need({ let t1199 = v_Bm2.clone(); Py::Bool({ let t1200 = Py::Int(0i64); if py::gt(&t1199, &t1200)? { true } else { false } }) }, Py::str("k-infinity <= 1: the medium cannot be critical at any size"))?;
    v_d = py::mul(&py::getitem(&c_EXTRAP()?, &py::method(&v_extrapolation.clone(), "lower", &[])?)?, &v_D.clone())?;
    v_ae = py::truediv(&Py::Float(std::f64::consts::PI), &py::math1("sqrt", &v_Bm2.clone())?)?;
    let _ = f__need({ let t1201 = py::sub(&v_ae.clone(), &py::mul(&Py::Int(2i64), &v_d.clone())?)?; Py::Bool({ let t1202 = Py::Int(0i64); if py::gt(&t1201, &t1202)? { true } else { false } }) }, Py::str("the extrapolation distance exceeds the critical size"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("a"), py::sub(&v_ae.clone(), &py::mul(&Py::Int(2i64), &v_d.clone())?)?), (Py::str("a_ext"), v_ae.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_critical_sphere(mut v_D: Py, mut v_Sigma_a: Py, mut v_nuSigma_f: Py, mut v_extrapolation: Py) -> R {
    let mut v_Bm2: Py = Py::None;
    let mut v_Re: Py = Py::None;
    let mut v_d: Py = Py::None;
    v_Bm2 = f__group1(v_D.clone(), v_Sigma_a.clone(), v_nuSigma_f.clone())?;
    let _ = f__need({ let t1247 = v_Bm2.clone(); Py::Bool({ let t1248 = Py::Int(0i64); if py::gt(&t1247, &t1248)? { true } else { false } }) }, Py::str("k-infinity <= 1: the medium cannot be critical at any size"))?;
    v_d = py::mul(&py::getitem(&c_EXTRAP()?, &py::method(&v_extrapolation.clone(), "lower", &[])?)?, &v_D.clone())?;
    v_Re = py::truediv(&Py::Float(std::f64::consts::PI), &py::math1("sqrt", &v_Bm2.clone())?)?;
    let _ = f__need({ let t1249 = py::sub(&v_Re.clone(), &v_d.clone())?; Py::Bool({ let t1250 = Py::Int(0i64); if py::gt(&t1249, &t1250)? { true } else { false } }) }, Py::str("the extrapolation distance exceeds the critical radius"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("R"), py::sub(&v_Re.clone(), &v_d.clone())?), (Py::str("R_ext"), v_Re.clone()), (Py::str("V"), py::mul(&py::mul(&py::truediv(&Py::Float(4.0), &Py::Float(3.0))?, &Py::Float(std::f64::consts::PI))?, &py::pow(&py::sub(&v_Re.clone(), &v_d.clone())?, &Py::Int(3i64))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_currie(mut v_B: Py, mut v_t: Py, mut v_eff: Py, mut v_y: Py) -> R {
    let mut v_LC: Py = Py::None;
    let mut v_LD: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__need({ let t1293 = f__isnum(v_B.clone())?; if !t1293.truthy() { t1293 } else { { let t1291 = v_B.clone(); Py::Bool({ let t1292 = Py::Int(0i64); if py::ge(&t1291, &t1292)? { true } else { false } }) } } }, Py::str("the background counts must be zero or positive"))?;
    v_k = c__K95()?;
    v_LC = py::mul(&v_k.clone(), &py::math1("sqrt", &py::mul(&Py::Float(2.0), &v_B.clone())?)?)?;
    v_LD = py::add(&py::mul(&v_k.clone(), &v_k.clone())?, &py::mul(&Py::Float(2.0), &v_LC.clone())?)?;
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("L_C"), v_LC.clone()), (Py::str("L_D"), v_LD.clone())])?;
    if ({ let t1294 = { let t1296 = v_t.clone(); Py::Bool({ let t1297 = Py::None; if py::is(&t1296, &t1297) { true } else { false } }) }; Py::Bool({ let t1295 = { let t1298 = v_eff.clone(); Py::Bool({ let t1299 = Py::None; if py::is(&t1298, &t1299) { true } else { false } }) }; if py::ne(&t1294, &t1295) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the minimum detectable activity needs both the counting time t and the efficiency eff"))));
    }
    if ({ let t1300 = v_t.clone(); Py::Bool({ let t1301 = Py::None; if !py::is(&t1300, &t1301) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("t"), Py::tuple(vec![v_t.clone()]))?;
        let _ = f__need({ let t1312 = { let t1306 = Py::Int(0i64); Py::Bool({ let t1307 = v_eff.clone(); if py::lt(&t1306, &t1307)? { { let t1308 = Py::Int(1i64); if py::le(&t1307, &t1308)? { true } else { false } } } else { false } }) }; if !t1312.truthy() { t1312 } else { { let t1309 = Py::Int(0i64); Py::Bool({ let t1310 = v_y.clone(); if py::lt(&t1309, &t1310)? { { let t1311 = Py::Int(1i64); if py::le(&t1310, &t1311)? { true } else { false } } } else { false } }) } } }, Py::str("the efficiency and the emission yield must lie in (0, 1]"))?;
        py::setitem(&v_out.clone(), &Py::str("MDA"), py::truediv(&v_LD.clone(), &py::mul(&py::mul(&v_eff.clone(), &v_y.clone())?, &v_t.clone())?)?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_daughter_max_time(mut v_T_P: Py, mut v_T_D: Py, mut v_A_P0: Py) -> R {
    let mut v_ld: Py = Py::None;
    let mut v_lp: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_ratio: Py = Py::None;
    let mut v_t: Py = Py::None;
    let _ = f__pos(Py::str("the half-lives"), Py::tuple(vec![v_T_P.clone(), v_T_D.clone()]))?;
    let t1358 = (Py::tuple(vec![py::truediv(&c_LN2()?, &v_T_P.clone())?, py::truediv(&c_LN2()?, &v_T_D.clone())?])).items()?;
    if t1358.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lp = t1358[0].clone();
    v_ld = t1358[1].clone();
    v_t = f__tmax(v_lp.clone(), v_ld.clone())?;
    v_ratio = py::math1("exp", &py::mul(&py::neg(&v_lp.clone())?, &v_t.clone())?)?;
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("t_max"), v_t.clone()), (Py::str("ratio_max"), v_ratio.clone()), (Py::str("t_max_half_lives"), py::truediv(&v_t.clone(), &v_T_D.clone())?)])?;
    if ({ let t1361 = v_A_P0.clone(); Py::Bool({ let t1362 = Py::None; if !py::is(&t1361, &t1362) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the initial parent activity A_P0"), Py::tuple(vec![v_A_P0.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("A_D_max"), py::mul(&v_A_P0.clone(), &v_ratio.clone())?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dead_time_nonparalyzable(mut v_m: Py, mut v_tau: Py) -> R {
    let mut v_n: Py = Py::None;
    let _ = f__pos(Py::str("the observed count rate and the dead time"), Py::tuple(vec![v_m.clone(), v_tau.clone()]))?;
    let _ = f__need({ let t1408 = py::mul(&v_m.clone(), &v_tau.clone())?; Py::Bool({ let t1409 = Py::Int(1i64); if py::lt(&t1408, &t1409)? { true } else { false } }) }, Py::str("m tau must be below 1 for a non-paralyzable detector"))?;
    v_n = py::truediv(&v_m.clone(), &py::sub(&Py::Float(1.0), &py::mul(&v_m.clone(), &v_tau.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("n"), v_n.clone()), (Py::str("loss"), py::mul(&v_m.clone(), &v_tau.clone())?), (Py::str("corr"), py::truediv(&v_n.clone(), &v_m.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dead_time_paralyzable(mut v_m: Py, mut v_tau: Py, mut v_branch: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_ly: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let _ = f__pos(Py::str("the observed count rate and the dead time"), Py::tuple(vec![v_m.clone(), v_tau.clone()]))?;
    let _ = f__need({ let t1454 = v_branch.clone(); Py::Bool({ let t1455 = Py::tuple(vec![Py::str("low"), Py::str("high")]); if py::contains(&t1455, &t1454)? { true } else { false } }) }, Py::str("branch must be low (n tau < 1) or high (n tau > 1)"))?;
    v_y = py::mul(&v_m.clone(), &v_tau.clone())?;
    let _ = f__need({ let t1456 = v_y.clone(); Py::Bool({ let t1457 = py::math1("exp", &py::neg(&Py::Float(1.0))?)?; if py::le(&t1456, &t1457)? { true } else { false } }) }, Py::str("m tau above 1/e is impossible for a paralyzable detector"))?;
    v_ly = py::math1("log", &v_y.clone())?;
    v_g = { let v_branch = v_branch.clone(); let v_ly = v_ly.clone(); let v_m = v_m.clone(); let v_tau = v_tau.clone(); let v_y = v_y.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::sub(&py::math1("log", &v_x.clone())?, &v_x.clone())?, &v_ly.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    if ({ let t1458 = v_branch.clone(); Py::Bool({ let t1459 = Py::str("low"); if py::eq(&t1458, &t1459) { true } else { false } }) }).truthy() {
        v_x = f__bisect(v_g.clone(), v_y.clone(), Py::Float(1.0), Py::Int(400i64))?;
    } else {
        v_x = f__bisect(v_g.clone(), Py::Float(1.0), py::add(&py::mul(&py::neg(&Py::Float(2.0))?, &v_ly.clone())?, &Py::Float(10.0))?, Py::Int(400i64))?;
    }
    v_n = py::truediv(&v_x.clone(), &v_tau.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("n"), v_n.clone()), (Py::str("loss"), py::sub(&Py::Float(1.0), &py::truediv(&v_m.clone(), &v_n.clone())?)?), (Py::str("m_max"), py::truediv(&Py::Float(1.0), &py::mul(&Py::Float(std::f64::consts::E), &v_tau.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_decay_fit(mut v_t: Py, mut v_C: Py, mut v_t_unit: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_St: Py = Py::None;
    let mut v_Stt: Py = Py::None;
    let mut v_Sty: Py = Py::None;
    let mut v_Sy: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_det: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_sig_l: Py = Py::None;
    let mut v_ts: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__need({ let t1524 = v_t_unit.clone(); Py::Bool({ let t1525 = c__TU()?; if py::contains(&t1525, &t1524)? { true } else { false } }) }, Py::str("t_unit must be s, min, h, d or y"))?;
    let _ = f__lists(v_t.clone(), v_C.clone(), Py::Int(3i64))?;
    v_ts = { let mut out = vec![]; for t1537 in py::iter(&(v_t.clone()))? { let mut v_x: Py; 
v_x = t1537;
out.push(py::mul(&v_x.clone(), &py::getitem(&c__TU()?, &v_t_unit.clone())?)?); } Py::list(out) };
    v_w = Py::list((v_C.clone()).items()?);
    let t1542 = (Py::tuple(vec![py::fsum(&v_w.clone())?, py::fsum(&{ let mut out = vec![]; for t1538 in py::iter(&(py::zip(&[v_w.clone(), v_ts.clone()])?))? { let mut v_wi: Py; let mut v_x: Py; 
let t1539 = (t1538).items()?;
if t1539.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_wi = t1539[0].clone();
v_x = t1539[1].clone();
out.push(py::mul(&v_wi.clone(), &v_x.clone())?); } Py::list(out) })?, py::fsum(&{ let mut out = vec![]; for t1540 in py::iter(&(py::zip(&[v_w.clone(), v_ts.clone()])?))? { let mut v_wi: Py; let mut v_x: Py; 
let t1541 = (t1540).items()?;
if t1541.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_wi = t1541[0].clone();
v_x = t1541[1].clone();
out.push(py::mul(&py::mul(&v_wi.clone(), &v_x.clone())?, &v_x.clone())?); } Py::list(out) })?])).items()?;
    if t1542.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_S = t1542[0].clone();
    v_St = t1542[1].clone();
    v_Stt = t1542[2].clone();
    v_Sy = py::fsum(&{ let mut out = vec![]; for t1543 in py::iter(&(py::zip(&[v_w.clone(), v_C.clone()])?))? { let mut v_c: Py; let mut v_wi: Py; 
let t1544 = (t1543).items()?;
if t1544.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_wi = t1544[0].clone();
v_c = t1544[1].clone();
out.push(py::mul(&v_wi.clone(), &py::math1("log", &v_c.clone())?)?); } Py::list(out) })?;
    v_Sty = py::fsum(&{ let mut out = vec![]; for t1545 in py::iter(&(py::zip(&[v_w.clone(), v_ts.clone(), v_C.clone()])?))? { let mut v_c: Py; let mut v_wi: Py; let mut v_x: Py; 
let t1546 = (t1545).items()?;
if t1546.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_wi = t1546[0].clone();
v_x = t1546[1].clone();
v_c = t1546[2].clone();
out.push(py::mul(&py::mul(&v_wi.clone(), &v_x.clone())?, &py::math1("log", &v_c.clone())?)?); } Py::list(out) })?;
    v_det = py::sub(&py::mul(&v_S.clone(), &v_Stt.clone())?, &py::mul(&v_St.clone(), &v_St.clone())?)?;
    let _ = f__need({ let t1547 = v_det.clone(); Py::Bool({ let t1548 = Py::Int(0i64); if py::gt(&t1547, &t1548)? { true } else { false } }) }, Py::str("the times do not determine a slope"))?;
    v_b = py::truediv(&py::sub(&py::mul(&v_S.clone(), &v_Sty.clone())?, &py::mul(&v_St.clone(), &v_Sy.clone())?)?, &v_det.clone())?;
    v_a = py::truediv(&py::sub(&py::mul(&v_Stt.clone(), &v_Sy.clone())?, &py::mul(&v_St.clone(), &v_Sty.clone())?)?, &v_det.clone())?;
    v_lam = py::neg(&v_b.clone())?;
    let _ = f__need({ let t1549 = v_lam.clone(); Py::Bool({ let t1550 = Py::Int(0i64); if py::gt(&t1549, &t1550)? { true } else { false } }) }, Py::str("the counts do not decrease: no decay constant can be fitted"))?;
    v_sig_l = py::math1("sqrt", &py::truediv(&v_S.clone(), &v_det.clone())?)?;
    v_T = py::truediv(&c_LN2()?, &v_lam.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("lambda_"), v_lam.clone()), (Py::str("T_half"), v_T.clone()), (Py::str("C0"), py::math1("exp", &v_a.clone())?), (Py::str("sigma_T"), py::truediv(&py::mul(&v_T.clone(), &v_sig_l.clone())?, &v_lam.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_decay_heat(mut v_P0: Py, mut v_T_op: Py, mut v_t: Py) -> R {
    let mut v_ratio: Py = Py::None;
    let _ = f__ww_check(v_P0.clone(), v_T_op.clone(), Py::tuple(vec![v_t.clone()]))?;
    v_ratio = py::mul(&c__WW()?, &py::sub(&py::pow(&v_t.clone(), &py::neg(&Py::Float(0.2))?)?, &py::pow(&py::add(&v_t.clone(), &v_T_op.clone())?, &py::neg(&Py::Float(0.2))?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("P_d"), py::mul(&v_P0.clone(), &v_ratio.clone())?), (Py::str("ratio"), v_ratio.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_decay_heat_energy(mut v_P0: Py, mut v_T_op: Py, mut v_t1: Py, mut v_t2: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_F: Py = Py::None;
    let _ = f__ww_check(v_P0.clone(), v_T_op.clone(), Py::tuple(vec![v_t1.clone(), v_t2.clone()]))?;
    let _ = f__need({ let t1600 = v_t2.clone(); Py::Bool({ let t1601 = v_t1.clone(); if py::gt(&t1600, &t1601)? { true } else { false } }) }, Py::str("t2 must be later than t1"))?;
    v_F = { let v_P0 = v_P0.clone(); let v_T_op = v_T_op.clone(); let v_t1 = v_t1.clone(); let v_t2 = v_t2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::sub(&py::pow(&v_t.clone(), &Py::Float(0.8))?, &py::pow(&py::add(&v_t.clone(), &v_T_op.clone())?, &Py::Float(0.8))?)?, &Py::Float(0.8))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_E = py::mul(&py::mul(&c__WW()?, &v_P0.clone())?, &py::sub(&py::call(&v_F.clone(), &[v_t2.clone()])?, &py::call(&v_F.clone(), &[v_t1.clone()])?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("E"), v_E.clone()), (Py::str("E_MWh"), py::truediv(&v_E.clone(), &Py::Float(3600000000.0))?), (Py::str("P_mean"), py::truediv(&v_E.clone(), &py::sub(&v_t2.clone(), &v_t1.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_depletion_constant_flux(mut v_m0: Py, mut v_sigma_a: Py, mut v_sigma_f: Py, mut v_phi: Py, mut v_t: Py, mut v_E_f: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_N0: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__pos(Py::str("m0, the cross sections, the flux and E_f"), Py::tuple(vec![v_m0.clone(), v_sigma_a.clone(), v_sigma_f.clone(), v_phi.clone(), v_E_f.clone()]))?;
    let _ = f__need({ let t1695 = v_sigma_f.clone(); Py::Bool({ let t1696 = v_sigma_a.clone(); if py::le(&t1695, &t1696)? { true } else { false } }) }, Py::str("the fission cross section cannot exceed the absorption cross section"))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    v_N0 = py::mul(&py::truediv(&py::mul(&v_m0.clone(), &Py::Float(1000.0))?, &c__M5()?)?, &c_NA()?)?;
    v_x = py::mul(&py::mul(&v_sigma_a.clone(), &v_phi.clone())?, &v_t.clone())?;
    v_f = py::math1("exp", &py::neg(&v_x.clone())?)?;
    v_E = py::mul(&py::mul(&py::truediv(&py::mul(&v_E_f.clone(), &v_sigma_f.clone())?, &v_sigma_a.clone())?, &v_N0.clone())?, &py::neg(&py::math1("expm1", &py::neg(&v_x.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("m"), py::mul(&v_m0.clone(), &v_f.clone())?), (Py::str("burnt"), py::neg(&py::math1("expm1", &py::neg(&v_x.clone())?)?)?), (Py::str("E"), v_E.clone()), (Py::str("E_MWd"), py::truediv(&v_E.clone(), &py::mul(&Py::Float(1000000.0), &c_DAY()?)?)?), (Py::str("P_t"), py::mul(&py::mul(&py::mul(&py::mul(&v_E_f.clone(), &v_sigma_f.clone())?, &v_phi.clone())?, &v_N0.clone())?, &v_f.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_detector_efficiency(mut v_d: Py, mut v_a: Py, mut v_t: Py, mut v_mu: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_c0: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let _ = f__pos(Py::str("the distance, radius, thickness and attenuation coefficient"), Py::tuple(vec![v_d.clone(), v_a.clone(), v_t.clone(), v_mu.clone()]))?;
    v_c0 = py::truediv(&v_d.clone(), &py::hypot(&[v_d.clone(), v_a.clone()])?)?;
    v_b = py::mul(&v_mu.clone(), &v_t.clone())?;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(20i64))?;
        v_tot = py::float(&ax_engine::pymp::call("quad", &[{ let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c0 = v_c0.clone(); let v_d = v_d.clone(); let v_mu = v_mu.clone(); let v_t = v_t.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::sub(&Py::Int(1i64), &ax_engine::pymp::call("exp", &[py::truediv(&py::neg(&v_b.clone())?, &v_u.clone())?], &[])?)?, &Py::Int(2i64))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::list(vec![v_c0.clone(), Py::Int(1i64)])], &[])?)?;
    }
    v_g = py::truediv(&py::sub(&Py::Int(1i64), &v_c0.clone())?, &Py::Int(2i64))?;
    return Ok(f__out(Py::dict(vec![(Py::str("eff"), v_tot.clone()), (Py::str("g"), v_g.clone()), (Py::str("intrinsic"), py::truediv(&v_tot.clone(), &v_g.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_diffusion_length(mut v_D: Py, mut v_Sigma_a: Py, mut v_tau: Py) -> R {
    let mut v_L2: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__pos(Py::str("D and Sigma_a"), Py::tuple(vec![v_D.clone(), v_Sigma_a.clone()]))?;
    v_L2 = py::truediv(&v_D.clone(), &v_Sigma_a.clone())?;
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("L"), py::math1("sqrt", &v_L2.clone())?), (Py::str("L2"), v_L2.clone()), (Py::str("L_cm"), py::mul(&py::math1("sqrt", &v_L2.clone())?, &Py::Int(100i64))?)])?;
    if ({ let t1789 = v_tau.clone(); Py::Bool({ let t1790 = Py::None; if !py::is(&t1789, &t1790) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the Fermi age"), Py::tuple(vec![v_tau.clone()]))?;
        let _ = { let t1791 = v_out.clone(); py::setitem(&t1791, &Py::str("M2"), py::add(&v_L2.clone(), &v_tau.clone())?)?; py::setitem(&t1791, &Py::str("M"), py::math1("sqrt", &py::add(&v_L2.clone(), &v_tau.clone())?)?)?; Py::None };
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_disk_source(mut v_S_A: Py, mut v_R: Py, mut v_h: Py, mut v_mu: Py, mut v_t: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let mut v_phi0: Py = Py::None;
    let mut v_sec: Py = Py::None;
    let _ = f__pos(Py::str("the source strength, the disk radius and the distance"), Py::tuple(vec![v_S_A.clone(), v_R.clone(), v_h.clone()]))?;
    let _ = f__nonneg(Py::str("mu and the shield thickness"), Py::tuple(vec![v_mu.clone(), v_t.clone()]))?;
    let _ = f__need({ let t1841 = v_t.clone(); Py::Bool({ let t1842 = v_h.clone(); if py::lt(&t1841, &t1842)? { true } else { false } }) }, Py::str("the shield must lie between the disk and the point (t < h)"))?;
    v_b = py::mul(&v_mu.clone(), &v_t.clone())?;
    v_sec = py::truediv(&py::math1("sqrt", &py::add(&py::mul(&v_h.clone(), &v_h.clone())?, &py::mul(&v_R.clone(), &v_R.clone())?)?)?, &v_h.clone())?;
    v_phi0 = py::mul(&py::truediv(&v_S_A.clone(), &Py::Float(4.0))?, &py::math1("log1p", &py::pow(&py::truediv(&v_R.clone(), &v_h.clone())?, &Py::Int(2i64))?)?)?;
    if ({ let t1843 = v_b.clone(); Py::Bool({ let t1844 = Py::Int(0i64); if py::eq(&t1843, &t1844) { true } else { false } }) }).truthy() {
        v_phi = v_phi0.clone();
    } else {
        {
            let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(25i64))?;
            v_phi = py::float(&py::mul(&py::truediv(&v_S_A.clone(), &Py::Float(2.0))?, &py::sub(&ax_engine::pymp::call("e1", &[v_b.clone()], &[])?, &ax_engine::pymp::call("e1", &[py::mul(&v_b.clone(), &v_sec.clone())?], &[])?)?)?)?;
        }
    }
    return Ok(f__out(Py::dict(vec![(Py::str("phi"), v_phi.clone()), (Py::str("phi0"), v_phi0.clone()), (Py::str("T"), py::truediv(&v_phi.clone(), &v_phi0.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_effective_dose(mut v_H: Py, mut v_w: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_sw: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_n = py::len(&v_H.clone())?;
    let _ = f__need({ let t1888 = { let t1883 = Py::Int(1i64); Py::Bool({ let t1884 = v_n.clone(); if py::le(&t1883, &t1884)? { { let t1885 = Py::Int(30i64); if py::le(&t1884, &t1885)? { true } else { false } } } else { false } }) }; if !t1888.truthy() { t1888 } else { { let t1886 = py::len(&v_w.clone())?; Py::Bool({ let t1887 = v_n.clone(); if py::eq(&t1886, &t1887) { true } else { false } }) } } }, Py::str("give one tissue weighting factor per equivalent dose (1 to 30 tissues)"))?;
    for t1889 in py::iter(&(v_H.clone()))? {
        v_x = t1889;
        let _ = f__need({ let t1894 = f__isnum(v_x.clone())?; if !t1894.truthy() { t1894 } else { { let t1892 = v_x.clone(); Py::Bool({ let t1893 = Py::Int(0i64); if py::ge(&t1892, &t1893)? { true } else { false } }) } } }, Py::str("every equivalent dose must be zero or positive"))?;
    }
    for t1895 in py::iter(&(v_w.clone()))? {
        v_x = t1895;
        let _ = f__need({ let t1899 = f__isnum(v_x.clone())?; if !t1899.truthy() { t1899 } else { { let t1896 = Py::Int(0i64); Py::Bool({ let t1897 = v_x.clone(); if py::lt(&t1896, &t1897)? { { let t1898 = Py::Int(1i64); if py::le(&t1897, &t1898)? { true } else { false } } } else { false } }) } } }, Py::str("every tissue weighting factor must lie in (0, 1]"))?;
    }
    v_sw = py::fsum(&v_w.clone())?;
    let _ = f__need({ let t1900 = v_sw.clone(); Py::Bool({ let t1901 = py::add(&Py::Int(1i64), &Py::Float(1e-09))?; if py::le(&t1900, &t1901)? { true } else { false } }) }, Py::str("the tissue weighting factors must not sum to more than 1"))?;
    v_E = py::fsum(&{ let mut out = vec![]; for t1902 in py::iter(&(py::zip(&[v_H.clone(), v_w.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1903 = (t1902).items()?;
if t1903.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1903[0].clone();
v_b = t1903[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?;
    return Ok(f__out(Py::dict(vec![(Py::str("E_mSv"), v_E.clone()), (Py::str("E"), py::mul(&v_E.clone(), &Py::Float(0.001))?), (Py::str("w_sum"), v_sw.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_effective_half_life(mut v_T_p: Py, mut v_T_b: Py, mut v_A0: Py) -> R {
    let mut v_Te: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__pos(Py::str("the half-lives"), Py::tuple(vec![v_T_p.clone(), v_T_b.clone()]))?;
    v_Te = py::truediv(&py::mul(&v_T_p.clone(), &v_T_b.clone())?, &py::add(&v_T_p.clone(), &v_T_b.clone())?)?;
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("T_eff"), v_Te.clone()), (Py::str("lambda_eff"), py::truediv(&c_LN2()?, &v_Te.clone())?)])?;
    if ({ let t1948 = v_A0.clone(); Py::Bool({ let t1949 = Py::None; if !py::is(&t1948, &t1949) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the initial activity A0"), Py::tuple(vec![v_A0.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("N_decays"), py::truediv(&py::mul(&v_A0.clone(), &v_Te.clone())?, &c_LN2()?)?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_elastic_scattering(mut v_A: Py, mut v_E: Py, mut v_theta: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_psi: Py = Py::None;
    let mut v_ratio: Py = Py::None;
    let _ = f__need({ let t1993 = f__isnum(v_A.clone())?; if !t1993.truthy() { t1993 } else { { let t1991 = v_A.clone(); Py::Bool({ let t1992 = Py::Int(1i64); if py::ge(&t1991, &t1992)? { true } else { false } }) } } }, Py::str("the mass ratio A must be at least 1"))?;
    let _ = f__pos(Py::str("the neutron energy"), Py::tuple(vec![v_E.clone()]))?;
    let _ = f__need({ let t2001 = f__isnum(v_theta.clone())?; if !t2001.truthy() { t2001 } else { { let t1998 = Py::Int(0i64); Py::Bool({ let t1999 = v_theta.clone(); if py::le(&t1998, &t1999)? { { let t2000 = Py::Float(std::f64::consts::PI); if py::le(&t1999, &t2000)? { true } else { false } } } else { false } }) } } }, Py::str("the centre-of-mass angle must lie between 0 and 180 degrees"))?;
    let _ = f__need(py::not(&{ let t2006 = { let t2002 = v_A.clone(); Py::Bool({ let t2003 = Py::Int(1i64); if py::eq(&t2002, &t2003) { true } else { false } }) }; if !t2006.truthy() { t2006 } else { { let t2004 = v_theta.clone(); Py::Bool({ let t2005 = Py::Float(std::f64::consts::PI); if py::eq(&t2004, &t2005) { true } else { false } }) } } }), Py::str("a head-on collision with A = 1 stops the neutron: no lab angle"))?;
    v_c = py::math1("cos", &v_theta.clone())?;
    v_ratio = py::truediv(&py::add(&py::add(&py::mul(&v_A.clone(), &v_A.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_A.clone())?, &v_c.clone())?)?, &Py::Int(1i64))?, &py::pow(&py::add(&v_A.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?)?;
    v_psi = py::atan2(&py::mul(&v_A.clone(), &py::math1("sin", &v_theta.clone())?)?, &py::add(&Py::Int(1i64), &py::mul(&v_A.clone(), &v_c.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("E_out"), py::mul(&v_E.clone(), &v_ratio.clone())?), (Py::str("ratio"), v_ratio.clone()), (Py::str("psi"), v_psi.clone()), (Py::str("du"), py::neg(&py::math1("log", &v_ratio.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_equilibrium_ratio(mut v_T_P: Py, mut v_T_D: Py) -> R {
    let mut v_ld: Py = Py::None;
    let mut v_lp: Py = Py::None;
    let _ = f__pos(Py::str("the half-lives"), Py::tuple(vec![v_T_P.clone(), v_T_D.clone()]))?;
    let _ = f__need({ let t2109 = v_T_P.clone(); Py::Bool({ let t2110 = v_T_D.clone(); if py::gt(&t2109, &t2110)? { true } else { false } }) }, Py::str("no equilibrium: the parent must live longer than the daughter (T_P > T_D)"))?;
    let t2111 = (Py::tuple(vec![py::truediv(&c_LN2()?, &v_T_P.clone())?, py::truediv(&c_LN2()?, &v_T_D.clone())?])).items()?;
    if t2111.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lp = t2111[0].clone();
    v_ld = t2111[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("ratio_A"), py::truediv(&v_T_P.clone(), &py::sub(&v_T_P.clone(), &v_T_D.clone())?)?), (Py::str("ratio_N"), py::truediv(&v_T_D.clone(), &py::sub(&v_T_P.clone(), &v_T_D.clone())?)?), (Py::str("t_99"), py::truediv(&py::math1("log", &Py::Float(100.0))?, &py::sub(&v_ld.clone(), &v_lp.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_escape_peaks(mut v_E: Py) -> R {
    let mut v_Em: Py = Py::None;
    let _ = f__pos(Py::str("the gamma-ray energy"), Py::tuple(vec![v_E.clone()]))?;
    v_Em = py::truediv(&v_E.clone(), &c_MEV()?)?;
    let _ = f__need({ let t2156 = v_Em.clone(); Py::Bool({ let t2157 = py::mul(&Py::Int(2i64), &c_ME_MEV()?)?; if py::gt(&t2156, &t2157)? { true } else { false } }) }, Py::str("escape peaks need pair production: the energy must exceed 1.022 MeV"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("single_escape_keV"), py::mul(&py::sub(&v_Em.clone(), &c_ME_MEV()?)?, &Py::Float(1000.0))?), (Py::str("double_escape_keV"), py::mul(&py::sub(&v_Em.clone(), &py::mul(&Py::Int(2i64), &c_ME_MEV()?)?)?, &Py::Float(1000.0))?), (Py::str("annihilation_keV"), py::mul(&c_ME_MEV()?, &Py::Float(1000.0))?), (Py::str("E_keV"), py::mul(&v_Em.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_exponential_pile(mut v_z: Py, mut v_phi: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_L2inv: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_mz: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_zs: Py = Py::None;
    let _ = f__need({ let t2200 = { let t2196 = py::len(&v_z.clone())?; Py::Bool({ let t2197 = py::len(&v_phi.clone())?; if py::eq(&t2196, &t2197) { true } else { false } }) }; if !t2200.truthy() { t2200 } else { { let t2198 = py::len(&v_z.clone())?; Py::Bool({ let t2199 = Py::Int(3i64); if py::ge(&t2198, &t2199)? { true } else { false } }) } } }, Py::str("give at least three positions with their fluxes"))?;
    let _ = f__need({ let t2206 = Py::Bool(({ let mut out = vec![]; for t2201 in py::iter(&(v_z.clone()))? { let mut v_v: Py; 
v_v = t2201;
out.push(f__isnum(v_v.clone())?); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t2206.truthy() { t2206 } else { { let t2204 = py::len(&py::set(&v_z.clone())?)?; Py::Bool({ let t2205 = py::len(&v_z.clone())?; if py::eq(&t2204, &t2205) { true } else { false } }) } } }, Py::str("positions must be distinct numbers"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t2207 in py::iter(&(v_phi.clone()))? { let mut v_p: Py; 
v_p = t2207;
out.push({ let t2210 = f__isnum(v_p.clone())?; if !t2210.truthy() { t2210 } else { { let t2208 = v_p.clone(); Py::Bool({ let t2209 = Py::Int(0i64); if py::gt(&t2208, &t2209)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("fluxes must be positive"))?;
    let _ = f__pos(Py::str("the transverse sides"), Py::tuple(vec![v_a.clone(), v_b.clone()]))?;
    v_zs = { let mut out = vec![]; for t2215 in py::iter(&(v_z.clone()))? { let mut v_v: Py; 
v_v = t2215;
out.push(py::truediv(&v_v.clone(), &Py::Float(100.0))?); } Py::list(out) };
    v_n = py::len(&v_zs.clone())?;
    v_y = { let mut out = vec![]; for t2216 in py::iter(&(v_phi.clone()))? { let mut v_p: Py; 
v_p = t2216;
out.push(py::math1("log", &v_p.clone())?); } Py::list(out) };
    let t2217 = (Py::tuple(vec![py::truediv(&py::fsum(&v_zs.clone())?, &v_n.clone())?, py::truediv(&py::fsum(&v_y.clone())?, &v_n.clone())?])).items()?;
    if t2217.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mz = t2217[0].clone();
    v_my = t2217[1].clone();
    v_s = py::truediv(&py::fsum(&{ let mut out = vec![]; for t2218 in py::iter(&(py::zip(&[v_zs.clone(), v_y.clone()])?))? { let mut v_u: Py; let mut v_w: Py; 
let t2219 = (t2218).items()?;
if t2219.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t2219[0].clone();
v_w = t2219[1].clone();
out.push(py::mul(&py::sub(&v_u.clone(), &v_mz.clone())?, &py::sub(&v_w.clone(), &v_my.clone())?)?); } Py::list(out) })?, &py::fsum(&{ let mut out = vec![]; for t2220 in py::iter(&(v_zs.clone()))? { let mut v_u: Py; 
v_u = t2220;
out.push(py::pow(&py::sub(&v_u.clone(), &v_mz.clone())?, &Py::Int(2i64))?); } Py::list(out) })?)?;
    v_g = py::neg(&v_s.clone())?;
    v_L2inv = py::sub(&py::sub(&py::mul(&v_g.clone(), &v_g.clone())?, &py::pow(&py::truediv(&Py::Float(std::f64::consts::PI), &v_a.clone())?, &Py::Int(2i64))?)?, &py::pow(&py::truediv(&Py::Float(std::f64::consts::PI), &v_b.clone())?, &Py::Int(2i64))?)?;
    let _ = f__need({ let t2225 = { let t2221 = v_g.clone(); Py::Bool({ let t2222 = Py::Int(0i64); if py::gt(&t2221, &t2222)? { true } else { false } }) }; if !t2225.truthy() { t2225 } else { { let t2223 = v_L2inv.clone(); Py::Bool({ let t2224 = Py::Int(0i64); if py::gt(&t2223, &t2224)? { true } else { false } }) } } }, Py::str("the measured relaxation is too slow for these transverse dimensions: no diffusion length"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("gamma"), v_g.clone()), (Py::str("L"), py::truediv(&Py::Float(1.0), &py::math1("sqrt", &v_L2inv.clone())?)?), (Py::str("L2"), py::truediv(&Py::Float(1.0), &v_L2inv.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_flux_from_foil(mut v_A_d: Py, mut v_m: Py, mut v_M: Py, mut v_theta: Py, mut v_sigma: Py, mut v_T_half: Py, mut v_t_irr: Py, mut v_t_d: Py) -> R {
    let mut v_A_sat: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let _ = f__pos(Py::str("the measured activity, mass, molar mass, cross section, half-life and irradiation time"), Py::tuple(vec![v_A_d.clone(), v_m.clone(), v_M.clone(), v_sigma.clone(), v_T_half.clone(), v_t_irr.clone()]))?;
    let _ = f__frac(Py::str("the isotopic abundance theta"), v_theta.clone(), Py::Bool(true), Py::Bool(false))?;
    let _ = f__nonneg(Py::str("the decay time t_d"), Py::tuple(vec![v_t_d.clone()]))?;
    v_lam = py::truediv(&c_LN2()?, &v_T_half.clone())?;
    v_N = py::mul(&py::mul(&py::truediv(&v_m.clone(), &v_M.clone())?, &c_NA()?)?, &v_theta.clone())?;
    v_A_sat = py::truediv(&py::mul(&v_A_d.clone(), &py::math1("exp", &py::mul(&v_lam.clone(), &v_t_d.clone())?)?)?, &py::neg(&py::math1("expm1", &py::mul(&py::neg(&v_lam.clone())?, &v_t_irr.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("phi"), py::truediv(&v_A_sat.clone(), &py::mul(&v_N.clone(), &v_sigma.clone())?)?), (Py::str("A_sat"), v_A_sat.clone()), (Py::str("R_atom"), py::truediv(&v_A_sat.clone(), &v_N.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_flux_shape(mut v_geometry: Py, mut v_a: Py, mut v_R: Py, mut v_H: Py, mut v_x: Py, mut v_r: Py, mut v_z: Py) -> R {
    let mut v_j0: Py = Py::None;
    let mut v_u: Py = Py::None;
    let _ = f__need({ let t2321 = v_geometry.clone(); Py::Bool({ let t2322 = Py::tuple(vec![Py::str("slab"), Py::str("sphere"), Py::str("cylinder")]); if py::contains(&t2322, &t2321)? { true } else { false } }) }, Py::str("geometry must be slab, sphere or cylinder"))?;
    if ({ let t2323 = v_geometry.clone(); Py::Bool({ let t2324 = Py::str("slab"); if py::eq(&t2323, &t2324) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t2335 = { let t2325 = v_R.clone(); Py::Bool({ let t2326 = Py::None; if py::is(&t2325, &t2326) { true } else { false } }) }; if !t2335.truthy() { t2335 } else { { let t2334 = { let t2327 = v_H.clone(); Py::Bool({ let t2328 = Py::None; if py::is(&t2327, &t2328) { true } else { false } }) }; if !t2334.truthy() { t2334 } else { { let t2333 = { let t2329 = v_r.clone(); Py::Bool({ let t2330 = Py::None; if py::is(&t2329, &t2330) { true } else { false } }) }; if !t2333.truthy() { t2333 } else { { let t2331 = v_z.clone(); Py::Bool({ let t2332 = Py::None; if py::is(&t2331, &t2332) { true } else { false } }) } } } } } } }, Py::str("a slab takes a and x only"))?;
        let _ = f__pos(Py::str("the thickness a"), Py::tuple(vec![v_a.clone()]))?;
        let _ = f__need({ let t2346 = { let t2342 = v_x.clone(); Py::Bool({ let t2343 = Py::None; if !py::is(&t2342, &t2343) { true } else { false } }) }; if !t2346.truthy() { t2346 } else { { let t2344 = py::abs(&v_x.clone())?; Py::Bool({ let t2345 = py::truediv(&v_a.clone(), &Py::Int(2i64))?; if py::le(&t2344, &t2345)? { true } else { false } }) } } }, Py::str("x must lie inside the slab (|x| <= a/2)"))?;
        return Ok(f__out(Py::dict(vec![(Py::str("ratio"), py::add(&py::math1("cos", &py::truediv(&py::mul(&Py::Float(std::f64::consts::PI), &v_x.clone())?, &v_a.clone())?)?, &Py::Float(0.0))?), (Py::str("B2"), py::pow(&py::truediv(&Py::Float(std::f64::consts::PI), &v_a.clone())?, &Py::Int(2i64))?)]))?);
    }
    if ({ let t2359 = v_geometry.clone(); Py::Bool({ let t2360 = Py::str("sphere"); if py::eq(&t2359, &t2360) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t2371 = { let t2361 = v_a.clone(); Py::Bool({ let t2362 = Py::None; if py::is(&t2361, &t2362) { true } else { false } }) }; if !t2371.truthy() { t2371 } else { { let t2370 = { let t2363 = v_H.clone(); Py::Bool({ let t2364 = Py::None; if py::is(&t2363, &t2364) { true } else { false } }) }; if !t2370.truthy() { t2370 } else { { let t2369 = { let t2365 = v_x.clone(); Py::Bool({ let t2366 = Py::None; if py::is(&t2365, &t2366) { true } else { false } }) }; if !t2369.truthy() { t2369 } else { { let t2367 = v_z.clone(); Py::Bool({ let t2368 = Py::None; if py::is(&t2367, &t2368) { true } else { false } }) } } } } } } }, Py::str("a sphere takes R and r only"))?;
        let _ = f__pos(Py::str("the radius R"), Py::tuple(vec![v_R.clone()]))?;
        let _ = f__need({ let t2377 = { let t2372 = v_r.clone(); Py::Bool({ let t2373 = Py::None; if !py::is(&t2372, &t2373) { true } else { false } }) }; if !t2377.truthy() { t2377 } else { { let t2374 = Py::Int(0i64); Py::Bool({ let t2375 = v_r.clone(); if py::le(&t2374, &t2375)? { { let t2376 = v_R.clone(); if py::le(&t2375, &t2376)? { true } else { false } } } else { false } }) } } }, Py::str("r must lie inside the sphere"))?;
        v_u = py::truediv(&py::mul(&Py::Float(std::f64::consts::PI), &v_r.clone())?, &v_R.clone())?;
        return Ok(f__out(Py::dict(vec![(Py::str("ratio"), (if ({ let t2378 = v_r.clone(); Py::Bool({ let t2379 = Py::Int(0i64); if py::eq(&t2378, &t2379) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { py::truediv(&py::math1("sin", &v_u.clone())?, &v_u.clone())? })), (Py::str("B2"), py::pow(&py::truediv(&Py::Float(std::f64::consts::PI), &v_R.clone())?, &Py::Int(2i64))?)]))?);
    }
    let _ = f__need({ let t2384 = { let t2380 = v_a.clone(); Py::Bool({ let t2381 = Py::None; if py::is(&t2380, &t2381) { true } else { false } }) }; if !t2384.truthy() { t2384 } else { { let t2382 = v_x.clone(); Py::Bool({ let t2383 = Py::None; if py::is(&t2382, &t2383) { true } else { false } }) } } }, Py::str("a cylinder takes R, H, r and z"))?;
    let _ = f__pos(Py::str("the radius and height"), Py::tuple(vec![v_R.clone(), v_H.clone()]))?;
    let _ = f__need({ let t2396 = { let t2385 = v_r.clone(); Py::Bool({ let t2386 = Py::None; if !py::is(&t2385, &t2386) { true } else { false } }) }; if !t2396.truthy() { t2396 } else { { let t2395 = { let t2387 = Py::Int(0i64); Py::Bool({ let t2388 = v_r.clone(); if py::le(&t2387, &t2388)? { { let t2389 = v_R.clone(); if py::le(&t2388, &t2389)? { true } else { false } } } else { false } }) }; if !t2395.truthy() { t2395 } else { { let t2394 = { let t2390 = v_z.clone(); Py::Bool({ let t2391 = Py::None; if !py::is(&t2390, &t2391) { true } else { false } }) }; if !t2394.truthy() { t2394 } else { { let t2392 = py::abs(&v_z.clone())?; Py::Bool({ let t2393 = py::truediv(&v_H.clone(), &Py::Int(2i64))?; if py::le(&t2392, &t2393)? { true } else { false } }) } } } } } } }, Py::str("r and z must lie inside the cylinder"))?;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(20i64))?;
        v_j0 = py::float(&ax_engine::pymp::call("besselj", &[Py::Int(0i64), py::truediv(&py::mul(&c_J01()?, &v_r.clone())?, &v_R.clone())?], &[])?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("ratio"), py::mul(&v_j0.clone(), &py::math1("cos", &py::truediv(&py::mul(&Py::Float(std::f64::consts::PI), &v_z.clone())?, &v_H.clone())?)?)?), (Py::str("B2"), py::add(&py::pow(&py::truediv(&c_J01()?, &v_R.clone())?, &Py::Int(2i64))?, &py::pow(&py::truediv(&Py::Float(std::f64::consts::PI), &v_H.clone())?, &Py::Int(2i64))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fuel_cross_sections(mut v_rho: Py, mut v_w: Py, mut v_sigma_a5: Py, mut v_sigma_f5: Py, mut v_sigma_a8: Py, mut v_sigma_aO: Py, mut v_nu: Py) -> R {
    let mut v_MU: Py = Py::None;
    let mut v_N5: Py = Py::None;
    let mut v_N8: Py = Py::None;
    let mut v_NO: Py = Py::None;
    let mut v_Nmol: Py = Py::None;
    let mut v_Sa: Py = Py::None;
    let mut v_Sf: Py = Py::None;
    let mut v_a5: Py = Py::None;
    let mut v_rg: Py = Py::None;
    let _ = f__pos(Py::str("the density, the cross sections and nu"), Py::tuple(vec![v_rho.clone(), v_sigma_a5.clone(), v_sigma_f5.clone(), v_nu.clone()]))?;
    let _ = f__nonneg(Py::str("sigma_a8 and sigma_aO"), Py::tuple(vec![v_sigma_a8.clone(), v_sigma_aO.clone()]))?;
    let _ = f__frac(Py::str("the enrichment w"), v_w.clone(), Py::Bool(true), Py::Bool(false))?;
    let _ = f__need({ let t2442 = v_sigma_f5.clone(); Py::Bool({ let t2443 = v_sigma_a5.clone(); if py::le(&t2442, &t2443)? { true } else { false } }) }, Py::str("the fission cross section cannot exceed the absorption cross section"))?;
    v_rg = py::truediv(&v_rho.clone(), &Py::Float(1000.0))?;
    v_a5 = py::truediv(&py::truediv(&v_w.clone(), &c__M5()?)?, &py::add(&py::truediv(&v_w.clone(), &c__M5()?)?, &py::truediv(&py::sub(&Py::Int(1i64), &v_w.clone())?, &c__M8()?)?)?)?;
    v_MU = py::add(&py::mul(&v_a5.clone(), &c__M5()?)?, &py::mul(&py::sub(&Py::Int(1i64), &v_a5.clone())?, &c__M8()?)?)?;
    v_Nmol = py::mul(&py::truediv(&py::mul(&v_rg.clone(), &c_NA()?)?, &py::add(&v_MU.clone(), &py::mul(&Py::Int(2i64), &c__MO()?)?)?)?, &Py::Float(1000000.0))?;
    let t2444 = (Py::tuple(vec![py::mul(&v_Nmol.clone(), &v_a5.clone())?, py::mul(&v_Nmol.clone(), &py::sub(&Py::Int(1i64), &v_a5.clone())?)?, py::mul(&Py::Int(2i64), &v_Nmol.clone())?])).items()?;
    if t2444.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_N5 = t2444[0].clone();
    v_N8 = t2444[1].clone();
    v_NO = t2444[2].clone();
    v_Sa = py::add(&py::add(&py::mul(&v_N5.clone(), &v_sigma_a5.clone())?, &py::mul(&v_N8.clone(), &v_sigma_a8.clone())?)?, &py::mul(&v_NO.clone(), &v_sigma_aO.clone())?)?;
    v_Sf = py::mul(&v_N5.clone(), &v_sigma_f5.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("N5"), v_N5.clone()), (Py::str("N8"), v_N8.clone()), (Py::str("NO"), v_NO.clone()), (Py::str("Sigma_a"), v_Sa.clone()), (Py::str("Sigma_f"), v_Sf.clone()), (Py::str("eta"), py::truediv(&py::mul(&v_nu.clone(), &v_Sf.clone())?, &v_Sa.clone())?), (Py::str("mfp_a"), py::truediv(&Py::Float(1.0), &v_Sa.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fuel_rod_temperatures(mut v_q_lin: Py, mut v_R_f: Py, mut v_k_f: Py, mut v_h_g: Py, mut v_t_c: Py, mut v_k_c: Py, mut v_h: Py, mut v_T_cool: Py) -> R {
    let mut v_Ro: Py = Py::None;
    let mut v_T_0: Py = Py::None;
    let mut v_T_ci: Py = Py::None;
    let mut v_T_co: Py = Py::None;
    let mut v_T_fs: Py = Py::None;
    let _ = f__pos(Py::str("q', the radii, the conductivities, the conductances and the coolant temperature"), Py::tuple(vec![v_q_lin.clone(), v_R_f.clone(), v_k_f.clone(), v_h_g.clone(), v_t_c.clone(), v_k_c.clone(), v_h.clone(), v_T_cool.clone()]))?;
    v_Ro = py::add(&v_R_f.clone(), &v_t_c.clone())?;
    v_T_co = py::add(&v_T_cool.clone(), &py::truediv(&v_q_lin.clone(), &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_Ro.clone())?, &v_h.clone())?)?)?;
    v_T_ci = py::add(&v_T_co.clone(), &py::truediv(&py::mul(&v_q_lin.clone(), &py::math1("log", &py::truediv(&v_Ro.clone(), &v_R_f.clone())?)?)?, &py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_k_c.clone())?)?)?;
    v_T_fs = py::add(&v_T_ci.clone(), &py::truediv(&v_q_lin.clone(), &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_R_f.clone())?, &v_h_g.clone())?)?)?;
    v_T_0 = py::add(&v_T_fs.clone(), &py::truediv(&v_q_lin.clone(), &py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &v_k_f.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("T_0"), v_T_0.clone()), (Py::str("T_fs"), v_T_fs.clone()), (Py::str("T_ci"), v_T_ci.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_generator(mut v_A_P0: Py, mut v_T_P: Py, mut v_T_D: Py, mut v_b: Py, mut v_t: Py) -> R {
    let mut v_A_D: Py = Py::None;
    let mut v_A_Dmax: Py = Py::None;
    let mut v_A_P: Py = Py::None;
    let mut v_ld: Py = Py::None;
    let mut v_lp: Py = Py::None;
    let mut v_nd: Py = Py::None;
    let mut v_tm: Py = Py::None;
    let _ = f__pos(Py::str("the parent activity A_P0"), Py::tuple(vec![v_A_P0.clone()]))?;
    let _ = f__pos(Py::str("the half-lives"), Py::tuple(vec![v_T_P.clone(), v_T_D.clone()]))?;
    let _ = f__frac(Py::str("the branching fraction b"), v_b.clone(), Py::Bool(true), Py::Bool(false))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    let t2546 = (Py::tuple(vec![py::truediv(&c_LN2()?, &v_T_P.clone())?, py::truediv(&c_LN2()?, &v_T_D.clone())?])).items()?;
    if t2546.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lp = t2546[0].clone();
    v_ld = t2546[1].clone();
    v_nd = f__ad_build(v_lp.clone(), v_ld.clone(), v_t.clone())?;
    v_A_D = py::mul(&py::mul(&py::truediv(&py::mul(&v_b.clone(), &v_A_P0.clone())?, &v_lp.clone())?, &v_ld.clone())?, &v_nd.clone())?;
    v_A_P = py::mul(&v_A_P0.clone(), &py::math1("exp", &py::mul(&py::neg(&v_lp.clone())?, &v_t.clone())?)?)?;
    v_tm = f__tmax(v_lp.clone(), v_ld.clone())?;
    v_A_Dmax = py::mul(&py::mul(&py::truediv(&py::mul(&v_b.clone(), &v_A_P0.clone())?, &v_lp.clone())?, &v_ld.clone())?, &f__ad_build(v_lp.clone(), v_ld.clone(), v_tm.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("A_D"), v_A_D.clone()), (Py::str("A_P"), v_A_P.clone()), (Py::str("ratio"), py::truediv(&v_A_D.clone(), &v_A_P.clone())?), (Py::str("t_max"), v_tm.clone()), (Py::str("A_D_max"), v_A_Dmax.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hvl_tvl(mut v_mu: Py, mut v_x: Py) -> R {
    let mut v_out: Py = Py::None;
    let _ = f__pos(Py::str("the attenuation coefficient"), Py::tuple(vec![v_mu.clone()]))?;
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("HVL"), py::truediv(&py::math1("log", &Py::Int(2i64))?, &v_mu.clone())?), (Py::str("TVL"), py::truediv(&py::math1("log", &Py::Int(10i64))?, &v_mu.clone())?), (Py::str("mfp"), py::truediv(&Py::Float(1.0), &v_mu.clone())?), (Py::str("HVL_cm"), py::mul(&py::truediv(&py::math1("log", &Py::Int(2i64))?, &v_mu.clone())?, &Py::Int(100i64))?), (Py::str("TVL_cm"), py::mul(&py::truediv(&py::math1("log", &Py::Int(10i64))?, &v_mu.clone())?, &Py::Int(100i64))?)])?;
    if ({ let t2596 = v_x.clone(); Py::Bool({ let t2597 = Py::None; if !py::is(&t2596, &t2597) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t2600 = f__isnum(v_x.clone())?; if !t2600.truthy() { t2600 } else { { let t2598 = v_x.clone(); Py::Bool({ let t2599 = Py::Int(0i64); if py::ge(&t2598, &t2599)? { true } else { false } }) } } }, Py::str("the thickness must be zero or positive"))?;
        let _ = { let t2601 = v_out.clone(); py::setitem(&t2601, &Py::str("transmission"), py::math1("exp", &py::mul(&py::neg(&v_mu.clone())?, &v_x.clone())?)?)?; py::setitem(&t2601, &Py::str("n_HVL"), py::truediv(&py::mul(&v_mu.clone(), &v_x.clone())?, &py::math1("log", &Py::Int(2i64))?)?)?; Py::None };
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_inverse_multiplication(mut v_x: Py, mut v_C: Py) -> R {
    let mut v_icpt: Py = Py::None;
    let mut v_inv: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let mut v_slope: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    let mut v_sxy: Py = Py::None;
    let _ = f__need({ let t2645 = { let t2641 = py::len(&v_x.clone())?; Py::Bool({ let t2642 = py::len(&v_C.clone())?; if py::eq(&t2641, &t2642) { true } else { false } }) }; if !t2645.truthy() { t2645 } else { { let t2643 = py::len(&v_x.clone())?; Py::Bool({ let t2644 = Py::Int(3i64); if py::ge(&t2643, &t2644)? { true } else { false } }) } } }, Py::str("give at least three loadings with their count rates"))?;
    let _ = f__need({ let t2651 = Py::Bool(({ let mut out = vec![]; for t2646 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t2646;
out.push(f__isnum(v_v.clone())?); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t2651.truthy() { t2651 } else { { let t2649 = py::len(&py::set(&v_x.clone())?)?; Py::Bool({ let t2650 = py::len(&v_x.clone())?; if py::eq(&t2649, &t2650) { true } else { false } }) } } }, Py::str("loadings must be distinct numbers"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t2652 in py::iter(&(v_C.clone()))? { let mut v_c: Py; 
v_c = t2652;
out.push({ let t2655 = f__isnum(v_c.clone())?; if !t2655.truthy() { t2655 } else { { let t2653 = v_c.clone(); Py::Bool({ let t2654 = Py::Int(0i64); if py::gt(&t2653, &t2654)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("count rates must be positive"))?;
    v_inv = { let mut out = vec![]; for t2656 in py::iter(&(v_C.clone()))? { let mut v_c: Py; 
v_c = t2656;
out.push(py::truediv(&py::getitem(&v_C.clone(), &Py::Int(0i64))?, &v_c.clone())?); } Py::list(out) };
    v_n = py::len(&v_x.clone())?;
    let t2657 = (Py::tuple(vec![py::truediv(&py::fsum(&v_x.clone())?, &v_n.clone())?, py::truediv(&py::fsum(&v_inv.clone())?, &v_n.clone())?])).items()?;
    if t2657.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t2657[0].clone();
    v_my = t2657[1].clone();
    v_sxx = py::fsum(&{ let mut out = vec![]; for t2658 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t2658;
out.push(py::pow(&py::sub(&v_v.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) })?;
    v_sxy = py::fsum(&{ let mut out = vec![]; for t2659 in py::iter(&(py::zip(&[v_x.clone(), v_inv.clone()])?))? { let mut v_v: Py; let mut v_w: Py; 
let t2660 = (t2659).items()?;
if t2660.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t2660[0].clone();
v_w = t2660[1].clone();
out.push(py::mul(&py::sub(&v_v.clone(), &v_mx.clone())?, &py::sub(&v_w.clone(), &v_my.clone())?)?); } Py::list(out) })?;
    v_slope = py::truediv(&v_sxy.clone(), &v_sxx.clone())?;
    let _ = f__need({ let t2661 = v_slope.clone(); Py::Bool({ let t2662 = Py::Int(0i64); if py::lt(&t2661, &t2662)? { true } else { false } }) }, Py::str("1/M does not decrease with loading: no extrapolation to criticality"))?;
    v_icpt = py::sub(&v_my.clone(), &py::mul(&v_slope.clone(), &v_mx.clone())?)?;
    v_s2 = py::truediv(&py::sub(&py::getitem(&v_inv.clone(), &py::neg(&Py::Int(1i64))?)?, &py::getitem(&v_inv.clone(), &py::neg(&Py::Int(2i64))?)?)?, &py::sub(&py::getitem(&v_x.clone(), &py::neg(&Py::Int(1i64))?)?, &py::getitem(&v_x.clone(), &py::neg(&Py::Int(2i64))?)?)?)?;
    let _ = f__need({ let t2663 = v_s2.clone(); Py::Bool({ let t2664 = Py::Int(0i64); if py::lt(&t2663, &t2664)? { true } else { false } }) }, Py::str("the last two points do not decrease: no two-point extrapolation"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("x_crit"), py::truediv(&py::neg(&v_icpt.clone())?, &v_slope.clone())?), (Py::str("slope"), v_slope.clone()), (Py::str("x_crit_last2"), py::sub(&py::getitem(&v_x.clone(), &py::neg(&Py::Int(1i64))?)?, &py::truediv(&py::getitem(&v_inv.clone(), &py::neg(&Py::Int(1i64))?)?, &v_s2.clone())?)?), (Py::str("inv_M_last"), py::getitem(&v_inv.clone(), &py::neg(&Py::Int(1i64))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_irradiation_time(mut v_T_half: Py, mut v_f: Py) -> R {
    let mut v_t: Py = Py::None;
    let _ = f__pos(Py::str("the half-life"), Py::tuple(vec![v_T_half.clone()]))?;
    let _ = f__frac(Py::str("the saturation fraction f"), v_f.clone(), Py::Bool(true), Py::Bool(true))?;
    v_t = py::truediv(&py::mul(&py::neg(&py::math1("log1p", &py::neg(&v_f.clone())?)?)?, &v_T_half.clone())?, &c_LN2()?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("t_irr"), v_t.clone()), (Py::str("n_half"), py::truediv(&v_t.clone(), &v_T_half.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_klein_nishina(mut v_E: Py) -> R {
    let mut v_Emev: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sig: Py = Py::None;
    let mut v_sigtr: Py = Py::None;
    let mut v_st: Py = Py::None;
    let _ = f__pos(Py::str("the photon energy"), Py::tuple(vec![v_E.clone()]))?;
    v_Emev = py::truediv(&v_E.clone(), &c_MEV()?)?;
    let _ = f__need({ let t2762 = Py::Float(0.01); Py::Bool({ let t2763 = v_Emev.clone(); if py::le(&t2762, &t2763)? { { let t2764 = Py::Float(100.0); if py::le(&t2763, &t2764)? { true } else { false } } } else { false } }) }, Py::str("the photon energy must lie between 10 keV and 100 MeV"))?;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        v_k = py::truediv(&f__mpf(v_Emev.clone())?, &f__mpf(c_ME_MEV()?)?)?;
        v_L = ax_engine::pymp::call("log", &[py::add(&Py::Int(1i64), &py::mul(&Py::Int(2i64), &v_k.clone())?)?], &[])?;
        v_s = py::mul(&py::mul(&Py::Int(2i64), &ax_engine::pymp::constant("pi")?)?, &py::sub(&py::add(&py::mul(&py::truediv(&py::add(&Py::Int(1i64), &v_k.clone())?, &py::pow(&v_k.clone(), &Py::Int(2i64))?)?, &py::sub(&py::truediv(&py::mul(&Py::Int(2i64), &py::add(&Py::Int(1i64), &v_k.clone())?)?, &py::add(&Py::Int(1i64), &py::mul(&Py::Int(2i64), &v_k.clone())?)?)?, &py::truediv(&v_L.clone(), &v_k.clone())?)?)?, &py::truediv(&v_L.clone(), &py::mul(&Py::Int(2i64), &v_k.clone())?)?)?, &py::truediv(&py::add(&Py::Int(1i64), &py::mul(&Py::Int(3i64), &v_k.clone())?)?, &py::pow(&py::add(&Py::Int(1i64), &py::mul(&Py::Int(2i64), &v_k.clone())?)?, &Py::Int(2i64))?)?)?)?;
        v_st = py::mul(&py::mul(&Py::Int(2i64), &ax_engine::pymp::constant("pi")?)?, &py::sub(&py::sub(&py::sub(&py::sub(&py::truediv(&py::mul(&Py::Int(2i64), &py::pow(&py::add(&Py::Int(1i64), &v_k.clone())?, &Py::Int(2i64))?)?, &py::mul(&py::pow(&v_k.clone(), &Py::Int(2i64))?, &py::add(&Py::Int(1i64), &py::mul(&Py::Int(2i64), &v_k.clone())?)?)?)?, &py::truediv(&py::add(&Py::Int(1i64), &py::mul(&Py::Int(3i64), &v_k.clone())?)?, &py::pow(&py::add(&Py::Int(1i64), &py::mul(&Py::Int(2i64), &v_k.clone())?)?, &Py::Int(2i64))?)?)?, &py::truediv(&py::mul(&py::add(&Py::Int(1i64), &v_k.clone())?, &py::sub(&py::sub(&py::mul(&Py::Int(2i64), &py::pow(&v_k.clone(), &Py::Int(2i64))?)?, &py::mul(&Py::Int(2i64), &v_k.clone())?)?, &Py::Int(1i64))?)?, &py::mul(&py::pow(&v_k.clone(), &Py::Int(2i64))?, &py::pow(&py::add(&Py::Int(1i64), &py::mul(&Py::Int(2i64), &v_k.clone())?)?, &Py::Int(2i64))?)?)?)?, &py::truediv(&py::mul(&Py::Int(4i64), &py::pow(&v_k.clone(), &Py::Int(2i64))?)?, &py::mul(&Py::Int(3i64), &py::pow(&py::add(&Py::Int(1i64), &py::mul(&Py::Int(2i64), &v_k.clone())?)?, &Py::Int(3i64))?)?)?)?, &py::mul(&py::add(&py::sub(&py::truediv(&py::add(&Py::Int(1i64), &v_k.clone())?, &py::pow(&v_k.clone(), &Py::Int(3i64))?)?, &py::truediv(&Py::Int(1i64), &py::mul(&Py::Int(2i64), &v_k.clone())?)?)?, &py::truediv(&Py::Int(1i64), &py::mul(&Py::Int(2i64), &py::pow(&v_k.clone(), &Py::Int(3i64))?)?)?)?, &v_L.clone())?)?)?;
        let t2765 = (Py::tuple(vec![py::mul(&py::mul(&py::float(&v_s.clone())?, &c_RE()?)?, &c_RE()?)?, py::mul(&py::mul(&py::float(&v_st.clone())?, &c_RE()?)?, &c_RE()?)?])).items()?;
        if t2765.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_sig = t2765[0].clone();
        v_sigtr = t2765[1].clone();
    }
    return Ok(f__out(Py::dict(vec![(Py::str("sigma"), v_sig.clone()), (Py::str("sigma_b"), py::truediv(&v_sig.clone(), &c_BARN()?)?), (Py::str("sigma_tr"), v_sigtr.clone()), (Py::str("T_mean"), py::truediv(&py::mul(&v_Emev.clone(), &v_sigtr.clone())?, &v_sig.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_line_source(mut v_S_L: Py, mut v_h: Py, mut v_y1: Py, mut v_y2: Py, mut v_mu: Py, mut v_t: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let mut v_phi0: Py = Py::None;
    let mut v_th1: Py = Py::None;
    let mut v_th2: Py = Py::None;
    let _ = f__pos(Py::str("the source strength and the perpendicular distance"), Py::tuple(vec![v_S_L.clone(), v_h.clone()]))?;
    let _ = f__need({ let t2813 = f__isnum(v_y1.clone())?; if !t2813.truthy() { t2813 } else { { let t2812 = f__isnum(v_y2.clone())?; if !t2812.truthy() { t2812 } else { { let t2810 = v_y2.clone(); Py::Bool({ let t2811 = v_y1.clone(); if py::gt(&t2810, &t2811)? { true } else { false } }) } } } } }, Py::str("the line runs from y1 to y2 with y2 > y1"))?;
    let _ = f__nonneg(Py::str("mu and the shield thickness"), Py::tuple(vec![v_mu.clone(), v_t.clone()]))?;
    let _ = f__need({ let t2818 = v_t.clone(); Py::Bool({ let t2819 = v_h.clone(); if py::lt(&t2818, &t2819)? { true } else { false } }) }, Py::str("the shield must lie between the line and the point (t < h)"))?;
    v_b = py::mul(&v_mu.clone(), &v_t.clone())?;
    let t2820 = (Py::tuple(vec![py::atan2(&v_y1.clone(), &v_h.clone())?, py::atan2(&v_y2.clone(), &v_h.clone())?])).items()?;
    if t2820.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_th1 = t2820[0].clone();
    v_th2 = t2820[1].clone();
    v_phi = py::mul(&py::truediv(&v_S_L.clone(), &py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &v_h.clone())?)?, &py::sub(&f__sievert(v_th2.clone(), v_b.clone())?, &f__sievert(v_th1.clone(), v_b.clone())?)?)?;
    v_phi0 = py::mul(&py::truediv(&v_S_L.clone(), &py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &v_h.clone())?)?, &py::sub(&v_th2.clone(), &v_th1.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("phi"), v_phi.clone()), (Py::str("phi0"), v_phi0.clone()), (Py::str("T"), py::truediv(&v_phi.clone(), &v_phi0.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_log_decrement(mut v_A: Py) -> R {
    let _ = f__need({ let t2867 = f__isnum(v_A.clone())?; if !t2867.truthy() { t2867 } else { { let t2865 = v_A.clone(); Py::Bool({ let t2866 = Py::Int(1i64); if py::ge(&t2865, &t2866)? { true } else { false } }) } } }, Py::str("the mass ratio A must be at least 1"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("alpha"), f__alpha(v_A.clone())?), (Py::str("xi"), f__xi(v_A.clone())?), (Py::str("mu_bar"), py::truediv(&Py::Float(2.0), &py::mul(&Py::Float(3.0), &v_A.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mass_for_activity(mut v_nuclide: Py, mut v_A: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_m: Py = Py::None;
    let _ = f__need({ let t2908 = v_nuclide.clone(); Py::Bool({ let t2909 = c__ND()?; if py::contains(&t2909, &t2908)? { true } else { false } }) }, Py::str("the nuclide must be one of the tabulated radionuclides"))?;
    let _ = f__pos(Py::str("the activity A"), Py::tuple(vec![v_A.clone()]))?;
    let t2916 = (py::getitem(&c__ND()?, &v_nuclide.clone())?).items()?;
    if t2916.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T = t2916[0].clone();
    v_M = t2916[1].clone();
    v_N = py::truediv(&py::mul(&v_A.clone(), &v_T.clone())?, &c_LN2()?)?;
    v_m = py::truediv(&py::truediv(&py::mul(&v_N.clone(), &v_M.clone())?, &c_NA()?)?, &Py::Float(1000.0))?;
    return Ok(f__out(Py::dict(vec![(Py::str("m"), v_m.clone()), (Py::str("m_ug"), py::mul(&v_m.clone(), &Py::Float(1000000000.0))?), (Py::str("N"), v_N.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_max_clad_temperature(mut v_q0: Py, mut v_H: Py, mut v_H_e: Py, mut v_m_dot: Py, mut v_c_p: Py, mut v_T_in: Py, mut v_R_o: Py, mut v_h: Py) -> R {
    let mut v_Tc: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_s0: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__ch_check(v_q0.clone(), v_H.clone(), v_H_e.clone(), v_m_dot.clone(), v_c_p.clone(), v_T_in.clone())?;
    let _ = f__pos(Py::str("the cladding radius and the heat transfer coefficient"), Py::tuple(vec![v_R_o.clone(), v_h.clone()]))?;
    v_k = py::truediv(&py::mul(&v_q0.clone(), &v_H_e.clone())?, &py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_m_dot.clone())?, &v_c_p.clone())?)?;
    v_s0 = py::math1("sin", &py::truediv(&py::mul(&Py::Float(std::f64::consts::PI), &v_H.clone())?, &py::mul(&Py::Int(2i64), &v_H_e.clone())?)?)?;
    v_Tc = { let v_H = v_H.clone(); let v_H_e = v_H_e.clone(); let v_R_o = v_R_o.clone(); let v_T_in = v_T_in.clone(); let v_c_p = v_c_p.clone(); let v_h = v_h.clone(); let v_k = v_k.clone(); let v_m_dot = v_m_dot.clone(); let v_q0 = v_q0.clone(); let v_s0 = v_s0.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_z: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::add(&v_T_in.clone(), &py::mul(&v_k.clone(), &py::add(&py::math1("sin", &py::truediv(&py::mul(&Py::Float(std::f64::consts::PI), &v_z.clone())?, &v_H_e.clone())?)?, &v_s0.clone())?)?)?, &py::truediv(&py::mul(&v_q0.clone(), &py::math1("cos", &py::truediv(&py::mul(&Py::Float(std::f64::consts::PI), &v_z.clone())?, &v_H_e.clone())?)?)?, &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_R_o.clone())?, &v_h.clone())?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_z = py::mul(&py::truediv(&v_H_e.clone(), &Py::Float(std::f64::consts::PI))?, &py::math1("atan", &py::truediv(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_R_o.clone())?, &v_h.clone())?, &v_H_e.clone())?, &py::mul(&v_m_dot.clone(), &v_c_p.clone())?)?)?)?;
    v_z = py::minmax(&[v_z.clone(), py::truediv(&v_H.clone(), &Py::Int(2i64))?], false)?;
    return Ok(f__out(Py::dict(vec![(Py::str("z_max"), v_z.clone()), (Py::str("T_max"), py::call(&v_Tc.clone(), &[v_z.clone()])?), (Py::str("T_out"), py::add(&v_T_in.clone(), &py::mul(&py::mul(&Py::Int(2i64), &v_k.clone())?, &v_s0.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_maxwellian_average(mut v_sigma0: Py, mut v_T: Py) -> R {
    let mut v_f: Py = Py::None;
    let _ = f__pos(Py::str("the 2200 m/s cross section and the neutron temperature"), Py::tuple(vec![v_sigma0.clone(), v_T.clone()]))?;
    v_f = py::mul(&py::truediv(&py::math1("sqrt", &Py::Float(std::f64::consts::PI))?, &Py::Float(2.0))?, &py::math1("sqrt", &py::truediv(&c__E0_2200()?, &py::mul(&c_KB()?, &v_T.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("sigma_avg"), py::mul(&v_sigma0.clone(), &v_f.clone())?), (Py::str("sigma_avg_b"), py::truediv(&py::mul(&v_sigma0.clone(), &v_f.clone())?, &c_BARN()?)?), (Py::str("g"), v_f.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_moderator_mixture(mut v_n: Py, mut v_A: Py, mut v_sigma_s: Py, mut v_sigma_a: Py, mut v_rho: Py, mut v_M: Py) -> R {
    let mut v_Nm: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_Sa: Py = Py::None;
    let mut v_Ss: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_xi: Py = Py::None;
    v_k = py::len(&v_n.clone())?;
    let _ = f__need({ let t3056 = { let t3045 = Py::Int(1i64); Py::Bool({ let t3046 = v_k.clone(); if py::le(&t3045, &t3046)? { { let t3047 = Py::Int(8i64); if py::le(&t3046, &t3047)? { true } else { false } } } else { false } }) }; if !t3056.truthy() { t3056 } else { { let t3055 = { let t3048 = py::len(&v_A.clone())?; Py::Bool({ let t3049 = v_k.clone(); if py::eq(&t3048, &t3049) { true } else { false } }) }; if !t3055.truthy() { t3055 } else { { let t3054 = { let t3050 = py::len(&v_sigma_s.clone())?; Py::Bool({ let t3051 = v_k.clone(); if py::eq(&t3050, &t3051) { true } else { false } }) }; if !t3054.truthy() { t3054 } else { { let t3052 = py::len(&v_sigma_a.clone())?; Py::Bool({ let t3053 = v_k.clone(); if py::eq(&t3052, &t3053) { true } else { false } }) } } } } } } }, Py::str("give n, A, sigma_s and sigma_a for every element"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3057 in py::iter(&(v_n.clone()))? { let mut v_x: Py; 
v_x = t3057;
out.push({ let t3062 = f__isnum(v_x.clone())?; if !t3062.truthy() { t3062 } else { { let t3060 = v_x.clone(); Py::Bool({ let t3061 = Py::Int(0i64); if py::gt(&t3060, &t3061)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("atoms per molecule must be positive"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3063 in py::iter(&(v_A.clone()))? { let mut v_x: Py; 
v_x = t3063;
out.push({ let t3066 = f__isnum(v_x.clone())?; if !t3066.truthy() { t3066 } else { { let t3064 = v_x.clone(); Py::Bool({ let t3065 = Py::Int(1i64); if py::ge(&t3064, &t3065)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("mass ratios must be at least 1"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3067 in py::iter(&(py::add(&v_sigma_s.clone(), &v_sigma_a.clone())?))? { let mut v_x: Py; 
v_x = t3067;
out.push({ let t3070 = f__isnum(v_x.clone())?; if !t3070.truthy() { t3070 } else { { let t3068 = v_x.clone(); Py::Bool({ let t3069 = Py::Int(0i64); if py::ge(&t3068, &t3069)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("cross sections must be >= 0"))?;
    let _ = f__pos(Py::str("the density and the molar mass"), Py::tuple(vec![v_rho.clone(), v_M.clone()]))?;
    v_Nm = py::truediv(&py::mul(&v_rho.clone(), &c_NA()?)?, &v_M.clone())?;
    v_Ss = { let mut out = vec![]; for t3075 in py::iter(&(py::zip(&[v_n.clone(), v_sigma_s.clone()])?))? { let mut v_ni: Py; let mut v_s: Py; 
let t3076 = (t3075).items()?;
if t3076.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_ni = t3076[0].clone();
v_s = t3076[1].clone();
out.push(py::mul(&py::mul(&py::mul(&v_Nm.clone(), &v_ni.clone())?, &v_s.clone())?, &c_BARN()?)?); } Py::list(out) };
    v_Sa = py::fsum(&{ let mut out = vec![]; for t3077 in py::iter(&(py::zip(&[v_n.clone(), v_sigma_a.clone()])?))? { let mut v_ni: Py; let mut v_s: Py; 
let t3078 = (t3077).items()?;
if t3078.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_ni = t3078[0].clone();
v_s = t3078[1].clone();
out.push(py::mul(&py::mul(&py::mul(&v_Nm.clone(), &v_ni.clone())?, &v_s.clone())?, &c_BARN()?)?); } Py::list(out) })?;
    v_S = py::fsum(&v_Ss.clone())?;
    let _ = f__need({ let t3079 = v_S.clone(); Py::Bool({ let t3080 = Py::Int(0i64); if py::gt(&t3079, &t3080)? { true } else { false } }) }, Py::str("the scattering cross section is zero"))?;
    v_xi = py::truediv(&py::fsum(&{ let mut out = vec![]; for t3081 in py::iter(&(py::zip(&[v_A.clone(), v_Ss.clone()])?))? { let mut v_a: Py; let mut v_s: Py; 
let t3082 = (t3081).items()?;
if t3082.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3082[0].clone();
v_s = t3082[1].clone();
out.push(py::mul(&f__xi(v_a.clone())?, &v_s.clone())?); } Py::list(out) })?, &v_S.clone())?;
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("xi"), v_xi.clone()), (Py::str("MP"), py::mul(&v_xi.clone(), &v_S.clone())?)])?;
    if ({ let t3085 = v_Sa.clone(); Py::Bool({ let t3086 = Py::Int(0i64); if py::gt(&t3085, &t3086)? { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("MR"), py::truediv(&py::mul(&v_xi.clone(), &v_S.clone())?, &v_Sa.clone())?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_multilayer(mut v_mu: Py, mut v_x: Py, mut v_H0: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let _ = f__need({ let t3132 = { let t3127 = py::len(&v_mu.clone())?; Py::Bool({ let t3128 = py::len(&v_x.clone())?; if py::eq(&t3127, &t3128) { true } else { false } }) }; if !t3132.truthy() { t3132 } else { { let t3129 = Py::Int(1i64); Py::Bool({ let t3130 = py::len(&v_mu.clone())?; if py::le(&t3129, &t3130)? { { let t3131 = Py::Int(10i64); if py::le(&t3130, &t3131)? { true } else { false } } } else { false } }) } } }, Py::str("give one attenuation coefficient per layer thickness (1 to 10 layers)"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3133 in py::iter(&(v_mu.clone()))? { let mut v_m: Py; 
v_m = t3133;
out.push({ let t3138 = f__isnum(v_m.clone())?; if !t3138.truthy() { t3138 } else { { let t3136 = v_m.clone(); Py::Bool({ let t3137 = Py::Int(0i64); if py::ge(&t3136, &t3137)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("attenuation coefficients must be >= 0"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3139 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t3139;
out.push({ let t3142 = f__isnum(v_v.clone())?; if !t3142.truthy() { t3142 } else { { let t3140 = v_v.clone(); Py::Bool({ let t3141 = Py::Int(0i64); if py::ge(&t3140, &t3141)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("thicknesses must be >= 0"))?;
    v_tau = py::fsum(&{ let mut out = vec![]; for t3143 in py::iter(&(py::zip(&[v_mu.clone(), v_x.clone()])?))? { let mut v_m: Py; let mut v_v: Py; 
let t3144 = (t3143).items()?;
if t3144.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_m = t3144[0].clone();
v_v = t3144[1].clone();
out.push(py::mul(&v_m.clone(), &v_v.clone())?); } Py::list(out) })?;
    v_T = py::math1("exp", &py::neg(&v_tau.clone())?)?;
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("T"), v_T.clone()), (Py::str("tau"), v_tau.clone()), (Py::str("n_hvl"), py::truediv(&v_tau.clone(), &c_LN2()?)?), (Py::str("attenuation"), py::truediv(&Py::Float(1.0), &v_T.clone())?)])?;
    if ({ let t3145 = v_H0.clone(); Py::Bool({ let t3146 = Py::None; if !py::is(&t3145, &t3146) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the dose rate in front of the shield"), Py::tuple(vec![v_H0.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("H"), py::mul(&v_H0.clone(), &v_T.clone())?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_period_one_group(mut v_rho: Py, mut v_beta: Py, mut v_lam: Py, mut v_Lambda: Py) -> R {
    let mut v__a: Py = Py::None;
    let mut v_roots: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__kin_check(v_rho.clone(), Py::list(vec![v_beta.clone()]), Py::list(vec![v_lam.clone()]), v_Lambda.clone())?;
    let t3833 = (f__inhour_roots(v_rho.clone(), Py::list(vec![v_beta.clone()]), Py::list(vec![v_lam.clone()]), v_Lambda.clone())?).items()?;
    if t3833.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_roots = t3833[0].clone();
    v__a = t3833[1].clone();
    v_w = py::float(&py::getitem(&v_roots.clone(), &Py::Int(0i64))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("omega"), v_w.clone()), (Py::str("T"), py::truediv(&Py::Float(1.0), &v_w.clone())?), (Py::str("T_d"), py::truediv(&c_LN2()?, &v_w.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_plane_source_slab(mut v_S: Py, mut v_D: Py, mut v_Sigma_a: Py, mut v_a: Py, mut v_x: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_leak: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let mut v_phi0: Py = Py::None;
    let _ = f__pos(Py::str("S, D, Sigma_a and the slab thickness"), Py::tuple(vec![v_S.clone(), v_D.clone(), v_Sigma_a.clone(), v_a.clone()]))?;
    let _ = f__need({ let t3200 = f__isnum(v_x.clone())?; if !t3200.truthy() { t3200 } else { { let t3198 = py::abs(&v_x.clone())?; Py::Bool({ let t3199 = py::truediv(&v_a.clone(), &Py::Int(2i64))?; if py::le(&t3198, &t3199)? { true } else { false } }) } } }, Py::str("the position must lie inside the slab (|x| <= a/2)"))?;
    v_L = py::math1("sqrt", &py::truediv(&v_D.clone(), &v_Sigma_a.clone())?)?;
    v_h = py::truediv(&v_a.clone(), &py::mul(&Py::Int(2i64), &v_L.clone())?)?;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        v_phi = py::float(&py::truediv(&py::mul(&py::truediv(&py::mul(&f__mpf(v_S.clone())?, &v_L.clone())?, &py::mul(&Py::Int(2i64), &v_D.clone())?)?, &ax_engine::pymp::call("sinh", &[py::truediv(&py::sub(&py::truediv(&f__mpf(v_a.clone())?, &Py::Int(2i64))?, &py::abs(&f__mpf(v_x.clone())?)?)?, &v_L.clone())?], &[])?)?, &ax_engine::pymp::call("cosh", &[v_h.clone()], &[])?)?)?;
        v_phi0 = py::float(&py::mul(&py::truediv(&py::mul(&f__mpf(v_S.clone())?, &v_L.clone())?, &py::mul(&Py::Int(2i64), &v_D.clone())?)?, &ax_engine::pymp::call("tanh", &[v_h.clone()], &[])?)?)?;
        v_leak = py::float(&py::truediv(&Py::Int(1i64), &ax_engine::pymp::call("cosh", &[v_h.clone()], &[])?)?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("phi"), v_phi.clone()), (Py::str("phi0"), v_phi0.clone()), (Py::str("leak"), v_leak.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_point_dose_rate(mut v_Gamma: Py, mut v_A: Py, mut v_r: Py, mut v_mu: Py, mut v_x: Py, mut v_buildup: Py, mut v_A1: Py, mut v_alpha1: Py, mut v_alpha2: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_H: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_b: Py = Py::None;
    let _ = f__pos(Py::str("Gamma, the activity and the distance"), Py::tuple(vec![v_Gamma.clone(), v_A.clone(), v_r.clone()]))?;
    let _ = f__nonneg(Py::str("mu and the shield thickness"), Py::tuple(vec![v_mu.clone(), v_x.clone()]))?;
    let _ = f__need({ let t3249 = v_x.clone(); Py::Bool({ let t3250 = v_r.clone(); if py::lt(&t3249, &t3250)? { true } else { false } }) }, Py::str("the shield must lie between the source and the dose point (x < r)"))?;
    v_b = py::mul(&v_mu.clone(), &v_x.clone())?;
    v_B = f__buildup(v_buildup.clone(), v_b.clone(), v_A1.clone(), v_alpha1.clone(), v_alpha2.clone())?;
    v_T = py::mul(&v_B.clone(), &py::math1("exp", &py::neg(&v_b.clone())?)?)?;
    v_H = py::truediv(&py::mul(&py::mul(&v_Gamma.clone(), &v_A.clone())?, &v_T.clone())?, &py::mul(&v_r.clone(), &v_r.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("H_dot"), v_H.clone()), (Py::str("H_uSv_h"), py::mul(&v_H.clone(), &Py::Float(3600000000.0))?), (Py::str("B"), v_B.clone()), (Py::str("T"), v_T.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_point_source_sphere(mut v_S: Py, mut v_D: Py, mut v_Sigma_a: Py, mut v_R: Py, mut v_r: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_leak: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let mut v_phi_inf: Py = Py::None;
    let _ = f__pos(Py::str("S, D, Sigma_a, the radius and the position"), Py::tuple(vec![v_S.clone(), v_D.clone(), v_Sigma_a.clone(), v_R.clone(), v_r.clone()]))?;
    let _ = f__need({ let t3324 = v_r.clone(); Py::Bool({ let t3325 = v_R.clone(); if py::lt(&t3324, &t3325)? { true } else { false } }) }, Py::str("the point must lie inside the sphere (r < R)"))?;
    v_L = py::math1("sqrt", &py::truediv(&v_D.clone(), &v_Sigma_a.clone())?)?;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        v_phi = py::float(&py::truediv(&py::mul(&f__mpf(v_S.clone())?, &ax_engine::pymp::call("sinh", &[py::truediv(&py::sub(&f__mpf(v_R.clone())?, &v_r.clone())?, &v_L.clone())?], &[])?)?, &py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(4i64), &ax_engine::pymp::constant("pi")?)?, &v_D.clone())?, &v_r.clone())?, &ax_engine::pymp::call("sinh", &[py::truediv(&f__mpf(v_R.clone())?, &v_L.clone())?], &[])?)?)?)?;
        v_leak = py::float(&py::truediv(&py::truediv(&f__mpf(v_R.clone())?, &v_L.clone())?, &ax_engine::pymp::call("sinh", &[py::truediv(&f__mpf(v_R.clone())?, &v_L.clone())?], &[])?)?)?;
    }
    v_phi_inf = py::truediv(&py::mul(&v_S.clone(), &py::math1("exp", &py::truediv(&py::neg(&v_r.clone())?, &v_L.clone())?)?)?, &py::mul(&py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &v_D.clone())?, &v_r.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("phi"), v_phi.clone()), (Py::str("leak"), v_leak.clone()), (Py::str("phi_inf"), v_phi_inf.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_power_after_step_multi(mut v_rho: Py, mut v_beta: Py, mut v_lam: Py, mut v_Lambda: Py, mut v_t: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_amps: Py = Py::None;
    let mut v_roots: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__kin_check(v_rho.clone(), v_beta.clone(), v_lam.clone(), v_Lambda.clone())?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    let t3545 = (f__inhour_roots(v_rho.clone(), v_beta.clone(), v_lam.clone(), v_Lambda.clone())?).items()?;
    if t3545.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_roots = t3545[0].clone();
    v_amps = t3545[1].clone();
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(40i64))?;
        v_P = py::float(&py::sum(&{ let mut out = vec![]; for t3546 in py::iter(&(py::zip(&[v_roots.clone(), v_amps.clone()])?))? { let mut v_a: Py; let mut v_w: Py; 
let t3547 = (t3546).items()?;
if t3547.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_w = t3547[0].clone();
v_a = t3547[1].clone();
out.push(py::mul(&v_a.clone(), &ax_engine::pymp::call("exp", &[py::mul(&v_w.clone(), &f__mpf(v_t.clone())?)?], &[])?)?); } Py::list(out) }, None)?)?;
    }
    v_w = py::float(&py::getitem(&v_roots.clone(), &Py::Int(0i64))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("omega"), v_w.clone()), (Py::str("T"), py::truediv(&Py::Float(1.0), &v_w.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_power_after_step_one_group(mut v_rho: Py, mut v_beta: Py, mut v_lam: Py, mut v_Lambda: Py, mut v_t: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_amps: Py = Py::None;
    let mut v_roots: Py = Py::None;
    let _ = f__kin_check(v_rho.clone(), Py::list(vec![v_beta.clone()]), Py::list(vec![v_lam.clone()]), v_Lambda.clone())?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    let t3434 = (f__inhour_roots(v_rho.clone(), Py::list(vec![v_beta.clone()]), Py::list(vec![v_lam.clone()]), v_Lambda.clone())?).items()?;
    if t3434.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_roots = t3434[0].clone();
    v_amps = t3434[1].clone();
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(40i64))?;
        v_P = py::float(&py::sum(&{ let mut out = vec![]; for t3435 in py::iter(&(py::zip(&[v_roots.clone(), v_amps.clone()])?))? { let mut v_a: Py; let mut v_w: Py; 
let t3436 = (t3435).items()?;
if t3436.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_w = t3436[0].clone();
v_a = t3436[1].clone();
out.push(py::mul(&v_a.clone(), &ax_engine::pymp::call("exp", &[py::mul(&v_w.clone(), &f__mpf(v_t.clone())?)?], &[])?)?); } Py::list(out) }, None)?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("A1"), py::float(&py::getitem(&v_amps.clone(), &Py::Int(0i64))?)?), (Py::str("omega"), py::float(&py::getitem(&v_roots.clone(), &Py::Int(0i64))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reaction_power(mut v_E_f: Py, mut v_N: Py, mut v_sigma: Py, mut v_phi: Py, mut v_V: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_q: Py = Py::None;
    let _ = f__pos(Py::str("E_f, N, sigma and phi"), Py::tuple(vec![v_E_f.clone(), v_N.clone(), v_sigma.clone(), v_phi.clone()]))?;
    v_R = py::mul(&py::mul(&v_N.clone(), &v_sigma.clone())?, &v_phi.clone())?;
    v_q = py::mul(&v_E_f.clone(), &v_R.clone())?;
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("R"), v_R.clone()), (Py::str("q"), v_q.clone()), (Py::str("q_MW_m3"), py::truediv(&v_q.clone(), &Py::Float(1000000.0))?)])?;
    if ({ let t3592 = v_V.clone(); Py::Bool({ let t3593 = Py::None; if !py::is(&t3592, &t3593) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the volume"), Py::tuple(vec![v_V.clone()]))?;
        let _ = { let t3594 = v_out.clone(); py::setitem(&t3594, &Py::str("P"), py::mul(&v_q.clone(), &v_V.clone())?)?; py::setitem(&t3594, &Py::str("P_MW"), py::truediv(&py::mul(&v_q.clone(), &v_V.clone())?, &Py::Float(1000000.0))?)?; Py::None };
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reactivity_from_period(mut v_beta: Py, mut v_lam: Py, mut v_Lambda: Py, mut v_T: Py, mut v_T_d: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_per: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__need({ let t3693 = { let t3695 = v_T.clone(); Py::Bool({ let t3696 = Py::None; if py::is(&t3695, &t3696) { true } else { false } }) }; Py::Bool({ let t3694 = { let t3697 = v_T_d.clone(); Py::Bool({ let t3698 = Py::None; if py::is(&t3697, &t3698) { true } else { false } }) }; if py::ne(&t3693, &t3694) { true } else { false } }) }, Py::str("give either the stable period T or the doubling time T_d (one of them)"))?;
    let _ = f__need({ let t3704 = { let t3699 = py::len(&v_beta.clone())?; Py::Bool({ let t3700 = py::len(&v_lam.clone())?; if py::eq(&t3699, &t3700) { true } else { false } }) }; if !t3704.truthy() { t3704 } else { { let t3701 = Py::Int(1i64); Py::Bool({ let t3702 = py::len(&v_beta.clone())?; if py::le(&t3701, &t3702)? { { let t3703 = Py::Int(8i64); if py::le(&t3702, &t3703)? { true } else { false } } } else { false } }) } } }, Py::str("give one decay constant per delayed group"))?;
    let _ = f__need({ let t3715 = Py::Bool(({ let mut out = vec![]; for t3705 in py::iter(&(v_beta.clone()))? { let mut v_b: Py; 
v_b = t3705;
out.push({ let t3710 = f__isnum(v_b.clone())?; if !t3710.truthy() { t3710 } else { { let t3708 = v_b.clone(); Py::Bool({ let t3709 = Py::Int(0i64); if py::gt(&t3708, &t3709)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t3715.truthy() { t3715 } else { Py::Bool(({ let mut out = vec![]; for t3711 in py::iter(&(v_lam.clone()))? { let mut v_x: Py; 
v_x = t3711;
out.push({ let t3714 = f__isnum(v_x.clone())?; if !t3714.truthy() { t3714 } else { { let t3712 = v_x.clone(); Py::Bool({ let t3713 = Py::Int(0i64); if py::gt(&t3712, &t3713)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str("delayed data must be positive"))?;
    let _ = f__pos(Py::str("the generation time"), Py::tuple(vec![v_Lambda.clone()]))?;
    v_per = (if ({ let t3720 = v_T.clone(); Py::Bool({ let t3721 = Py::None; if !py::is(&t3720, &t3721) { true } else { false } }) }).truthy() { v_T.clone() } else { py::truediv(&v_T_d.clone(), &c_LN2()?)? });
    let _ = f__need({ let t3724 = f__isnum(v_per.clone())?; if !t3724.truthy() { t3724 } else { { let t3722 = v_per.clone(); Py::Bool({ let t3723 = Py::Int(0i64); if py::ne(&t3722, &t3723) { true } else { false } }) } } }, Py::str("the period must be a non-zero time"))?;
    v_w = py::truediv(&Py::Float(1.0), &v_per.clone())?;
    let _ = f__need({ let t3725 = v_w.clone(); Py::Bool({ let t3726 = py::neg(&py::minmax(&[v_lam.clone()], false)?)?; if py::gt(&t3725, &t3726)? { true } else { false } }) }, Py::str("a negative period shorter than 1/lambda_min is not a stable period"))?;
    v_rho = py::add(&py::mul(&v_w.clone(), &v_Lambda.clone())?, &py::fsum(&{ let mut out = vec![]; for t3727 in py::iter(&(py::zip(&[v_beta.clone(), v_lam.clone()])?))? { let mut v_b: Py; let mut v_l: Py; 
let t3728 = (t3727).items()?;
if t3728.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_b = t3728[0].clone();
v_l = t3728[1].clone();
out.push(py::truediv(&py::mul(&v_b.clone(), &v_w.clone())?, &py::add(&v_w.clone(), &v_l.clone())?)?); } Py::list(out) })?)?;
    v_B = py::fsum(&v_beta.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("rho"), v_rho.clone()), (Py::str("rho_dollars"), py::truediv(&v_rho.clone(), &v_B.clone())?), (Py::str("rho_pcm"), py::mul(&v_rho.clone(), &Py::Float(100000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reflected_slab(mut v_D_c: Py, mut v_Sigma_ac: Py, mut v_nuSigma_fc: Py, mut v_D_r: Py, mut v_Sigma_ar: Py, mut v_T_r: Py, mut v_extrapolation: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_Bm2: Py = Py::None;
    let mut v_Lr: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_q: Py = Py::None;
    v_Bm2 = f__group1(v_D_c.clone(), v_Sigma_ac.clone(), v_nuSigma_fc.clone())?;
    let _ = f__need({ let t3878 = v_Bm2.clone(); Py::Bool({ let t3879 = Py::Int(0i64); if py::gt(&t3878, &t3879)? { true } else { false } }) }, Py::str("k-infinity of the core <= 1: it cannot be critical"))?;
    let _ = f__pos(Py::str("D_r, Sigma_ar and the reflector thickness"), Py::tuple(vec![v_D_r.clone(), v_Sigma_ar.clone(), v_T_r.clone()]))?;
    v_B = py::math1("sqrt", &v_Bm2.clone())?;
    v_Lr = py::math1("sqrt", &py::truediv(&v_D_r.clone(), &v_Sigma_ar.clone())?)?;
    v_d = py::mul(&py::getitem(&c_EXTRAP()?, &py::method(&v_extrapolation.clone(), "lower", &[])?)?, &v_D_r.clone())?;
    v_q = py::truediv(&py::truediv(&v_D_r.clone(), &py::mul(&py::mul(&v_D_c.clone(), &v_B.clone())?, &v_Lr.clone())?)?, &py::math1("tanh", &py::truediv(&py::add(&v_T_r.clone(), &v_d.clone())?, &v_Lr.clone())?)?)?;
    v_a = py::mul(&py::truediv(&Py::Float(2.0), &v_B.clone())?, &py::math1("atan", &v_q.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("delta"), py::truediv(&py::sub(&py::truediv(&Py::Float(std::f64::consts::PI), &v_B.clone())?, &v_a.clone())?, &Py::Float(2.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rod_worth(mut v_rho_tot: Py, mut v_H: Py, mut v_x: Py, mut v_beta: Py) -> R {
    let mut v_frac: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_u: Py = Py::None;
    let _ = f__need({ let t708 = f__isnum(v_rho_tot.clone())?; if !t708.truthy() { t708 } else { { let t706 = v_rho_tot.clone(); Py::Bool({ let t707 = Py::Int(0i64); if py::gt(&t706, &t707)? { true } else { false } }) } } }, Py::str("the total rod worth must be positive"))?;
    let _ = f__pos(Py::str("the core height"), Py::tuple(vec![v_H.clone()]))?;
    let _ = f__need({ let t716 = f__isnum(v_x.clone())?; if !t716.truthy() { t716 } else { { let t713 = Py::Int(0i64); Py::Bool({ let t714 = v_x.clone(); if py::le(&t713, &t714)? { { let t715 = v_H.clone(); if py::le(&t714, &t715)? { true } else { false } } } else { false } }) } } }, Py::str("the insertion depth must lie between 0 and H"))?;
    v_u = py::truediv(&v_x.clone(), &v_H.clone())?;
    v_frac = py::sub(&v_u.clone(), &py::truediv(&py::math1("sin", &py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_u.clone())?)?, &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?)?;
    if ({ let t717 = py::abs(&v_frac.clone())?; Py::Bool({ let t718 = Py::Float(1e-15); if py::lt(&t717, &t718)? { true } else { false } }) }).truthy() {
        v_frac = Py::Float(0.0);
    }
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("rho"), py::mul(&v_rho_tot.clone(), &v_frac.clone())?), (Py::str("frac"), v_frac.clone()), (Py::str("drho_dx"), py::mul(&py::truediv(&v_rho_tot.clone(), &v_H.clone())?, &py::sub(&Py::Int(1i64), &py::math1("cos", &py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_u.clone())?)?)?)?)])?;
    if ({ let t719 = v_beta.clone(); Py::Bool({ let t720 = Py::None; if !py::is(&t719, &t720) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("beta"), Py::tuple(vec![v_beta.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("rho_dollars"), py::truediv(&py::mul(&v_rho_tot.clone(), &v_frac.clone())?, &v_beta.clone())?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_samarium(mut v_Sigma_f: Py, mut v_phi: Py, mut v_gamma_Pm: Py, mut v_sigma_Sm: Py, mut v_t: Py) -> R {
    let mut v_Pm: Py = Py::None;
    let mut v_Sm: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__pos(Py::str("Sigma_f, the flux and sigma_Sm"), Py::tuple(vec![v_Sigma_f.clone(), v_phi.clone(), v_sigma_Sm.clone()]))?;
    let _ = f__frac(Py::str("gamma_Pm"), v_gamma_Pm.clone(), Py::Bool(true), Py::Bool(false))?;
    v_Pm = py::truediv(&py::mul(&py::mul(&v_gamma_Pm.clone(), &v_Sigma_f.clone())?, &v_phi.clone())?, &c__LAM_PM()?)?;
    v_Sm = py::truediv(&py::mul(&v_gamma_Pm.clone(), &v_Sigma_f.clone())?, &v_sigma_Sm.clone())?;
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("Pm_eq"), v_Pm.clone()), (Py::str("Sm_eq"), v_Sm.clone()), (Py::str("Sm_max"), py::add(&v_Sm.clone(), &v_Pm.clone())?), (Py::str("max_ratio"), py::truediv(&py::add(&v_Sm.clone(), &v_Pm.clone())?, &v_Sm.clone())?)])?;
    if ({ let t3933 = v_t.clone(); Py::Bool({ let t3934 = Py::None; if !py::is(&t3933, &t3934) { true } else { false } }) }).truthy() {
        let _ = f__nonneg(Py::str("the time after shutdown"), Py::tuple(vec![v_t.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("Sm_t"), py::add(&v_Sm.clone(), &py::mul(&v_Pm.clone(), &py::neg(&py::math1("expm1", &py::mul(&py::neg(&c__LAM_PM()?)?, &v_t.clone())?)?)?)?)?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_shield_thickness(mut v_Gamma: Py, mut v_A: Py, mut v_r: Py, mut v_mu: Py, mut v_H_target: Py, mut v_buildup: Py) -> R {
    let mut v_H0: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_lq: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__pos(Py::str("Gamma, the activity, the distance, mu and the target dose rate"), Py::tuple(vec![v_Gamma.clone(), v_A.clone(), v_r.clone(), v_mu.clone(), v_H_target.clone()]))?;
    let _ = f__need({ let t3984 = v_buildup.clone(); Py::Bool({ let t3985 = Py::tuple(vec![Py::str("none"), Py::str("linear")]); if py::contains(&t3985, &t3984)? { true } else { false } }) }, Py::str("buildup must be none or linear for a thickness"))?;
    v_H0 = py::truediv(&py::mul(&v_Gamma.clone(), &v_A.clone())?, &py::mul(&v_r.clone(), &v_r.clone())?)?;
    v_q = py::truediv(&v_H_target.clone(), &v_H0.clone())?;
    let _ = f__need({ let t3986 = v_q.clone(); Py::Bool({ let t3987 = Py::Int(1i64); if py::lt(&t3986, &t3987)? { true } else { false } }) }, Py::str("the unshielded dose rate is already below the target: no shield is needed"))?;
    if ({ let t3988 = v_buildup.clone(); Py::Bool({ let t3989 = Py::str("none"); if py::eq(&t3988, &t3989) { true } else { false } }) }).truthy() {
        v_b = py::neg(&py::math1("log", &v_q.clone())?)?;
    } else {
        v_lq = py::math1("log", &v_q.clone())?;
        v_b = f__bisect({ let v_A = v_A.clone(); let v_Gamma = v_Gamma.clone(); let v_H0 = v_H0.clone(); let v_H_target = v_H_target.clone(); let v_b = v_b.clone(); let v_buildup = v_buildup.clone(); let v_lq = v_lq.clone(); let v_mu = v_mu.clone(); let v_q = v_q.clone(); let v_r = v_r.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_z: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::sub(&py::math1("log1p", &v_z.clone())?, &v_z.clone())?, &v_lq.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::Float(0.0), py::add(&py::mul(&py::neg(&Py::Float(2.0))?, &v_lq.clone())?, &Py::Float(10.0))?, Py::Int(400i64))?;
    }
    v_x = py::truediv(&v_b.clone(), &v_mu.clone())?;
    let _ = f__need({ let t4016 = v_x.clone(); Py::Bool({ let t4017 = v_r.clone(); if py::lt(&t4016, &t4017)? { true } else { false } }) }, Py::str("the shield would be thicker than the source distance"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("x"), v_x.clone()), (Py::str("n_mfp"), v_b.clone()), (Py::str("B"), (if ({ let t4030 = v_buildup.clone(); Py::Bool({ let t4031 = Py::str("none"); if py::eq(&t4030, &t4031) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { py::add(&Py::Float(1.0), &v_b.clone())? }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_six_factor(mut v_eta: Py, mut v_f: Py, mut v_p: Py, mut v_eps: Py, mut v_P_FNL: Py, mut v_P_TNL: Py) -> R {
    let mut v_PN: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_kinf: Py = Py::None;
    let mut v_nm: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t4058 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("eta"), v_eta.clone()]), Py::tuple(vec![Py::str("eps"), v_eps.clone()])])))? {
        let t4065 = (t4058).items()?;
        if t4065.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_nm = t4065[0].clone();
        v_v = t4065[1].clone();
        let _ = f__need({ let t4064 = f__isnum(v_v.clone())?; if !t4064.truthy() { t4064 } else { { let t4061 = Py::Int(0i64); Py::Bool({ let t4062 = v_v.clone(); if py::lt(&t4061, &t4062)? { { let t4063 = Py::Int(3i64); if py::le(&t4062, &t4063)? { true } else { false } } } else { false } }) } } }, Py::str(&[py::format(&v_nm.clone(), "")?, " must lie in (0, 3]".to_string()].concat()))?;
    }
    for t4066 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("f"), v_f.clone()]), Py::tuple(vec![Py::str("p"), v_p.clone()])])))? {
        let t4071 = (t4066).items()?;
        if t4071.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_nm = t4071[0].clone();
        v_v = t4071[1].clone();
        let _ = f__need({ let t4070 = f__isnum(v_v.clone())?; if !t4070.truthy() { t4070 } else { { let t4067 = Py::Int(0i64); Py::Bool({ let t4068 = v_v.clone(); if py::lt(&t4067, &t4068)? { { let t4069 = Py::Int(1i64); if py::le(&t4068, &t4069)? { true } else { false } } } else { false } }) } } }, Py::str(&[py::format(&v_nm.clone(), "")?, " must lie in (0, 1]".to_string()].concat()))?;
    }
    for t4072 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("P_FNL"), v_P_FNL.clone()]), Py::tuple(vec![Py::str("P_TNL"), v_P_TNL.clone()])])))? {
        let t4080 = (t4072).items()?;
        if t4080.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_nm = t4080[0].clone();
        v_v = t4080[1].clone();
        let _ = f__need({ let t4079 = { let t4073 = v_v.clone(); Py::Bool({ let t4074 = Py::None; if py::is(&t4073, &t4074) { true } else { false } }) }; if t4079.truthy() { t4079 } else { { let t4078 = f__isnum(v_v.clone())?; if !t4078.truthy() { t4078 } else { { let t4075 = Py::Int(0i64); Py::Bool({ let t4076 = v_v.clone(); if py::lt(&t4075, &t4076)? { { let t4077 = Py::Int(1i64); if py::le(&t4076, &t4077)? { true } else { false } } } else { false } }) } } } } }, Py::str(&[py::format(&v_nm.clone(), "")?, " must lie in (0, 1]".to_string()].concat()))?;
    }
    v_kinf = py::mul(&py::mul(&py::mul(&v_eta.clone(), &v_f.clone())?, &v_p.clone())?, &v_eps.clone())?;
    v_PN = py::mul(&(if ({ let t4081 = v_P_FNL.clone(); Py::Bool({ let t4082 = Py::None; if py::is(&t4081, &t4082) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { v_P_FNL.clone() }), &(if ({ let t4083 = v_P_TNL.clone(); Py::Bool({ let t4084 = Py::None; if py::is(&t4083, &t4084) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { v_P_TNL.clone() }))?;
    v_k = py::mul(&v_kinf.clone(), &v_PN.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("k_inf"), v_kinf.clone()), (Py::str("k_eff"), v_k.clone()), (Py::str("P_NL"), v_PN.clone()), (Py::str("rho"), py::truediv(&py::sub(&v_k.clone(), &Py::Int(1i64))?, &v_k.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_solid_angle_axis(mut v_d: Py, mut v_a: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_om: Py = Py::None;
    let _ = f__pos(Py::str("the distance and the radius"), Py::tuple(vec![v_d.clone(), v_a.clone()]))?;
    v_c = py::truediv(&v_d.clone(), &py::hypot(&[v_d.clone(), v_a.clone()])?)?;
    v_om = py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::sub(&Py::Int(1i64), &v_c.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Omega"), v_om.clone()), (Py::str("g"), py::truediv(&v_om.clone(), &py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_solid_angle_off_axis(mut v_d: Py, mut v_a: Py, mut v_rho0: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_om: Py = Py::None;
    let mut v_ph0: Py = Py::None;
    let _ = f__pos(Py::str("the distance and the radius"), Py::tuple(vec![v_d.clone(), v_a.clone()]))?;
    let _ = f__nonneg(Py::str("the lateral offset"), Py::tuple(vec![v_rho0.clone()]))?;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(20i64))?;
        let t4133 = (Py::tuple(vec![f__mpf(v_d.clone())?, f__mpf(v_a.clone())?, f__mpf(v_rho0.clone())?])).items()?;
        if t4133.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_D = t4133[0].clone();
        v_A = t4133[1].clone();
        v_P = t4133[2].clone();
        if ({ let t4134 = v_rho0.clone(); Py::Bool({ let t4135 = v_a.clone(); if py::le(&t4134, &t4135)? { true } else { false } }) }).truthy() {
            v_f = { let v_A = v_A.clone(); let v_D = v_D.clone(); let v_P = v_P.clone(); let v_a = v_a.clone(); let v_d = v_d.clone(); let v_rho0 = v_rho0.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_ph: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&Py::Int(1i64), &py::truediv(&v_D.clone(), &ax_engine::pymp::call("sqrt", &[py::add(&py::mul(&v_D.clone(), &v_D.clone())?, &py::pow(&py::add(&py::mul(&v_P.clone(), &ax_engine::pymp::call("cos", &[v_ph.clone()], &[])?)?, &ax_engine::pymp::call("sqrt", &[py::sub(&py::mul(&v_A.clone(), &v_A.clone())?, &py::pow(&py::mul(&v_P.clone(), &ax_engine::pymp::call("sin", &[v_ph.clone()], &[])?)?, &Py::Int(2i64))?)?], &[])?)?, &Py::Int(2i64))?)?], &[])?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
            v_om = py::mul(&Py::Int(2i64), &ax_engine::pymp::call("quad", &[v_f.clone(), Py::list(vec![Py::Int(0i64), py::truediv(&ax_engine::pymp::constant("pi")?, &Py::Int(2i64))?, ax_engine::pymp::constant("pi")?])], &[])?)?;
        } else {
            v_ph0 = ax_engine::pymp::call("asin", &[py::truediv(&v_A.clone(), &v_P.clone())?], &[])?;
            v_f = { let v_A = v_A.clone(); let v_D = v_D.clone(); let v_P = v_P.clone(); let v_a = v_a.clone(); let v_d = v_d.clone(); let v_f = v_f.clone(); let v_om = v_om.clone(); let v_ph0 = v_ph0.clone(); let v_rho0 = v_rho0.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_ph: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_r1: Py = Py::None;
            let mut v_r2: Py = Py::None;
            let mut v_s: Py = Py::None;
            v_s = ax_engine::pymp::call("sqrt", &[py::minmax(&[py::sub(&py::mul(&v_A.clone(), &v_A.clone())?, &py::pow(&py::mul(&v_P.clone(), &ax_engine::pymp::call("sin", &[v_ph.clone()], &[])?)?, &Py::Int(2i64))?)?, Py::Int(0i64)], true)?], &[])?;
            let t4136 = (Py::tuple(vec![py::sub(&py::mul(&v_P.clone(), &ax_engine::pymp::call("cos", &[v_ph.clone()], &[])?)?, &v_s.clone())?, py::add(&py::mul(&v_P.clone(), &ax_engine::pymp::call("cos", &[v_ph.clone()], &[])?)?, &v_s.clone())?])).items()?;
            if t4136.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_r1 = t4136[0].clone();
            v_r2 = t4136[1].clone();
            return Ok(py::sub(&py::truediv(&v_D.clone(), &ax_engine::pymp::call("sqrt", &[py::add(&py::mul(&v_D.clone(), &v_D.clone())?, &py::mul(&v_r1.clone(), &v_r1.clone())?)?], &[])?)?, &py::truediv(&v_D.clone(), &ax_engine::pymp::call("sqrt", &[py::add(&py::mul(&v_D.clone(), &v_D.clone())?, &py::mul(&v_r2.clone(), &v_r2.clone())?)?], &[])?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
            v_om = py::mul(&Py::Int(2i64), &ax_engine::pymp::call("quad", &[v_f.clone(), Py::list(vec![Py::Int(0i64), v_ph0.clone()])], &[])?)?;
        }
        v_om = py::float(&v_om.clone())?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("Omega"), v_om.clone()), (Py::str("g"), py::truediv(&v_om.clone(), &py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_specific_activity(mut v_nuclide: Py, mut v_m: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_a: Py = Py::None;
    let _ = f__need({ let t4219 = v_nuclide.clone(); Py::Bool({ let t4220 = c__ND()?; if py::contains(&t4220, &t4219)? { true } else { false } }) }, Py::str("the nuclide must be one of the tabulated radionuclides"))?;
    let _ = f__pos(Py::str("the mass m"), Py::tuple(vec![v_m.clone()]))?;
    let t4227 = (py::getitem(&c__ND()?, &v_nuclide.clone())?).items()?;
    if t4227.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T = t4227[0].clone();
    v_M = t4227[1].clone();
    v_N = py::mul(&py::truediv(&py::mul(&v_m.clone(), &Py::Float(1000.0))?, &v_M.clone())?, &c_NA()?)?;
    v_A = py::mul(&py::truediv(&c_LN2()?, &v_T.clone())?, &v_N.clone())?;
    v_a = py::mul(&py::truediv(&py::mul(&py::truediv(&c_LN2()?, &v_T.clone())?, &c_NA()?)?, &v_M.clone())?, &Py::Float(1000.0))?;
    return Ok(f__out(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("A_Ci"), py::truediv(&v_A.clone(), &c_CI()?)?), (Py::str("a"), v_a.clone()), (Py::str("a_Ci_per_g"), py::truediv(&py::truediv(&v_a.clone(), &Py::Float(1000.0))?, &c_CI()?)?), (Py::str("N"), v_N.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stable_period_multi(mut v_rho: Py, mut v_beta: Py, mut v_lam: Py, mut v_Lambda: Py) -> R {
    let mut v__a: Py = Py::None;
    let mut v_roots: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_wp: Py = Py::None;
    let _ = f__kin_check(v_rho.clone(), v_beta.clone(), v_lam.clone(), v_Lambda.clone())?;
    let t4332 = (f__inhour_roots(v_rho.clone(), v_beta.clone(), v_lam.clone(), v_Lambda.clone())?).items()?;
    if t4332.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_roots = t4332[0].clone();
    v__a = t4332[1].clone();
    let t4333 = (Py::tuple(vec![py::float(&py::getitem(&v_roots.clone(), &Py::Int(0i64))?)?, py::float(&py::getitem(&v_roots.clone(), &py::neg(&Py::Int(1i64))?)?)?])).items()?;
    if t4333.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_w = t4333[0].clone();
    v_wp = t4333[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("omega"), v_w.clone()), (Py::str("T"), py::truediv(&Py::Float(1.0), &v_w.clone())?), (Py::str("T_d"), py::truediv(&c_LN2()?, &v_w.clone())?), (Py::str("omega_p"), v_wp.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stay_time(mut v_H0: Py, mut v_T_half: Py, mut v_D_lim: Py) -> R {
    let mut v_lam: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_t: Py = Py::None;
    let _ = f__pos(Py::str("the dose rate, the half-life and the dose limit"), Py::tuple(vec![v_H0.clone(), v_T_half.clone(), v_D_lim.clone()]))?;
    v_lam = py::truediv(&c_LN2()?, &v_T_half.clone())?;
    v_q = py::truediv(&py::mul(&v_lam.clone(), &v_D_lim.clone())?, &v_H0.clone())?;
    let _ = f__need({ let t4378 = v_q.clone(); Py::Bool({ let t4379 = Py::Int(1i64); if py::lt(&t4378, &t4379)? { true } else { false } }) }, Py::str("the dose limit is never reached: the source decays first (D_lim >= H0 T_half/ln 2)"))?;
    v_t = py::truediv(&py::neg(&py::math1("log1p", &py::neg(&v_q.clone())?)?)?, &v_lam.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("t_stay"), v_t.clone()), (Py::str("frac"), v_q.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_swu(mut v_x_p: Py, mut v_x_f: Py, mut v_x_w: Py, mut v_P: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_W: Py = Py::None;
    let mut v_nm: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t2045 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("x_p"), v_x_p.clone()]), Py::tuple(vec![Py::str("x_f"), v_x_f.clone()]), Py::tuple(vec![Py::str("x_w"), v_x_w.clone()])])))? {
        let t2057 = (t2045).items()?;
        if t2057.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_nm = t2057[0].clone();
        v_v = t2057[1].clone();
        let _ = f__frac(v_nm.clone(), v_v.clone(), Py::Bool(true), Py::Bool(true))?;
    }
    let _ = f__need({ let t2058 = v_x_w.clone(); Py::Bool({ let t2059 = v_x_f.clone(); if py::lt(&t2058, &t2059)? { { let t2060 = v_x_p.clone(); if py::lt(&t2059, &t2060)? { true } else { false } } } else { false } }) }, Py::str("the assays must satisfy tails < feed < product"))?;
    let _ = f__pos(Py::str("the product mass"), Py::tuple(vec![v_P.clone()]))?;
    v_F = py::truediv(&py::mul(&v_P.clone(), &py::sub(&v_x_p.clone(), &v_x_w.clone())?)?, &py::sub(&v_x_f.clone(), &v_x_w.clone())?)?;
    v_W = py::sub(&v_F.clone(), &v_P.clone())?;
    v_S = py::sub(&py::add(&py::mul(&v_P.clone(), &f__swu_value(v_x_p.clone())?)?, &py::mul(&v_W.clone(), &f__swu_value(v_x_w.clone())?)?)?, &py::mul(&v_F.clone(), &f__swu_value(v_x_f.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("F"), v_F.clone()), (Py::str("W"), v_W.clone()), (Py::str("SWU"), v_S.clone()), (Py::str("F_per_P"), py::truediv(&v_F.clone(), &v_P.clone())?), (Py::str("SWU_per_P"), py::truediv(&v_S.clone(), &v_P.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_thermal_speeds(mut v_T: Py) -> R {
    let mut v_vp: Py = Py::None;
    let _ = f__pos(Py::str("the temperature"), Py::tuple(vec![v_T.clone()]))?;
    v_vp = py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &c_KB()?)?, &v_T.clone())?, &c_MN()?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("v_p"), v_vp.clone()), (Py::str("v_mean"), py::truediv(&py::mul(&Py::Int(2i64), &v_vp.clone())?, &py::math1("sqrt", &Py::Float(std::f64::consts::PI))?)?), (Py::str("v_rms"), py::mul(&v_vp.clone(), &py::math1("sqrt", &Py::Float(1.5))?)?), (Py::str("E_mean"), py::mul(&py::mul(&Py::Float(1.5), &c_KB()?)?, &v_T.clone())?), (Py::str("E_mean_eV"), py::truediv(&py::mul(&py::mul(&Py::Float(1.5), &c_KB()?)?, &v_T.clone())?, &c_QE()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_three_member_chain(mut v_T1: Py, mut v_T2: Py, mut v_T3: Py, mut v_A10: Py, mut v_A20: Py, mut v_A30: Py, mut v_t: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_Ts: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_n0: Py = Py::None;
    let _ = f__pos(Py::str("the half-lives"), Py::tuple(vec![v_T1.clone(), v_T2.clone(), v_T3.clone()]))?;
    let _ = f__pos(Py::str("the initial parent activity A10"), Py::tuple(vec![v_A10.clone()]))?;
    let _ = f__nonneg(Py::str("the initial activities A20, A30"), Py::tuple(vec![v_A20.clone(), v_A30.clone()]))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    v_Ts = py::sorted(&Py::list(vec![v_T1.clone(), v_T2.clone(), v_T3.clone()]), (Py::Bool(false)).truthy())?;
    let _ = f__need({ let t4476 = { let t4472 = py::truediv(&py::getitem(&v_Ts.clone(), &Py::Int(1i64))?, &py::getitem(&v_Ts.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t4473 = Py::Float(1.0001); if py::gt(&t4472, &t4473)? { true } else { false } }) }; if !t4476.truthy() { t4476 } else { { let t4474 = py::truediv(&py::getitem(&v_Ts.clone(), &Py::Int(2i64))?, &py::getitem(&v_Ts.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t4475 = Py::Float(1.0001); if py::gt(&t4474, &t4475)? { true } else { false } }) } } }, Py::str("the three half-lives must differ (by more than 0.01 %)"))?;
    v_lam = Py::dict(vec![(Py::str("1"), py::truediv(&c_LN2()?, &v_T1.clone())?), (Py::str("2"), py::truediv(&c_LN2()?, &v_T2.clone())?), (Py::str("3"), py::truediv(&c_LN2()?, &v_T3.clone())?)]);
    v_n0 = Py::dict(vec![(Py::str("1"), py::truediv(&v_A10.clone(), &py::getitem(&v_lam.clone(), &Py::str("1"))?)?), (Py::str("2"), py::truediv(&v_A20.clone(), &py::getitem(&v_lam.clone(), &Py::str("2"))?)?), (Py::str("3"), py::truediv(&v_A30.clone(), &py::getitem(&v_lam.clone(), &Py::str("3"))?)?)]);
    v_N = f__bateman_dag(Py::list(vec![Py::str("1"), Py::str("2"), Py::str("3")]), v_lam.clone(), Py::list(vec![Py::tuple(vec![Py::str("1"), Py::str("2"), Py::Float(1.0)]), Py::tuple(vec![Py::str("2"), Py::str("3"), Py::Float(1.0)])]), v_n0.clone(), v_t.clone())?;
    v_A = { let mut out = vec![]; for t4488 in py::iter(&(Py::tuple(vec![Py::str("1"), Py::str("2"), Py::str("3")])))? { let mut v_k: Py; 
v_k = t4488;
out.push(py::mul(&py::getitem(&v_lam.clone(), &v_k.clone())?, &py::getitem(&v_N.clone(), &v_k.clone())?)?); } Py::list(out) };
    return Ok(f__out(Py::dict(vec![(Py::str("A1"), py::getitem(&v_A.clone(), &Py::Int(0i64))?), (Py::str("A2"), py::getitem(&v_A.clone(), &Py::Int(1i64))?), (Py::str("A3"), py::getitem(&v_A.clone(), &Py::Int(2i64))?), (Py::str("A_total"), py::fsum(&v_A.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_transit_dose(mut v_Gamma: Py, mut v_A: Py, mut v_d: Py, mut v_v: Py, mut v_L: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_Dinf: Py = Py::None;
    let _ = f__pos(Py::str("Gamma, the activity, the closest distance, the speed and the path length"), Py::tuple(vec![v_Gamma.clone(), v_A.clone(), v_d.clone(), v_v.clone(), v_L.clone()]))?;
    v_D = py::mul(&py::mul(&py::truediv(&py::mul(&v_Gamma.clone(), &v_A.clone())?, &py::mul(&v_d.clone(), &v_v.clone())?)?, &Py::Float(2.0))?, &py::math1("atan", &py::truediv(&v_L.clone(), &py::mul(&Py::Float(2.0), &v_d.clone())?)?)?)?;
    v_Dinf = py::truediv(&py::mul(&py::mul(&v_Gamma.clone(), &v_A.clone())?, &Py::Float(std::f64::consts::PI))?, &py::mul(&v_d.clone(), &v_v.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("D"), v_D.clone()), (Py::str("D_uSv"), py::mul(&v_D.clone(), &Py::Float(1000000.0))?), (Py::str("frac"), py::truediv(&v_D.clone(), &v_Dinf.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_two_component_fit(mut v_t: Py, mut v_C: Py, mut v_T1: Py, mut v_T2: Py, mut v_t_unit: Py) -> R {
    let mut v_a11: Py = Py::None;
    let mut v_a12: Py = Py::None;
    let mut v_a22: Py = Py::None;
    let mut v_b1: Py = Py::None;
    let mut v_b2: Py = Py::None;
    let mut v_c1: Py = Py::None;
    let mut v_c2: Py = Py::None;
    let mut v_det: Py = Py::None;
    let mut v_f1: Py = Py::None;
    let mut v_f2: Py = Py::None;
    let mut v_l1: Py = Py::None;
    let mut v_l2: Py = Py::None;
    let mut v_ts: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__need({ let t4571 = v_t_unit.clone(); Py::Bool({ let t4572 = c__TU()?; if py::contains(&t4572, &t4571)? { true } else { false } }) }, Py::str("t_unit must be s, min, h, d or y"))?;
    let _ = f__lists(v_t.clone(), v_C.clone(), Py::Int(3i64))?;
    let _ = f__pos(Py::str("the half-lives"), Py::tuple(vec![v_T1.clone(), v_T2.clone()]))?;
    let _ = f__need({ let t4590 = py::abs(&py::sub(&py::truediv(&v_T1.clone(), &v_T2.clone())?, &Py::Float(1.0))?)?; Py::Bool({ let t4591 = Py::Float(0.001); if py::gt(&t4590, &t4591)? { true } else { false } }) }, Py::str("the two half-lives must differ"))?;
    v_ts = { let mut out = vec![]; for t4592 in py::iter(&(v_t.clone()))? { let mut v_x: Py; 
v_x = t4592;
out.push(py::mul(&v_x.clone(), &py::getitem(&c__TU()?, &v_t_unit.clone())?)?); } Py::list(out) };
    let t4593 = (Py::tuple(vec![py::truediv(&c_LN2()?, &v_T1.clone())?, py::truediv(&c_LN2()?, &v_T2.clone())?])).items()?;
    if t4593.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_l1 = t4593[0].clone();
    v_l2 = t4593[1].clone();
    v_f1 = { let mut out = vec![]; for t4594 in py::iter(&(v_ts.clone()))? { let mut v_x: Py; 
v_x = t4594;
out.push(py::math1("exp", &py::mul(&py::neg(&v_l1.clone())?, &v_x.clone())?)?); } Py::list(out) };
    v_f2 = { let mut out = vec![]; for t4595 in py::iter(&(v_ts.clone()))? { let mut v_x: Py; 
v_x = t4595;
out.push(py::math1("exp", &py::mul(&py::neg(&v_l2.clone())?, &v_x.clone())?)?); } Py::list(out) };
    v_w = { let mut out = vec![]; for t4596 in py::iter(&(v_C.clone()))? { let mut v_c: Py; 
v_c = t4596;
out.push(py::truediv(&Py::Float(1.0), &v_c.clone())?); } Py::list(out) };
    v_a11 = py::fsum(&{ let mut out = vec![]; for t4597 in py::iter(&(py::zip(&[v_w.clone(), v_f1.clone()])?))? { let mut v_wi: Py; let mut v_x: Py; 
let t4598 = (t4597).items()?;
if t4598.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_wi = t4598[0].clone();
v_x = t4598[1].clone();
out.push(py::mul(&py::mul(&v_wi.clone(), &v_x.clone())?, &v_x.clone())?); } Py::list(out) })?;
    v_a22 = py::fsum(&{ let mut out = vec![]; for t4599 in py::iter(&(py::zip(&[v_w.clone(), v_f2.clone()])?))? { let mut v_wi: Py; let mut v_x: Py; 
let t4600 = (t4599).items()?;
if t4600.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_wi = t4600[0].clone();
v_x = t4600[1].clone();
out.push(py::mul(&py::mul(&v_wi.clone(), &v_x.clone())?, &v_x.clone())?); } Py::list(out) })?;
    v_a12 = py::fsum(&{ let mut out = vec![]; for t4601 in py::iter(&(py::zip(&[v_w.clone(), v_f1.clone(), v_f2.clone()])?))? { let mut v_wi: Py; let mut v_x: Py; let mut v_y: Py; 
let t4602 = (t4601).items()?;
if t4602.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_wi = t4602[0].clone();
v_x = t4602[1].clone();
v_y = t4602[2].clone();
out.push(py::mul(&py::mul(&v_wi.clone(), &v_x.clone())?, &v_y.clone())?); } Py::list(out) })?;
    v_b1 = py::fsum(&{ let mut out = vec![]; for t4603 in py::iter(&(py::zip(&[v_w.clone(), v_f1.clone(), v_C.clone()])?))? { let mut v_c: Py; let mut v_wi: Py; let mut v_x: Py; 
let t4604 = (t4603).items()?;
if t4604.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_wi = t4604[0].clone();
v_x = t4604[1].clone();
v_c = t4604[2].clone();
out.push(py::mul(&py::mul(&v_wi.clone(), &v_x.clone())?, &v_c.clone())?); } Py::list(out) })?;
    v_b2 = py::fsum(&{ let mut out = vec![]; for t4605 in py::iter(&(py::zip(&[v_w.clone(), v_f2.clone(), v_C.clone()])?))? { let mut v_c: Py; let mut v_wi: Py; let mut v_x: Py; 
let t4606 = (t4605).items()?;
if t4606.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_wi = t4606[0].clone();
v_x = t4606[1].clone();
v_c = t4606[2].clone();
out.push(py::mul(&py::mul(&v_wi.clone(), &v_x.clone())?, &v_c.clone())?); } Py::list(out) })?;
    v_det = py::sub(&py::mul(&v_a11.clone(), &v_a22.clone())?, &py::mul(&v_a12.clone(), &v_a12.clone())?)?;
    let _ = f__need({ let t4607 = v_det.clone(); Py::Bool({ let t4608 = py::mul(&py::mul(&Py::Float(0.001), &v_a11.clone())?, &v_a22.clone())?; if py::gt(&t4607, &t4608)? { true } else { false } }) }, Py::str("the two components cannot be separated from these times"))?;
    v_c1 = py::truediv(&py::sub(&py::mul(&v_b1.clone(), &v_a22.clone())?, &py::mul(&v_b2.clone(), &v_a12.clone())?)?, &v_det.clone())?;
    v_c2 = py::truediv(&py::sub(&py::mul(&v_a11.clone(), &v_b2.clone())?, &py::mul(&v_a12.clone(), &v_b1.clone())?)?, &v_det.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("C1"), v_c1.clone()), (Py::str("C2"), v_c2.clone()), (Py::str("frac1"), py::truediv(&v_c1.clone(), &py::add(&v_c1.clone(), &v_c2.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_two_group_keff(mut v_geometry: Py, mut v_D1: Py, mut v_D2: Py, mut v_Sigma_a1: Py, mut v_Sigma_12: Py, mut v_Sigma_a2: Py, mut v_nuSigma_f1: Py, mut v_nuSigma_f2: Py, mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_R: Py, mut v_H: Py) -> R {
    let mut v_B2: Py = Py::None;
    let mut v_R1: Py = Py::None;
    let mut v_T2: Py = Py::None;
    let mut v_dims: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_kinf: Py = Py::None;
    let _ = f__pos(Py::str("D1, D2, Sigma_12 and Sigma_a2"), Py::tuple(vec![v_D1.clone(), v_D2.clone(), v_Sigma_12.clone(), v_Sigma_a2.clone()]))?;
    let _ = f__nonneg(Py::str("Sigma_a1 and the fission cross sections"), Py::tuple(vec![v_Sigma_a1.clone(), v_nuSigma_f1.clone(), v_nuSigma_f2.clone()]))?;
    let _ = f__need({ let t4657 = py::add(&v_nuSigma_f1.clone(), &v_nuSigma_f2.clone())?; Py::Bool({ let t4658 = Py::Int(0i64); if py::gt(&t4657, &t4658)? { true } else { false } }) }, Py::str("there is no fission"))?;
    v_dims = f__dims(v_geometry.clone(), Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone()), (Py::str("c"), v_c.clone()), (Py::str("R"), v_R.clone()), (Py::str("H"), v_H.clone())]))?;
    v_B2 = f__bg2(v_geometry.clone(), v_dims.clone())?;
    v_R1 = py::add(&py::add(&py::mul(&v_D1.clone(), &v_B2.clone())?, &v_Sigma_a1.clone())?, &v_Sigma_12.clone())?;
    v_T2 = py::add(&py::mul(&v_D2.clone(), &v_B2.clone())?, &v_Sigma_a2.clone())?;
    v_k = py::truediv(&py::add(&v_nuSigma_f1.clone(), &py::truediv(&py::mul(&v_nuSigma_f2.clone(), &v_Sigma_12.clone())?, &v_T2.clone())?)?, &v_R1.clone())?;
    v_kinf = py::truediv(&py::add(&v_nuSigma_f1.clone(), &py::truediv(&py::mul(&v_nuSigma_f2.clone(), &v_Sigma_12.clone())?, &v_Sigma_a2.clone())?)?, &py::add(&v_Sigma_a1.clone(), &v_Sigma_12.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("B2"), v_B2.clone()), (Py::str("k_eff"), v_k.clone()), (Py::str("k_inf"), v_kinf.clone()), (Py::str("flux_ratio"), py::truediv(&v_Sigma_12.clone(), &v_T2.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_unit_conversion(mut v_A: Py, mut v_D: Py, mut v_H: Py) -> R {
    let mut v_given: Py = Py::None;
    let mut v_si: Py = Py::None;
    let mut v_tr: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_given = { let mut out = vec![]; for t3634 in py::iter(&(Py::tuple(vec![v_A.clone(), v_D.clone(), v_H.clone()])))? { let mut v_v: Py; 
v_v = t3634;
if ({ let t3635 = v_v.clone(); Py::Bool({ let t3636 = Py::None; if !py::is(&t3635, &t3636) { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) };
    let _ = f__need({ let t3637 = py::len(&v_given.clone())?; Py::Bool({ let t3638 = Py::Int(1i64); if py::eq(&t3637, &t3638) { true } else { false } }) }, Py::str("give exactly one quantity: an activity A, an absorbed dose D or an equivalent dose H"))?;
    if ({ let t3639 = v_A.clone(); Py::Bool({ let t3640 = Py::None; if !py::is(&t3639, &t3640) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the activity"), Py::tuple(vec![v_A.clone()]))?;
        return Ok(f__out(Py::dict(vec![(Py::str("SI_value"), v_A.clone()), (Py::str("traditional_value"), py::truediv(&v_A.clone(), &c_CI()?)?), (Py::str("si_unit"), Py::str("Bq")), (Py::str("traditional_unit"), Py::str("Ci")), (Py::str("value_mCi"), py::mul(&py::truediv(&v_A.clone(), &c_CI()?)?, &Py::Float(1000.0))?), (Py::str("value_MBq"), py::truediv(&v_A.clone(), &Py::Float(1000000.0))?)]))?);
    }
    v_v = (if ({ let t3659 = v_D.clone(); Py::Bool({ let t3660 = Py::None; if !py::is(&t3659, &t3660) { true } else { false } }) }).truthy() { v_D.clone() } else { v_H.clone() });
    let _ = f__need({ let t3663 = f__isnum(v_v.clone())?; if !t3663.truthy() { t3663 } else { { let t3661 = v_v.clone(); Py::Bool({ let t3662 = Py::Int(0i64); if py::ge(&t3661, &t3662)? { true } else { false } }) } } }, Py::str("the dose must be zero or positive"))?;
    let t3666 = ((if ({ let t3664 = v_D.clone(); Py::Bool({ let t3665 = Py::None; if !py::is(&t3664, &t3665) { true } else { false } }) }).truthy() { Py::tuple(vec![Py::str("Gy"), Py::str("rad")]) } else { Py::tuple(vec![Py::str("Sv"), Py::str("rem")]) })).items()?;
    if t3666.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_si = t3666[0].clone();
    v_tr = t3666[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("SI_value"), v_v.clone()), (Py::str("traditional_value"), py::mul(&v_v.clone(), &Py::Float(100.0))?), (Py::str("si_unit"), v_si.clone()), (Py::str("traditional_unit"), v_tr.clone()), (Py::str("value_mSv_or_mGy"), py::mul(&v_v.clone(), &Py::Float(1000.0))?), (Py::str("value_mrem_or_mrad"), py::mul(&v_v.clone(), &Py::Float(100000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_xenon_after_shutdown(mut v_Sigma_f: Py, mut v_phi: Py, mut v_gamma_I: Py, mut v_gamma_X: Py, mut v_sigma_X: Py, mut v_t: Py) -> R {
    let mut v_I0: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_X0: Py = Py::None;
    let mut v_Xp: Py = Py::None;
    let mut v_Xt: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_arg: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_lI: Py = Py::None;
    let mut v_lX: Py = Py::None;
    let mut v_tp: Py = Py::None;
    let t4732 = (f__xe_eq(v_Sigma_f.clone(), v_phi.clone(), v_gamma_I.clone(), v_gamma_X.clone(), v_sigma_X.clone())?).items()?;
    if t4732.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_I0 = t4732[0].clone();
    v_X0 = t4732[1].clone();
    let _ = f__nonneg(Py::str("the time after shutdown"), Py::tuple(vec![v_t.clone()]))?;
    let t4737 = (Py::tuple(vec![c__LAM_I()?, c__LAM_X()?])).items()?;
    if t4737.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lI = t4737[0].clone();
    v_lX = t4737[1].clone();
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(40i64))?;
        let t4738 = (Py::tuple(vec![f__mpf(v_lI.clone())?, f__mpf(v_lX.clone())?])).items()?;
        if t4738.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t4738[0].clone();
        v_b = t4738[1].clone();
        v_Xt = { let v_I0 = v_I0.clone(); let v_Sigma_f = v_Sigma_f.clone(); let v_X0 = v_X0.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_gamma_I = v_gamma_I.clone(); let v_gamma_X = v_gamma_X.clone(); let v_lI = v_lI.clone(); let v_lX = v_lX.clone(); let v_phi = v_phi.clone(); let v_sigma_X = v_sigma_X.clone(); let v_t = v_t.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_s: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::mul(&f__mpf(v_X0.clone())?, &ax_engine::pymp::call("exp", &[py::mul(&py::neg(&v_b.clone())?, &v_s.clone())?], &[])?)?, &py::mul(&py::truediv(&py::mul(&f__mpf(v_I0.clone())?, &v_a.clone())?, &py::sub(&v_b.clone(), &v_a.clone())?)?, &py::sub(&ax_engine::pymp::call("exp", &[py::mul(&py::neg(&v_a.clone())?, &v_s.clone())?], &[])?, &ax_engine::pymp::call("exp", &[py::mul(&py::neg(&v_b.clone())?, &v_s.clone())?], &[])?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
        v_X = py::float(&py::call(&v_Xt.clone(), &[f__mpf(v_t.clone())?])?)?;
        v_arg = py::truediv(&py::mul(&v_b.clone(), &py::sub(&py::mul(&f__mpf(v_I0.clone())?, &v_a.clone())?, &py::mul(&f__mpf(v_X0.clone())?, &py::sub(&v_b.clone(), &v_a.clone())?)?)?)?, &py::mul(&py::mul(&v_a.clone(), &v_a.clone())?, &f__mpf(v_I0.clone())?)?)?;
        v_tp = (if ({ let t4739 = v_arg.clone(); Py::Bool({ let t4740 = Py::Int(0i64); if py::gt(&t4739, &t4740)? { true } else { false } }) }).truthy() { py::truediv(&ax_engine::pymp::call("log", &[v_arg.clone()], &[])?, &py::sub(&v_b.clone(), &v_a.clone())?)? } else { f__mpf(py::neg(&Py::Int(1i64))?)? });
        if ({ let t4745 = { let t4741 = py::mul(&v_lI.clone(), &v_I0.clone())?; Py::Bool({ let t4742 = py::mul(&v_lX.clone(), &v_X0.clone())?; if py::gt(&t4741, &t4742)? { true } else { false } }) }; if !t4745.truthy() { t4745 } else { { let t4743 = v_tp.clone(); Py::Bool({ let t4744 = Py::Int(0i64); if py::gt(&t4743, &t4744)? { true } else { false } }) } } }).truthy() {
            v_Xp = py::float(&py::call(&v_Xt.clone(), &[v_tp.clone()])?)?;
            v_tp = py::float(&v_tp.clone())?;
        } else {
            let t4746 = (Py::tuple(vec![Py::Float(0.0), v_X0.clone()])).items()?;
            if t4746.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_tp = t4746[0].clone();
            v_Xp = t4746[1].clone();
        }
    }
    return Ok(f__out(Py::dict(vec![(Py::str("X"), v_X.clone()), (Py::str("ratio"), py::truediv(&v_X.clone(), &v_X0.clone())?), (Py::str("t_peak"), v_tp.clone()), (Py::str("peak_ratio"), py::truediv(&v_Xp.clone(), &v_X0.clone())?), (Py::str("I"), py::mul(&v_I0.clone(), &py::math1("exp", &py::mul(&py::neg(&v_lI.clone())?, &v_t.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_xenon_equilibrium(mut v_Sigma_f: Py, mut v_phi: Py, mut v_gamma_I: Py, mut v_gamma_X: Py, mut v_sigma_X: Py) -> R {
    let mut v_I: Py = Py::None;
    let mut v_X: Py = Py::None;
    let t4802 = (f__xe_eq(v_Sigma_f.clone(), v_phi.clone(), v_gamma_I.clone(), v_gamma_X.clone(), v_sigma_X.clone())?).items()?;
    if t4802.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_I = t4802[0].clone();
    v_X = t4802[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("I_eq"), v_I.clone()), (Py::str("X_eq"), v_X.clone()), (Py::str("abs_per_fission"), py::truediv(&py::mul(&v_sigma_X.clone(), &v_X.clone())?, &v_Sigma_f.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1019(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1021: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1020 = v_inputs.clone(); f_counts_during_decay(py::getitem(&t1020, &Py::str("A0"))?, py::getitem(&t1020, &Py::str("T_half"))?, py::getitem(&t1020, &Py::str("tc"))?, py::getitem(&t1020, &Py::str("eff"))?, py::getitem(&t1020, &Py::str("y"))?, py::method(&t1020, "get", &[Py::str("td"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1021 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1022 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1044 = (t1022).items()?;
        if t1044.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1044[0].clone();
        v_v = t1044[1].clone();
        if ({ let t1023 = v_k.clone(); Py::Bool({ let t1024 = v_outputs.clone(); if !py::contains(&t1024, &t1023)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1069(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1071: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1070 = v_inputs.clone(); f_critical_cuboid(py::getitem(&t1070, &Py::str("D"))?, py::getitem(&t1070, &Py::str("Sigma_a"))?, py::getitem(&t1070, &Py::str("nuSigma_f"))?, py::getitem(&t1070, &Py::str("extrapolation"))?, py::getitem(&t1070, &Py::str("a"))?, py::getitem(&t1070, &Py::str("b"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1071 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1072 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1094 = (t1072).items()?;
        if t1094.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1094[0].clone();
        v_v = t1094[1].clone();
        if ({ let t1073 = v_k.clone(); Py::Bool({ let t1074 = v_outputs.clone(); if !py::contains(&t1074, &t1073)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1119(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1121: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1120 = v_inputs.clone(); f_critical_cylinder(py::getitem(&t1120, &Py::str("D"))?, py::getitem(&t1120, &Py::str("Sigma_a"))?, py::getitem(&t1120, &Py::str("nuSigma_f"))?, py::getitem(&t1120, &Py::str("extrapolation"))?, py::getitem(&t1120, &Py::str("H"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1121 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1122 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1144 = (t1122).items()?;
        if t1144.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1144[0].clone();
        v_v = t1144[1].clone();
        if ({ let t1123 = v_k.clone(); Py::Bool({ let t1124 = v_outputs.clone(); if !py::contains(&t1124, &t1123)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1167(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1169: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1168 = v_inputs.clone(); f_critical_cylinder_min(py::getitem(&t1168, &Py::str("D"))?, py::getitem(&t1168, &Py::str("Sigma_a"))?, py::getitem(&t1168, &Py::str("nuSigma_f"))?, py::getitem(&t1168, &Py::str("extrapolation"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1169 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1170 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1192 = (t1170).items()?;
        if t1192.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1192[0].clone();
        v_v = t1192[1].clone();
        if ({ let t1171 = v_k.clone(); Py::Bool({ let t1172 = v_outputs.clone(); if !py::contains(&t1172, &t1171)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1215(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1217: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1216 = v_inputs.clone(); f_critical_slab(py::getitem(&t1216, &Py::str("D"))?, py::getitem(&t1216, &Py::str("Sigma_a"))?, py::getitem(&t1216, &Py::str("nuSigma_f"))?, py::getitem(&t1216, &Py::str("extrapolation"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1217 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1218 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1240 = (t1218).items()?;
        if t1240.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1240[0].clone();
        v_v = t1240[1].clone();
        if ({ let t1219 = v_k.clone(); Py::Bool({ let t1220 = v_outputs.clone(); if !py::contains(&t1220, &t1219)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1263(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1265: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1264 = v_inputs.clone(); f_critical_sphere(py::getitem(&t1264, &Py::str("D"))?, py::getitem(&t1264, &Py::str("Sigma_a"))?, py::getitem(&t1264, &Py::str("nuSigma_f"))?, py::getitem(&t1264, &Py::str("extrapolation"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1265 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1266 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1288 = (t1266).items()?;
        if t1288.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1288[0].clone();
        v_v = t1288[1].clone();
        if ({ let t1267 = v_k.clone(); Py::Bool({ let t1268 = v_outputs.clone(); if !py::contains(&t1268, &t1267)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_132(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t134: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t133 = v_inputs.clone(); f_activation_burnup(py::getitem(&t133, &Py::str("N_T0"))?, py::getitem(&t133, &Py::str("sigma_T"))?, py::getitem(&t133, &Py::str("sigma_P"))?, py::getitem(&t133, &Py::str("phi"))?, py::getitem(&t133, &Py::str("T_half"))?, py::getitem(&t133, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t134 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t135 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t157 = (t135).items()?;
        if t157.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t157[0].clone();
        v_v = t157[1].clone();
        if ({ let t136 = v_k.clone(); Py::Bool({ let t137 = v_outputs.clone(); if !py::contains(&t137, &t136)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1326(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1328: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1327 = v_inputs.clone(); f_currie(py::getitem(&t1327, &Py::str("B"))?, py::method(&t1327, "get", &[Py::str("t"), Py::None])?, py::method(&t1327, "get", &[Py::str("eff"), Py::None])?, py::method(&t1327, "get", &[Py::str("y"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1328 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1329 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1351 = (t1329).items()?;
        if t1351.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1351[0].clone();
        v_v = t1351[1].clone();
        if ({ let t1330 = v_k.clone(); Py::Bool({ let t1331 = v_outputs.clone(); if !py::contains(&t1331, &t1330)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1376(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1378: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1377 = v_inputs.clone(); f_daughter_max_time(py::getitem(&t1377, &Py::str("T_P"))?, py::getitem(&t1377, &Py::str("T_D"))?, py::method(&t1377, "get", &[Py::str("A_P0"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1378 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1379 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1401 = (t1379).items()?;
        if t1401.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1401[0].clone();
        v_v = t1401[1].clone();
        if ({ let t1380 = v_k.clone(); Py::Bool({ let t1381 = v_outputs.clone(); if !py::contains(&t1381, &t1380)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1422(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1424: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1423 = v_inputs.clone(); f_dead_time_nonparalyzable(py::getitem(&t1423, &Py::str("m"))?, py::getitem(&t1423, &Py::str("tau"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1424 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1425 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1447 = (t1425).items()?;
        if t1447.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1447[0].clone();
        v_v = t1447[1].clone();
        if ({ let t1426 = v_k.clone(); Py::Bool({ let t1427 = v_outputs.clone(); if !py::contains(&t1427, &t1426)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1498(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1500: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1499 = v_inputs.clone(); f_dead_time_paralyzable(py::getitem(&t1499, &Py::str("m"))?, py::getitem(&t1499, &Py::str("tau"))?, py::getitem(&t1499, &Py::str("branch"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1500 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1501 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1523 = (t1501).items()?;
        if t1523.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1523[0].clone();
        v_v = t1523[1].clone();
        if ({ let t1502 = v_k.clone(); Py::Bool({ let t1503 = v_outputs.clone(); if !py::contains(&t1503, &t1502)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1563(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1565: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1564 = v_inputs.clone(); f_decay_fit(py::getitem(&t1564, &Py::str("t"))?, py::getitem(&t1564, &Py::str("C"))?, py::getitem(&t1564, &Py::str("t_unit"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1565 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1566 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1588 = (t1566).items()?;
        if t1588.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1588[0].clone();
        v_v = t1588[1].clone();
        if ({ let t1567 = v_k.clone(); Py::Bool({ let t1568 = v_outputs.clone(); if !py::contains(&t1568, &t1567)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1614(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1616: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1615 = v_inputs.clone(); f_decay_heat_energy(py::getitem(&t1615, &Py::str("P0"))?, py::getitem(&t1615, &Py::str("T_op"))?, py::getitem(&t1615, &Py::str("t1"))?, py::getitem(&t1615, &Py::str("t2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1616 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1617 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1639 = (t1617).items()?;
        if t1639.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1639[0].clone();
        v_v = t1639[1].clone();
        if ({ let t1618 = v_k.clone(); Py::Bool({ let t1619 = v_outputs.clone(); if !py::contains(&t1619, &t1618)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1663(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1665: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1664 = v_inputs.clone(); f_decay_heat(py::getitem(&t1664, &Py::str("P0"))?, py::getitem(&t1664, &Py::str("T_op"))?, py::getitem(&t1664, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1665 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1666 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1688 = (t1666).items()?;
        if t1688.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1688[0].clone();
        v_v = t1688[1].clone();
        if ({ let t1667 = v_k.clone(); Py::Bool({ let t1668 = v_outputs.clone(); if !py::contains(&t1668, &t1667)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1713(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1715: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1714 = v_inputs.clone(); f_depletion_constant_flux(py::getitem(&t1714, &Py::str("m0"))?, py::getitem(&t1714, &Py::str("sigma_a"))?, py::getitem(&t1714, &Py::str("sigma_f"))?, py::getitem(&t1714, &Py::str("phi"))?, py::getitem(&t1714, &Py::str("t"))?, py::getitem(&t1714, &Py::str("E_f"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1715 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1716 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1738 = (t1716).items()?;
        if t1738.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1738[0].clone();
        v_v = t1738[1].clone();
        if ({ let t1717 = v_k.clone(); Py::Bool({ let t1718 = v_outputs.clone(); if !py::contains(&t1718, &t1717)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1757(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1759: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1758 = v_inputs.clone(); f_detector_efficiency(py::getitem(&t1758, &Py::str("d"))?, py::getitem(&t1758, &Py::str("a"))?, py::getitem(&t1758, &Py::str("t"))?, py::getitem(&t1758, &Py::str("mu"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1759 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1760 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1782 = (t1760).items()?;
        if t1782.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1782[0].clone();
        v_v = t1782[1].clone();
        if ({ let t1761 = v_k.clone(); Py::Bool({ let t1762 = v_outputs.clone(); if !py::contains(&t1762, &t1761)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1805(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1807: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1806 = v_inputs.clone(); f_diffusion_length(py::getitem(&t1806, &Py::str("D"))?, py::getitem(&t1806, &Py::str("Sigma_a"))?, py::method(&t1806, "get", &[Py::str("tau"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1807 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1808 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1830 = (t1808).items()?;
        if t1830.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1830[0].clone();
        v_v = t1830[1].clone();
        if ({ let t1809 = v_k.clone(); Py::Bool({ let t1810 = v_outputs.clone(); if !py::contains(&t1810, &t1809)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1857(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1859: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1858 = v_inputs.clone(); f_disk_source(py::getitem(&t1858, &Py::str("S_A"))?, py::getitem(&t1858, &Py::str("R"))?, py::getitem(&t1858, &Py::str("h"))?, py::getitem(&t1858, &Py::str("mu"))?, py::getitem(&t1858, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1859 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1860 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1882 = (t1860).items()?;
        if t1882.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1882[0].clone();
        v_v = t1882[1].clone();
        if ({ let t1861 = v_k.clone(); Py::Bool({ let t1862 = v_outputs.clone(); if !py::contains(&t1862, &t1861)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1916(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1918: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1917 = v_inputs.clone(); f_effective_dose(py::getitem(&t1917, &Py::str("H"))?, py::getitem(&t1917, &Py::str("w"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1918 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1919 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1941 = (t1919).items()?;
        if t1941.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1941[0].clone();
        v_v = t1941[1].clone();
        if ({ let t1920 = v_k.clone(); Py::Bool({ let t1921 = v_outputs.clone(); if !py::contains(&t1921, &t1920)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_196(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t198: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t197 = v_inputs.clone(); f_attenuation_fit(py::getitem(&t197, &Py::str("x"))?, py::getitem(&t197, &Py::str("I"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t198 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t199 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t221 = (t199).items()?;
        if t221.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t221[0].clone();
        v_v = t221[1].clone();
        if ({ let t200 = v_k.clone(); Py::Bool({ let t201 = v_outputs.clone(); if !py::contains(&t201, &t200)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1963(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1965: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1964 = v_inputs.clone(); f_effective_half_life(py::getitem(&t1964, &Py::str("T_p"))?, py::getitem(&t1964, &Py::str("T_b"))?, py::method(&t1964, "get", &[Py::str("A0"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1965 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1966 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1988 = (t1966).items()?;
        if t1988.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1988[0].clone();
        v_v = t1988[1].clone();
        if ({ let t1967 = v_k.clone(); Py::Bool({ let t1968 = v_outputs.clone(); if !py::contains(&t1968, &t1967)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2019(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2021: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2020 = v_inputs.clone(); f_elastic_scattering(py::getitem(&t2020, &Py::str("A"))?, py::getitem(&t2020, &Py::str("E"))?, py::getitem(&t2020, &Py::str("theta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2021 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2022 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2044 = (t2022).items()?;
        if t2044.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2044[0].clone();
        v_v = t2044[1].clone();
        if ({ let t2023 = v_k.clone(); Py::Bool({ let t2024 = v_outputs.clone(); if !py::contains(&t2024, &t2023)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2077(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2079: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2078 = v_inputs.clone(); f_swu(py::getitem(&t2078, &Py::str("x_p"))?, py::getitem(&t2078, &Py::str("x_f"))?, py::getitem(&t2078, &Py::str("x_w"))?, py::getitem(&t2078, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2079 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2080 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2102 = (t2080).items()?;
        if t2102.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2102[0].clone();
        v_v = t2102[1].clone();
        if ({ let t2081 = v_k.clone(); Py::Bool({ let t2082 = v_outputs.clone(); if !py::contains(&t2082, &t2081)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2124(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2126: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2125 = v_inputs.clone(); f_equilibrium_ratio(py::getitem(&t2125, &Py::str("T_P"))?, py::getitem(&t2125, &Py::str("T_D"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2126 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2127 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2149 = (t2127).items()?;
        if t2149.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2149[0].clone();
        v_v = t2149[1].clone();
        if ({ let t2128 = v_k.clone(); Py::Bool({ let t2129 = v_outputs.clone(); if !py::contains(&t2129, &t2128)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2170(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2172: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2171 = v_inputs.clone(); f_escape_peaks(py::getitem(&t2171, &Py::str("E"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2172 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2173 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2195 = (t2173).items()?;
        if t2195.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2195[0].clone();
        v_v = t2195[1].clone();
        if ({ let t2174 = v_k.clone(); Py::Bool({ let t2175 = v_outputs.clone(); if !py::contains(&t2175, &t2174)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2238(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2240: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2239 = v_inputs.clone(); f_exponential_pile(py::getitem(&t2239, &Py::str("z"))?, py::getitem(&t2239, &Py::str("phi"))?, py::getitem(&t2239, &Py::str("a"))?, py::getitem(&t2239, &Py::str("b"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2240 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2241 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2263 = (t2241).items()?;
        if t2263.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2263[0].clone();
        v_v = t2263[1].clone();
        if ({ let t2242 = v_k.clone(); Py::Bool({ let t2243 = v_outputs.clone(); if !py::contains(&t2243, &t2242)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2295(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2297: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2296 = v_inputs.clone(); f_flux_from_foil(py::getitem(&t2296, &Py::str("A_d"))?, py::getitem(&t2296, &Py::str("m"))?, py::getitem(&t2296, &Py::str("M"))?, py::getitem(&t2296, &Py::str("theta"))?, py::getitem(&t2296, &Py::str("sigma"))?, py::getitem(&t2296, &Py::str("T_half"))?, py::getitem(&t2296, &Py::str("t_irr"))?, py::getitem(&t2296, &Py::str("t_d"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2297 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2298 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2320 = (t2298).items()?;
        if t2320.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2320[0].clone();
        v_v = t2320[1].clone();
        if ({ let t2299 = v_k.clone(); Py::Bool({ let t2300 = v_outputs.clone(); if !py::contains(&t2300, &t2299)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_23(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t25: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t24 = v_inputs.clone(); f_accumulated_dose(py::getitem(&t24, &Py::str("H0"))?, py::getitem(&t24, &Py::str("T_half"))?, py::getitem(&t24, &Py::str("t"))?, py::method(&t24, "get", &[Py::str("td"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t25 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t26 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t48 = (t26).items()?;
        if t48.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t48[0].clone();
        v_v = t48[1].clone();
        if ({ let t27 = v_k.clone(); Py::Bool({ let t28 = v_outputs.clone(); if !py::contains(&t28, &t27)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2397(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2399: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2398 = v_inputs.clone(); f_flux_shape(py::getitem(&t2398, &Py::str("geometry"))?, py::method(&t2398, "get", &[Py::str("a"), Py::None])?, py::method(&t2398, "get", &[Py::str("R"), Py::None])?, py::method(&t2398, "get", &[Py::str("H"), Py::None])?, py::method(&t2398, "get", &[Py::str("x"), Py::None])?, py::method(&t2398, "get", &[Py::str("r"), Py::None])?, py::method(&t2398, "get", &[Py::str("z"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2399 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2400 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2422 = (t2400).items()?;
        if t2422.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2422[0].clone();
        v_v = t2422[1].clone();
        if ({ let t2401 = v_k.clone(); Py::Bool({ let t2402 = v_outputs.clone(); if !py::contains(&t2402, &t2401)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2457(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2459: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2458 = v_inputs.clone(); f_fuel_cross_sections(py::getitem(&t2458, &Py::str("rho"))?, py::getitem(&t2458, &Py::str("w"))?, py::getitem(&t2458, &Py::str("sigma_a5"))?, py::getitem(&t2458, &Py::str("sigma_f5"))?, py::getitem(&t2458, &Py::str("sigma_a8"))?, py::getitem(&t2458, &Py::str("sigma_aO"))?, py::getitem(&t2458, &Py::str("nu"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2459 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2460 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2482 = (t2460).items()?;
        if t2482.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2482[0].clone();
        v_v = t2482[1].clone();
        if ({ let t2461 = v_k.clone(); Py::Bool({ let t2462 = v_outputs.clone(); if !py::contains(&t2462, &t2461)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2501(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2503: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2502 = v_inputs.clone(); f_fuel_rod_temperatures(py::getitem(&t2502, &Py::str("q_lin"))?, py::getitem(&t2502, &Py::str("R_f"))?, py::getitem(&t2502, &Py::str("k_f"))?, py::getitem(&t2502, &Py::str("h_g"))?, py::getitem(&t2502, &Py::str("t_c"))?, py::getitem(&t2502, &Py::str("k_c"))?, py::getitem(&t2502, &Py::str("h"))?, py::getitem(&t2502, &Py::str("T_cool"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2503 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2504 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2526 = (t2504).items()?;
        if t2526.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2526[0].clone();
        v_v = t2526[1].clone();
        if ({ let t2505 = v_k.clone(); Py::Bool({ let t2506 = v_outputs.clone(); if !py::contains(&t2506, &t2505)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2564(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2566: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2565 = v_inputs.clone(); f_generator(py::getitem(&t2565, &Py::str("A_P0"))?, py::getitem(&t2565, &Py::str("T_P"))?, py::getitem(&t2565, &Py::str("T_D"))?, py::getitem(&t2565, &Py::str("b"))?, py::getitem(&t2565, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2566 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2567 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2589 = (t2567).items()?;
        if t2589.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2589[0].clone();
        v_v = t2589[1].clone();
        if ({ let t2568 = v_k.clone(); Py::Bool({ let t2569 = v_outputs.clone(); if !py::contains(&t2569, &t2568)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2615(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2617: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2616 = v_inputs.clone(); f_hvl_tvl(py::getitem(&t2616, &Py::str("mu"))?, py::method(&t2616, "get", &[Py::str("x"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2617 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2618 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2640 = (t2618).items()?;
        if t2640.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2640[0].clone();
        v_v = t2640[1].clone();
        if ({ let t2619 = v_k.clone(); Py::Bool({ let t2620 = v_outputs.clone(); if !py::contains(&t2620, &t2619)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_263(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t265: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t264 = v_inputs.clone(); f_bare_keff(py::getitem(&t264, &Py::str("D"))?, py::getitem(&t264, &Py::str("Sigma_a"))?, py::getitem(&t264, &Py::str("nuSigma_f"))?, py::getitem(&t264, &Py::str("extrapolation"))?, py::getitem(&t264, &Py::str("geometry"))?, py::method(&t264, "get", &[Py::str("a"), Py::None])?, py::method(&t264, "get", &[Py::str("b"), Py::None])?, py::method(&t264, "get", &[Py::str("c"), Py::None])?, py::method(&t264, "get", &[Py::str("R"), Py::None])?, py::method(&t264, "get", &[Py::str("H"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t265 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t266 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t288 = (t266).items()?;
        if t288.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t288[0].clone();
        v_v = t288[1].clone();
        if ({ let t267 = v_k.clone(); Py::Bool({ let t268 = v_outputs.clone(); if !py::contains(&t268, &t267)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2677(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2679: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2678 = v_inputs.clone(); f_inverse_multiplication(py::getitem(&t2678, &Py::str("x"))?, py::getitem(&t2678, &Py::str("C"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2679 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2680 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2702 = (t2680).items()?;
        if t2702.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2702[0].clone();
        v_v = t2702[1].clone();
        if ({ let t2681 = v_k.clone(); Py::Bool({ let t2682 = v_outputs.clone(); if !py::contains(&t2682, &t2681)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2730(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2732: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2731 = v_inputs.clone(); f_irradiation_time(py::getitem(&t2731, &Py::str("T_half"))?, py::getitem(&t2731, &Py::str("f"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2732 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2733 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2755 = (t2733).items()?;
        if t2755.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2755[0].clone();
        v_v = t2755[1].clone();
        if ({ let t2734 = v_k.clone(); Py::Bool({ let t2735 = v_outputs.clone(); if !py::contains(&t2735, &t2734)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2778(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2780: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2779 = v_inputs.clone(); f_klein_nishina(py::getitem(&t2779, &Py::str("E"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2780 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2781 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2803 = (t2781).items()?;
        if t2803.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2803[0].clone();
        v_v = t2803[1].clone();
        if ({ let t2782 = v_k.clone(); Py::Bool({ let t2783 = v_outputs.clone(); if !py::contains(&t2783, &t2782)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2837(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2839: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2838 = v_inputs.clone(); f_line_source(py::getitem(&t2838, &Py::str("S_L"))?, py::getitem(&t2838, &Py::str("h"))?, py::getitem(&t2838, &Py::str("y1"))?, py::getitem(&t2838, &Py::str("y2"))?, py::getitem(&t2838, &Py::str("mu"))?, py::getitem(&t2838, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2839 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2840 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2862 = (t2840).items()?;
        if t2862.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2862[0].clone();
        v_v = t2862[1].clone();
        if ({ let t2841 = v_k.clone(); Py::Bool({ let t2842 = v_outputs.clone(); if !py::contains(&t2842, &t2841)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2882(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2884: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2883 = v_inputs.clone(); f_log_decrement(py::getitem(&t2883, &Py::str("A"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2884 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2885 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2907 = (t2885).items()?;
        if t2907.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2907[0].clone();
        v_v = t2907[1].clone();
        if ({ let t2886 = v_k.clone(); Py::Bool({ let t2887 = v_outputs.clone(); if !py::contains(&t2887, &t2886)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2929(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2931: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2930 = v_inputs.clone(); f_mass_for_activity(py::getitem(&t2930, &Py::str("nuclide"))?, py::getitem(&t2930, &Py::str("A"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2931 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2932 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2954 = (t2932).items()?;
        if t2954.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2954[0].clone();
        v_v = t2954[1].clone();
        if ({ let t2933 = v_k.clone(); Py::Bool({ let t2934 = v_outputs.clone(); if !py::contains(&t2934, &t2933)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2975(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2977: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2976 = v_inputs.clone(); f_max_clad_temperature(py::getitem(&t2976, &Py::str("q0"))?, py::getitem(&t2976, &Py::str("H"))?, py::getitem(&t2976, &Py::str("H_e"))?, py::getitem(&t2976, &Py::str("m_dot"))?, py::getitem(&t2976, &Py::str("c_p"))?, py::getitem(&t2976, &Py::str("T_in"))?, py::getitem(&t2976, &Py::str("R_o"))?, py::getitem(&t2976, &Py::str("h"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2977 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2978 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3000 = (t2978).items()?;
        if t3000.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3000[0].clone();
        v_v = t3000[1].clone();
        if ({ let t2979 = v_k.clone(); Py::Bool({ let t2980 = v_outputs.clone(); if !py::contains(&t2980, &t2979)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3019(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3021: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3020 = v_inputs.clone(); f_maxwellian_average(py::getitem(&t3020, &Py::str("sigma0"))?, py::getitem(&t3020, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3021 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3022 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3044 = (t3022).items()?;
        if t3044.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3044[0].clone();
        v_v = t3044[1].clone();
        if ({ let t3023 = v_k.clone(); Py::Bool({ let t3024 = v_outputs.clone(); if !py::contains(&t3024, &t3023)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_307(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t309: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t308 = v_inputs.clone(); f_centreline_variable_k(py::getitem(&t308, &Py::str("q_lin"))?, py::getitem(&t308, &Py::str("T_s"))?, py::getitem(&t308, &Py::str("A"))?, py::getitem(&t308, &Py::str("B"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t309 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t310 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t332 = (t310).items()?;
        if t332.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t332[0].clone();
        v_v = t332[1].clone();
        if ({ let t311 = v_k.clone(); Py::Bool({ let t312 = v_outputs.clone(); if !py::contains(&t312, &t311)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3100(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3102: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3101 = v_inputs.clone(); f_moderator_mixture(py::getitem(&t3101, &Py::str("n"))?, py::getitem(&t3101, &Py::str("A"))?, py::getitem(&t3101, &Py::str("sigma_s"))?, py::getitem(&t3101, &Py::str("sigma_a"))?, py::getitem(&t3101, &Py::str("rho"))?, py::getitem(&t3101, &Py::str("M"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3102 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3103 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3125 = (t3103).items()?;
        if t3125.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3125[0].clone();
        v_v = t3125[1].clone();
        if ({ let t3104 = v_k.clone(); Py::Bool({ let t3105 = v_outputs.clone(); if !py::contains(&t3105, &t3104)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3126(mut v_mu: Py, mut v_x: Py, mut v_H0: Py) -> R {
    return Ok(f_multilayer({ let mut out = vec![]; for t3164 in py::iter(&(v_mu.clone()))? { let mut v_m: Py; 
v_m = t3164;
out.push(py::mul(&v_m.clone(), &Py::Float(100.0))?); } Py::list(out) }, { let mut out = vec![]; for t3165 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t3165;
out.push(py::truediv(&v_v.clone(), &Py::Float(100.0))?); } Py::list(out) }, v_H0.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3166(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3168: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3167 = v_inputs.clone(); s_3126(py::getitem(&t3167, &Py::str("mu"))?, py::getitem(&t3167, &Py::str("x"))?, py::method(&t3167, "get", &[Py::str("H0"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3168 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3169 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3191 = (t3169).items()?;
        if t3191.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3191[0].clone();
        v_v = t3191[1].clone();
        if ({ let t3170 = v_k.clone(); Py::Bool({ let t3171 = v_outputs.clone(); if !py::contains(&t3171, &t3170)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3213(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3215: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3214 = v_inputs.clone(); f_plane_source_slab(py::getitem(&t3214, &Py::str("S"))?, py::getitem(&t3214, &Py::str("D"))?, py::getitem(&t3214, &Py::str("Sigma_a"))?, py::getitem(&t3214, &Py::str("a"))?, py::getitem(&t3214, &Py::str("x"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3215 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3216 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3238 = (t3216).items()?;
        if t3238.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3238[0].clone();
        v_v = t3238[1].clone();
        if ({ let t3217 = v_k.clone(); Py::Bool({ let t3218 = v_outputs.clone(); if !py::contains(&t3218, &t3217)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3292(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3294: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3293 = v_inputs.clone(); f_point_dose_rate(py::getitem(&t3293, &Py::str("Gamma"))?, py::getitem(&t3293, &Py::str("A"))?, py::getitem(&t3293, &Py::str("r"))?, py::getitem(&t3293, &Py::str("mu"))?, py::getitem(&t3293, &Py::str("x"))?, py::getitem(&t3293, &Py::str("buildup"))?, py::method(&t3293, "get", &[Py::str("A1"), Py::None])?, py::method(&t3293, "get", &[Py::str("alpha1"), Py::None])?, py::method(&t3293, "get", &[Py::str("alpha2"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3294 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3295 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3317 = (t3295).items()?;
        if t3317.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3317[0].clone();
        v_v = t3317[1].clone();
        if ({ let t3296 = v_k.clone(); Py::Bool({ let t3297 = v_outputs.clone(); if !py::contains(&t3297, &t3296)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3338(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3340: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3339 = v_inputs.clone(); f_point_source_sphere(py::getitem(&t3339, &Py::str("S"))?, py::getitem(&t3339, &Py::str("D"))?, py::getitem(&t3339, &Py::str("Sigma_a"))?, py::getitem(&t3339, &Py::str("R"))?, py::getitem(&t3339, &Py::str("r"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3340 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3341 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3363 = (t3341).items()?;
        if t3363.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3363[0].clone();
        v_v = t3363[1].clone();
        if ({ let t3342 = v_k.clone(); Py::Bool({ let t3343 = v_outputs.clone(); if !py::contains(&t3343, &t3342)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3449(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3451: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3450 = v_inputs.clone(); f_power_after_step_one_group(py::getitem(&t3450, &Py::str("rho"))?, py::getitem(&t3450, &Py::str("beta"))?, py::getitem(&t3450, &Py::str("lam"))?, py::getitem(&t3450, &Py::str("Lambda"))?, py::getitem(&t3450, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3451 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3452 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3474 = (t3452).items()?;
        if t3474.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3474[0].clone();
        v_v = t3474[1].clone();
        if ({ let t3453 = v_k.clone(); Py::Bool({ let t3454 = v_outputs.clone(); if !py::contains(&t3454, &t3453)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3560(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3562: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3561 = v_inputs.clone(); f_power_after_step_multi(py::getitem(&t3561, &Py::str("rho"))?, py::getitem(&t3561, &Py::str("beta"))?, py::getitem(&t3561, &Py::str("lam"))?, py::getitem(&t3561, &Py::str("Lambda"))?, py::getitem(&t3561, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3562 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3563 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3585 = (t3563).items()?;
        if t3585.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3585[0].clone();
        v_v = t3585[1].clone();
        if ({ let t3564 = v_k.clone(); Py::Bool({ let t3565 = v_outputs.clone(); if !py::contains(&t3565, &t3564)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3608(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3610: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3609 = v_inputs.clone(); f_reaction_power(py::getitem(&t3609, &Py::str("E_f"))?, py::getitem(&t3609, &Py::str("N"))?, py::getitem(&t3609, &Py::str("sigma"))?, py::getitem(&t3609, &Py::str("phi"))?, py::method(&t3609, "get", &[Py::str("V"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3610 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3611 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3633 = (t3611).items()?;
        if t3633.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3633[0].clone();
        v_v = t3633[1].clone();
        if ({ let t3612 = v_k.clone(); Py::Bool({ let t3613 = v_outputs.clone(); if !py::contains(&t3613, &t3612)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3667(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3669: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3668 = v_inputs.clone(); f_unit_conversion(py::method(&t3668, "get", &[Py::str("A"), Py::None])?, py::method(&t3668, "get", &[Py::str("D"), Py::None])?, py::method(&t3668, "get", &[Py::str("H"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3669 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3670 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3692 = (t3670).items()?;
        if t3692.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3692[0].clone();
        v_v = t3692[1].clone();
        if ({ let t3671 = v_k.clone(); Py::Bool({ let t3672 = v_outputs.clone(); if !py::contains(&t3672, &t3671)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_370(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t372: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t371 = v_inputs.clone(); f_chain_activity(py::getitem(&t371, &Py::str("nuclide"))?, py::getitem(&t371, &Py::str("A0"))?, py::getitem(&t371, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t372 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t373 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t395 = (t373).items()?;
        if t395.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t395[0].clone();
        v_v = t395[1].clone();
        if ({ let t374 = v_k.clone(); Py::Bool({ let t375 = v_outputs.clone(); if !py::contains(&t375, &t374)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3741(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3743: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3742 = v_inputs.clone(); f_reactivity_from_period(py::getitem(&t3742, &Py::str("beta"))?, py::getitem(&t3742, &Py::str("lam"))?, py::getitem(&t3742, &Py::str("Lambda"))?, py::method(&t3742, "get", &[Py::str("T"), Py::None])?, py::method(&t3742, "get", &[Py::str("T_d"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3743 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3744 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3766 = (t3744).items()?;
        if t3766.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3766[0].clone();
        v_v = t3766[1].clone();
        if ({ let t3745 = v_k.clone(); Py::Bool({ let t3746 = v_outputs.clone(); if !py::contains(&t3746, &t3745)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3846(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3848: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3847 = v_inputs.clone(); f_period_one_group(py::getitem(&t3847, &Py::str("rho"))?, py::getitem(&t3847, &Py::str("beta"))?, py::getitem(&t3847, &Py::str("lam"))?, py::getitem(&t3847, &Py::str("Lambda"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3848 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3849 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3871 = (t3849).items()?;
        if t3871.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3871[0].clone();
        v_v = t3871[1].clone();
        if ({ let t3850 = v_k.clone(); Py::Bool({ let t3851 = v_outputs.clone(); if !py::contains(&t3851, &t3850)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3892(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3894: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3893 = v_inputs.clone(); f_reflected_slab(py::getitem(&t3893, &Py::str("D_c"))?, py::getitem(&t3893, &Py::str("Sigma_ac"))?, py::getitem(&t3893, &Py::str("nuSigma_fc"))?, py::getitem(&t3893, &Py::str("D_r"))?, py::getitem(&t3893, &Py::str("Sigma_ar"))?, py::getitem(&t3893, &Py::str("T_r"))?, py::getitem(&t3893, &Py::str("extrapolation"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3894 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3895 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3917 = (t3895).items()?;
        if t3917.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3917[0].clone();
        v_v = t3917[1].clone();
        if ({ let t3896 = v_k.clone(); Py::Bool({ let t3897 = v_outputs.clone(); if !py::contains(&t3897, &t3896)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3952(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3954: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3953 = v_inputs.clone(); f_samarium(py::getitem(&t3953, &Py::str("Sigma_f"))?, py::getitem(&t3953, &Py::str("phi"))?, py::getitem(&t3953, &Py::str("gamma_Pm"))?, py::getitem(&t3953, &Py::str("sigma_Sm"))?, py::method(&t3953, "get", &[Py::str("t"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3954 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3955 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3977 = (t3955).items()?;
        if t3977.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3977[0].clone();
        v_v = t3977[1].clone();
        if ({ let t3956 = v_k.clone(); Py::Bool({ let t3957 = v_outputs.clone(); if !py::contains(&t3957, &t3956)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4032(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4034: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4033 = v_inputs.clone(); f_shield_thickness(py::getitem(&t4033, &Py::str("Gamma"))?, py::getitem(&t4033, &Py::str("A"))?, py::getitem(&t4033, &Py::str("r"))?, py::getitem(&t4033, &Py::str("mu"))?, py::getitem(&t4033, &Py::str("H_target"))?, py::getitem(&t4033, &Py::str("buildup"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4034 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4035 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4057 = (t4035).items()?;
        if t4057.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4057[0].clone();
        v_v = t4057[1].clone();
        if ({ let t4036 = v_k.clone(); Py::Bool({ let t4037 = v_outputs.clone(); if !py::contains(&t4037, &t4036)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4097(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4099: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4098 = v_inputs.clone(); f_six_factor(py::getitem(&t4098, &Py::str("eta"))?, py::getitem(&t4098, &Py::str("f"))?, py::getitem(&t4098, &Py::str("p"))?, py::getitem(&t4098, &Py::str("eps"))?, py::method(&t4098, "get", &[Py::str("P_FNL"), Py::None])?, py::method(&t4098, "get", &[Py::str("P_TNL"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4099 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4100 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4122 = (t4100).items()?;
        if t4122.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4122[0].clone();
        v_v = t4122[1].clone();
        if ({ let t4101 = v_k.clone(); Py::Bool({ let t4102 = v_outputs.clone(); if !py::contains(&t4102, &t4101)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4149(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4151: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4150 = v_inputs.clone(); f_solid_angle_off_axis(py::getitem(&t4150, &Py::str("d"))?, py::getitem(&t4150, &Py::str("a"))?, py::getitem(&t4150, &Py::str("rho0"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4151 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4152 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4174 = (t4152).items()?;
        if t4174.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4174[0].clone();
        v_v = t4174[1].clone();
        if ({ let t4153 = v_k.clone(); Py::Bool({ let t4154 = v_outputs.clone(); if !py::contains(&t4154, &t4153)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_419(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t421: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t420 = v_inputs.clone(); f_channel_temperature(py::getitem(&t420, &Py::str("q0"))?, py::getitem(&t420, &Py::str("H"))?, py::getitem(&t420, &Py::str("H_e"))?, py::getitem(&t420, &Py::str("m_dot"))?, py::getitem(&t420, &Py::str("c_p"))?, py::getitem(&t420, &Py::str("T_in"))?, py::getitem(&t420, &Py::str("z"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t421 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t422 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t444 = (t422).items()?;
        if t444.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t444[0].clone();
        v_v = t444[1].clone();
        if ({ let t423 = v_k.clone(); Py::Bool({ let t424 = v_outputs.clone(); if !py::contains(&t424, &t423)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4193(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4195: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4194 = v_inputs.clone(); f_solid_angle_axis(py::getitem(&t4194, &Py::str("d"))?, py::getitem(&t4194, &Py::str("a"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4195 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4196 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4218 = (t4196).items()?;
        if t4218.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4218[0].clone();
        v_v = t4218[1].clone();
        if ({ let t4197 = v_k.clone(); Py::Bool({ let t4198 = v_outputs.clone(); if !py::contains(&t4198, &t4197)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4240(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4242: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4241 = v_inputs.clone(); f_specific_activity(py::getitem(&t4241, &Py::str("nuclide"))?, py::getitem(&t4241, &Py::str("m"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4242 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4243 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4265 = (t4243).items()?;
        if t4265.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4265[0].clone();
        v_v = t4265[1].clone();
        if ({ let t4244 = v_k.clone(); Py::Bool({ let t4245 = v_outputs.clone(); if !py::contains(&t4245, &t4244)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4346(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4348: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4347 = v_inputs.clone(); f_stable_period_multi(py::getitem(&t4347, &Py::str("rho"))?, py::getitem(&t4347, &Py::str("beta"))?, py::getitem(&t4347, &Py::str("lam"))?, py::getitem(&t4347, &Py::str("Lambda"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4348 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4349 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4371 = (t4349).items()?;
        if t4371.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4371[0].clone();
        v_v = t4371[1].clone();
        if ({ let t4350 = v_k.clone(); Py::Bool({ let t4351 = v_outputs.clone(); if !py::contains(&t4351, &t4350)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4392(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4394: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4393 = v_inputs.clone(); f_stay_time(py::getitem(&t4393, &Py::str("H0"))?, py::getitem(&t4393, &Py::str("T_half"))?, py::getitem(&t4393, &Py::str("D_lim"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4394 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4395 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4417 = (t4395).items()?;
        if t4417.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4417[0].clone();
        v_v = t4417[1].clone();
        if ({ let t4396 = v_k.clone(); Py::Bool({ let t4397 = v_outputs.clone(); if !py::contains(&t4397, &t4396)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4436(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4438: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4437 = v_inputs.clone(); f_thermal_speeds(py::getitem(&t4437, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4438 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4439 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4461 = (t4439).items()?;
        if t4461.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4461[0].clone();
        v_v = t4461[1].clone();
        if ({ let t4440 = v_k.clone(); Py::Bool({ let t4441 = v_outputs.clone(); if !py::contains(&t4441, &t4440)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4501(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4503: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4502 = v_inputs.clone(); f_three_member_chain(py::getitem(&t4502, &Py::str("T1"))?, py::getitem(&t4502, &Py::str("T2"))?, py::getitem(&t4502, &Py::str("T3"))?, py::getitem(&t4502, &Py::str("A10"))?, py::getitem(&t4502, &Py::str("A20"))?, py::getitem(&t4502, &Py::str("A30"))?, py::getitem(&t4502, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4503 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4504 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4526 = (t4504).items()?;
        if t4526.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4526[0].clone();
        v_v = t4526[1].clone();
        if ({ let t4505 = v_k.clone(); Py::Bool({ let t4506 = v_outputs.clone(); if !py::contains(&t4506, &t4505)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4545(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4547: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4546 = v_inputs.clone(); f_transit_dose(py::getitem(&t4546, &Py::str("Gamma"))?, py::getitem(&t4546, &Py::str("A"))?, py::getitem(&t4546, &Py::str("d"))?, py::getitem(&t4546, &Py::str("v"))?, py::getitem(&t4546, &Py::str("L"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4547 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4548 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4570 = (t4548).items()?;
        if t4570.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4570[0].clone();
        v_v = t4570[1].clone();
        if ({ let t4549 = v_k.clone(); Py::Bool({ let t4550 = v_outputs.clone(); if !py::contains(&t4550, &t4549)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4621(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4623: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4622 = v_inputs.clone(); f_two_component_fit(py::getitem(&t4622, &Py::str("t"))?, py::getitem(&t4622, &Py::str("C"))?, py::getitem(&t4622, &Py::str("T1"))?, py::getitem(&t4622, &Py::str("T2"))?, py::getitem(&t4622, &Py::str("t_unit"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4623 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4624 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4646 = (t4624).items()?;
        if t4646.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4646[0].clone();
        v_v = t4646[1].clone();
        if ({ let t4625 = v_k.clone(); Py::Bool({ let t4626 = v_outputs.clone(); if !py::contains(&t4626, &t4625)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4689(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4691: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4690 = v_inputs.clone(); f_two_group_keff(py::getitem(&t4690, &Py::str("geometry"))?, py::getitem(&t4690, &Py::str("D1"))?, py::getitem(&t4690, &Py::str("D2"))?, py::getitem(&t4690, &Py::str("Sigma_a1"))?, py::getitem(&t4690, &Py::str("Sigma_12"))?, py::getitem(&t4690, &Py::str("Sigma_a2"))?, py::getitem(&t4690, &Py::str("nuSigma_f1"))?, py::getitem(&t4690, &Py::str("nuSigma_f2"))?, py::method(&t4690, "get", &[Py::str("a"), Py::None])?, py::method(&t4690, "get", &[Py::str("b"), Py::None])?, py::method(&t4690, "get", &[Py::str("c"), Py::None])?, py::method(&t4690, "get", &[Py::str("R"), Py::None])?, py::method(&t4690, "get", &[Py::str("H"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4691 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4692 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4714 = (t4692).items()?;
        if t4714.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4714[0].clone();
        v_v = t4714[1].clone();
        if ({ let t4693 = v_k.clone(); Py::Bool({ let t4694 = v_outputs.clone(); if !py::contains(&t4694, &t4693)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_470(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t472: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t471 = v_inputs.clone(); f_collisions_to_thermalize(py::getitem(&t471, &Py::str("A"))?, py::getitem(&t471, &Py::str("E0"))?, py::getitem(&t471, &Py::str("Ef"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t472 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t473 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t495 = (t473).items()?;
        if t495.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t495[0].clone();
        v_v = t495[1].clone();
        if ({ let t474 = v_k.clone(); Py::Bool({ let t475 = v_outputs.clone(); if !py::contains(&t475, &t474)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4759(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4761: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4760 = v_inputs.clone(); f_xenon_after_shutdown(py::getitem(&t4760, &Py::str("Sigma_f"))?, py::getitem(&t4760, &Py::str("phi"))?, py::getitem(&t4760, &Py::str("gamma_I"))?, py::getitem(&t4760, &Py::str("gamma_X"))?, py::getitem(&t4760, &Py::str("sigma_X"))?, py::getitem(&t4760, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4761 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4762 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4784 = (t4762).items()?;
        if t4784.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4784[0].clone();
        v_v = t4784[1].clone();
        if ({ let t4763 = v_k.clone(); Py::Bool({ let t4764 = v_outputs.clone(); if !py::contains(&t4764, &t4763)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4815(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4817: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4816 = v_inputs.clone(); f_xenon_equilibrium(py::getitem(&t4816, &Py::str("Sigma_f"))?, py::getitem(&t4816, &Py::str("phi"))?, py::getitem(&t4816, &Py::str("gamma_I"))?, py::getitem(&t4816, &Py::str("gamma_X"))?, py::getitem(&t4816, &Py::str("sigma_X"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4817 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4818 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4840 = (t4818).items()?;
        if t4840.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4840[0].clone();
        v_v = t4840[1].clone();
        if ({ let t4819 = v_k.clone(); Py::Bool({ let t4820 = v_outputs.clone(); if !py::contains(&t4820, &t4819)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_516(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t518: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t517 = v_inputs.clone(); f_committed_dose(py::getitem(&t517, &Py::str("I"))?, py::getitem(&t517, &Py::str("e"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t518 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t519 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t541 = (t519).items()?;
        if t541.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t541[0].clone();
        v_v = t541[1].clone();
        if ({ let t520 = v_k.clone(); Py::Bool({ let t521 = v_outputs.clone(); if !py::contains(&t521, &t520)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_542(mut v_I: Py, mut v_f: Py, mut v_Tb: Py, mut v_t_unit: Py, mut v_T_p: Py, mut v_E: Py, mut v_w_R: Py, mut v_m: Py, mut v_tau: Py) -> R {
    let mut v_H: Py = Py::None;
    let mut v_U: Py = Py::None;
    let mut v_lp: Py = Py::None;
    let _ = f__need({ let t543 = v_t_unit.clone(); Py::Bool({ let t544 = c__TU()?; if py::contains(&t544, &t543)? { true } else { false } }) }, Py::str("t_unit must be s, min, h, d or y"))?;
    let _ = f__pos(Py::str("the intake, physical half-life, energy, weighting factor, organ mass and integration period"), Py::tuple(vec![v_I.clone(), v_T_p.clone(), v_E.clone(), v_w_R.clone(), v_m.clone(), v_tau.clone()]))?;
    let _ = f__need({ let t556 = { let t551 = py::len(&v_f.clone())?; Py::Bool({ let t552 = py::len(&v_Tb.clone())?; if py::eq(&t551, &t552) { true } else { false } }) }; if !t556.truthy() { t556 } else { { let t553 = Py::Int(1i64); Py::Bool({ let t554 = py::len(&v_f.clone())?; if py::le(&t553, &t554)? { { let t555 = Py::Int(6i64); if py::le(&t554, &t555)? { true } else { false } } } else { false } }) } } }, Py::str("give one retention fraction per biological half-life (1 to 6)"))?;
    let _ = f__need({ let t563 = Py::Bool(({ let mut out = vec![]; for t557 in py::iter(&(v_f.clone()))? { let mut v_x: Py; 
v_x = t557;
out.push({ let t560 = f__isnum(v_x.clone())?; if !t560.truthy() { t560 } else { { let t558 = v_x.clone(); Py::Bool({ let t559 = Py::Int(0i64); if py::ge(&t558, &t559)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t563.truthy() { t563 } else { { let t561 = py::fsum(&v_f.clone())?; Py::Bool({ let t562 = py::add(&Py::Int(1i64), &Py::Float(1e-12))?; if py::le(&t561, &t562)? { true } else { false } }) } } }, Py::str("retention fractions must be >= 0 and sum to at most 1"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t564 in py::iter(&(v_Tb.clone()))? { let mut v_x: Py; 
v_x = t564;
out.push({ let t567 = f__isnum(v_x.clone())?; if !t567.truthy() { t567 } else { { let t565 = v_x.clone(); Py::Bool({ let t566 = Py::Int(0i64); if py::gt(&t565, &t566)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("biological half-lives must be positive"))?;
    v_lp = py::truediv(&c_LN2()?, &v_T_p.clone())?;
    v_U = py::fsum(&{ let mut out = vec![]; for t568 in py::iter(&(py::zip(&[v_f.clone(), v_Tb.clone()])?))? { let mut v_Tb_i: Py; let mut v_fi: Py; 
let t569 = (t568).items()?;
if t569.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_fi = t569[0].clone();
v_Tb_i = t569[1].clone();
out.push(py::truediv(&py::mul(&py::mul(&v_I.clone(), &v_fi.clone())?, &py::neg(&py::math1("expm1", &py::mul(&py::neg(&py::add(&v_lp.clone(), &py::truediv(&c_LN2()?, &py::mul(&v_Tb_i.clone(), &py::getitem(&c__TU()?, &v_t_unit.clone())?)?)?)?)?, &v_tau.clone())?)?)?)?, &py::add(&v_lp.clone(), &py::truediv(&c_LN2()?, &py::mul(&v_Tb_i.clone(), &py::getitem(&c__TU()?, &v_t_unit.clone())?)?)?)?)?); } Py::list(out) })?;
    v_H = py::truediv(&py::mul(&py::mul(&v_U.clone(), &v_E.clone())?, &v_w_R.clone())?, &v_m.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("U"), v_U.clone()), (Py::str("H"), v_H.clone()), (Py::str("H_mSv"), py::mul(&v_H.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_582(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t584: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t583 = v_inputs.clone(); s_542(py::getitem(&t583, &Py::str("I"))?, py::getitem(&t583, &Py::str("f"))?, py::getitem(&t583, &Py::str("Tb"))?, py::getitem(&t583, &Py::str("t_unit"))?, py::getitem(&t583, &Py::str("T_p"))?, py::getitem(&t583, &Py::str("E"))?, py::getitem(&t583, &Py::str("w_R"))?, py::getitem(&t583, &Py::str("m"))?, py::getitem(&t583, &Py::str("tau"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t584 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t585 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t607 = (t585).items()?;
        if t607.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t607[0].clone();
        v_v = t607[1].clone();
        if ({ let t586 = v_k.clone(); Py::Bool({ let t587 = v_outputs.clone(); if !py::contains(&t587, &t586)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_626(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t628: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t627 = v_inputs.clone(); f_compton_edge(py::getitem(&t627, &Py::str("E"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t628 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t629 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t651 = (t629).items()?;
        if t651.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t651[0].clone();
        v_v = t651[1].clone();
        if ({ let t630 = v_k.clone(); Py::Bool({ let t631 = v_outputs.clone(); if !py::contains(&t631, &t630)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_678(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t680: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t679 = v_inputs.clone(); f_constant_power_depletion(py::getitem(&t679, &Py::str("m0"))?, py::getitem(&t679, &Py::str("P"))?, py::getitem(&t679, &Py::str("t"))?, py::getitem(&t679, &Py::str("E_f"))?, py::getitem(&t679, &Py::str("sigma_a"))?, py::getitem(&t679, &Py::str("sigma_f"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t680 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t681 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t703 = (t681).items()?;
        if t703.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t703[0].clone();
        v_v = t703[1].clone();
        if ({ let t682 = v_k.clone(); Py::Bool({ let t683 = v_outputs.clone(); if !py::contains(&t683, &t682)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_734(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t736: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t735 = v_inputs.clone(); f_rod_worth(py::getitem(&t735, &Py::str("rho_tot"))?, py::getitem(&t735, &Py::str("H"))?, py::getitem(&t735, &Py::str("x"))?, py::method(&t735, "get", &[Py::str("beta"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t736 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t737 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t759 = (t737).items()?;
        if t759.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t759[0].clone();
        v_v = t759[1].clone();
        if ({ let t738 = v_k.clone(); Py::Bool({ let t739 = v_outputs.clone(); if !py::contains(&t739, &t738)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_80(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t82: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t81 = v_inputs.clone(); f_activation(py::getitem(&t81, &Py::str("m"))?, py::getitem(&t81, &Py::str("M"))?, py::getitem(&t81, &Py::str("theta"))?, py::getitem(&t81, &Py::str("sigma"))?, py::getitem(&t81, &Py::str("phi"))?, py::getitem(&t81, &Py::str("T_half"))?, py::getitem(&t81, &Py::str("t_irr"))?, py::method(&t81, "get", &[Py::str("t_d"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t82 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t83 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t105 = (t83).items()?;
        if t105.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t105[0].clone();
        v_v = t105[1].clone();
        if ({ let t84 = v_k.clone(); Py::Bool({ let t85 = v_outputs.clone(); if !py::contains(&t85, &t84)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_806(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t808: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t807 = v_inputs.clone(); f_core_power(py::getitem(&t807, &Py::str("geometry"))?, py::getitem(&t807, &Py::str("Sigma_f"))?, py::getitem(&t807, &Py::str("E_f"))?, py::method(&t807, "get", &[Py::str("phi_max"), Py::None])?, py::method(&t807, "get", &[Py::str("P"), Py::None])?, py::method(&t807, "get", &[Py::str("R"), Py::None])?, py::method(&t807, "get", &[Py::str("H"), Py::None])?, py::method(&t807, "get", &[Py::str("a"), Py::None])?, py::method(&t807, "get", &[Py::str("b"), Py::None])?, py::method(&t807, "get", &[Py::str("c"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t808 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t809 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t831 = (t809).items()?;
        if t831.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t831[0].clone();
        v_v = t831[1].clone();
        if ({ let t810 = v_k.clone(); Py::Bool({ let t811 = v_outputs.clone(); if !py::contains(&t811, &t810)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_859(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t861: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t860 = v_inputs.clone(); f_count_rate_point_source(py::getitem(&t860, &Py::str("A"))?, py::getitem(&t860, &Py::str("y"))?, py::getitem(&t860, &Py::str("d"))?, py::getitem(&t860, &Py::str("a"))?, py::getitem(&t860, &Py::str("eps_int"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t861 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t862 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t884 = (t862).items()?;
        if t884.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t884[0].clone();
        v_v = t884[1].clone();
        if ({ let t863 = v_k.clone(); Py::Bool({ let t864 = v_outputs.clone(); if !py::contains(&t864, &t863)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_915(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t917: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t916 = v_inputs.clone(); f_count_time_for_precision(py::getitem(&t916, &Py::str("Rg"))?, py::getitem(&t916, &Py::str("Rb"))?, py::getitem(&t916, &Py::str("r"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t917 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t918 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t940 = (t918).items()?;
        if t940.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t940[0].clone();
        v_v = t940[1].clone();
        if ({ let t919 = v_k.clone(); Py::Bool({ let t920 = v_outputs.clone(); if !py::contains(&t920, &t919)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_962(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t964: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t963 = v_inputs.clone(); f_count_time_split(py::getitem(&t963, &Py::str("T"))?, py::getitem(&t963, &Py::str("Rg"))?, py::getitem(&t963, &Py::str("Rb"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t964 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t965 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t987 = (t965).items()?;
        if t987.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t987[0].clone();
        v_v = t987[1].clone();
        if ({ let t966 = v_k.clone(); Py::Bool({ let t967 = v_outputs.clone(); if !py::contains(&t967, &t966)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}
