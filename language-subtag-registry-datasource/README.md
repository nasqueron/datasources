The `language-subtag-registry-datasource` utility allows to download
IANA language subtag registry datasource defined in the RFC 5646,
parse it, and transform the output.

This registry shares language codes with the different ISO-639 lists,
but is more inclusive and descriptive.

It has been designed to output the index in an arbitrary format,
so we can export a Darkbot database for Odderon, one of our IRC bot.

## Usage

```
language-subtag-registry-datasource
    --format <format string>
    [--languages-only]
    [--aggregation-separator <separator string>]
    [--source /path/to/registry.txt]`
```

The format string can be arbitrary text or variables:

| **Variable**    | **Description**                           |
|-----------------|-------------------------------------------|
| %%id%%          | The Tag or Subtag field of the entry      |
| %%<key>%%       | A field in the registry entry             |
| %%fullstatus%%  | A string built with description, comments |

If an entry doesn't have the required field, it left blank.

Examples for the variables:
  - `%%Description%%` will output `Inupiaq` for the `ik` subtag
  - `%%Description%%` will output `Sichuan Yi / Nuosu` for the `ii` subtag
  - `%%Comments%%` will output an empty string for both `ik` and `ii` subtags
  - `%%fulldescription%%` will output "Serbo-Croatian - sr, hr, bs are preferred for most modern uses" for `sh`

If a language has several values, they are coalesced and a specific string
is used as separator. Default separator is " / ". It can be overridden with
`--aggregation-separator`.

The database contains entries of other types than languages, like variants, regions or redundant.
To only parse languages, use `-l` or `--languages-only` flag.

The utility uses as source, by order of priority:
    - the path specified to the `--source` argument
    - any `registry.txt` file available in the current directory
    - https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry

## Recipes

### Darkbot database

    language-subtag-registry-datasource -l --format "lang+%%id%% %%fulldescription%%"

### CSV export

Identify the fields and the order you wish to use.

For example, to create a CSV with the following header:

    Type,Subtag,Tag,Added,Suppress-Script,Preferred-Value,Comments,Scope,Macrolanguage,Deprecated,Description

Use:

    language-subtag-registry-datasource --format '"%%Type%%","%%Subtag%%","%%Tag%%","%%Added%%","%%Suppress-Script%%","%%Preferred-Value%%","%%Comments%%","%%Scope%%","%%Macrolanguage%%","%%Deprecated%%","%%Description%%"'
