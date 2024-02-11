use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get, 
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String)
}

#[derive(Debug)]
pub struct  HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String
}

impl From<String> for HttpRequest {
    fn from(value: String) -> Self {

        // STRUCTURE OF HTTP REQUEST
        // =================================        
        // METHOD RESOURCE VERSION
        // HEADERS
        // (EMPTY LINE)
        // MESSAGE_BODY

        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        for line in value.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":")   {
                let (key, val) = process_header_line(line);
                parsed_headers.insert(key, val);
            } else if line.len() == 0 {
                
            } else {
                parsed_msg_body = line;
            }
        }

        HttpRequest {
            method: parsed_method,
            resource: parsed_resource,
            version: parsed_version,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }

    }
}

fn process_req_line(s: &str) -> (Method, Resource, Version) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();

    (
        method.into(), Resource::Path(resource.to_string()), version.into()
    )
}

fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.trim().split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = header_items.next() {
        key = k.to_string()
    }
    if let Some(v) = header_items.next() {
        value = v.to_string()
    }

    ( key, value )
}