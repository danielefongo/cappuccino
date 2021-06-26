cappuccino::tests!({
    it "should pass" {
        assert_eq!(1, 1);
    }

    when "empty" {}

    when "has space in description" {}

    when "has single it" {
        it "should pass" {
            assert_eq!(1, 1);
        }
    }

    when "has multiple it" {
        it "should pass 1" {
            assert_eq!(1, 1);
        }
        it "should pass 2" {
            assert_eq!(1, 1);
        }
    }

    when "has inner when" {
        when "and has an it" {
            it "should pass" {
                assert_eq!(1, 1);
            }
        }
    }

    when "has hybrid when and it" {
        when "empty" {}

        it "should pass" {
            assert_eq!(1, 1);
        }
    }

    before {
        let a = 42;
        let b = 42;
    }

    it "should pass using setup" {
        assert_eq!(a, b);
    }

    when "has no inner setup" {
        it "should use super setup" {
            assert_eq!(a, 42);
        }
    }

    when "has have a nested when" {
        when "has no inner setup" {
            it "should use super setup" {
                assert_eq!(a, 42);
            }
        }

        when "has inner setup" {
            before {
                let a = 24;
            }

            it "should use inner setup" {
                assert_eq!(a, 24);
            }
        }
    }

    when "has inner setup" {
        before {
            let a = 24;
        }

        it "should pass" {
            assert_eq!(a, 24);
        }
    }

    when "using utility functions" {
        it "can use define functions" {
            assert_eq!(the_answer(), 42);
        }

        fn the_answer() -> i32 {
            42
        }
    }
});

cappuccino::tests!("custom root" {
    it "should pass" {
        assert_eq!(1,1);
    }
});
