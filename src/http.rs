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
        .timeout(timeout.map(std::time::Duration::from_secs))
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
    use serde_json::{json, Value};
    use std::time::Duration;
    use url::Url;
    use wiremock::{matchers::any, Mock, MockServer, ResponseTemplate};

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

    #[async_std::test]
    async fn build_client_api_endpoint() -> Result<(), anyhow::Error> {
        let mock_server = MockServer::start().await;
        let response = ResponseTemplate::new(200).set_body_json(json!({}));
        let mock = Mock::given(any()).respond_with(response);
        mock_server.register(mock).await;

        for (version, path) in [(1, "carddata"), (2, "carddata"), (3, "cards")]
            .iter()
            .cloned()
            .collect::<std::collections::HashMap<u8, &str>>()
        {
            let _ = get::<serde_json::Value>(url::Url::parse(&mock_server.uri())?, version, None)?;
            match &mock_server.received_requests().await {
                Some(requests) if requests.last().is_some() => {
                    let request = requests.last().unwrap();

                    assert_eq!(
                        request.url.path(),
                        format!("{}/{}/{}", ENDPOINT_PREFIX, version, path)
                    );
                }
                _ => bail!("Missing requests"),
            }
        }

        Ok(())
    }
    #[async_std::test]
    async fn build_client_without_timeout() -> Result<(), anyhow::Error> {
        let mock_server = MockServer::start().await;
        let response = ResponseTemplate::new(200)
            .set_body_json(json!({"foo":"bar"}))
            .set_delay(Duration::from_secs(2));
        let mock = Mock::given(any()).respond_with(response);
        mock_server.register(mock).await;

        let res = get::<Value>(Url::parse(&mock_server.uri())?, 1, None);

        assert!(res.is_ok(), "{:#?}", res);
        assert_eq!(res?, json!({ "foo": "bar" }));

        Ok(())
    }

    #[async_std::test]
    async fn build_client_with_timeout() -> Result<(), anyhow::Error> {
        let mock_server = MockServer::start().await;
        let response = ResponseTemplate::new(200)
            .set_body_json(json!({"foo":"bar"}))
            .set_delay(Duration::from_secs(2));
        let mock = Mock::given(any()).respond_with(response);
        mock_server.register(mock).await;

        let res = get::<Value>(Url::parse(&mock_server.uri())?, 1, Some(2));

        assert_eq!(
            format!("{}", res.err().unwrap()),
            format!(
                "Failed to send request for {}/k2/public/api/1/carddata",
                &mock_server.uri()
            )
        );

        Ok(())
    }
}
