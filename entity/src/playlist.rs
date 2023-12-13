//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "playlists")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl Related<super::song::Entity> for Entity {
    // The final relation is Playlist -> PlaylistSong -> Song
    fn to() -> RelationDef {
        super::playlist_song::Relation::Song.def()
    }

    fn via() -> Option<RelationDef> {
        // The original relation is PlaylistSong -> Playlist,
        // after `rev` it becomes Playlist -> PlaylistSong
        Some(super::playlist_song::Relation::Playlist.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
