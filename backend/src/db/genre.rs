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
impl Related<super::game_genres::Entity> for Entity {
    fn to() -> RelationDef {
        super::game::Relation::GameGenres.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}
pub type Genre = Model;
