use pyo3::prelude::*;

#[pyfunction]
fn simulate(
    attacker_units_str: &str,
    defender_units_str: &str,
    sim_count: u32,
) -> PyResult<Vec<u32>> {
    let attacker_units = aa_rs::create_units_vector(&attacker_units_str);
    let defender_units = aa_rs::create_units_vector(&defender_units_str);

    let result = aa_rs::simulate(attacker_units, defender_units, sim_count);

    let output = vec![result.attacker_wins, result.defender_wins, result.draws];
    Ok(output)
}

/// A Python module implemented in Rust.
#[pymodule]
fn aa_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(simulate, m)?)?;
    Ok(())
}
