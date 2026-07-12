/// Pure function: Convert Julian day number to ephemeris date components.
/// Uses the standard astronomical Julian day to Gregorian calendar algorithm.
pub fn julian_day_to_date(julian_day: f64) -> (i32, u8, u8) {
    let jd = julian_day + 0.5;
    let z = jd as i64;
    let _fractional = jd - z as f64;

    let a = if z < 2299161 {
        z
    } else {
        let alpha = ((z as f64 - 1867216.25) / 36524.25) as i64;
        z + 1 + alpha - alpha / 4
    };

    let b = a + 1524;
    let c = ((b as f64 - 122.1) / 365.25) as i64;
    let d = (365.25 * c as f64) as i64;
    let e = ((b - d) as f64 / 30.6001) as i64;

    let day = (b - d - (30.6001 * e as f64) as i64) as u8;
    let month = if e < 14 {
        (e - 1) as u8
    } else {
        (e - 13) as u8
    };
    let year = if month > 2 { c - 4716 } else { c - 4715 };

    (year as i32, month, day)
}

/// Pure function: Compute VSOP87 approximation for a planet.
pub fn compute_vsop87_approximation(julian_day: f64, planet_index: u8) -> f64 {
    let t = (julian_day - 2451545.0) / 36525.0;
    let base_longitude = match planet_index {
        0 => 357.529 + 35999.05 * t, // Mercury
        1 => 181.980 + 58517.82 * t, // Venus
        2 => 100.464 + 35999.37 * t, // Earth
        3 => 355.433 + 19140.30 * t, // Mars
        4 => 34.351 + 3034.91 * t,   // Jupiter
        5 => 49.944 + 1222.11 * t,   // Saturn
        6 => 313.232 + 428.47 * t,   // Uranus
        7 => 304.880 + 218.49 * t,   // Neptune
        _ => 0.0,
    };
    base_longitude.rem_euclid(360.0)
}

/// Pure function: Convert ecliptic longitude to zodiac sign index.
pub fn longitude_to_sign_index(ecliptic_longitude: f64) -> u8 {
    let normalized = ecliptic_longitude.rem_euclid(360.0);
    (normalized / 30.0) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn julian_day_to_date_basic() {
        let (year, month, day) = julian_day_to_date(2451545.0);
        assert_eq!(year, 2000);
        assert_eq!(month, 1);
        assert_eq!(day, 1);
    }

    #[test]
    fn compute_vsop87_approximation_basic() {
        let longitude = compute_vsop87_approximation(2451545.0, 2);
        assert!(longitude >= 0.0 && longitude < 360.0);
    }

    #[test]
    fn longitude_to_sign_index_basic() {
        assert_eq!(longitude_to_sign_index(0.0), 0);
        assert_eq!(longitude_to_sign_index(30.0), 1);
        assert_eq!(longitude_to_sign_index(359.0), 11);
    }
}
