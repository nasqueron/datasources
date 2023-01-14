## Opendatasoft Explore API

The crate opendatasoft-explore-api is an Opendatasoft Explore API v2 client,
intended to be used in Rust projects to query any open data server using
this software, such as the open data portal from French economy agency.

As the API is stable, this tool can be used to query metadata or records
of open data portals from various companies and public administrations.

It's used at Nasqueron in the fantoir-datasource tool to query attachments
information for the FANTOIR file and determine if a new file is available.

## How to use it?

First, add `opendatasoft-explore-api` to your Cargo.toml dependencies.
You also need an async implementation. Our code is tested with Tokio.

Then, you can create an instance of our ExploreApiEndPoint structure.
It's then ready to query the API:

```rust
use opendatasoft_explore_api::requests::ExploreApiEndPoint;

static API_URL: &'static str = "https://data.economie.gouv.fr/api/v2";
static DATASET_ID: &'static str = "fichier-fantoir-des-voies-et-lieux-dits";

#[tokio::main]
async fn main() {
    let endpoint = ExploreApiEndPoint::new(API_URL);

    let dataset = endpoint.get_dataset_information(DATASET_ID).await;
    println!("{:?}", dataset);
}
```

Documentation is available at https://docs.rs/opendatasoft-explore-api

A real-use example can also be found in the same repository
in the fantoir-datasource/src/commands/fetch folder.

## License

Source code is released under BSD-2-Clause license.
(c) 2022-2023 Nasqueron project, some rights reserved.

Nasqueron is a free culture and open source project,
not affiliated to the Opendatasoft company.

**Note:** the files in tests/requests/ used by integration tests
describe datasets licensed under Licence Ouverte v2.0 (Etalab).
They are NOT included in the compiled library.

### Known limitations

Currently, this implementation doesn't cover:
- authentication, as the code is currently used on platforms not requiring authentication
- optional parameters, only mandatory ones are implemented

Get in touch if you're interested either in implementing or if you need one of those.
That will help us to prioritize those.

## Contribute

### Useful resources

* [Project board](https://devcentral.nasqueron.org/project/view/6/) for issues and features requests
* [How to contribute code](https://agora.nasqueron.org/How_to_contribute_code)


### Tests

Integration tests for requests are located in the tests/ folders.

The files in test/requests/ are cached from real queries
made against the data.economie.gouv.fr API portal:
https://data.economie.gouv.fr/api/v2/console
