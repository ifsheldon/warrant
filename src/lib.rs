///
/// Checks a condition.
/// If the condition is not satisfied, the block of code following the `else` keyword is executed.
///
#[macro_export] macro_rules! warrant {
    ($condition:expr, else $else_block:block) => {
        if !$condition {
            $else_block
        }
    };
}

pub use warrant as guard;


#[cfg(test)]
mod macro_tests {
    use super::*;

    #[test]
    fn expression_test() {
        let a = 1;
        warrant!(a % 2 == 0, else {
            return
        });
        unreachable!();
    }

    #[test]
    fn variable_test() {
        let a = false;
        warrant!(a, else {
            return
        });

        unreachable!();
    }

    fn some_condition_satisfied() -> bool {
        false
    }

    #[test]
    fn function_test() {
        warrant!(some_condition_satisfied(), else {
            return
        });
        unreachable!();
    }

    #[test]
    fn test_guard() {
        guard!(some_condition_satisfied(), else {
            return
        });
        unreachable!();
    }
}
