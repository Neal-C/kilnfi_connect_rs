# Design

## Patterns

### Typestate pattern

Generics and Monomorphization increase the binary size (& compilation time) but at the benefit of compile time type safety.

An invalid Kiln connect client cannot be built.

## Sync / Async

It's a sync , blocking client.

### Async

Async client incoming.

## Error Handling

The .unwrap() calls are for prototyping and testing.
I swear, they'll be gone with time. trust me bro.

## Decimal Precision

For the first iteration of the client, I'll assume that f64 can represent all the decimal places needed.

decimal::Decimal is considered for the 2nd iteration.

while f64 could technically store this number precisely, the safest choice would be decimal::Decimal. Here's why:

While this specific number happens to be representable exactly in f64, using decimal::Decimal ensures consistent precision regardless of the specific values that the client is working with.

performing arithmetic operations or work with slightly different numbers, decimal::Decimal maintains exact precision throughout calculations.
Therefore, the recommended Rust type is decimal::Decimal.

## Typing

Enums and Addresses will be added