#![allow(non_camel_case_types,non_snake_case)]
use rustc_serialize::{Decoder, Decodable, Encoder, Encodable};
use std::collections::HashMap;

// Mesos types

#[derive(RustcDecodable, RustcEncodable, PartialEq, Eq, Debug)]
pub enum VolumeMode {
    RW,
    RO
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Volume {
    pub containerPath: String,
    pub hostPath: Option<String>,
    pub mode: VolumeMode, 
}

#[derive(RustcDecodable, RustcEncodable, PartialEq, Eq, Debug)]
enum ContainerType {
    DOCKER,
    MESOS
}

#[derive(RustcDecodable, RustcEncodable, PartialEq, Eq, Debug)]
pub enum DockerNetwork {
    HOST,
    BRIDGE,
    NONE
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Parameter {
    pub key: String,
    pub value: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct PortMapping {
    pub hostPort: u32,
    pub containerPort: u32,
    pub protocol: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct DockerContainerInfo {
    pub image: String,
    pub network: Option<DockerNetwork>,
    pub portMappings: Option<Vec<PortMapping>>,
    pub privileged: Option<bool>,
    pub parameters: Option<Vec<Parameter>>,
    pub force_pull_image: Option<bool>,
}

pub struct ContainerInfo {
    pub container_type: String,
    pub volumes: Vec<Volume>,
    pub hostname: Option<String>,
    pub docker: Option<DockerContainerInfo>,
}

impl Decodable for ContainerInfo {
    fn decode<D: Decoder>(d: &mut D) -> Result<ContainerInfo, D::Error> {
        d.read_struct("ContainerInfo", 4, |d| {
            Ok(ContainerInfo{
                container_type: try!(d.read_struct_field("type", 0, |d| Decodable::decode(d))),
                volumes: try!(d.read_struct_field("volumes", 1, |d| Decodable::decode(d))),
                hostname: try!(d.read_struct_field("hostname", 2, |d| Decodable::decode(d))),
                docker: try!(d.read_struct_field("docker", 3, |d| Decodable::decode(d))),
            })
        })
    }
}

impl Encodable for ContainerInfo {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_struct("ContainerInfo", 4, |e| {
          try!(e.emit_struct_field("type", 0, |e| self.container_type.encode(e)));
          try!(e.emit_struct_field("volumes", 1, |e| self.volumes.encode(e)));
          try!(e.emit_struct_field("hostname", 2, |e| self.hostname.encode(e)));
          try!(e.emit_struct_field("docker", 3, |e| self.docker.encode(e)));
          Ok(())
       })
    }
}

// Marathon types

#[derive(RustcDecodable, RustcEncodable, PartialEq, Eq, Debug)]
pub enum ConstraintOperator {
    UNIQUE,
    LIKE,
    CLUSTER,
    GROUP_BY,
    UNLIKE,
}

pub struct Constraint {
    pub field: String,
    pub operator: ConstraintOperator,
    pub value: Option<String>,
}

impl Decodable for Constraint {
    fn decode<D: Decoder>(d: &mut D) -> Result<Constraint, D::Error> {
        d.read_seq(|d, len| {
            let field = try!(d.read_seq_elt(0, |d| Decodable::decode(d)));
            let operator = try!(d.read_seq_elt(1, |d| Decodable::decode(d)));
            let value = match len {
                3 => Some(try!(d.read_seq_elt(2, |d| Decodable::decode(d)))),
                _ => None
            };
            Ok(Constraint{field: field, operator: operator, value: value})
        })
    }
}

impl Encodable for Constraint {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        let len = match self.value.is_none() {
            true  => 2,
            false => 3,
        };
        s.emit_seq(len, |s| {
            try!(s.emit_seq_elt(0, |s| self.field.encode(s)));
            try!(s.emit_seq_elt(1, |s| self.operator.encode(s)));
            if self.value.is_some() {
                try!(s.emit_seq_elt(2, |s| self.value.encode(s)));
            }
            Ok(())
        })
    }
}

#[derive(RustcDecodable, RustcEncodable, PartialEq, Eq, Debug)]
pub enum HealthCheckProtocol {
    HTTP,
    TCP,
    COMMAND,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct HealthCheck {
    pub protocol: HealthCheckProtocol,
    pub portIndex: u32,
    pub gracePeriodSeconds: Option<u32>,
    pub intervalSeconds: Option<u32>,
    pub timeoutSeconds: Option<u32>,
    pub path: Option<String>,
    pub maxConsecutiveFailures: Option<u32>,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Deployment {
    pub id: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct UpgradeStrategy {
    pub minimumHealthCapacity: f64,
    pub maximumOverCapacity: f64,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct ServiceDefinition {
    pub id: String,
    pub cmd: Option<String>,
    pub args: Option<Vec<String>>,
    pub user: Option<String>,
    pub env: HashMap<String, String>,
    pub instances: i32,
    pub cpus: f64,
    pub mem: f64,
    pub disk: f64,
    pub executor: String,
    pub constraints: Vec<Constraint>,
    pub uris: Vec<String>,
    pub storeUrls: Vec<String>,
    pub ports: Vec<i32>,
    pub requirePorts: bool,
    pub backoffSeconds: i64,
    pub backoffFactor: f64,
    pub maxLaunchDelaySeconds: i64,
    pub container: Option<ContainerInfo>,
    pub healthChecks: Vec<HealthCheck>,
    pub dependencies: Vec<String>,
    pub upgradeStrategy: UpgradeStrategy,
    pub labels: HashMap<String, String>,
    pub version: String,
    pub tasksStaged: i64,
    pub tasksRunning: i64,
    pub deployments: Vec<Deployment>,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct HealthCheckResult {
    pub taskId: String,
    pub firstSuccess: String,
    pub lastSuccess: String,
    pub lastFailure: Option<String>,
    pub consecutiveFailures: i64,
    pub alive: bool,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct MarathonTask {
    pub appId: String,
    pub id: String,
    pub host: Option<String>,
    pub ports: Vec<u32>,
    pub stagedAt: Option<String>,
    pub startedAt: Option<String>,
    pub version: Option<String>,
    pub servicePorts: Option<Vec<u32>>,
    pub healthCheckResults: Vec<HealthCheckResult>,
}

#[cfg(test)]
mod tests {

    extern crate test;
    use rustc_serialize::json;
    use super::{Constraint, ConstraintOperator, ContainerInfo};

    #[test]
    fn test_decode_container_info() {
        let s = r##"{"type":"DOCKER","volumes":[],"hostname":"example.com","docker":null}"##;
        let o: ContainerInfo = json::decode(s).unwrap();
        assert_eq!(o.container_type, "DOCKER");
        assert!(o.volumes.is_empty());
        assert_eq!(o.hostname.unwrap(), "example.com");
        assert!(o.docker.is_none());
    }

    #[test]
    fn test_encode_container_info() {
        let c = ContainerInfo{container_type: "DOCKER".to_string(), volumes: vec![], hostname: None, docker: None};
        let r = json::encode(&c).unwrap();
        assert_eq!(&r[..], r##"{"type":"DOCKER","volumes":[],"hostname":null,"docker":null}"##);
    }

    #[test]
    fn test_decode_constraints_2() {
        let s = "[\"hostname\",\"UNIQUE\"]";
        let o: Constraint = json::decode(s).unwrap();
        assert_eq!(o.field, "hostname");
        assert_eq!(o.operator, ConstraintOperator::UNIQUE);
        assert!(o.value.is_none());
    }

    #[test]
    fn test_decode_constraints_3() {
        let s = "[\"attribute\",\"CLUSTER\",\"value\"]";
        let o: Constraint = json::decode(s).unwrap();
        assert_eq!(o.field, "attribute");
        assert_eq!(o.operator, ConstraintOperator::CLUSTER);
        assert_eq!(o.value.expect("value"), "value");
    }

    #[test]
    fn test_encode_constraints_2() {
        let c = Constraint{field: "hostname".to_string(), operator: ConstraintOperator::UNIQUE, value: None};
        let r = json::encode(&c).unwrap();
        assert_eq!(&r[..], "[\"hostname\",\"UNIQUE\"]");
    }

    #[test]
    fn test_encode_constraints_3() {
        let c = Constraint{field: "attribute".to_string(), operator: ConstraintOperator::CLUSTER, value: Some("value".to_string())};
        let r = json::encode(&c).unwrap();
        assert_eq!(&r[..], "[\"attribute\",\"CLUSTER\",\"value\"]");
    }

}

