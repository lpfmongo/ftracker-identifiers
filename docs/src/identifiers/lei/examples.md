# Examples

## Quick start

```rust,ignore
use ftracker_identifiers::Lei;

let bbc = Lei::parse("5493000IBP32UQZ0KL24").unwrap();
assert_eq!(bbc.lou_prefix(), "5493");
assert_eq!(bbc.entity_id(), "000IBP32UQZ0KL");
assert_eq!(bbc.check_digits(), 24);

let jlr = Lei::parse("213800WSGIIZCXF1P572").unwrap();
assert_eq!(jlr.entity_id(), "00WSGIIZCXF1P5"); // an alphanumeric entity part
```

## Validating untrusted input

Use `Lei::parse` right at the boundary where data enters your system (HTTP handler, a CSV import, a CLI argument) so
that everything downstream can assume a `Lei` is already valid:

```rust,ignore
use ftracker_identifiers::Lei;

fn handle_report(raw_lei: &str) -> Result<(), String> {
    let lei = Lei::parse(raw_lei).map_err(|e| e.to_string())?;
    // From here on, `lei` is guaranteed valid — no need to re-check it.
    file_report(lei);
    Ok(())
}
# fn file_report(_: Lei) {}
```

## Sorting and deduplicating a batch

A common data-cleaning task: importing a spreadsheet or CSV export that may contain the same LEI written multiple ways (
mixed case, surrounded by whitespace), and needing a deduplicated, sorted list:

```rust,ignore
use ftracker_identifiers::Lei;

let mut leis: Vec<Lei> = [
    "213800WSGIIZCXF1P572",
    "5493000IBP32UQZ0KL24",
    " 5493000ibp32uqz0kl24 ", // same LEI as above, lower-cased and padded
]
.into_iter()
.map(|s| Lei::parse(s).unwrap())
.collect();

leis.sort();
leis.dedup();
assert_eq!(leis.len(), 2);
```

## Using `Lei` as a map or set key

Because `Lei` implements `Eq` and `Hash` consistently with `PartialEq`, it works directly as a `HashMap`/`HashSet` key
(or `BTreeMap`/`BTreeSet`, via `Ord`). Useful for deduplicating records or indexing data by entity:

```rust,ignore
use ftracker_identifiers::Lei;
use std::collections::HashMap;

let mut names: HashMap<Lei, &str> = HashMap::new();
names.insert(Lei::parse("5493000IBP32UQZ0KL24").unwrap(), "British Broadcasting Corporation");

let lookup = Lei::parse("5493000ibp32uqz0kl24").unwrap();
assert_eq!(names.get(&lookup), Some(&"British Broadcasting Corporation"));
```

## Grouping entities by issuing LOU

Since `Lei::lou_prefix()` identifies the Local Operating Unit that issued the code, it's a natural grouping key when you
have a mixed list of entities:

```rust,ignore
use ftracker_identifiers::Lei;
use std::collections::HashMap;

fn group_by_lou(leis: &[Lei]) -> HashMap<&str, Vec<Lei>> {
    let mut groups: HashMap<&str, Vec<Lei>> = HashMap::new();
    for &lei in leis {
        groups.entry(lei.lou_prefix()).or_default().push(lei);
    }
    groups
}
```

For a config-file or API round-trip example using `serde`, see [Feature Flags](./feature-flags.md).
