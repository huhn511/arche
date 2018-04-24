use std::collections::BTreeMap;
// use std::default::Default;
use std::path::PathBuf;

use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::extension::ClientId;
use lettre::{EmailTransport, SmtpTransport};
use lettre_email::EmailBuilder;
use log;
use mime;
use serde_json;
use sys_info;

use super::super::orm::Connection as Db;
use super::super::queue::Consumer;
use super::super::result::Result;
use super::super::security::Encryptor;
use super::super::settings::Setting;

pub const SEND_EMAIL: &'static str = "send-email";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Smtp {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Email {
    pub to: String,
    pub subject: String,
    pub body: String,
    pub attachments: BTreeMap<PathBuf, String>,
}

pub trait SendEmail {
    fn send_email(&self, db: &Db, enc: &Encryptor, payload: &[u8], perform: bool) -> Result<()>;
}

impl SendEmail for Consumer {
    fn send_email(&self, db: &Db, enc: &Encryptor, payload: &[u8], perform: bool) -> Result<()> {
        let it: Email = serde_json::from_slice(payload)?;
        if !perform {
            log::debug!("send email to {}: {}\n{}", it.to, it.subject, it.body);
            return Ok(());
        }

        let smtp: Smtp = Setting::get(db, enc, &s!("site.smtp"))?;
        // let smtp: Smtp = Default::default();

        let mut email = EmailBuilder::new()
            .to(it.to)
            .from(&smtp.user[..])
            .subject(it.subject)
            .html(it.body);
        for (file, name) in it.attachments {
            email.set_attachment(file.as_path(), Some(&name[..]), &mime::TEXT_PLAIN)?;
        }
        let email = email.build()?;

        let mut mailer = SmtpTransport::simple_builder(&smtp.host)?
            .hello_name(ClientId::Domain(sys_info::hostname()?))
            .credentials(Credentials::new(smtp.user, smtp.password))
            .smtp_utf8(true)
            .authentication_mechanism(Mechanism::Plain)
            .build();

        mailer.send(&email)?;

        Ok(())
    }
}