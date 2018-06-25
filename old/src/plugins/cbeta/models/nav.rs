use std::{fs::OpenOptions, path::PathBuf};

use serde_xml_rs;

use super::super::super::super::result::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Html {
    pub head: Head,
    pub body: Body,
}

impl Html {
    pub fn load(file: &PathBuf) -> Result<Self> {
        let fd = OpenOptions::new().read(true).open(file)?;
        let it: Html = serde_xml_rs::deserialize(fd)?;
        Ok(it)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Head {
    pub meta: Meta,
    pub title: Title,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Title {
    #[serde(rename = "$value")]
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub charset: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    pub nav: Vec<Nav>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Nav {
    #[serde(rename = "type")]
    pub type_: String,
    pub span: Option<Span>,
    pub ol: Ol,
    pub li: Option<Vec<Li>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Span {
    #[serde(rename = "$value")]
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ol {
    pub li: Vec<Li>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Li {
    pub span: Option<Span>,
    pub ol: Option<Ol>,
    pub a: Option<A>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cblink {
    pub href: String,
    #[serde(rename = "$value")]
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct A {
    pub href: String,
    #[serde(rename = "$value")]
    pub text: String,
}