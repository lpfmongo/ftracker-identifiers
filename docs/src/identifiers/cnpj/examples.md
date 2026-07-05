# Examples

## Quick start

```rust,ignore
use ftracker_identifiers::Cnpj;

let numeric = Cnpj::parse("00.000.000/0001-91").unwrap();
assert!(numeric.is_root());
assert_eq!(numeric.as_str(), "00000000000191");
assert_eq!(numeric.formatted().as_str(), "00.000.000/0001-91");

let alpha = Cnpj::parse("12ABC34501DE35").unwrap();
assert_eq!(alpha.branch_code(), "01DE");
assert_eq!(alpha.branch_number(), None);
```

## Validating untrusted input

Use `Cnpj::parse` right at the boundary where data enters your system — an HTTP handler, a CSV
import, a CLI argument — so that everything downstream can assume a `Cnpj` is already valid:

```rust,ignore
use ftracker_identifiers::Cnpj;

fn handle_signup(raw_cnpj: &str) -> Result<(), String> {
    let cnpj = Cnpj::parse(raw_cnpj).map_err(|e| e.to_string())?;
    // From here on, `cnpj` is guaranteed valid — no need to re-check it.
    save_company(cnpj);
    Ok(())
}
# fn save_company(_: Cnpj) {}
```

## Sorting and deduplicating a batch

A common data-cleaning task: importing a spreadsheet or CSV export that may contain the same CNPJ
written multiple ways (with or without punctuation, mixed case), and needing a deduplicated,
sorted list:

```rust,ignore
use ftracker_identifiers::Cnpj;

let mut cnpjs: Vec<Cnpj> = [
    "11.222.333/0002-62",
    "00.000.000/0001-91",
    "00000000000191", // same CNPJ as above, written without punctuation
]
.into_iter()
.map(|s| Cnpj::parse(s).unwrap())
.collect();

cnpjs.sort();
cnpjs.dedup();
assert_eq!(cnpjs.len(), 2);
```

## Using `Cnpj` as a map or set key

Because `Cnpj` implements `Eq` and `Hash` consistently with `PartialEq`, it works directly as a
`HashMap`/`HashSet` key (or `BTreeMap`/`BTreeSet`, via `Ord`) — useful for deduplicating records or
indexing data by company:

```rust,ignore
use ftracker_identifiers::Cnpj;
use std::collections::HashMap;

let mut companies: HashMap<Cnpj, &str> = HashMap::new();
companies.insert(Cnpj::parse("00.000.000/0001-91").unwrap(), "Banco do Brasil");

let lookup = Cnpj::parse("00000000000191").unwrap();
assert_eq!(companies.get(&lookup), Some(&"Banco do Brasil"));
```

## Grouping branches by root

Since `Cnpj::root()` identifies the entity regardless of branch, it's a natural grouping key when
you have several branch records for the same company:

```rust,ignore
use ftracker_identifiers::Cnpj;
use std::collections::HashMap;

fn group_by_company(cnpjs: &[Cnpj]) -> HashMap<&str, Vec<Cnpj>> {
    let mut groups: HashMap<&str, Vec<Cnpj>> = HashMap::new();
    for &cnpj in cnpjs {
        groups.entry(cnpj.root()).or_default().push(cnpj);
    }
    groups
}
```

For a config-file or API round-trip example using `serde`, see
[Feature Flags](./feature-flags.md).