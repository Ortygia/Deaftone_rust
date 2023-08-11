//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::{
    entity::prelude::*,
    sea_query::{IntoColumnRef, SimpleExpr},
    Condition,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, ToSchema)]
#[sea_orm(table_name = "artists")]
#[schema(as = entity::artist::Model)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[schema(example = "Artist")]
    pub id: String,
    pub name: String,
    pub image: Option<String>,
    pub biography: Option<String>,
    pub mb_artist_id: Option<String>,
    pub link_twitter: Option<String>,
    pub link_discogs: Option<String>,
    pub link_youtube: Option<String>,
    pub link_facebook: Option<String>,
    pub link_spotify: Option<String>,
    pub link_itunes: Option<String>,
    pub link_wiki: Option<String>,
    pub link_apple_music: Option<String>,
    pub link_amazon_music: Option<String>,
    pub link_all_music: Option<String>,
    pub link_deezer: Option<String>,
    pub link_tidal: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::album::Entity")]
    Album,
    #[sea_orm(
        has_many = "super::album::Entity",
        on_condition = "Condition::all().add(
            SimpleExpr::Column(
                super::artist::Column::MbArtistId
                    .as_column_ref()
                    .into_column_ref(),
            )
            .eq(SimpleExpr::Column(
                super::album::Column::MbArtistId
                    .as_column_ref()
                    .into_column_ref(),
            )),
        )"
        condition_type = "any",
    )]
    MbAlbum,
}
impl Related<super::album::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Album.def();
        Relation::MbAlbum.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
