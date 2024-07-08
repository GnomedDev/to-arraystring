## 0.2.2

- Added implementations for `NonZero<$Int>` that just forward to the internal type under the `nonzero_impls` feature flag.

## 0.2.1

- Added a public re-export of `arrayvec::ArrayString` to avoid having to depend on it yourself.

## 0.2.0

- Added a required `ToArrayString::MAX_LENGTH` associated constant.

## 0.1.3

- Added inline to all implementations, which actually reduces codegen as constant folding becomes much better.

## 0.1.2

- Added implementation for `char`
- Added identity implementation for `ArrayString`

## 0.1.1

- Documented MSRV as 1.56.
- Fixed broken documentation links to ToString
- Added documentation for ToArrayString::ArrayString
