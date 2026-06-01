/// Build a suffix array using the O(N log N) prefix-doubling algorithm with
/// radix sort.
///
/// Returns `sa` where `sa[i]` is the starting position of the i-th
/// lexicographically smallest suffix in `text`.
#[expect(
    clippy::cast_possible_truncation,
    reason = "ranks are bounded by text length which fits in usize"
)]
pub(super) fn build_suffix_array(text: &[i64]) -> Vec<usize> {
    let n = text.len();
    if n == 0 {
        return vec![];
    }

    let min_val = text.iter().copied().min().unwrap_or(0);
    let mut rank: Vec<i64> = text.iter().map(|&v| v - min_val).collect();
    let mut sa: Vec<usize> = (0..n).collect();
    let mut tmp: Vec<i64> = vec![0; n];
    let mut k: usize = 1;
    let mut iterations = 0u32;

    let mut sa_tmp: Vec<usize> = vec![0; n];
    let mut counts: Vec<usize> = Vec::new();

    let mut max_rank = rank.iter().copied().max().unwrap_or(0) as usize;

    while k < n {
        iterations += 1;

        let bucket_count = max_rank + 2; // ranks 0..=max_rank plus -1 mapped to 0

        counts.clear();
        counts.resize(bucket_count + 1, 0);
        for &i in &sa {
            let r2 = if i + k < n {
                rank[i + k] as usize + 1
            } else {
                0
            };
            counts[r2] += 1;
        }
        let mut sum = 0;
        for c in &mut counts {
            let v = *c;
            *c = sum;
            sum += v;
        }
        for &i in &sa {
            let r2 = if i + k < n {
                rank[i + k] as usize + 1
            } else {
                0
            };
            sa_tmp[counts[r2]] = i;
            counts[r2] += 1;
        }

        counts.fill(0);
        counts.resize(bucket_count + 1, 0);
        for &i in &sa_tmp {
            let r1 = rank[i] as usize;
            counts[r1] += 1;
        }
        sum = 0;
        for c in &mut counts {
            let v = *c;
            *c = sum;
            sum += v;
        }
        for &i in &sa_tmp {
            let r1 = rank[i] as usize;
            sa[counts[r1]] = i;
            counts[r1] += 1;
        }

        tmp[sa[0]] = 0;
        for i in 1..n {
            let prev = sa[i - 1];
            let curr = sa[i];
            let same = rank[prev] == rank[curr] && {
                let rp2 = if prev + k < n { rank[prev + k] } else { -1 };
                let rc2 = if curr + k < n { rank[curr + k] } else { -1 };
                rp2 == rc2
            };
            tmp[curr] = tmp[prev] + i64::from(!same);
        }

        let new_max_rank = tmp[sa[n - 1]];
        std::mem::swap(&mut rank, &mut tmp);

        if new_max_rank as usize == n - 1 {
            break;
        }

        max_rank = new_max_rank as usize;
        k *= 2;
    }

    tracing::trace!(n, iterations, "suffix array constructed");
    sa
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_suffix_order(text: &[i64], sa: &[usize]) {
        assert_eq!(
            text.len(),
            sa.len(),
            "suffix array length must equal text length"
        );
        for i in 1..sa.len() {
            let suffix_a = &text[sa[i - 1]..];
            let suffix_b = &text[sa[i]..];
            assert!(
                suffix_a <= suffix_b,
                "suffix order violated at SA[{}]={} vs SA[{}]={}: {:?} > {:?}",
                i - 1,
                sa[i - 1],
                i,
                sa[i],
                suffix_a,
                suffix_b,
            );
        }
    }

    fn assert_is_permutation(sa: &[usize], n: usize) {
        let mut seen = vec![false; n];
        for &idx in sa {
            assert!(idx < n, "suffix array index {idx} out of bounds (n={n})");
            assert!(!seen[idx], "duplicate index {idx} in suffix array");
            seen[idx] = true;
        }
    }

    #[test]
    fn empty_input() {
        let sa = build_suffix_array(&[]);
        assert!(sa.is_empty());
    }

    #[test]
    fn single_element() {
        let text = [42];
        let sa = build_suffix_array(&text);
        assert_eq!(sa, vec![0]);
        assert_suffix_order(&text, &sa);
    }

    #[test]
    fn two_elements_already_sorted() {
        let text = [1, 2];
        let sa = build_suffix_array(&text);
        assert_is_permutation(&sa, 2);
        assert_suffix_order(&text, &sa);
    }

    #[test]
    fn two_elements_reverse_sorted() {
        let text = [2, 1];
        let sa = build_suffix_array(&text);
        assert_is_permutation(&sa, 2);
        assert_suffix_order(&text, &sa);
        assert_eq!(sa[0], 1);
        assert_eq!(sa[1], 0);
    }

    #[test]
    fn already_sorted_input() {
        let text = [1, 2, 3, 4, 5];
        let sa = build_suffix_array(&text);
        assert_is_permutation(&sa, 5);
        assert_suffix_order(&text, &sa);
    }

    #[test]
    fn reverse_sorted_input() {
        let text = [5, 4, 3, 2, 1];
        let sa = build_suffix_array(&text);
        assert_is_permutation(&sa, 5);
        assert_suffix_order(&text, &sa);
        assert_eq!(sa[0], 4);
    }

    #[test]
    fn all_identical_elements() {
        let text = [7, 7, 7, 7];
        let sa = build_suffix_array(&text);
        assert_is_permutation(&sa, 4);
        assert_suffix_order(&text, &sa);
        assert_eq!(sa, vec![3, 2, 1, 0]);
    }

    #[test]
    fn mixed_input_banana_like() {
        let text = [2, 1, 3, 1, 3, 1];
        let sa = build_suffix_array(&text);
        assert_is_permutation(&sa, 6);
        assert_suffix_order(&text, &sa);
    }

    #[test]
    fn input_with_negative_sentinels() {
        let text = [3, 1, 2, -1, 4, 5, -2, 6];
        let sa = build_suffix_array(&text);
        assert_is_permutation(&sa, 8);
        assert_suffix_order(&text, &sa);
        assert_eq!(sa[0], 6);
    }

    #[test]
    fn single_sentinel_only() {
        let text = [-1];
        let sa = build_suffix_array(&text);
        assert_eq!(sa, vec![0]);
        assert_suffix_order(&text, &sa);
    }

    #[test]
    fn multiple_sentinels_decreasing() {
        let text = [-1, -2];
        let sa = build_suffix_array(&text);
        assert_is_permutation(&sa, 2);
        assert_suffix_order(&text, &sa);
        assert_eq!(sa[0], 1);
        assert_eq!(sa[1], 0);
    }

    #[test]
    fn realistic_concatenated_files() {
        let text = [10, 20, 30, -1, 20, 30, 40];
        let sa = build_suffix_array(&text);
        assert_is_permutation(&sa, 7);
        assert_suffix_order(&text, &sa);
    }

    #[test]
    fn repeated_pattern() {
        let text = [1, 2, 1, 2];
        let sa = build_suffix_array(&text);
        assert_is_permutation(&sa, 4);
        assert_suffix_order(&text, &sa);
    }

    #[test]
    fn large_input_stress() {
        let text: Vec<i64> = (0..256).map(|i| i64::from(i % 17)).collect();
        let sa = build_suffix_array(&text);
        assert_is_permutation(&sa, 256);
        assert_suffix_order(&text, &sa);
    }

    #[test]
    fn large_identical_stress() {
        let text = vec![42i64; 128];
        let sa = build_suffix_array(&text);
        assert_is_permutation(&sa, 128);
        assert_suffix_order(&text, &sa);
        for (i, &pos) in sa.iter().enumerate() {
            assert_eq!(pos, 127 - i);
        }
    }

    #[test]
    fn alternating_sentinels_and_tokens() {
        let text = [5, -1, 5, -2];
        let sa = build_suffix_array(&text);
        assert_is_permutation(&sa, 4);
        assert_suffix_order(&text, &sa);
    }

    #[test]
    fn all_same_with_trailing_sentinel() {
        let text = [3, 3, 3, -1];
        let sa = build_suffix_array(&text);
        assert_is_permutation(&sa, 4);
        assert_suffix_order(&text, &sa);
        assert_eq!(sa[0], 3);
    }

    #[test]
    fn suffix_array_is_inverse_of_rank() {
        let text = [4, 2, 3, 1, 5];
        let sa = build_suffix_array(&text);
        let n = text.len();
        let mut rank = vec![0usize; n];
        for i in 0..n {
            rank[sa[i]] = i;
        }
        for i in 0..n {
            assert_eq!(
                sa[rank[i]], i,
                "rank/sa inverse property violated at position {i}"
            );
        }
    }
}
