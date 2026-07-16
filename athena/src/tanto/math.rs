//! Tanto math operations dispatch.
//!
//! Provides `compute_named()` for dispatching named operations with parsed arguments,
//! and `format_f64()` for deterministic float formatting.

use super::get_constant;

/// Dispatch named operations with parsed arguments.
///
/// Returns the computed value if the operation is recognized.
/// Supports: trig functions, rounding, stats, unit conversions, physics.
pub fn compute_named(op: &[u8], args: &[f64]) -> Option<f64> {
    let s = std::str::from_utf8(op).ok()?;
    let n = args.len();
    match s {
        // Core arithmetic
        "add" if n == 2 => Some(args[0] + args[1]),
        "sub" if n == 2 => Some(args[0] - args[1]),
        "mul" if n == 2 => Some(args[0] * args[1]),
        "div" if n == 2 && args[1] != 0.0 => Some(args[0] / args[1]),
        "neg" if n == 1 => Some(-args[0]),

        // Math functions
        "abs" if n == 1 => Some(args[0].abs()),
        "hypot" if n == 2 => Some((args[0] * args[0] + args[1] * args[1]).sqrt()),
        "sqrt" if n == 1 => {
            if args[0] < 0.0 {
                None
            } else {
                Some(args[0].sqrt())
            }
        }

        // Trigonometry (radians)
        "sin" if n == 1 => Some(args[0].sin()),
        "cos" if n == 1 => Some(args[0].cos()),
        "tan" if n == 1 => Some(args[0].tan()),
        "asin" if n == 1 => {
            if args[0] < -1.0 || args[0] > 1.0 {
                None
            } else {
                Some(args[0].asin())
            }
        }
        "acos" if n == 1 => {
            if args[0] < -1.0 || args[0] > 1.0 {
                None
            } else {
                Some(args[0].acos())
            }
        }
        "atan" if n == 1 => Some(args[0].atan()),
        "atan2" if n == 2 => Some(args[1].atan2(args[0])),

        // Exponentials and logs
        "exp" if n == 1 => Some(args[0].exp()),
        "ln" | "log" if n == 1 => {
            if args[0] <= 0.0 {
                None
            } else {
                Some(args[0].ln())
            }
        }
        "log10" if n == 1 => {
            if args[0] <= 0.0 {
                None
            } else {
                Some(args[0].log10())
            }
        }
        "log2" if n == 1 => {
            if args[0] <= 0.0 {
                None
            } else {
                Some(args[0].log2())
            }
        }
        "pow" if n == 2 => Some(args[0].powf(args[1])),

        // Rounding
        "round" if n == 1 => Some(args[0].round()),
        "round" if n == 2 => {
            let factor = 10.0_f64.powf(args[1]);
            Some((args[0] * factor).round() / factor)
        }
        "floor" if n == 1 => Some(args[0].floor()),
        "ceil" if n == 1 => Some(args[0].ceil()),

        // Statistics
        "min" if n >= 2 => {
            let mut m = args[0];
            for &a in &args[1..] {
                if a < m {
                    m = a;
                }
            }
            Some(m)
        }
        "max" if n >= 2 => {
            let mut m = args[0];
            for &a in &args[1..] {
                if a > m {
                    m = a;
                }
            }
            Some(m)
        }
        "clamp" if n == 3 => Some(args[0].max(args[1]).min(args[2])),
        "sum" if n >= 1 => Some(args.iter().sum()),
        "avg" if n >= 1 => Some(args.iter().sum::<f64>() / n as f64),

        // Percentage helpers
        "pct" if n == 2 => Some(args[0] * args[1] / 100.0),
        "pct_change" if n == 2 => {
            if args[0] == 0.0 {
                None
            } else {
                Some((args[1] - args[0]) / args[0] * 100.0)
            }
        }
        "split" if n == 2 => {
            if args[1] == 0.0 {
                None
            } else {
                Some(((args[0] / args[1]) * 100.0).round() / 100.0)
            }
        }

        // Angle conversion
        "deg2rad" if n == 1 => Some(args[0] * std::f64::consts::PI / 180.0),
        "rad2deg" if n == 1 => Some(args[0] * 180.0 / std::f64::consts::PI),

        // Vector math
        "dot" if n == 4 => Some(args[0] * args[2] + args[1] * args[3]),
        "norm" if n == 2 => Some((args[0] * args[0] + args[1] * args[1]).sqrt()),

        // ─── Unit conversions ───────────────────────────────── ────

        // Length
        "mi_to_km" if n == 1 => Some(args[0] * 1.609_344),
        "km_to_mi" if n == 1 => Some(args[0] / 1.609_344),
        "mi_to_m" if n == 1 => Some(args[0] * 1609.344),
        "m_to_mi" if n == 1 => Some(args[0] / 1609.344),
        "ft_to_m" if n == 1 => Some(args[0] * 0.3048),
        "m_to_ft" if n == 1 => Some(args[0] / 0.3048),
        "in_to_cm" if n == 1 => Some(args[0] * 2.54),
        "cm_to_in" if n == 1 => Some(args[0] / 2.54),

        // Speed
        "mph_to_kmh" if n == 1 => Some(args[0] * 1.609_344),
        "kmh_to_mph" if n == 1 => Some(args[0] / 1.609_344),
        "mph_to_ms" if n == 1 => Some(args[0] * 0.447_04),
        "ms_to_mph" if n == 1 => Some(args[0] / 0.447_04),
        "kmh_to_ms" if n == 1 => Some(args[0] / 3.6),
        "ms_to_kmh" if n == 1 => Some(args[0] * 3.6),
        "knot_to_mph" if n == 1 => Some(args[0] * 1.150_779),
        "mph_to_knot" if n == 1 => Some(args[0] / 1.150_779),

        // Temperature
        "f_to_c" if n == 1 => Some((args[0] - 32.0) * 5.0 / 9.0),
        "c_to_f" if n == 1 => Some(args[0] * 9.0 / 5.0 + 32.0),
        "c_to_k" if n == 1 => Some(args[0] + 273.15),
        "k_to_c" if n == 1 => Some(args[0] - 273.15),

        // Mass/weight
        "lb_to_kg" if n == 1 => Some(args[0] * 0.453_592),
        "kg_to_lb" if n == 1 => Some(args[0] / 0.453_592),

        // Digital storage
        "kb_to_mb" if n == 1 => Some(args[0] / 1024.0),
        "mb_to_gb" if n == 1 => Some(args[0] / 1024.0),
        "gb_to_mb" if n == 1 => Some(args[0] * 1024.0),
        "mb_to_kb" if n == 1 => Some(args[0] * 1024.0),
        "b_to_mb" if n == 1 => Some(args[0] / 1_048_576.0),
        "mb_to_b" if n == 1 => Some(args[0] * 1_048_576.0),

        // Time
        "hours_to_min" if n == 1 => Some(args[0] * 60.0),
        "min_to_hours" if n == 1 => Some(args[0] / 60.0),
        "days_to_hours" if n == 1 => Some(args[0] * 24.0),
        "sec_to_min" if n == 1 => Some(args[0] / 60.0),

        // Named constants (0-arg)
        "pi" | "e" | "c" | "c_squared" | "R_air" | "g" | "G" | "h" | "hbar" | "kB" | "e_charge"
        | "me" | "mp" | "NA" | "R" | "atm" | "Rearth" | "GMearth"
            if n == 0 =>
        {
            let op_str = std::str::from_utf8(op).ok()?;
            get_constant(op_str)
        }

        // Physics/math utilities
        "energy" if n == 1 => Some(args[0] * 89_875_517_873_681_764.0),
        "gas_temp" if n == 2 => {
            let r_air = get_constant("R_air").unwrap_or(287.058_694_911_492_5);
            if args[1] == 0.0 {
                None
            } else {
                Some(args[0] / (r_air * args[1]))
            }
        }
        "ratio" if n == 3 => {
            if args[2] == 0.0 {
                None
            } else {
                Some(args[0] * args[1] / args[2])
            }
        }
        "dilute" if n == 2 => {
            if args[1] == 0.0 {
                None
            } else {
                Some(args[0] / args[1])
            }
        }
        "ppm9" if n == 2 => {
            if args[1] == 0.0 {
                None
            } else {
                Some(args[0] / args[1] * 1e6)
            }
        }
        "ppm602" if n == 2 => {
            if args[1] == 0.0 {
                None
            } else {
                Some(args[0] / args[1] * 6.022_140_76e23)
            }
        }
        "factor15" if n == 2 => Some(args[0] * args[1] * 998_003.992_015_968_1),

        _ => None,
    }
}

