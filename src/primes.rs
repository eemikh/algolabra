/// Laskee Eratostheneen seulalla kaikki alkuluvut, jotka ovat vähemmän kuin `n`.
///
/// Algoritmi on esitelty artikkelissa <https://research.cs.wisc.edu/techreports/1990/TR909.pdf>.
pub fn primes_up_to(n: usize) -> impl Iterator<Item = usize> {
    assert!(n > 2);

    // true kaikilla nillä indekseillä, jotka ovat alkulukuja
    let mut prime_state = vec![true; n];

    for num in 2..=n.isqrt() {
        if !prime_state[num] {
            continue;
        }

        for multiple_of_num in (num * num..n).step_by(num) {
            prime_state[multiple_of_num] = false;
        }
    }

    prime_state
        .into_iter()
        .enumerate()
        // ensimmäiset kaksi ovat 0 ja 1, ne skipataan
        .skip(2)
        // otetaan ne indeksit, joilla arvo on true
        .filter(|&(_, is_prime)| is_prime)
        .map(|(index, _)| index)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Laskee alkuluvut, jotka ovat vähemmän kuin `n`. Funktio on toteutettu tarkistamalla
    /// jokaisella kokonaisluvulla, onko mikään sitä pienempi kokonaisluku, kuitenkin vähintään 2,
    /// sen jakaja.
    fn slow_primes_up_to_n(n: usize) -> Vec<usize> {
        let mut primes = Vec::new();

        for i in 2..n {
            let mut is_prime = true;

            for j in 2..i {
                if i.is_multiple_of(j) {
                    is_prime = false;
                    break;
                }
            }

            if is_prime {
                primes.push(i);
            }
        }

        primes
    }

    #[test]
    fn test_primes_up_to() {
        assert_eq!(primes_up_to(3).collect::<Vec<_>>(), vec![2]);
        assert_eq!(primes_up_to(5).collect::<Vec<_>>(), vec![2, 3]);
        assert_eq!(primes_up_to(6).collect::<Vec<_>>(), vec![2, 3, 5]);
        assert_eq!(
            primes_up_to(32).collect::<Vec<_>>(),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31]
        );

        assert_eq!(
            primes_up_to(10_000).collect::<Vec<_>>(),
            slow_primes_up_to_n(10_000)
        );
    }

    #[test]
    #[should_panic]
    fn test_primes_up_to_invalid_n() {
        let _ = primes_up_to(2);
    }
}
