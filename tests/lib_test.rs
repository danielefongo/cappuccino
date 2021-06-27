cappuccino::tests!({
    it "should pass" {
        assert_eq!(1, 1);
    }

    it "should pass with final void expression" {
        ()
    }

    it "should pass with result" -> Result<(), String> {
        assert_eq!(1, 1);
        Ok(())
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

    it "should pass using before" {
        assert_eq!(a, b);
    }

    when "has no inner before" {
        it "should use super before" {
            assert_eq!(a, 42);
        }
    }

    when "has have a nested when" {
        when "has no inner before" {
            it "should use super before" {
                assert_eq!(a, 42);
            }
        }

        when "has inner before" {
            before {
                let a = 24;
            }

            it "should use inner before" {
                assert_eq!(a, 24);
            }
        }
    }

    when "has inner before" {
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
