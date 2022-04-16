use crate::entity::votes::*;
use sea_orm::entity::prelude::*;
use sea_orm::entity::{ActiveValue, IntoActiveModel};
use sea_orm::Condition;
use serde::{Deserialize, Serialize};
use woof::{Create, Filter, Rest, Update};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DeriveIntoActiveModel)]
pub struct CreateModel {
    pub voter: Uuid,
    pub post: Uuid,
    pub positive: bool,
}

impl Create<ActiveModel> for CreateModel {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateModel {
    pub positive: Option<bool>,
}

impl IntoActiveModel<ActiveModel> for UpdateModel {
    fn into_active_model(self) -> ActiveModel {
        let mut active_model = <ActiveModel as ActiveModelTrait>::default();
        if let Some(positive) = self.positive {
            active_model.positive = ActiveValue::Set(positive);
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
    positive: Option<bool>,
    voter: Option<Uuid>,
    post: Option<Uuid>,
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
        if let Some(positive) = self.positive {
            condition = condition.add(Column::Positive.eq(positive));
        }
        if let Some(voter) = self.voter {
            condition = condition.add(Column::Voter.eq(voter));
        }
        if let Some(post) = self.post {
            condition = condition.add(Column::Post.eq(post));
        }
        condition
    }
}

impl Rest for Entity {
    type ActiveModel = ActiveModel;
    type Update = UpdateModel;
    type Create = CreateModel;
    type Filter = FilterModel;
}
