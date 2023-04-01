# Measurement

Adds quantities and units to rust to allow unit safe computation

## Define new quantity

### Base Quantity

```rust
quantity! {
    name: Time,
    base_unit: Second,
    units: {
        Second:      1e+0,
        Millisecond: 1e-3,
    }
}
```

### Derived Quantity

```rust
quantity! {
    name: Velocity,
    base_unit: MeterPerSecond,
    units: {
        MeterPerSecond: 1,
        KilometerPerHour: 3.6,
    },
    derive: Length / Time,
}
```
