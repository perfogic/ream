pub fn is_justifiable_after(candidate_slot: u64, finalized_slot: u64) -> bool {
    if candidate_slot < finalized_slot {
        return false;
    }

    let delta = candidate_slot - finalized_slot;
    let pentagonal_test = 4u128 * u128::from(delta) + 1;
    delta <= 5
        || delta.isqrt().pow(2) == delta
        || pentagonal_test.isqrt().pow(2) == pentagonal_test && pentagonal_test.isqrt() % 2 == 1
}

pub fn justified_index_after(candidate_slot: u64, finalized_slot: u64) -> Option<u64> {
    if candidate_slot <= finalized_slot {
        return None;
    }

    Some(candidate_slot - finalized_slot - 1)
}

#[cfg(test)]
mod tests {
    use super::is_justifiable_after;

    #[test]
    fn justifiability_is_false_before_finalized_slot() {
        assert!(!is_justifiable_after(9, 10));
        assert!(!is_justifiable_after(90, 100));
    }
}
