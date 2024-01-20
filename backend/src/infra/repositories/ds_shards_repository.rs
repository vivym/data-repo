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

use crate::domain::models::ds_shards::DatasetShardModel;
use crate::error::AppResult;
use crate::infra::db::schema::ds_shards;
use super::{default_skip, default_limit};

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = ds_shards)]    // Use the 'ds_shards' table
#[diesel(check_for_backend(diesel::pg::Pg))]    // Check compatibility with PostgreSQL
pub struct DatasetShardDB {
    pub id: i32,
    pub ds_id: i32,
    pub uri: String,
    pub num_samples: i32,
    pub verified: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Into<DatasetShardModel> for DatasetShardDB {
    fn into(self) -> DatasetShardModel {
        DatasetShardModel {
            id: self.id,
            ds_id: self.ds_id,
            uri: self.uri,
            num_samples: self.num_samples,
            verified: self.verified,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = ds_shards)]
pub struct NewDatasetShardDB {
    pub ds_id: i32,
    pub uri: String,
    pub num_samples: i32,
}

#[derive(Debug, Deserialize)]
pub struct DatasetShardsFilter {
    ds_id: Option<i32>,
    #[serde(default = "default_skip")]
    skip: i64,
    #[serde(default = "default_limit")]
    limit: i64,
}

pub async fn create(
    pool: &deadpool_diesel::postgres::Pool,
    new_ds_shard: NewDatasetShardDB,
) -> AppResult<DatasetShardModel> {
    let conn = pool.get().await?;

    let res = conn
        .interact(|conn| {
            diesel::insert_into(ds_shards::table)
                .values(new_ds_shard)
                .returning(DatasetShardDB::as_returning())
                .get_result(conn)
        })
        .await??;

    Ok(res.into())
}

pub async fn get_by_id(
    pool: &deadpool_diesel::postgres::Pool,
    ds_shard_id: i32,
) -> AppResult<DatasetShardModel> {
    let conn = pool.get().await?;

    let res = conn
        .interact(move |conn| {
            ds_shards::table
                .filter(ds_shards::id.eq(ds_shard_id))
                .select(DatasetShardDB::as_select())
                .first(conn)
        })
        .await??;

    Ok(res.into())
}

pub async fn get_all(
    pool: &deadpool_diesel::postgres::Pool,
    filter: DatasetShardsFilter,
) -> AppResult<Vec<DatasetShardModel>> {
    let conn = pool.get().await?;

    let res = conn
        .interact(move |conn| {
            let mut query = ds_shards::table
                .into_boxed::<diesel::pg::Pg>();

            if let Some(ds_id) = filter.ds_id {
                query = query.filter(ds_shards::ds_id.eq(ds_id));
            }

            query = query
                .offset(filter.skip)
                .limit(filter.limit);

            query.select(DatasetShardDB::as_select()).load::<DatasetShardDB>(conn)
        })
        .await??;

    let shards: Vec<DatasetShardModel> = res
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(shards)
}

pub async fn delete_by_id(
    pool: &deadpool_diesel::postgres::Pool,
    ds_shard_id: i32,
) -> AppResult<()> {
    let conn = pool.get().await?;

    conn
        .interact(move |conn| {
            diesel::delete(
                ds_shards::table
                    .filter(ds_shards::id.eq(ds_shard_id))
            )
            .execute(conn)
        })
        .await??;

    Ok(())
}