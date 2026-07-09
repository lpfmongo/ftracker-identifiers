# Examples

## Quick start

```rust,ignore
use ftracker_identifiers::CountryCode;

let usa = CountryCode::parse("US").unwrap();
assert_eq!(usa.as_str(), "US");

let brazil = CountryCode::parse("br").unwrap(); // lowercase is folded
assert_eq!(brazil.as_str(), "BR");
```

## Validating untrusted input

Use `CountryCode::parse` right at the boundary where data enters your system (an HTTP handler, a CSV import, a CLI
argument) so that everything downstream can assume a `CountryCode` is already valid:

```rust,ignore
use ftracker_identifiers::CountryCode;

fn register(raw_code: &str) -> Result<(), String> {
    let code = CountryCode::parse(raw_code).map_err(|e| e.to_string())?;
    // From here on, `code` is guaranteed to be an assigned ISO 3166-1 alpha-2 code.
    record(code);
    Ok(())
}
# fn record(_: CountryCode) {}
```

## Sorting and deduplicating a batch

A common data cleaning task: importing a spreadsheet or CSV export that may contain the same code written multiple ways
(mixed case, surrounded by whitespace), and needing a deduplicated, sorted list:

```rust,ignore
use ftracker_identifiers::CountryCode;

let mut codes: Vec<CountryCode> = [
    "US",
    "BR",
    " us ", // same code as above, lower cased and padded
]
.into_iter()
.map(|s| CountryCode::parse(s).unwrap())
.collect();

codes.sort();
codes.dedup();
assert_eq!(codes.len(), 2);
```

## Using `CountryCode` as a map or set key

Because `CountryCode` implements `Eq` and `Hash` consistently with `PartialEq`, it works directly as a `HashMap` or
`HashSet` key (or `BTreeMap` or `BTreeSet`, via `Ord`). Useful for counting records by country:

```rust,ignore
use ftracker_identifiers::CountryCode;
use std::collections::HashMap;

fn count_by_country(codes: &[CountryCode]) -> HashMap<CountryCode, usize> {
    let mut counts: HashMap<CountryCode, usize> = HashMap::new();
    for &code in codes {
        *counts.entry(code).or_default() += 1;
    }
    counts
}
```

For a config file or API round trip example using `serde`, see [Feature Flags](./feature-flags.md).
