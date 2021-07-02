cappuccino::tests!({
    it should_pass() {
        assert_eq!(1, 1);
    }

    it should_pass_with_final_void_expression() {
        ()
    }

    it should_pass_with_result() -> Result<(), String> {
        assert_eq!(1, 1);
        Ok(())
    }

    when empty() {}

    when has_single_it() {
        it should_pass() {
            assert_eq!(1, 1);
        }
    }

    when has_multiple_it() {
        it should_pass_1() {
            assert_eq!(1, 1);
        }
        it should_pass_2() {
            assert_eq!(1, 1);
        }
    }

    when has_inner_when() {
        when and_has_an_it() {
            it should_pass() {
                assert_eq!(1, 1);
            }
        }
    }

    when has_hybrid_when_and_it() {
        when empty() {}

        it should_pass() {
            assert_eq!(1, 1);
        }
    }

    before {
        let a = 42;
        let b = 42;
    }

    it should_pass_using_before() {
        assert_eq!(a, b);
    }

    when has_no_inner_before() {
        it should_use_super_before() {
            assert_eq!(a, 42);
        }
    }

    when has_have_a_nested_when() {
        when has_no_inner_before() {
            it should_use_super_before() {
                assert_eq!(a, 42);
            }
        }

        when has_inner_before() {
            before {
                let a = 24;
            }

            it should_use_inner_before() {
                assert_eq!(a, 24);
            }
        }
    }

    when has_inner_before() {
        before {
            let a = 24;
        }

        it should_pass() {
            assert_eq!(a, 24);
        }
    }

    when using_utility_functions() {
        it can_use_define_functions() {
            assert_eq!(the_answer(), 42);
        }

        fn the_answer() -> i32 {
            42
        }
    }

    when using_external_utility_functions() {
        it should_pass() {
            assert_eq!(the_answer(), 42);
        }

        when even_inside_another_when() {
            it should_pass() {
                assert_eq!(the_answer(), 42);
            }
        }
    }

    fn the_answer() -> i32 {
        42
    }
});

#[cfg(feature = "async")]
cappuccino::tests!(async_tests() {
    before {
        let real_answer = 42;
    }

    it should_pass() async {
        assert_eq!(the_long_waited_answer().await, real_answer);
    }

    it should_pass_and_return_result() async -> Result<(), String> {
        assert_eq!(the_long_waited_answer().await, real_answer);
        Ok(())
    }

    async fn the_long_waited_answer() -> i32 {
        // after seven and a half million years...
        42
    }
});

cappuccino::tests!("literal ident tests" {
    it "should pass" {
        assert_eq!(1, 1);
    }

    when "condition" {
        it "should pass" {
            assert_eq!(1, 1);
        }
    }
});
