use hyper::client::Client as HttpClient;
use hyper::header::{qitem, Accept, ContentType};
use hyper::mime::Mime;
use hyper::mime::TopLevel::Application;
use hyper::mime::SubLevel::Json;
use hyper::method::Method;
use rustc_serialize::Decodable;
use rustc_serialize::json;
use structs::{Leader, Tasks};
use url::{Url, UrlParser};

pub struct Client {
    base_url: Url,
    username: Option<String>,
    password: Option<String>,
}

impl Client {

    pub fn new(url: String, username: Option<String>, password: Option<String>) -> Client {
        Client{
            base_url: Url::parse(&url).unwrap(),
            username: username,
            password: password
        }
    }

    pub fn leader(self) -> Leader {
        self.request("GET", "/v2/leader")
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
            .header(Accept(vec![qitem(Mime(Application, Json, vec![]))]))
            .header(ContentType(Mime(Application, Json, vec![])))
            .send();

        json::decode(&response.unwrap().read_to_string().unwrap()[]).unwrap()
    }

}

