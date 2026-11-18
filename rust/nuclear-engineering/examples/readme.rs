//! Nuclear Engineering: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p nuclear-engineering-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&nuclear_engineering_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "nuc_activation_with_burnup", "inputs": {"N_T0": "1e+22", "sigma_T": "3.718e-27 m^2", "sigma_P": "2e-28 m^2", "phi": "1e+14 1/(cm^2*s)", "T_half": "166346024.4 s", "t": "31557600 s"}}));
    println!();
    show(json!({"key": "nuc_compton_edge", "inputs": {"E": "662 keV"}}));
    println!();
    show(json!({"key": "nuc_committed_dose_coefficient", "inputs": {"I": "1000000 (1/s)", "e": "2.2e-08 (J/kg)/(1/s)"}}));
    println!();
    show(json!({"key": "nuc_enrichment_swu", "inputs": {"x_p": "0.045", "x_f": "0.00711", "x_w": "0.0025", "P": "1 kg"}}));
    println!();
    show(json!({"key": "nuc_logarithmic_energy_decrement", "inputs": {"A": "12"}}));
    println!();
    show(json!({"key": "nuc_control_rod_worth", "inputs": {"rho_tot": "0.01", "H": "3.66 m", "x": "1.83 m"}}));
    println!();
    show(json!({"key": "nuc_inverse_multiplication", "inputs": {"x": [0, 20, 40, 60, 80], "C": [100, 125, 167, 250, 500]}}));
    println!();
    show(json!({"key": "nuc_half_value_layer", "inputs": {"mu": "0.656 1/cm"}}));
    println!();
    show(json!({"key": "nuc_accumulated_dose_decaying_source", "inputs": {"H0": "0.001 (J/kg)/h", "T_half": "6586.2 s", "t": "2 h"}}));
    println!();
    show(json!({"key": "nuc_channel_coolant_temperature", "inputs": {"q0": "43 kW/m", "H": "3.66 m", "H_e": "4 m", "m_dot": "0.3 kg/s", "c_p": "5500 J/(kg*K)", "T_in": "565 K", "z": "0 m"}}));
    println!();
    show(json!({"key": "nuc_specific_activity", "inputs": {"nuclide": "Co-60", "m": "1 g"}}));
    println!();
    show(json!({"key": "nuc_count_time_for_precision", "inputs": {"Rg": "100 (1/s)", "Rb": "25 (1/s)", "r": "0.01"}}));
    println!();
    show(json!({"key": "nuc_committed_dose_retention", "inputs": {"I": "1000000 (1/s)", "f": [0.3], "Tb": [80], "t_unit": "d", "T_p": "692988.48 s", "E": "0.18 MeV", "w_R": "1", "m": "20 g", "tau": "1577880000 s"}}));
    println!();
    show(json!({"key": "nuc_collisions_to_thermalize", "inputs": {"A": "12", "E0": "2 MeV", "Ef": "0.025 eV"}}));
    println!();
    show(json!({"key": "nuc_power_after_step_one_group", "inputs": {"rho": "0.001", "beta": "0.0065", "lam": "0.08 1/s", "Lambda": "0.0001 s", "t": "10 s"}}));
}
