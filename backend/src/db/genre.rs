use gami_sdk::GenreData;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "genres")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub metadata_source: String,
    pub metadata_id: String,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::game_genres::Entity")]
    GameGenres,
}
impl Related<super::game::Entity> for Entity {
    fn to() -> RelationDef {
        super::game_genres::Relation::Genre.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::game_genres::Relation::Game.def().rev())
    }
}
impl ActiveModelBehavior for ActiveModel {}
pub type Genre = Model;

impl Into<GenreData> for Model {
    fn into(self) -> GenreData {
        GenreData {
            library_id: self.metadata_id.into(),
            name: self.name.into(),
        }
    }
}
