# Cappuccino

Simple testing library for rust.

## Usage

To create a simple test, write the following:
```rust
cappuccino::tests!({
  it should_something() {
    assert_eq!(42, 42);
  }
});
```

### it
The keyword `it` is used to describe a single test.
#### Simple test
```rust
it should_something() {
  assert_eq!(42, 42);
}
```

#### Test with result
```rust
it should_something() -> Result<(), String> {
  assert_eq!(42, 42);
  Ok(())
}
```

#### Async test
```rust
async fn the_long_waited_answer() -> i32 {
  // after seven and a half million years...
  42
}
it should_something() async {
  assert_eq!(the_long_waited_answer().await, 42);
}
```

#### Async test with result
```rust
async fn the_long_waited_answer() -> i32 {
  // after seven and a half million years...
  42
}
it should_pass_and_return_result() async -> Result<(), String> {
  assert_eq!(the_long_waited_answer().await, 42);
  Ok(())
}
```

#### Test with literal string identifier
```rust
it "should something" {
  assert_eq!(42, 42);
}
```
**NB**: this will break the IDE test runner position for that test.

### when
The keyword `it` is used to describe a scenario.

#### Simple scenario
```rust
when condition() {
  it should_something() {
    assert_eq!(42, 42);
  }
  it should_something_too() {
    assert_eq!(42, 42);
  }
}
```

**NB**: scenarios automatically import outer imports and functions:
```rust
fn the_answer() -> i32 {
  42
}
when condition() {
  it should_something() {
    assert_eq!(the_answer(), 42);
  }
}
```

#### Nested scenarios
```rust
when condition() {
  when another_condition() {
    it should_something() {
      assert_eq!(42, 42);
    }
  }
}
```

#### Scenario with hybrid content
```rust
when condition() {
  when another_condition() {
    // content
  }
  it should_something() {
    assert_eq!(42, 42);
  }
}
```

#### Scenario with literal string identifier
```rust
when "condition" {
  when "another condition" {
    // content
  }
}
```

### before
The keyword `before` is used to create a setup method within `tests!` or `when` context.

#### Simple before
```rust
before {
  let (a, b) = (42, 42);
}
it should_something() {
  assert_eq!(a, b);
}
```

#### Inherited before
`before` is automatically propagated inside `nested` when if no inner `before` is defined.
```rust
before {
  let (a,b) = (42, 42);
}

when condition() {
  it should_something() {
    assert_eq!(a, b);
  }
}
```
