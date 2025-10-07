# random-json

A CLI tool to generate random data from a JSON schema to fill databases.

## Overview

`random-json` is a command-line utility that generates realistic fake data based on JSON schemas. It's designed to help developers quickly populate databases with test data during development and testing phases.

See list of all the support data types [here](https://github.com/t-webber/random-data?tab=readme-ov-file#available-data-types).

## Installation

```bash
cargo install random-json
```

## Options

```txt
  -c, --count <COUNT>         Number of times to repeat the JSON [default: 1]
  -b, --before <BEFORE>       String to print before every JSON generation
  -a, --after <AFTER>         String to print after every JSON generation
  -f, --schema <SCHEMA_FILE>  Path to the json schema [default: schema.json]
  -j, --json <JSON>           Pass the JSON from stdout instead of via a json file
  -t, --type <DATA_TYPE>      Generates some data of the given data type
  -u, --user <USER_DEFINED>   Add custom data types, with the format 'Type:Value1|Value2'
  -i, --interactive           Select the data type with a dialog and fuzzy search
  -l, --list                  List all available data types
  -v, --values <VALUES>       List all values of a type
  -d, --debug                 Debug errors with more precise information
  -s, --seed <SEED>           Generate with a given random seed
  -h, --help                  Print help
```

## Examples

```bash
# Generate data interactively
random-json --interactive

# Generate data from a specific schema
random-json --schema schema.json

# Generate multiple records that follow the json format.
random-json --count 1000

# List all options
random-json --help
```

### Usage guide

See [USAGE.md](USAGE.md) for more details on the schema and CLI syntax.

### Prisma example

```bash
echo '{
  "firstName": "FirstName",
  "lastName": "LastName",
  "phone_number": "PhoneNumber?",
  "email": "Email"
}' >schema.json


random-json --count 2 --before "await prisma.person.create({ data: " --after " });
"
```

and the output will be

```prisma
await prisma.person.create({ data: {
  "firstName": "Eliane",
  "lastName": "Perret",
  "phone_number": "08 32 72 03 11",
  "email": "stanislas_quia@free.fr"
} });

await prisma.person.create({ data: {
  "firstName": "Herbert",
  "lastName": "Salles",
  "phone_number": "02 33 18 29 16",
  "email": "pascal_amet@laposte.fr"
} });
```

> [!TIP]
>
> You can do this will any database/ORM, just customise the `--before` and `--after` options!

## Supported Data Types

The tool supports a lots of data types through the [random-data](https://github.com/t-webber/random-data) crate:

- _Addresses_: street, states, countries, coordinates, etc.
- _Dates and times_: timestamps, dates, etc.
- _Internet_: URLs, IP addresses, etc.
- _Identifiers_: UUIDs, ObjectIds, ULIDs, etc.
- _Financial_: currencies, decimal numbers, etc.
- _Information_: people names, emails, phone numbers, health insurance numbers, etc.
- _Text_: words, sentences, passwords, etc.

Use the `random-json --list` to list all supported data types.

## Dependencies

- `clap`: Command-line argument parsing
- `serde_json`: JSON serialization/deserialization
- `random-data`: Fake data generation
- `dialoguer`: Interactive command-line dialogue to select with fuzzy-finder
- `rand`: Random number generation (e.g. to choose when a nullable field becomes undefined)
