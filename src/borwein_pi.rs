pub fn approximate_pi(n: u32) {
    let mut yk: f64 = y(0, 0.0);
    let mut ak: f64 = a(0, 0.0, 0.0);
    let mut pi_approx: f64 = 1.0 / ak;
    println!("pi=      {:.14}", std::f64::consts::PI);
    for k in 1..=n {
        println!("1/a[{km1}] = {pi_approx:.14}", km1 = k - 1);
        yk = y(k, yk);
        ak = a(k, ak, yk);
        pi_approx = 1.0 / ak;
    }
}

fn f(x: f64) -> f64 {
    return (1.0 - x.powf(4.0)).powf(0.25);
}

fn y(k: u32, ykm1: f64) -> f64 {
    if k == 0 {
        return 2.0_f64.sqrt() - 1.0;
    } else {
        let yk: f64 = (1.0 - f(ykm1)) / (1.0 + f(ykm1));
        return yk;
    }
}

fn a(k: u32, akm1: f64, yk: f64) -> f64 {
    if k == 0 {
        return 6.0 - 4.0 * 2.0_f64.sqrt();
    } else {
        let ak: f64 = akm1 * (1.0 + yk).powf(4.0)
            - 2.0_f64.powf(2.0 * k as f64 + 1.0) * yk * (1.0 + yk + yk.powf(2.0));
        return ak;
    }
}
