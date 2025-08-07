# random-json

A CLI tool to generate random data from a JSON schema to fill databases.

## Overview

`random-json` is a command-line utility that generates realistic fake data based on JSON schemas. It's designed to help developers quickly populate databases with test data during development and testing phases.

## Features

- 🎯 **Schema-driven**: Generate data based on JSON schemas
- 🎲 **Rich data types**: Support for various data types including dates, emails, names, countries, address, and many more
- 🗃️ **Database-friendly**: Perfect for filling databases with realistic test data
- 🔧 **Interactive CLI**: User-friendly command-line interface with interactive prompts, for when you don't know what data-type to choose.

## Installation

```bash
cargo install random-json
```

## Options

```txt
  -c, --count <COUNT>        Number of times to repeat the JSON [default: 1]
  -b, --before <BEFORE>      String to print before every data generation of the JSON schema
  -a, --after <AFTER>        String to print after every data generation of the JSON schema
  -f, --file <SCHEMA_FILE>   Path to the json schema [default: schema.json]
  -j, --json <JSON>          Pass the JSON from stdout instead of via a json file. Overrides the --file option
  -t, --type <DATA_TYPE>     Generates some data of the given data type. Overrides the other options
  -u, --user <USER_DEFINED>  Add custom data types
  -i, --interactive          Select the data type with a dialog and fuzzy search. Overrides the other options
  -l, --list                 List all available data types. Overrides the other options
  -d, --debug                Debug errors with more precise information
  -h, --help                 Print help
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

And many more... Use the `random-json --list` to list all supported data types!

## Dependencies

- `clap`: Command-line argument parsing
- `serde_json`: JSON serialization/deserialization
- `dialoguer`: Interactive command-line dialogue to select with fuzzy-finder
- `chrono`: Date and time handling
- `rand`: Random number generation (e.g. to choose when a nullable field becomes undefined)

## Upcomming features

> [!NOTE]
>
> Yet to be implemented:
>
> - nullable arrays and objects
> - support for fakers with ranges directly in schemas
