# fake-json

A CLI tool to generate random data from a JSON schema to fill databases.

## Overview

`fake-json` is a command-line utility that generates realistic fake data based on JSON schemas. It's designed to help developers quickly populate databases with test data during development and testing phases.

## Features

- 🎯 **Schema-driven**: Generate data based on JSON schemas
- 🎲 **Rich data types**: Support for various data types including dates, emails, UUIDs, and more
- 🗃️ **Database-friendly**: Perfect for filling databases with realistic test data
- 🔧 **Interactive CLI**: User-friendly command-line interface with interactive prompts, for when you don't want data-type to choose.

## Installation

```bash
cargo install fake-json
```

## Usage

```txt
Usage: fake-json [OPTIONS]

Options:
  -c, --count <COUNT>    Number of times to repeat the JSON [default: 1]
  -b, --before <BEFORE>  String to print before every data generation of the JSON schema [default: ]
  -a, --after <AFTER>    String to print after every data generation of the JSON schema [default: ]
  -f, --file <FILE>      Path to the json schema [default: schema.json]
  -i, --interactive      List and select the random generator with a terminal dialogue. This option overrides the others
  -l, --list             List all available data types. This option does not generate any data and overrides the others
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

## Supported Data Types

The tool supports a lots of data types through the [fake](https://github.com/cksac/fake-rs) crate:

- _Basic types_: strings, numbers, booleans
- _Dates and times_: timestamps, dates, etc.
- _Internet_: emails, URLs, IP addresses
- _Identifiers_: UUIDs, ObjectIds, ULIDs
- _Geographic_: coordinates, addresses
- _Financial_: currencies, decimal numbers

And many more... Use the `fake-json --list` to list all support data types!

## Dependencies

- `clap`: Command-line argument parsing
- `fake`: Fake data generation with extensive feature support
- `serde_json`: JSON serialization/deserialization
- `dialoguer`: Interactive command-line prompts
- `chrono`: Date and time handling
- `rand`: Random number generation
