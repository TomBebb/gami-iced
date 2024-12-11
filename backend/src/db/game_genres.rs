use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "game_genres")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub game_id: i32,
    #[sea_orm(primary_key)]
    pub genre_id: i32,
}
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Game,
    Genre,
}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::Game => Entity::has_one(super::game::Entity).into(),
            Relation::Genre => Entity::has_one(super::genre::Entity).into(),
        }
    }
}

impl Related<super::game::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Game.def()
    }
}
impl Related<super::genre::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Genre.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
pub type GameGenre = Model;
