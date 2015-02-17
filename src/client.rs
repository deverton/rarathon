use hyper::client::Client as HttpClient;
use hyper::header::{qitem, Accept, Authorization, Basic, ContentType, Headers};
use hyper::mime::Mime;
use hyper::mime::TopLevel::Application;
use hyper::mime::SubLevel::Json;
use hyper::method::Method;
use rustc_serialize::Decodable;
use rustc_serialize::json;
use {Apps, Leader, Tasks};
use url::{Url, UrlParser};

pub struct Client {
    base_url: Url,
    headers: Headers,
}

impl Client {

    pub fn new(url: String, username: Option<String>, password: Option<String>) -> Client {
        let mut headers = Headers::new();

        // All JSON all the time
        headers.set(Accept(vec![qitem(Mime(Application, Json, vec![]))]));
        headers.set(ContentType(Mime(Application, Json, vec![])));

        if username.is_some() {
            headers.set(Authorization(Basic { username: username.unwrap(), password: password }));
        }

        Client{
            base_url: Url::parse(&url).unwrap(),
            headers: headers,
        }
    }

    pub fn leader(self) -> Leader {
        self.request("GET", "/v2/leader")
    }

    pub fn list_apps(self) -> Apps {
        self.request("GET", "/v2/apps")
    }

    pub fn list_tasks(self, id: String) -> Tasks {
        self.request("GET", &format!("/v2/apps/{}/tasks", id))
    }

    fn request<T: Decodable>(self, method: &str, path: &str) -> T {
        let mut client = HttpClient::new();

        let url = UrlParser::new()
            .base_url(&self.base_url)
            .parse(path)
            .unwrap();

        let response = client
            .request(method.parse::<Method>().unwrap(), &url.to_string()[])
            .headers(self.headers)
            .send();

        json::decode(&response.unwrap().read_to_string().unwrap()[]).unwrap()
    }

}

