use reqwest::{self, header, Client, Error};
pub struct QiitaClient {
    client: Client,
}

impl QiitaClient {
    pub fn new(api_key: &str) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(api_key).unwrap(),
        );
        let client = Client::builder().default_headers(headers).build();

        match client {
            Ok(client) => Self { client },
            Err(_) => Self {
                client: Client::new(),
            },
        }
    }

    pub async fn get<T: serde::de::DeserializeOwned>(self, url: &str) -> Result<T, Error> {
        println!("url: {}", url);
        let res = self.client.get(url).send().await?;
        let ans = res.json::<T>().await?;
        Ok(ans)
    }

    pub async fn find_user_by_name(self, name: &str) -> Result<bool, Error> {
        let url = format!("https://qiita.com/api/v2/users/{}", name);
        let res = self.client.get(&url).send().await?;
        if res.status() == 404 {
            Ok(false)
        } else {
            Ok(true)
        }
    }
}
