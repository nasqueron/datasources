# Nasqueron Datasources

This repository offers components to query, extract and enrich data.

Those components are intended to build data pipelines.

## Components

### FANTOIR import tool (fantoir-datasource)

Import a file from [FANTOIR file][1] into PostgreSQL.

Enrich it from other sources like Wikidata.

More information: [fantoir-datasource README](fantoir-datasource/README.md)

### IANA language subtag registry (language-subtag-registry-datasource)

Import IANA language subtag registry datasource from RFC 5646 and convert it to
the  specified text-based format.

Can be used to refresh language Darkbot database for IRC bots.

More information: [language-subtag-registry-datasource README](language-subtag-registry-datasource/README.md)

### RFC import fool (rfc-datasource)

Import RFC index and convert it to the specified text-based format.

Can be used to refresh RFC Darkbot database for IRC bots.

More information: [rfc-datasource README](rfc-datasource/README.md)

### Opendatasoft Explore API client (opendatasoft-explore-api)

The opendatasoft-explore-api crate allows to query the Opendatasoft Explore API from Rust code.

This API software is for example used for data.economie.gouv.fr for open data.

## Repository structure

The repository is structured in subdirectories for components.
It's a **monorepo** of tightly tied components to build our data pipelines.

To contribute to one of those components, simply clone this monorepo
and send a pull request with a branch against like any other repository.

To install only one component, you can use cargo. For example,
`cargo install fantoir-datasource` will only install the
`fantoir-datasource` binary.

To include a component in your own project, just include its name in Cargo.toml,
crates.io and Cargo supports crates in Git subdirectories and this will only
download and compile the needed component and its dependencies, ignoring others.

There is no plan to export this monorepo in polyrepo/manyrepo as long as
it contains only Rust code. We'd of course export Composer or npm packages,
as it's a requirement of their respective packages managers.

## License

Code is available under BSD-2-Clause license.

Datasets imported by those tools are published under their own respective licenses.

## Notes

### Interesting links

  * [Documentation on Agora](https://agora.nasqueron.org/Nasqueron_Datasources)
  * [Project board](https://devcentral.nasqueron.org/project/view/6/) for issues and features requests
  * [How to contribute](https://agora.nasqueron.org/How_to_contribute_code)

### Not to be confused with

  * ***Nasqueron API datasources*** (rAPIS): exposes API for data
    less easy to parse, see https://api.nasqueron.org/datasources/

  * ***Nasqueron Databases*** (rDB): front-end for datasources and
    other sources of databases, ie this datasources repository
    prepares and enriches data than can then be used in https://db.nasqueron.org


[1]: <https://data.economie.gouv.fr/explore/dataset/fichier-fantoir-des-voies-et-lieux-dits/information/> "FANTOIR sur data.economie.gouv.fr"
