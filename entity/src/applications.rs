//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.7
/// Copyright (c) 2023, Sean McNamara <smcnam@gmail.com>.
/// All code in this repository is disjunctively licensed under [CC-BY-SA 3.0](https://creativecommons.org/licenses/by-sa/3.0/) and [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0).
/// Direct dependencies are believed to be under a license which allows downstream code to have these licenses.
use sea_orm::entity::prelude::*;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize)]
#[sea_orm(table_name = "applications")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub application_id: String,
    pub site_id: Option<String>,
    pub preset_id: Option<String>,
    pub title: Option<String>,
    pub user_ip: Option<String>,
    pub created: Option<String>,
    pub username: Option<String>,
    pub user_id: Option<String>,
    #[serde(flatten)]
    pub user_data: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type AppApp = Model;
