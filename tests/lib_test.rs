use setup::setup;

mod external {
    pub fn hello() -> String {
        "Hello".to_string()
    }
}

fn before() -> i32 {
    1
}

fn complex_before() -> (i32, i32) {
    (1, 1)
}

#[setup]
#[test]
fn use_default_setup(data: i32) {
    assert_eq!(data, 1)
}

#[setup(before)]
#[test]
fn use_simple_setup(data: i32) {
    assert_eq!(data, 1)
}

#[setup(complex_before)]
#[test]
fn use_complex_setup((a, b): (i32, i32)) {
    assert_eq!(a, b)
}

#[setup(external::hello)]
#[test]
fn use_external_setup(hello: String) {
    assert_eq!(hello, "Hello".to_string())
}
