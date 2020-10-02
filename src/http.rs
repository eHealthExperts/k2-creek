const ENDPOINT_PREFIX: &str = "/k2/public/api";

fn api_endpoint_for_version(version: u8) -> anyhow::Result<&'static str> {
    let s = match version {
        1 => "1/carddata",
        2 => "2/carddata",
        3 => "3/cards",
        _ => bail!("Unsupported API version: {}", version),
    };
    Ok(s)
}

fn build_client(timeout: Option<u64>) -> anyhow::Result<reqwest::blocking::Client> {
    reqwest::blocking::Client::builder()
        .timeout(match timeout {
            Some(timeout) => Some(std::time::Duration::from_secs(timeout)),
            None => None,
        })
        .use_rustls_tls()
        .build()
        .map_err(anyhow::Error::from)
}

pub fn get<T>(url: url::Url, version: u8, timeout: Option<u64>) -> anyhow::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let addr = url.join(&format!(
        "{}/{}",
        ENDPOINT_PREFIX,
        api_endpoint_for_version(version)?
    ))?;

    let client = build_client(timeout)?;

    debug!("Requesting {} ...", addr);
    let response = client.get(addr.clone()).send().map_err(|error| {
        anyhow::Error::new(error).context(format!("Failed to send request for {}", addr))
    })?;
    trace!("{:?}", response);

    let t = response.json::<T>().map_err(|error| {
        anyhow::Error::new(error).context("Unable to map response to known API version!")
    })?;
    Ok(t)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[should_panic(expected = "Unsupported API version: 0")]
    fn unsupported_api_version_zero() {
        api_endpoint_for_version(0).unwrap();
    }

    #[test]
    #[should_panic(expected = "Unsupported API version: 4")]
    fn unsupported_api_version_four() {
        api_endpoint_for_version(4).unwrap();
    }

    #[test]
    fn supported_api_version() {
        assert_eq!(api_endpoint_for_version(1).unwrap(), "1/carddata");
        assert_eq!(api_endpoint_for_version(2).unwrap(), "2/carddata");
        assert_eq!(api_endpoint_for_version(3).unwrap(), "3/cards");
    }

    #[test]
    fn build_client_timeout() {
        assert!(build_client(None).is_ok());
        assert!(build_client(Some(10)).is_ok());
    }

    #[test]
    #[should_panic(expected = "Unsupported API version: 4")]
    fn build_client_wrong_api() {
        get::<serde_json::Value>(url::Url::parse("http://foo.bar").unwrap(), 4, None).unwrap();
    }

    #[test]
    fn build_client_api_endpoint() -> Result<(), anyhow::Error> {
        let server =
            test_server::new("127.0.0.1:0", || test_server::HttpResponse::Ok().body("{}"))?;

        for (version, path) in [(1, "carddata"), (2, "carddata"), (3, "cards")]
            .iter()
            .cloned()
            .collect::<std::collections::HashMap<u8, &str>>()
        {
            let _ = get::<serde_json::Value>(url::Url::parse(&server.url())?, version, None)?;
            let req = server.requests.next().unwrap();

            assert_eq!(
                req.uri().path(),
                format!("{}/{}/{}", ENDPOINT_PREFIX, version, path)
            );
        }

        Ok(())
    }

    #[test]
    fn build_client_without_timeout() -> Result<(), anyhow::Error> {
        let server = test_server::new("127.0.0.1:0", || {
            std::thread::sleep(std::time::Duration::from_secs(2));
            test_server::HttpResponse::Ok().body("{ \"foo\": \"bar\" }")
        })?;

        let res = get::<serde_json::Value>(url::Url::parse(&server.url())?, 1, None);

        assert!(res.is_ok(), format!("{:#?}", res));
        assert_eq!(res?, serde_json::json!({ "foo": "bar" }));

        Ok(())
    }

    #[test]
    fn build_client_with_timeout() -> Result<(), anyhow::Error> {
        let server = test_server::new("127.0.0.1:0", || {
            std::thread::sleep(std::time::Duration::from_secs(2));
            test_server::HttpResponse::Ok().body("{ \"foo\": \"bar\" }")
        })?;

        let res = get::<serde_json::Value>(url::Url::parse(&server.url())?, 1, Some(2));

        assert_eq!(
            format!("{}", res.err().unwrap()),
            format!(
                "Failed to send request for {}/k2/public/api/1/carddata",
                &server.url()
            )
        );

        Ok(())
    }
}
