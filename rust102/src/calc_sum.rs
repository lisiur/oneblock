pub fn calc_sum(list: &[u32]) -> Option<u32> {
    if list.len() == 0 {
        None
    } else {
        list.iter().fold(Some(0), |acc, x| {
            if let Some(v) = acc {
                v.checked_add(*x)
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_sum_test_case_1() {
        let list = [];
        assert_eq!(calc_sum(&list), None);
    }

    #[test]
    fn calc_sum_test_case_2() {
        let list = [1_u32];
        assert_eq!(calc_sum(&list), Some(1_u32));
    }

    #[test]
    fn calc_sum_test_case_3() {
        let list = [std::u32::MAX, std::u32::MIN];
        assert_eq!(calc_sum(&list), Some(std::u32::MAX));
    }

    #[test]
    fn calc_sum_test_case_4() {
        let list = [std::u32::MAX, 1];
        assert_eq!(calc_sum(&list), None);
    }
}
