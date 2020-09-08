fn sa_naive(s: &[i32]) -> Vec<usize> {
    let n = s.len();
    let mut sa: Vec<usize> = (0..n).collect();
    sa.sort_by(|&(mut l), &(mut r)| {
        if l == r {
            return std::cmp::Ordering::Equal;
        }
        while l < n && r < n {
            if s[l] != s[r] {
                return s[l].cmp(&s[r]);
            }
            l += 1;
            r += 1;
        }
        if l == n {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    sa
}

fn sa_doubling(s: &[i32]) -> Vec<usize> {
    let n = s.len();
    let mut sa: Vec<usize> = (0..n).collect();
    let mut rnk: Vec<i32> = s.to_vec();
    let mut tmp = vec![0; n];
    let mut k = 1;
    while k < n {
        let cmp = |&x: &usize, &y: &usize| {
            if rnk[x] != rnk[y] {
                return rnk[x].cmp(&rnk[y]);
            }
            let rx = if x + k < n { rnk[x + k] } else { -1 };
            let ry = if y + k < n { rnk[y + k] } else { -1 };
            rx.cmp(&ry)
        };
        sa.sort_by(cmp);
        tmp[sa[0]] = 0;
        for i in 1..n {
            tmp[sa[i]] = tmp[sa[i - 1]]
                + if cmp(&sa[i - 1], &sa[i]) == std::cmp::Ordering::Less {
                    1
                } else {
                    0
                };
        }
        std::mem::swap(&mut tmp, &mut rnk);
        k *= 2;
    }
    sa
}

fn sa_is(s: &[i32], upper: i32) -> Vec<usize> {
    sa_doubling(s)
}

pub fn suffix_array_manual(s: &[i32], upper: i32) -> Vec<usize> {
    assert!(upper >= 0);
    for &elem in s {
        assert!(0 <= elem && elem <= upper);
    }
    sa_is(s, upper)
}

pub fn suffix_array_arbitrary<T: Ord>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by_key(|&i| &s[i]);
    let mut s2 = vec![0; n];
    let mut now = 0;
    for i in 0..n {
        if i > 0 && s[idx[i - 1]] != s[idx[i]] {
            now += 1;
        }
        s2[idx[i]] = now;
    }
    sa_is(&s2, now)
}

pub fn suffix_array(s: impl IntoIterator<Item = char>) -> Vec<usize> {
    let mut s2: Vec<i32> = s.into_iter().map(|x| x as i32).collect();
    sa_is(&s2, 255)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sa_0() {
        let array = vec![0, 1, 2, 3, 4];
        let sa = sa_doubling(&array);
        assert_eq!(sa, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_sa_1() {
        let str = "abracadabra";
        let array: Vec<i32> = str.bytes().map(|x| x as i32).collect();
        let sa = sa_doubling(&array);
        assert_eq!(sa, vec![10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2]);
        let sa_naive = sa_naive(&array);
        assert_eq!(sa_naive, sa);
        let sa_is = sa_is(&array, 10);
        assert_eq!(sa_is, sa);

        let sa_str = suffix_array(str.chars());
        assert_eq!(sa_str, sa);
    }
}
