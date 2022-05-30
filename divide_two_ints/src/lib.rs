pub struct Solution;

impl Solution {
    pub fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
        // handles an edge case
        if dividend == -2147483648 && divisor == -1 {
            return 2147483647;
        }

        // if negative and positive answer will be negative
        let mut positives = 0;
        if dividend.is_positive() {
            dividend = -dividend;
            positives += 1;
        }
        if divisor.is_positive() {
            divisor = -divisor;
            positives += 1;
        }

        let mut quotient: i32 = 0;
        let mut doubled: i32 = divisor;
        if -2147483648 - divisor < divisor {
            while doubled + doubled > dividend {
                if quotient == 0 {
                    quotient = 2;
                } else {
                    quotient += quotient;
                }

                doubled += doubled;
                if -2147483648 - doubled > doubled {
                    break;
                }
            }
            if doubled != divisor {
                dividend -= doubled;
            }
        }

        // subtract the divisor the dividend until it reaches the floor
        while dividend - divisor < 0 {
            dividend -= divisor;
            quotient += 1;
        }

        if dividend == divisor {
            quotient += 1;
        }

        if positives == 1 {
            quotient = -quotient;
        }
        quotient
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_positives() {
        assert_eq!(Solution::divide(15, 3), 5);
        assert_eq!(Solution::divide(100, 9), 11);
        assert_eq!(Solution::divide(1738, 27), 64);
    }

    #[test]
    fn negative_and_positive() {
        assert_eq!(Solution::divide(-15, 3), -5);
        assert_eq!(Solution::divide(100, -9), -11);
        assert_eq!(Solution::divide(1738, -27), -64);
        assert_eq!(Solution::divide(-1738, 27), -64);
    }

    #[test]
    fn two_negatives() {
        assert_eq!(Solution::divide(-15, -3), 5);
        assert_eq!(Solution::divide(-100, -9), 11);
        assert_eq!(Solution::divide(-1738, -27), 64);
        assert_eq!(Solution::divide(-1738, -27), 64);
    }

    #[test]
    fn one() {
        assert_eq!(Solution::divide(1, 1), 1);
        assert_eq!(Solution::divide(-1, 1), -1);
        assert_eq!(Solution::divide(1, -1), -1);
        assert_eq!(Solution::divide(-1, -1), 1);
    }

    #[test]
    fn max() {
        assert_eq!(Solution::divide(-2147483648, -1), 2147483647);
        assert_eq!(Solution::divide(2147483647, 3), 715827882);
    }

    #[test]
    fn edge() {
        //assert_eq!(Solution::divide(1004958205, -2137325331), 0);
        //assert_eq!(Solution::divide(1076233784, -1766978113), 0);
        assert_eq!(Solution::divide(-2147483648, -2147483648), 0);
    }
}
