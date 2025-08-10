# Schema breakdown

Let's take an example of schema and explain every feature available:

```json
{
  "address": {
    "city": "Delhi",
    "country": "Singapore",
    "nb": "101",
    "owner": "Melton",
    "postal_code": "GU4Q 1XC",
    "street": "Las Olas Boulevard"
  },
  "age": 86,
  "data_origin": {
    "cli": "random-json",
    "url": "https://github.com/t-webber/random-json"
  },
  "family_name": "Melton",
  "given_name": "Roy",
  "job": ["Illustrator", "Professor"],
  "received_invitation": "True",
  "received_letter": false,
  "salary": "Low"
}
```

An example of output is

```json
{
  "address": {
    "city": "Helsinki",
    "country": "Montenegro",
    "nb": "30",
    "owner": "Perry",
    "postal_code": "B9I 1WY",
    "street": "La Rambla"
  },
  "age": 78,
  "data_origin": {
    "cli": "random-json",
    "url": "https://github.com/t-webber/random-json"
  },
  "family_name": "Perry",
  "given_name": "Jocelyn",
  "received_invitation": "True",
  "received_letter": true,
  "salary": "Medium"
}
```

- `age` is a number between 0 and 100, which is what `"0..100"` means.
- `family_name` and `owner` are identical, which is the meaning of `[1]`. You can put anything in the brackets and it will put the same value at all the other occurrences.
- `email` is missing, because it was marked as optional with `?`.
- the fields in `data_origin` weren't converted to data or interpreted as data types because it has the `!` suffix which means: leave this value intact.
- you can define enums in the value, like `High|Medium|Low`.
- you can ask for booleans in the bool format with `"Bool"` or in the string format with `"Boolean"`
- the `*` means that if you ask for multiple JSON with the `--count` option, it will never give you twice the same value, here `Country`.
- `job` contains a list of between 2 and 5 jobs:

        - `["Job"]` generates an array of length between 0 and 10 of jobs;
        - `["Job", 23]` generates an array of length 23 of jobs;
        - `["Job", 3, 17]` generates an array of length between 3 and 17 of jobs.

# CLI breakdown

## Interactive mode

Fuzzy find your data-type.

```bash
random-json -i
```

![Video example of Interactive mode](docs/interactive.mp4)

## Enums

Let's take a new JSON:

```JSON
{
    "language": "ProgrammingLanguage",
    "type": "Type"
}
```

Well, the `"Type"` doesn't exist, and we want to pass it through the CLI, we can do this:

```bash
random-json -u "Type:Compiled|JIT|Interpreted" -c 3 --after ','
```

This may output:

```json
{
  "language": "R",
  "type": "JIT"
},
{
  "language": "Elm",
  "type": "Compiled"
},
{
  "language": "Lisp",
  "type": "Interpreted"
},
```

The `--after` simply added a comma between the 3 generations.

## Further

Use `--help` to see all the options available!
