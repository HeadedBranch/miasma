pub fn page_count_per_bot(links_per_page: u8, max_depth: u32) -> Option<u128> {
    // This is just the number of nodes in a a k-ary tree
    // k^h - 1
    // -------
    //  k - 1

    let k = links_per_page as u128;
    let h = max_depth;

    let num = k.checked_pow(h)? - 1;
    let denom = k - 1;

    // If dividing by 0, that just means the link count is 1
    // so the total node count will be the depth
    num.checked_div(denom).or(Some(max_depth as u128))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculation_is_correct() {
        let test_cases = [
            (2, 1, 1),
            (5, 3, 31),
            (8, 5, 4681),
            (10, 5, 11111),
            (7, 9, 6725601),
        ];

        for (link_count, depth, expected) in test_cases {
            let result = page_count_per_bot(link_count, depth);
            assert_eq!(result, Some(expected));
        }
    }

    #[test]
    fn link_count_one_returns_max_depth() {
        let max_depth = 5;
        let result = page_count_per_bot(1, max_depth);
        assert_eq!(result, Some(max_depth as u128));
    }
}
