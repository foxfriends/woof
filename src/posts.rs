use crate::entity::posts::*;
use sea_orm::entity::prelude::*;
use sea_orm::entity::{ActiveValue, IntoActiveModel};
use sea_orm::Condition;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DeriveIntoActiveModel)]
pub struct CreateModel {
    pub title: String,
    pub content: String,
    pub author: Uuid,
}

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

#[derive(Serialize, Deserialize)]
pub struct FilterModel {
    limit: Option<usize>,
    offset: Option<usize>,
    page: Option<usize>,
    cursor: Option<String>,
    title: Option<String>,
    author: Option<Uuid>,
}

impl crate::rest_model::FilterSet for FilterModel {
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
