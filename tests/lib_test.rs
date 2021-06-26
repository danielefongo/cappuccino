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

    before i32 {
        42
    }

    it "should pass using setup" |a: i32| {
        assert_eq!(a, 42);
    }

    when "has no inner setup" {
        it "should use super setup" |a: i32| {
            assert_eq!(a, 42);
        }
    }

    when "has have a nested when" {
        when "has no inner setup" {
            it "should use super setup" |a: i32| {
                assert_eq!(a, 42);
            }
        }

        when "has inner setup" {
            before i32 {
                24
            }

            it "should use inner setup" |a: i32| {
                assert_eq!(a, 24);
            }
        }
    }

    when "has inner setup" {
        before i32 {
            24
        }

        it "should pass" |a: i32| {
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
