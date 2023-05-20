The `rfc-datasource` utility allows to download the RFC index, parse it,
and transform the output.

It has been designed to output the index in an arbitrary RFC format,
so we can export a Darkbot database for Odderon, one of our IRC bot.

## Usage

`rfc-datasource --format <format string> [--source /path/to/rfc-index.txt]`

The format string can be arbitrary text or variables:

| **Variable**    | **Description**                                               |
|-----------------|---------------------------------------------------------------|
| %%id%%          | The number of the RFC without leading 0                       |
| %%<len>id%%     | The number of the RFC with leading 0 to fill <len> digits (1) |
| %%description%% | The RFC title, authors and date                               |
| %%status%%      | The RFC status (2)                                            |
| %%fullstatus%%  | A string summarizing the different status notes (3)           |

Examples for the variables:
  - (1) e.g. `%%4id%%` will output `0065` for the RFC 65
  - (2) e.g. `INFORMATIONAL` for RFC 2286
  - (3) e.g. `Obsoletes RFC1938. Status: DRAFT STANDARD.` for RFC 2289

The utility uses as source, by order of priority:
    - the path specified to the --source argument
    - any `rfc-index.txt` file available in the current directory
    - https://www.ietf.org/download/rfc-index.txt

## Recipes

### Darkbot database

    rfc-datasource --format "rfc+%%id%% %%description%% %%fullstatus%%"

### CSV export

    rfc-datasource --format '%%id%%,"%%description%%", "%%status%%"'
