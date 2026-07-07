# Examples

## Quick start

```rust,ignore
use ftracker_identifiers::Isin;

let apple = Isin::parse("US0378331005").unwrap();
assert_eq!(apple.country_code(), "US");
assert_eq!(apple.nsin(), "037833100");
assert_eq!(apple.check_digit(), 5);

let petrobras = Isin::parse("BRPETRACNOR9").unwrap();
assert_eq!(petrobras.nsin(), "PETRACNOR"); // an alphanumeric NSIN
```

## Validating untrusted input

Use `Isin::parse` right at the boundary where data enters your system (HTTP handler, a CSV import, a CLI argument) so
that everything downstream can assume an `Isin` is already valid:

```rust,ignore
use ftracker_identifiers::Isin;

fn handle_order(raw_isin: &str) -> Result<(), String> {
    let isin = Isin::parse(raw_isin).map_err(|e| e.to_string())?;
    // From here on, `isin` is guaranteed valid — no need to re-check it.
    place_order(isin);
    Ok(())
}
# fn place_order(_: Isin) {}
```

## Sorting and deduplicating a batch

A common data-cleaning task: importing a spreadsheet or CSV export that may contain the same ISIN written multiple
ways (mixed case, surrounded by whitespace), and needing a deduplicated, sorted list:

```rust,ignore
use ftracker_identifiers::Isin;

let mut isins: Vec<Isin> = [
    "US0231351067",
    "US0378331005",
    " us0378331005 ", // same ISIN as above, lower-cased and padded
]
.into_iter()
.map(|s| Isin::parse(s).unwrap())
.collect();

isins.sort();
isins.dedup();
assert_eq!(isins.len(), 2);
```

## Using `Isin` as a map or set key

Because `Isin` implements `Eq` and `Hash` consistently with `PartialEq`, it works directly as a `HashMap`/`HashSet`
key (or `BTreeMap`/`BTreeSet`, via `Ord`). Useful for deduplicating records or indexing data by security:

```rust,ignore
use ftracker_identifiers::Isin;
use std::collections::HashMap;

let mut names: HashMap<Isin, &str> = HashMap::new();
names.insert(Isin::parse("US0378331005").unwrap(), "Apple Inc.");

let lookup = Isin::parse("us0378331005").unwrap();
assert_eq!(names.get(&lookup), Some(&"Apple Inc."));
```

## Grouping securities by issuing country

Since `Isin::country_code()` identifies the issuing national numbering agency, it's a natural grouping key when you have
a mixed list of securities:

```rust,ignore
use ftracker_identifiers::Isin;
use std::collections::HashMap;

fn group_by_country(isins: &[Isin]) -> HashMap<&str, Vec<Isin>> {
    let mut groups: HashMap<&str, Vec<Isin>> = HashMap::new();
    for &isin in isins {
        groups.entry(isin.country_code()).or_default().push(isin);
    }
    groups
}
```

For a config-file or API round-trip example using `serde`, see [Feature Flags](./feature-flags.md).
