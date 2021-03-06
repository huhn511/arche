use std::ops::Deref;

use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use rocket::http::Status;
use validator::Validate;

use super::super::super::super::{
    errors::Result,
    graphql::{context::Context, H},
    orm::{schema::members, Connection as Db},
    rfc::UtcDateTime,
    utils,
};
use super::super::super::caring;
use super::super::{dao::policy as policy_dao, models::Role};

fn can_view(db: &Db, user: &i64) -> Result<()> {
    for (n, rty) in vec![
        (Role::Admin, None),
        (Role::Manager, Some(caring::NAME.to_string())),
    ] {
        if policy_dao::can(db, user, &n, &rty) {
            return Ok(());
        }
    }
    Err(Status::Forbidden.reason.into())
}
fn can_edit(db: &Db, user: &i64) -> Result<()> {
    if policy_dao::is(db, user, &Role::Admin) {
        return Ok(());
    }
    Err(Status::Forbidden.reason.into())
}

#[derive(GraphQLObject, Debug, Serialize)]
pub struct Member {
    pub id: String,
    pub nick_name: String,
    pub real_name: String,
    pub gender: String,
    pub birthday: String, //FIXME NaiveDate,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub line: Option<String>,
    pub wechat: Option<String>,
    pub skype: Option<String>,
    pub weibo: Option<String>,
    pub facebook: Option<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Show {
    #[validate(length(min = "1"))]
    pub id: String,
}

impl Show {
    pub fn call(&self, ctx: &Context) -> Result<Member> {
        self.validate()?;
        let id: i64 = self.id.parse()?;
        let user = ctx.current_user()?;
        let db = ctx.db.deref();
        can_view(db, &user.id)?;
        let (
            nick_name,
            real_name,
            gender,
            birthday,
            phone,
            email,
            address,
            line,
            wechat,
            skype,
            weibo,
            facebook,
            updated_at,
        ) = members::dsl::members
            .select((
                members::dsl::nick_name,
                members::dsl::real_name,
                members::dsl::gender,
                members::dsl::birthday,
                members::dsl::phone,
                members::dsl::email,
                members::dsl::address,
                members::dsl::line,
                members::dsl::wechat,
                members::dsl::skype,
                members::dsl::weibo,
                members::dsl::facebook,
                members::dsl::updated_at,
            ))
            .filter(members::dsl::id.eq(&id))
            .first::<(
                String,
                String,
                String,
                NaiveDate,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                NaiveDateTime,
            )>(db)?;

        Ok(Member {
            id: self.id.clone(),
            nick_name: nick_name,
            real_name: real_name,
            gender: gender,
            birthday: birthday.format(utils::DATE_FORMAT).to_string(),
            phone: phone,
            email: email,
            address: address,
            line: line,
            wechat: wechat,
            skype: skype,
            weibo: weibo,
            facebook: facebook,
            updated_at: updated_at.to_utc(),
        })
    }
}
#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Remove {
    #[validate(length(min = "1"))]
    pub id: String,
}

impl Remove {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let id: i64 = self.id.parse()?;
        let user = ctx.current_user()?;
        let db = ctx.db.deref();
        can_edit(db, &user.id)?;
        let it = members::dsl::members.filter(members::dsl::id.eq(&id));
        delete(it).execute(db)?;
        Ok(H::new())
    }
}

