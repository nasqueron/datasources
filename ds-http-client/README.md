# HTTP client for Nasqueron Datasources components

The crate ds-http-client is a HTTP client
based on Hyper / reqwest components.

It can be used to download a file on an HTTP server,
or query an API with User-Agent header.

## Usage example

### Initialize a client

    ```
    use ds_http_client::Client;

    let mut headers = HashMap::new();
    headers.insert(
        "User-Agent".to_string(),
        "foo/1.2.3".to_string(),
    );

    let client = Client::new(Some(headers));
    ```

### Download a file

    ```
    let url = "http://www.example.com/example.tar.gz";
    let target_path = "/tmp/example.tar.gz";

    if let Err(error) = client().download(&url, &target_path).await {
        eprintln!("Can't download file: {:?}", error);
    }
    ```
