/// Pure function: Compute house cusp from ascendant and house number.
pub fn compute_house_cusp(ascendant_longitude: f64, house_number: u8) -> f64 {
    let offset = (house_number as f64 - 1.0) * 30.0;
    (ascendant_longitude + offset).rem_euclid(360.0)
}

/// Pure function: Determine which house a planet occupies.
pub fn determine_planet_house(planet_longitude: f64, house_cusps: &[f64; 12]) -> u8 {
    for (index, &cusp) in house_cusps.iter().enumerate() {
        let next_cusp = house_cusps[(index + 1) % 12];
        if cusp <= next_cusp {
            if planet_longitude >= cusp && planet_longitude < next_cusp {
                return (index + 1) as u8;
            }
        } else if planet_longitude >= cusp || planet_longitude < next_cusp {
            return (index + 1) as u8;
        }
    }
    1
}

/// Pure function: Compute aspect between two planetary positions.
pub fn compute_planetary_aspect(left_longitude: f64, right_longitude: f64) -> f64 {
    let difference = (left_longitude - right_longitude).abs();
    if difference > 180.0 {
        360.0 - difference
    } else {
        difference
    }
}

/// Pure function: Classify aspect type by angle.
pub fn classify_aspect_by_angle(aspect_angle: f64) -> &'static str {
    let normalized = aspect_angle.rem_euclid(360.0);
    let orb_from_nearest = normalized.min(360.0 - normalized);
    if orb_from_nearest < 5.0 {
        "conjunction"
    } else if (orb_from_nearest - 60.0).abs() < 5.0 {
        "sextile"
    } else if (orb_from_nearest - 90.0).abs() < 5.0 {
        "square"
    } else if (orb_from_nearest - 120.0).abs() < 5.0 {
        "trine"
    } else if (orb_from_nearest - 150.0).abs() < 5.0 {
        "quincunx"
    } else if (orb_from_nearest - 180.0).abs() < 5.0 {
        "opposition"
    } else {
        "minor"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_house_cusp_basic() {
        assert_eq!(compute_house_cusp(0.0, 1), 0.0);
        assert_eq!(compute_house_cusp(0.0, 2), 30.0);
        assert_eq!(compute_house_cusp(10.0, 1), 10.0);
    }

    #[test]
    fn determine_planet_house_basic() {
        let cusps = [
            0.0, 30.0, 60.0, 90.0, 120.0, 150.0, 180.0, 210.0, 240.0, 270.0, 300.0, 330.0,
        ];
        assert_eq!(determine_planet_house(15.0, &cusps), 1);
        assert_eq!(determine_planet_house(45.0, &cusps), 2);
    }

    #[test]
    fn compute_planetary_aspect_basic() {
        assert_eq!(compute_planetary_aspect(0.0, 0.0), 0.0);
        assert_eq!(compute_planetary_aspect(0.0, 60.0), 60.0);
        assert_eq!(compute_planetary_aspect(0.0, 300.0), 60.0);
    }

    #[test]
    fn classify_aspect_by_angle_basic() {
        assert_eq!(classify_aspect_by_angle(0.0), "conjunction");
        assert_eq!(classify_aspect_by_angle(2.0), "conjunction");
        assert_eq!(classify_aspect_by_angle(60.0), "sextile");
        assert_eq!(classify_aspect_by_angle(90.0), "square");
        assert_eq!(classify_aspect_by_angle(120.0), "trine");
        assert_eq!(classify_aspect_by_angle(180.0), "opposition");
        assert_eq!(classify_aspect_by_angle(150.0), "quincunx");
        assert_eq!(classify_aspect_by_angle(30.0), "minor");
    }
}