pub fn list(ctx: &Context) -> Result<Vec<Member>> {
    let user = ctx.current_user()?;
    let db = ctx.db.deref();
    can_view(db, &user.id)?;
    let items = members::dsl::members
        .select((
            members::dsl::id,
            members::dsl::nick_name,
            members::dsl::real_name,
            members::dsl::gender,
            members::dsl::birthday,
            members::dsl::phone,
            members::dsl::email,
            members::dsl::address,
            members::dsl::line,
            members::dsl::wechat,
            members::dsl::skype,
            members::dsl::weibo,
            members::dsl::facebook,
            members::dsl::updated_at,
        ))
        .order(members::dsl::nick_name.asc())
        .load::<(
            i64,
            String,
            String,
            String,
            NaiveDate,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            NaiveDateTime,
        )>(db)?;

    Ok(items
        .iter()
        .map(
            |(
                id,
                nick_name,
                real_name,
                gender,
                birthday,
                phone,
                email,
                address,
                line,
                wechat,
                skype,
                weibo,
                facebook,
                updated_at,
            )| Member {
                id: id.to_string(),
                nick_name: nick_name.clone(),
                real_name: real_name.clone(),
                gender: gender.clone(),
                birthday: birthday.format(utils::DATE_FORMAT).to_string(),
                phone: phone.clone(),
                email: email.clone(),
                address: address.clone(),
                line: line.clone(),
                wechat: wechat.clone(),
                skype: skype.clone(),
                weibo: weibo.clone(),
                facebook: facebook.clone(),
                updated_at: updated_at.to_utc(),
            },
        )
        .collect())
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Create {
    #[validate(length(min = "1"))]
    pub nick_name: String,
    #[validate(length(min = "1"))]
    pub real_name: String,
    pub gender: String,
    pub birthday: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub line: Option<String>,
    pub wechat: Option<String>,
    pub skype: Option<String>,
    pub weibo: Option<String>,
    pub facebook: Option<String>,
}

impl Create {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let birthday = NaiveDate::parse_from_str(&self.birthday, utils::DATE_FORMAT)?;
        let user = ctx.current_user()?;
        let db = ctx.db.deref();
        can_edit(db, &user.id)?;
        let now = Utc::now().naive_utc();

        let cnt = members::dsl::members
            .filter(members::dsl::nick_name.eq(&self.nick_name))
            .count()
            .get_result::<i64>(db)?;
        if cnt > 0 {
            return Err(Status::BadRequest.reason.into());
        }
        insert_into(members::dsl::members)
            .values((
                members::dsl::nick_name.eq(&self.nick_name),
                members::dsl::real_name.eq(&self.real_name),
                members::dsl::gender.eq(&self.gender),
                members::dsl::birthday.eq(&birthday),
                members::dsl::phone.eq(&self.phone),
                members::dsl::email.eq(&self.email),
                members::dsl::address.eq(&self.address),
                members::dsl::line.eq(&self.line),
                members::dsl::wechat.eq(&self.wechat),
                members::dsl::skype.eq(&self.skype),
                members::dsl::weibo.eq(&self.weibo),
                members::dsl::facebook.eq(&self.facebook),
                members::dsl::updated_at.eq(&now),
                members::dsl::created_at.eq(&now),
            ))
            .execute(db)?;
        Ok(H::new())
    }
}

#[derive(GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct Update {
    #[validate(length(min = "1"))]
    pub id: String,
    #[validate(length(min = "1"))]
    pub real_name: String,
    pub gender: String,
    pub birthday: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub line: Option<String>,
    pub wechat: Option<String>,
    pub skype: Option<String>,
    pub weibo: Option<String>,
    pub facebook: Option<String>,
}

impl Update {
    pub fn call(&self, ctx: &Context) -> Result<H> {
        self.validate()?;
        let birthday = NaiveDate::parse_from_str(&self.birthday, utils::DATE_FORMAT)?;
        let id = self.id.parse::<i64>()?;
        let db = ctx.db.deref();
        let now = Utc::now().naive_utc();
        let it = members::dsl::members.filter(members::dsl::id.eq(&id));
        update(it)
            .set((
                members::dsl::real_name.eq(&self.real_name),
                members::dsl::gender.eq(&self.gender),
                members::dsl::birthday.eq(&birthday),
                members::dsl::phone.eq(&self.phone),
                members::dsl::email.eq(&self.email),
                members::dsl::address.eq(&self.address),
                members::dsl::line.eq(&self.line),
                members::dsl::wechat.eq(&self.wechat),
                members::dsl::skype.eq(&self.skype),
                members::dsl::weibo.eq(&self.weibo),
                members::dsl::facebook.eq(&self.facebook),
                members::dsl::updated_at.eq(&now),
            ))
            .execute(db)?;
        Ok(H::new())
    }
}
