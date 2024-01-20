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

use crate::domain::models::ds_shard_ver_orders::DatasetShardVerificationOrderModel;
use crate::error::AppResult;
use crate::infra::db::schema::ds_shard_ver_orders;
use super::{default_skip, default_limit};

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = ds_shard_ver_orders)]    // Use the 'ds_shard_ver_orders' table
#[diesel(check_for_backend(diesel::pg::Pg))]    // Check compatibility with PostgreSQL
pub struct DatasetShardVerificationOrderDB {
    pub id: i32,
    pub ds_id: i32,
    pub shard_id: i32,
    pub sample_id: i32,
    pub score: i32,
    pub comment: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Into<DatasetShardVerificationOrderModel> for DatasetShardVerificationOrderDB {
    fn into(self) -> DatasetShardVerificationOrderModel {
        DatasetShardVerificationOrderModel {
            id: self.id,
            ds_id: self.ds_id,
            shard_id: self.shard_id,
            sample_id: self.sample_id,
            score: self.score,
            comment: self.comment,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = ds_shard_ver_orders)]
pub struct NewDatasetShardVerificationOrderDB {
    pub ds_id: i32,
    pub shard_id: i32,
    pub sample_id: i32,
    pub score: i32,
    pub comment: String,
}

#[derive(Debug, Deserialize)]
pub struct DatasetShardVerificationOrdersFilter {
    ds_id: Option<i32>,
    shard_id: Option<i32>,
    sample_id: Option<i32>,
    order_by_score: Option<bool>,
    #[serde(default = "default_skip")]
    skip: i64,
    #[serde(default = "default_limit")]
    limit: i64,
}

pub async fn create(
    pool: &deadpool_diesel::postgres::Pool,
    new_order: NewDatasetShardVerificationOrderDB,
) -> AppResult<DatasetShardVerificationOrderModel> {
    let conn = pool.get().await?;

    let res = conn
        .interact(|conn| {
            diesel::insert_into(ds_shard_ver_orders::table)
                .values(new_order)
                .returning(DatasetShardVerificationOrderDB::as_returning())
                .get_result(conn)
        })
        .await??;

    Ok(res.into())
}

pub async fn get_by_id(
    pool: &deadpool_diesel::postgres::Pool,
    order_id: i32,
) -> AppResult<DatasetShardVerificationOrderModel> {
    let conn = pool.get().await?;

    let res = conn
        .interact(move |conn| {
            ds_shard_ver_orders::table
                .filter(ds_shard_ver_orders::id.eq(order_id))
                .select(DatasetShardVerificationOrderDB::as_select())
                .first(conn)
        })
        .await??;

    Ok(res.into())
}

pub async fn get_all(
    pool: &deadpool_diesel::postgres::Pool,
    filter: DatasetShardVerificationOrdersFilter,
) -> AppResult<Vec<DatasetShardVerificationOrderModel>> {
    let conn = pool.get().await?;

    let res = conn
        .interact(move |conn| {
            let mut query = ds_shard_ver_orders::table
                .into_boxed::<diesel::pg::Pg>();

            if let Some(ds_id) = filter.ds_id {
                query = query.filter(ds_shard_ver_orders::ds_id.eq(ds_id));
            }

            if let Some(shard_id) = filter.shard_id {
                query = query.filter(ds_shard_ver_orders::shard_id.eq(shard_id));
            }

            if let Some(sample_id) = filter.sample_id {
                query = query.filter(ds_shard_ver_orders::sample_id.eq(sample_id));
            }

            if let Some(order_by_score) = filter.order_by_score {
                if order_by_score {
                    query = query.order_by(ds_shard_ver_orders::score.desc());
                } else {
                    query = query.order_by(ds_shard_ver_orders::score.asc());
                }
            }

            query = query
                .offset(filter.skip)
                .limit(filter.limit);

            query
                .select(DatasetShardVerificationOrderDB::as_select())
                .load::<DatasetShardVerificationOrderDB>(conn)
        })
        .await??;

    let shards: Vec<DatasetShardVerificationOrderModel> = res
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(shards)
}

pub async fn delete_by_id(
    pool: &deadpool_diesel::postgres::Pool,
    order_id: i32,
) -> AppResult<()> {
    let conn = pool.get().await?;

    conn
        .interact(move |conn| {
            diesel::delete(
                ds_shard_ver_orders::table
                    .filter(ds_shard_ver_orders::id.eq(order_id))
            )
            .execute(conn)
        })
        .await??;

    Ok(())
}