/// Format a floating-point value to a string.
/// Handles NaN and Infinity without panicking.
pub fn format_f64(val: f64) -> String {
    if val.is_nan() {
        return "NaN".to_string();
    }
    if val.is_infinite() {
        if val > 0.0 {
            return "Infinity".to_string();
        } else {
            return "-Infinity".to_string();
        }
    }
    format!("{}", val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = compute_named(b"add", &[2.0, 3.0]).unwrap();
        assert!((result - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_sqrt() {
        let result = compute_named(b"sqrt", &[9.0]).unwrap();
        assert!((result - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_sqrt_negative() {
        assert!(compute_named(b"sqrt", &[-1.0]).is_none());
    }

    #[test]
    fn test_hypot() {
        assert!((compute_named(b"hypot", &[3.0, 4.0]).unwrap() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_trig() {
        assert!((compute_named(b"sin", &[0.0]).unwrap() - 0.0).abs() < 1e-15);
        assert!((compute_named(b"cos", &[0.0]).unwrap() - 1.0).abs() < 1e-15);
    }

    #[test]
    fn test_asin_out_of_range() {
        assert!(compute_named(b"asin", &[1.5]).is_none());
    }

    #[test]
    fn test_log_domain() {
        assert!(compute_named(b"ln", &[-1.0]).is_none());
        assert!(compute_named(b"log10", &[0.0]).is_none());
    }

    #[test]
    fn test_round_decimal() {
        let result = compute_named(b"round", &[3.14159, 2.0]).unwrap();
        assert!((result - 3.14).abs() < 1e-10);
    }

    #[test]
    fn test_min_max() {
        assert!((compute_named(b"min", &[3.0, 7.0, 1.0, 9.0]).unwrap() - 1.0).abs() < 1e-10);
        assert!((compute_named(b"max", &[3.0, 7.0, 1.0, 9.0]).unwrap() - 9.0).abs() < 1e-10);
    }

    #[test]
    fn test_clamp() {
        assert!((compute_named(b"clamp", &[5.0, 0.0, 10.0]).unwrap() - 5.0).abs() < 1e-10);
        assert!((compute_named(b"clamp", &[-5.0, 0.0, 10.0]).unwrap() - 0.0).abs() < 1e-10);
        assert!((compute_named(b"clamp", &[15.0, 0.0, 10.0]).unwrap() - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_sum_avg() {
        assert!((compute_named(b"sum", &[1.0, 2.0, 3.0]).unwrap() - 6.0).abs() < 1e-10);
        assert!((compute_named(b"avg", &[1.0, 2.0, 3.0]).unwrap() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_div_by_zero_op() {
        assert!(compute_named(b"div", &[1.0, 0.0]).is_none());
    }

    #[test]
    fn test_unknown_op() {
        assert!(compute_named(b"nonexistent", &[1.0, 2.0]).is_none());
    }

    #[test]
    fn test_unit_conversions() {
        assert!((compute_named(b"f_to_c", &[32.0]).unwrap() - 0.0).abs() < 1e-10);
        assert!((compute_named(b"c_to_f", &[0.0]).unwrap() - 32.0).abs() < 1e-10);
        assert!((compute_named(b"mi_to_km", &[1.0]).unwrap() - 1.609_344).abs() < 1e-10);
        assert!((compute_named(b"lb_to_kg", &[1.0]).unwrap() - 0.453_592).abs() < 1e-10);
    }

    #[test]
    fn test_constants() {
        assert!((compute_named(b"pi", &[]).unwrap() - std::f64::consts::PI).abs() < 1e-15);
        assert!((compute_named(b"e", &[]).unwrap() - std::f64::consts::E).abs() < 1e-15);
    }

    #[test]
    fn test_energy() {
        // E = mc², m=1 → c²
        let result = compute_named(b"energy", &[1.0]).unwrap();
        assert!((result - 89_875_517_873_681_764.0).abs() < 1.0);
    }

    #[test]
    fn test_format_f64() {
        assert_eq!(format_f64(42.0), "42");
        assert_eq!(format_f64(std::f64::NAN), "NaN");
        assert_eq!(format_f64(std::f64::INFINITY), "Infinity");
        assert_eq!(format_f64(std::f64::NEG_INFINITY), "-Infinity");
    }
}
