//! Stub ureq implementation for WebAssembly (browser offline mode).
#![allow(dead_code)]

use std::time::Duration;
use std::io;

#[derive(Clone, Default)]
pub struct Agent;

#[derive(Default)]
pub struct AgentBuilder;

impl AgentBuilder {
    pub fn new() -> Self { Self }
    pub fn timeout(self, _d: Duration) -> Self { self }
    pub fn timeout_connect(self, _d: Duration) -> Self { self }
    pub fn timeout_read(self, _d: Duration) -> Self { self }
    pub fn redirects(self, _n: usize) -> Self { self }
    pub fn user_agent(self, _ua: &str) -> Self { self }
    pub fn build(self) -> Agent { Agent }
}

impl Agent {
    pub fn get(&self, _url: &str) -> Request { Request }
    pub fn post(&self, _url: &str) -> Request { Request }
    pub fn put(&self, _url: &str) -> Request { Request }
    pub fn delete(&self, _url: &str) -> Request { Request }
    pub fn head(&self, _url: &str) -> Request { Request }
    pub fn request(&self, _method: &str, _url: &str) -> Request { Request }
}

#[derive(Debug)]
pub enum Error {
    Status(u16, Response),
    Transport(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Status(code, _) => write!(f, "HTTP status {code}"),
            Self::Transport(s) => write!(f, "Transport: {s}"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub struct Response;

impl Response {
    pub fn content_type(&self) -> &str { "" }
    pub fn header(&self, _name: &str) -> Option<&str> { None }
    pub fn into_string(self) -> Result<String, io::Error> {
        Err(io::Error::new(io::ErrorKind::Unsupported, "network disabled in web client"))
    }
    pub fn into_reader(self) -> Box<dyn io::Read> {
        Box::new(io::empty())
    }
    pub fn status(&self) -> u16 { 503 }
}

pub struct Request;

impl Request {
    pub fn call(self) -> Result<Response, Error> {
        Err(Error::Transport("offline mode in browser".to_string()))
    }
    pub fn send(self, _reader: impl io::Read) -> Result<Response, Error> {
        Err(Error::Transport("offline mode in browser".to_string()))
    }
    pub fn send_bytes(self, _bytes: &[u8]) -> Result<Response, Error> {
        Err(Error::Transport("offline mode in browser".to_string()))
    }
    pub fn send_string(self, _data: &str) -> Result<Response, Error> {
        Err(Error::Transport("offline mode in browser".to_string()))
    }
    pub fn set(self, _header: &str, _value: &str) -> Self { self }
    pub fn timeout(self, _d: Duration) -> Self { self }
}

pub fn get(_url: &str) -> Request { Request }
pub fn post(_url: &str) -> Request { Request }
