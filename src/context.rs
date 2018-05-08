use frank_jwt::Algorithm;

use super::cache::{Cache, Redis};
use super::env;
use super::jwt::Jwt;
use super::queue::{Queue, RabbitMq};
use super::repositories::{PostgreSql, Repository};
use super::result::{Error, Result};
use super::security::{Security, Sodium};

pub struct Context {
    pub repository: Box<Repository>,
    pub cache: Box<Cache>,
    pub queue: Box<Queue>,
    pub security: Box<Security>,
    pub jwt: Jwt,
}

impl Context {
    pub fn new(cfg: &env::Config) -> Result<Self> {
        Ok(Self {
            repository: Self::open_database(&cfg.database)?,
            cache: Self::open_cache(&cfg.cache)?,
            queue: Self::open_queue(&cfg.queue)?,
            security: Box::new(Sodium::new(cfg.secret_key()?.as_slice())?),
            jwt: Jwt::new(cfg.secret_key()?.as_slice(), Algorithm::HS512),
        })
    }

    fn open_database(cfg: &env::Database) -> Result<Box<Repository>> {
        if let Some(ref c) = cfg.postgresql {
            return Ok(Box::new(PostgreSql::new(c.pool()?)));
        }
        Err(Error::WithDescription(s!(
            "unsupport messaging database provider"
        )))
    }

    fn open_cache(cfg: &env::Cache) -> Result<Box<Cache>> {
        if let Some(ref c) = cfg.redis {
            return Ok(Box::new(Redis::new(cfg.namespace.clone(), c.pool()?)));
        }
        Err(Error::WithDescription(s!(
            "unsupport messaging cache provider"
        )))
    }

    fn open_queue(cfg: &env::Queue) -> Result<Box<Queue>> {
        if let Some(ref c) = cfg.rabbitmq {
            return Ok(Box::new(RabbitMq::new(c.url(), cfg.name.clone())));
        }
        Err(Error::WithDescription(s!(
            "unsupport messaging queue provider"
        )))
    }
}