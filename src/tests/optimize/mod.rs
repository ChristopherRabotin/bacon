use crate::optimize::{curve_fit, curve_fit_jac, linear_fit};
use nalgebra::{VectorN, U1, U2};
use rand::prelude::*;
use std::f64;

fn model(x: f64, params: &VectorN<f64, U1>) -> f64 {
    x.powi(2) + params[0]
}

fn model_jac(_x: f64, _params: &VectorN<f64, U1>) -> VectorN<f64, U1> {
    VectorN::<f64, U1>::from_column_slice(&[1.0])
}

fn gaussian(x: f64, params: &VectorN<f64, U2>) -> f64 {
    (-0.5 * ((x - params[0]) / params[1]).powi(2)).exp()
        / (params[1] * (2.0 * f64::consts::PI).sqrt())
}

fn gaussian_jac(x: f64, params: &VectorN<f64, U2>) -> VectorN<f64, U2> {
    VectorN::<f64, U2>::from_column_slice(&[
        (-0.5 * ((x - params[0]) / params[1]).powi(2)).exp() * (x - params[0])
            / (params[1].powi(2) * (2.0 * f64::consts::PI).sqrt()),
        (-0.5 * ((x - params[0]) / params[1]).powi(2)).exp()
            / (params[1].powi(4) * (2.0 * f64::consts::PI).sqrt())
            * (x - params[0]).powi(2),
        -(-0.5 * ((x - params[0]) / params[1]).powi(2)).exp()
            / (params[1].powi(2) * (2.0 * f64::consts::PI).sqrt()),
    ])
}

#[test]
fn test_curve_fit() {
    let xs: Vec<f64> = (-50..=50).map(|x| x as f64 / 50.0).collect();
    let ys: Vec<f64> = xs
        .iter()
        .map(|&x| model(x, &VectorN::<f64, U1>::from_column_slice(&[2.0])))
        .collect();

    let solution = curve_fit(model, &xs, &ys, &[1.0], 1e-7, 0.1, 2.0).unwrap();
    assert!(approx_eq!(f64, solution[0], 2.0, epsilon = 1e-3));
    let solution = curve_fit(model, &xs, &ys, &[3.0], 1e-7, 0.1, 2.0).unwrap();
    assert!(approx_eq!(f64, solution[0], 2.0, epsilon = 1e-3));

    let ys: Vec<f64> = xs
        .iter()
        .map(|&x| gaussian(x, &VectorN::<f64, U2>::from_column_slice(&[5.0, 0.5])))
        .collect();
    let solution = curve_fit(gaussian, &xs, &ys, &[5.5, 1.0], 1e-7, 0.1, 2.0).unwrap();
    assert!(approx_eq!(f64, solution[0], 5.0, epsilon = 0.5));
    assert!(approx_eq!(f64, solution[1], 0.5, epsilon = 0.5));
}

#[test]
fn test_curve_fit_jac() {
    let xs: Vec<f64> = (-50..=50).map(|x| x as f64 / 50.0).collect();
    let ys: Vec<f64> = xs
        .iter()
        .map(|&x| model(x, &VectorN::<f64, U1>::from_column_slice(&[2.0])))
        .collect();

    let solution = curve_fit_jac(model, &xs, &ys, &[1.0], 1e-7, model_jac, 2.0).unwrap();
    assert!(approx_eq!(f64, solution[0], 2.0, epsilon = 1e-3));
    let solution = curve_fit_jac(model, &xs, &ys, &[3.0], 1e-7, model_jac, 2.0).unwrap();
    assert!(approx_eq!(f64, solution[0], 2.0, epsilon = 1e-3));

    let ys: Vec<f64> = xs
        .iter()
        .map(|&x| gaussian(x, &VectorN::<f64, U2>::from_column_slice(&[5.0, 0.5])))
        .collect();
    let solution = curve_fit_jac(gaussian, &xs, &ys, &[5.5, 1.0], 1e-7, gaussian_jac, 2.0).unwrap();
    assert!(approx_eq!(f64, solution[0], 5.0, epsilon = 1.0));
    assert!(approx_eq!(f64, solution[1], 0.5, epsilon = 1.0));
}

#[test]
fn test_linear_fit() {
    let line = polynomial![4.0, -1.0];
    let mut xs: Vec<f64> = vec![];
    let mut ys: Vec<f64> = vec![];

    for i in -5..=5 {
        xs.push(i as f64);
        ys.push(line.evaluate(i as f64));
    }

    let fit = linear_fit(&xs, &ys).unwrap();

    assert!(approx_eq!(
        f64,
        line.get_coefficient(0),
        fit.get_coefficient(0),
        epsilon = 1e-5
    ));
    assert!(approx_eq!(
        f64,
        line.get_coefficient(1),
        fit.get_coefficient(1),
        epsilon = 1e-5
    ));

    xs.clear();
    ys.clear();

    let distr = rand::distributions::Uniform::new_inclusive(-0.1, 0.1);
    let mut rng = thread_rng();
    for i in -5..=5 {
        xs.push(i as f64);
        let wiggle: f64 = rng.sample(distr);
        ys.push(wiggle + line.evaluate(i as f64));
    }

    let fit = linear_fit(&xs, &ys).unwrap();

    assert!(approx_eq!(
        f64,
        line.get_coefficient(0),
        fit.get_coefficient(0),
        epsilon = 0.1
    ));
    assert!(approx_eq!(
        f64,
        line.get_coefficient(1),
        fit.get_coefficient(1),
        epsilon = 0.1
    ));

    xs.clear();
    ys.clear();

    for i in -5..=5 {
        xs.push(i as f64);
        ys.push(((i as f64).exp() * 2.0).ln());
    }

    let fit = linear_fit(&xs, &ys).unwrap();

    assert!(approx_eq!(
        f64,
        2.0_f64.ln(),
        fit.get_coefficient(0),
        epsilon = 0.1
    ));
    assert!(approx_eq!(f64, 1.0, fit.get_coefficient(1), epsilon = 0.1));
}
