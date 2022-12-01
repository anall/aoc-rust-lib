pub struct ExtendedGCD {
    pub bezout_coefficients: (i128, i128),
    pub gcd: i128,
    pub quotients: (i128, i128),
}

#[must_use]
#[allow(clippy::similar_names, clippy::many_single_char_names)]
pub fn extended_gcd(a: i128, b: i128) -> ExtendedGCD {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let quotient = old_r / r;

        let old_r_tmp = old_r;
        old_r = r;
        r = old_r_tmp - quotient * r;

        let old_s_tmp = old_s;
        old_s = s;
        s = old_s_tmp - quotient * s;

        let old_t_tmp = old_t;
        old_t = t;
        t = old_t_tmp - quotient * t;
    }

    ExtendedGCD {
        bezout_coefficients: (old_s, old_t),
        gcd: old_r,
        quotients: (t, s),
    }
}

// FIXME: Make this generic
#[must_use]
pub fn chinese_remainder_theorem(inp: Vec<(i128, i128)>) -> (i128, i128) {
    let mut iter = inp.into_iter();
    let (mut n_0, mut a_0) = iter.next().unwrap();
    assert_ne!(n_0, 0);
    for (n_1, a_1) in iter {
        assert_ne!(n_1, 0);
        let gcd_result = extended_gcd(n_0, n_1);
        let coeff = gcd_result.bezout_coefficients;
        let mul = n_0 * n_1;
        let mut solution = a_1 * n_0 * coeff.0 + a_0 * n_1 * coeff.1;
        assert!(
            gcd_result.gcd == 1,
            "{} and {} are not coprime (gcd is {})",
            n_0,
            n_1,
            gcd_result.gcd
        );
        /*println!("{} * {} + {} * {} = {}",n_0,coeff.0,n_1,coeff.1,gcd_result.gcd);
        println!("{} * {} * {} + {} * {} * {} = {}",a_1,n_0,coeff.0,a_0,n_1,coeff.1,solution);*/

        solution = solution.rem_euclid(mul);

        n_0 = mul;
        a_0 = solution;
    }

    (n_0, a_0)
}

#[cfg(test)]
mod tests {
    use crate::aoc::math::{chinese_remainder_theorem, extended_gcd};

    #[test]
    fn test_extended_gcd() {
        let result = extended_gcd(4, 3);
        assert_eq!(result.bezout_coefficients, (1, -1));
        assert_eq!(result.gcd, 1);
    }

    #[test]
    fn test_chinese_remainder() {
        assert_eq!(chinese_remainder_theorem(vec![(3, 0), (4, 3)]), (12, 3));
        assert_eq!(
            chinese_remainder_theorem(vec![(3, 0), (4, 3), (5, 4)]),
            (60, 39)
        );
    }
}
