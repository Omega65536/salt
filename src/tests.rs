#[cfg(test)]
mod lexer_tests {
    /*#[test]
    fn test0() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }*/
}

#[cfg(test)]
mod parser_tests {
    /*#[test]
    fn test0() {
        assert_eq!(1, 1);
    }*/
}

#[cfg(test)]
mod interpreter_tests {
    use crate::{salt::Salt, value::Value};

    #[test]
    fn math() {
        let salt = Salt::new();
        let value = salt.run(
            "
        fn main() {
            result = 4 * 5 + 12 / (10 - 15 % 8);
            return result;
        }
        ",
        );
        assert_eq!(value, Value::Integer(24));
    }

    #[test]
    fn if_stmt() {
        let salt = Salt::new();
        let value = salt.run(
            "
        fn main() {
            if 2 >= 3 {
                return 1;
            }
            if 2 > 3 {
                return 2;
            }
            if 2 <= 3 {
                return 3;
            }
        }
        ",
        );
        assert_eq!(value, Value::Integer(3));
    }

    #[test]
    fn while_loop() {
        let salt = Salt::new();
        let value = salt.run(
            "
        fn main() {
            i = 1;
            product = 1;
            while i <= 10 {
                product = product * i;
                i = i + 1;
            }
            return product;
        }
        ",
        );
        assert_eq!(value, Value::Integer(3628800));
    }

    #[test]
    fn functions() {
        let salt = Salt::new();
        let value = salt.run(
            "
        fn main() {
            return a(1, 2, 3) + b(4, 5) + c(6) + d();
        }

        fn a(x, y, z) {
            return b(x, y) + c(z) + d() + 1;
        }

        fn b(x, y) {
            return c(x) + d() + 1;
        }

        fn c(x) {
            return d() + 1;
        }

        fn d() {
            return 1;
        }
        ",
        );

        assert_eq!(value, Value::Integer(15));
    }

    #[test]
    fn fib() {
        let salt = Salt::new();
        let value = salt.run(
            "
        fn main() {
            result_iter = fib_iter(10);
            print(result_iter);
            return fib_iter(10) == fib_rec(10);
        }

        fn fib_iter(i) {
            a = 0;
            b = 1;
            j = 0;
            while j < i {
                t = b;
                b = a + b;
                a = t;
                j = j + 1;
            }
            return a;
        }

        fn fib_rec(i) {
            if i == 0 {
                return 0;
            }
            if i == 1 {
                return 1;
            }
            return fib_rec(i - 1) + fib_rec(i - 2);
        }
        ",
        );
        assert_eq!(value, Value::Boolean(true));
    }
}
