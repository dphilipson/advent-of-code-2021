use crate::util::nums::Int;

pub fn gcd<T: Int>(a: T, b: T) -> T {
    let (x, y) = extended_euclidean(a, b);
    a * x + b * y
}

/// Returns the Bézout coefficients. That is, the returned (x, y) will satisfy
/// > ax + by = gcd(a, b)
pub fn extended_euclidean<T: Int>(a: T, b: T) -> (T, T) {
    if a < 0.into() {
        let (x, y) = extended_euclidean(a.unsafe_negate(), b);
        return (x.unsafe_negate(), y);
    }
    if b < 0.into() {
        let (x, y) = extended_euclidean(a, b.unsafe_negate());
        return (x, y.unsafe_negate());
    }
    let mut rs = (a, b);
    let mut ss: (T, T) = (1.into(), 0.into());
    let mut ts: (T, T) = (0.into(), 1.into());
    while rs.1 != 0.into() {
        let quotient = rs.0 / rs.1;
        rs = euclidean_step(rs, quotient);
        ss = euclidean_step(ss, quotient);
        ts = euclidean_step(ts, quotient);
    }
    (ss.0, ts.0)
}

fn euclidean_step<T: Int>((old_x, x): (T, T), quotient: T) -> (T, T) {
    (x, old_x - quotient * x)
}

/// Solves a system of congruences, where each congruence is given in the form
/// `(a, n)`, which represents `x = a (mod n)`. The result is given in the
/// same format `(b, m)`, meaning the solution to the system is `x = b (mod m)`.
pub fn solve_congruences<T: Int>(congruences: &[(T, T)]) -> Option<(T, T)> {
    let (&head, rest) = congruences.split_first()?;
    let mut acc = head;
    for &c in rest {
        acc = solve_congruence_pair(acc, c)?;
    }
    Some(acc)
}

fn solve_congruence_pair<T: Int>((a, n): (T, T), (b, m): (T, T)) -> Option<(T, T)> {
    let (x, y) = extended_euclidean(n, m);
    let gcd = n * x + m * y;
    let a_quotient = a / gcd;
    if a_quotient * gcd != a {
        return None;
    }
    let b_quotient = b / gcd;
    if b_quotient * gcd != b {
        return None;
    }
    let lcm = m / gcd * n;
    Some(((a_quotient * m * y + b_quotient * n * x).modulus(lcm), lcm))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(3, 5), 1);
        assert_eq!(gcd(30, 25), 5);
        assert_eq!(gcd(1, 5), 1);
        assert_eq!(gcd(-3, 5), 1);
    }

    #[test]
    fn test_solve_congruence_pair() {
        assert_eq!(solve_congruence_pair((1, 3), (3, 5)), Some((13, 15)));
        assert_eq!(solve_congruence_pair((3, 9), (9, 15)), Some((39, 45)));
        assert_eq!(solve_congruence_pair((3, 12), (4, 5)), Some((39, 60)));
        assert_eq!(solve_congruence_pair((2, 9), (9, 15)), None);
    }

    #[test]
    fn test_solve_congruences() {
        assert_eq!(solve_congruences(&[(0, 3), (3, 4), (4, 5)]), Some((39, 60)));
    }
}
