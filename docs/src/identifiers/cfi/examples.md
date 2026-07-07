# Examples

## Quick start

```rust,ignore
use ftracker_identifiers::Cfi;

let equity = Cfi::parse("ESVUFR").unwrap();
assert_eq!(equity.category(), 'E');
assert_eq!(equity.group(), 'S');
assert_eq!(equity.attributes(), ['V', 'U', 'F', 'R']);

let bond = Cfi::parse("DBFTFB").unwrap();
assert_eq!(bond.category(), 'D'); // debt instrument
```

## Validating untrusted input

Use `Cfi::parse` right at the boundary where data enters your system (HTTP handler, a CSV import, a
CLI argument) so that everything downstream can assume a `Cfi` is already valid:

```rust,ignore
use ftracker_identifiers::Cfi;

fn classify(raw_cfi: &str) -> Result<(), String> {
    let cfi = Cfi::parse(raw_cfi).map_err(|e| e.to_string())?;
    // From here on, `cfi` is guaranteed to be a valid ISO 10962 classification.
    record(cfi);
    Ok(())
}
# fn record(_: Cfi) {}
```

## Sorting and deduplicating a batch

A common data-cleaning task: importing a spreadsheet or CSV export that may contain the same CFI
written multiple ways (mixed case, surrounded by whitespace), and needing a deduplicated, sorted
list:

```rust,ignore
use ftracker_identifiers::Cfi;

let mut cfis: Vec<Cfi> = [
    "DBFTFB",
    "ESVUFR",
    " esvufr ", // same CFI as above, lower-cased and padded
]
.into_iter()
.map(|s| Cfi::parse(s).unwrap())
.collect();

cfis.sort();
cfis.dedup();
assert_eq!(cfis.len(), 2);
```

## Using `Cfi` as a map or set key

Because `Cfi` implements `Eq` and `Hash` consistently with `PartialEq`, it works directly as a
`HashMap`/`HashSet` key (or `BTreeMap`/`BTreeSet`, via `Ord`). Useful for counting instruments by
classification:

```rust,ignore
use ftracker_identifiers::Cfi;
use std::collections::HashMap;

fn count_by_classification(cfis: &[Cfi]) -> HashMap<Cfi, usize> {
    let mut counts: HashMap<Cfi, usize> = HashMap::new();
    for &cfi in cfis {
        *counts.entry(cfi).or_default() += 1;
    }
    counts
}
```

## Grouping instruments by category

Since `Cfi::category()` is the broadest classification level, it's a natural grouping key when you
have a mixed list of instruments:

```rust,ignore
use ftracker_identifiers::Cfi;
use std::collections::HashMap;

fn group_by_category(cfis: &[Cfi]) -> HashMap<char, Vec<Cfi>> {
    let mut groups: HashMap<char, Vec<Cfi>> = HashMap::new();
    for &cfi in cfis {
        groups.entry(cfi.category()).or_default().push(cfi);
    }
    groups
}
```

For a config-file or API round-trip example using `serde`, see [Feature Flags](./feature-flags.md).
