#![allow(non_snake_case)]

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct HealthCheckResult {
    pub taskId: String,
    pub firstSuccess: String,
    pub lastSuccess: String,
    pub lastFailure: Option<String>,
    pub consecutiveFailures: i32,
    pub alive: bool,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Task {
    pub appId: String,
    pub id: String,
    pub host: String,
    pub ports: Vec<i16>,
    pub startedAt: String,
    pub stagedAt: String,
    pub version: String,
    pub servicePorts: Option<Vec<i16>>,
    pub healthCheckResults: Vec<HealthCheckResult>,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Tasks {
    pub tasks: Vec<Task>,
}

