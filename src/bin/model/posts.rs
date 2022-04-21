use crate::entity::posts::*;
use actix_web::dev::{Path, Url};
use sea_orm::entity::prelude::*;
use sea_orm::entity::{ActiveValue, IntoActiveModel};
use sea_orm::Condition;
use serde::{Deserialize, Serialize};
use woof::{Create, Filter, Rest, Update};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DeriveIntoActiveModel)]
pub struct CreateModel {
    pub title: String,
    pub content: String,
    pub author: Uuid,
}

impl Create<ActiveModel> for CreateModel {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateModel {
    pub title: Option<String>,
    pub content: Option<String>,
}

impl IntoActiveModel<ActiveModel> for UpdateModel {
    fn into_active_model(self) -> ActiveModel {
        let mut active_model = <ActiveModel as ActiveModelTrait>::default();
        if let Some(title) = self.title {
            active_model.title = ActiveValue::Set(title);
        }
        if let Some(content) = self.content {
            active_model.content = ActiveValue::Set(content);
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
    title: Option<String>,
    author: Option<Uuid>,
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
        if let Some(title) = &self.title {
            condition = condition.add(Column::Title.eq(title.to_owned()));
        }
        if let Some(author) = self.author {
            condition = condition.add(Column::Author.eq(author));
        }
        condition
    }
}

pub struct RestModel;
impl Rest for RestModel {
    type Entity = Entity;
    type Repr = Model;
    type ActiveModel = ActiveModel;
    type Update = UpdateModel;
    type Create = CreateModel;
    type Filter = FilterModel;

    fn id_from_path(
        scope: Option<&str>,
        path: &Path<Url>,
    ) -> woof::Result<<PrimaryKey as PrimaryKeyTrait>::ValueType> {
        let scope = scope.map(|scope| format!("{scope}_")).unwrap_or_default();
        let id_path = scope + "id";
        let id = path
            .get(&id_path)
            .ok_or_else(|| woof::error::MissingPathSegment(&id_path))?
            .parse()
            .map_err(|_| woof::error::InvalidPathSegment(&id_path))?;
        Ok(id)
    }
}
