# fake-json

A CLI tool to generate random data from a JSON schema to fill databases.

## Overview

`fake-json` is a command-line utility that generates realistic fake data based on JSON schemas. It's designed to help developers quickly populate databases with test data during development and testing phases.

## Features

- 🎯 **Schema-driven**: Generate data based on JSON schemas
- 🎲 **Rich data types**: Support for various data types including dates, emails, names, countries, address, and many more
- 🗃️ **Database-friendly**: Perfect for filling databases with realistic test data
- 🔧 **Interactive CLI**: User-friendly command-line interface with interactive prompts, for when you don't know what data-type to choose.

## Installation

```bash
cargo install fake-json
```

## Usage

```txt
Usage: fake-json [OPTIONS]

Options:
  -c, --count <COUNT>    Number of times to repeat the JSON [default: 1]
  -b, --before <BEFORE>  String to print before every data generation of the JSON schema
  -a, --after <AFTER>    String to print after every data generation of the JSON schema
  -f, --file <FILE>      Path to the json schema [default: schema.json]
  -i, --interactive      Select the data type with a terminal dialogue with fuzzy search. This option overrides the others
  -l, --list             List all available data types. This option does not generate any data and overrides the others
  -d, --debug            Debug errors with more precise information
  -h, --help             Print help
```

## Examples

```bash
# Generate data interactively
fake-json --interactive

# Generate data from a specific schema
fake-json --schema schema.json

# Generate multiple records that follow the json format.
fake-json --count 1000

# List all options
fake-json --help
```

### Prisma example

```bash
echo '{
  "firstName": "FirstName",
  "lastName": "LastName",
  "phone_number": "PhoneNumber",
  "email": "FreeEmail"
}' >schema.json


fake-json --count 3 --before "await prisma.person.create({ data: " --after " });
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

await prisma.person.create({ data: {
  "firstName": "Johan",
  "lastName": "Mangin",
  "phone_number": "03 79 01 68 02",
  "email": "thibaud_officiis@hotmail.fr"
} });
```

> [!TIP]
>
> You can do this will any database/ORM, just customise the `--before` and `--after` options!

## Supported Data Types

The tool supports a lots of data types through the [fake](https://github.com/cksac/fake-rs) crate:

- _Addresses_: street, states, countries, coordinates, etc.
- _Dates and times_: timestamps, dates, etc.
- _Internet_: URLs, IP addresses, etc.
- _Identifiers_: UUIDs, ObjectIds, ULIDs, etc.
- _Financial_: currencies, decimal numbers, etc.
- _Information_: people names, emails, phone numbers, health insurance numbers, etc.
- _Text_: words, sentences, passwords, etc.

And many more... Use the `fake-json --list` to list all supported data types!

## Dependencies

- `clap`: Command-line argument parsing
- `fake`: Fake data generation
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
> - random-size arrays
> - pass a single data-type directly
