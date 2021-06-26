# Cappuccino

Simple testing library for rust.

## Usage

To create a simple test, write the following:
```rust
cappuccino::tests!({
    it "should something" {
        assert_eq!(42, 42);
    }
});
```

### it
The keyword `it` is used to describe a single test:
```rust
it "should something" {
  assert_eq!(42, 42);
}
```

### when
The keyword `it` is used to describe a scenario:
```rust
when "condition" {
  it "should something" {
    assert_eq!(42, 42);
  }
  it "should something too" {
    assert_eq!(42, 42);
  }
}
```

You can have nested when:
```rust
when "condition" {
  when "another condition" {
    it "should something" {
      assert_eq!(42, 42);
    }
  }
}
```

You can have hybrid content:
```rust
when "condition" {
  when "another condition" {
    // content
  }
  it "should something" {
    assert_eq!(42, 42);
  }
}
```

### before
The keyword `before` is used to create a setup method within `tests!` or `when` context:
```rust
before (i32, i32) {
  (42, 42)
}

it "should something" |(a, b): (i32, i32)| {
  assert_eq!(a, b);
}
```

before is automatically propagated inside nested when if no inner `before` is defined
```rust
before (i32, i32) {
  (42, 42)
}

when "condition" {
  it "should something" |(a, b): (i32, i32)| {
    assert_eq!(a, b);
  }
}
```
