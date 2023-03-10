use sqlx::FromRow;

#[derive(FromRow, Debug, PartialEq, Clone)]
pub struct Test {
    pub id: String,
    pub name: String,
}
