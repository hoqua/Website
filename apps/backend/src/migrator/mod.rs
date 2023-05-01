use sea_orm::{entity::prelude::*, TryGetError};
use sea_orm_migration::prelude::*;
use serde::{Deserialize, Serialize};

mod m20230410_000001_create_metadata;
mod m20230416_000002_create_creator;
mod m20230416_000003_create_book;
mod m20230417_000004_create_user;
mod m20230419_000005_create_seen;
mod m20230423_000006_create_movie;
mod m20230425_000007_create_show;

pub use m20230410_000001_create_metadata::{Metadata, MetadataImageLot, MetadataLot};
pub use m20230416_000002_create_creator::Creator;
pub use m20230416_000003_create_book::{Book, BookSource};
pub use m20230417_000004_create_user::{TokenLot, UserLot};
pub use m20230423_000006_create_movie::MovieSource;
pub use m20230425_000007_create_show::ShowSource;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230410_000001_create_metadata::Migration),
            Box::new(m20230416_000002_create_creator::Migration),
            Box::new(m20230416_000003_create_book::Migration),
            Box::new(m20230417_000004_create_user::Migration),
            Box::new(m20230419_000005_create_seen::Migration),
            Box::new(m20230423_000006_create_movie::Migration),
            Box::new(m20230425_000007_create_show::Migration),
        ]
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StringVec(pub Vec<String>);

impl From<StringVec> for Value {
    fn from(source: StringVec) -> Self {
        Value::String(serde_json::to_string(&source).ok().map(Box::new))
    }
}

impl sea_orm::TryGetable for StringVec {
    fn try_get_by<I: sea_orm::ColIdx>(res: &QueryResult, idx: I) -> Result<Self, TryGetError> {
        let json_str: String = res.try_get_by(idx).map_err(TryGetError::DbErr)?;
        serde_json::from_str(&json_str).map_err(|e| TryGetError::DbErr(DbErr::Json(e.to_string())))
    }
}

impl sea_query::ValueType for StringVec {
    fn try_from(v: Value) -> Result<Self, sea_query::ValueTypeErr> {
        match v {
            Value::String(Some(x)) => Ok(StringVec(
                serde_json::from_str(&x).map_err(|_| sea_query::ValueTypeErr)?,
            )),
            _ => Err(sea_query::ValueTypeErr),
        }
    }

    fn type_name() -> String {
        stringify!(StringVec).to_owned()
    }

    fn array_type() -> sea_orm::sea_query::ArrayType {
        sea_orm::sea_query::ArrayType::String
    }

    fn column_type() -> sea_query::ColumnType {
        sea_query::ColumnType::String(None)
    }
}

impl sea_query::Nullable for StringVec {
    fn null() -> Value {
        Value::String(None)
    }
}