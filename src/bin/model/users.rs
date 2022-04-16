use crate::entity::users::*;
use actix_web::dev::{Path, Url};
use sea_orm::entity::prelude::*;
use sea_orm::entity::{ActiveValue, IntoActiveModel};
use sea_orm::{Condition, PrimaryKeyTrait};
use serde::{Deserialize, Serialize};
use woof::{Create, Filter, Rest, Update};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DeriveIntoActiveModel)]
pub struct CreateModel {
    pub username: String,
    pub email: String,
}

impl Create<ActiveModel> for CreateModel {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateModel {
    pub username: Option<String>,
    pub email: Option<String>,
}

impl IntoActiveModel<ActiveModel> for UpdateModel {
    fn into_active_model(self) -> ActiveModel {
        let mut active_model = <ActiveModel as ActiveModelTrait>::default();
        if let Some(username) = self.username {
            active_model.username = ActiveValue::Set(username);
        }
        if let Some(email) = self.email {
            active_model.email = ActiveValue::Set(email);
        }
        active_model
    }
}

impl Update<ActiveModel> for UpdateModel {}

#[derive(Serialize, Deserialize)]
pub struct FilterModel {
    limit: Option<usize>,
    offset: Option<usize>,
    page: Option<usize>,
    cursor: Option<String>,
    email: Option<String>,
    username: Option<String>,
}

impl Filter for FilterModel {
    fn limit(&self) -> usize {
        self.limit.unwrap_or(20)
    }

    fn offset(&self) -> usize {
        self.limit.unwrap_or(0)
    }

    fn page(&self) -> usize {
        self.limit.unwrap_or(0)
    }

    fn cursor(&self) -> Option<&str> {
        self.cursor.as_deref()
    }

    fn condition(&self) -> Condition {
        let mut condition = Condition::all();
        if let Some(email) = &self.email {
            condition = condition.add(Column::Email.eq(email.to_owned()));
        }
        if let Some(username) = &self.username {
            condition = condition.add(Column::Username.eq(username.to_owned()));
        }
        condition
    }
}

impl Rest for Entity {
    type ActiveModel = ActiveModel;
    type Create = CreateModel;
    type Update = UpdateModel;
    type Filter = FilterModel;

    fn id_from_path(
        scope: Option<&str>,
        path: &Path<Url>,
    ) -> woof::Result<<Self::PrimaryKey as PrimaryKeyTrait>::ValueType> {
        let scope = scope.map(|scope| format!("{scope}_")).unwrap_or_default();
        let id_path = scope.to_owned() + "id";
        let id = path
            .get(&id_path)
            .ok_or_else(|| woof::error::MissingPathSegment(&id_path))?
            .parse()
            .map_err(|_| woof::error::InvalidPathSegment(&id_path))?;
        Ok(id)
    }
}
