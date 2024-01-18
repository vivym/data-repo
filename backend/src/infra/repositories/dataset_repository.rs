use chrono::NaiveDateTime;
use diesel::{
    ExpressionMethods,
    Insertable,
    Queryable,
    Selectable,
    SelectableHelper,
    RunQueryDsl,
    QueryDsl,
};
use serde::{Deserialize, Serialize};

use crate::domain::models::datasets::DatasetModel;
use crate::error::AppResult;
use crate::infra::db::schema::datasets;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = datasets)]    // Use the 'datasets' table
#[diesel(check_for_backend(diesel::pg::Pg))]    // Check compatibility with PostgreSQL
pub struct DatasetDB {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Into<DatasetModel> for DatasetDB {
    fn into(self) -> DatasetModel {
        DatasetModel {
            id: self.id,
            name: self.name,
            description: self.description,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = datasets)]
pub struct NewDatasetDB {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct DatasetsFilter {
    skip: i64,
    limit: i64,
}

pub async fn create(
    pool: &deadpool_diesel::postgres::Pool,
    new_dataset: NewDatasetDB,
) -> AppResult<DatasetModel> {
    let conn = pool.get().await?;

    let res = conn
        .interact(|conn| {
            diesel::insert_into(datasets::table)
                .values(new_dataset)
                .returning(DatasetDB::as_returning())
                .get_result(conn)
        })
        .await??;

    Ok(res.into())
}

pub async fn get_by_id(
    pool: &deadpool_diesel::postgres::Pool,
    dataset_id: i32,
) -> AppResult<DatasetModel> {
    let conn = pool.get().await?;

    let res = conn
        .interact(move |conn| {
            datasets::table
                .filter(datasets::id.eq(dataset_id))
                .select(DatasetDB::as_select())
                .first(conn)
        })
        .await??;

    Ok(res.into())
}

pub async fn get_all(
    pool: &deadpool_diesel::postgres::Pool,
    filter: DatasetsFilter,
) -> AppResult<Vec<DatasetModel>> {
    let conn = pool.get().await?;

    let res = conn
        .interact(move |conn| {
            let query = datasets::table
                .into_boxed::<diesel::pg::Pg>()
                .offset(filter.skip)
                .limit(filter.limit);

            query.select(DatasetDB::as_select()).load::<DatasetDB>(conn)
        })
        .await??;

    let datasets: Vec<DatasetModel> = res
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(datasets)
}

pub async fn delete_by_id(
    pool: &deadpool_diesel::postgres::Pool,
    dataset_id: i32,
) -> AppResult<()> {
    let conn = pool.get().await?;

    conn
        .interact(move |conn| {
            diesel::delete(
                datasets::table
                    .filter(datasets::id.eq(dataset_id))
            )
            .execute(conn)
        })
        .await??;

    Ok(())
}
