## How to use?

Define your PostgreSQL connection URL in environment:

```
export DATABASE_URL="postgres://fantoir:fantoir@localhost/fantoir"
```

## Development

### Build instructions

The PostgreSQL library is required to link against it.
If not found, you can add the path to the LIB environment variable.

### Prepare a test database

Execute the following queries as postgres user:

```
CREATE ROLE fantoir WITH PASSWORD 'fantoir' LOGIN;
CREATE DATABASE fantoir OWNER fantoir;
```

Connected as your database role, enable the pg_trgm extension
to be able to generate the index for full-text search with trigrams:

```
CREATE EXTENSION pg_trgm;
```

If the extension doesn't exist, it can be included in a package
named for example `postgresql-contrib`.

You can then use the code with the default DATABASE_URL documented above.

### Database pitfalls

The FANTOIR database uses INSEE department code, they can contain a letter,
currently only for Corse (2A and 2B).
That also applies when building the INSEE commune code.

If a record is canceled, the cancel date can be omitted.
The creation date can be omitted too.

The last line of the FANTOIR database must be ignored.

Wikidata uses the "code FANTOIR", matching the "code RIVOLI"
documented in FANTOIR file description. This code matches the
11 first characters of a record.
See also https://www.wikidata.org/wiki/Property:P3182.
