use axum::{Router, extract::{State, Query}, Json, routing::{post, get, delete}};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::{ToSchema, IntoParams};

use crate::{
    domain::models::ds_shard_ver_orders::{
        DatasetShardVerificationOrderError,
        DatasetShardVerificationOrderModel,
    },
    infra::repositories::ds_shard_ver_order_repository::{
        self,
        NewDatasetShardVerificationOrderDB,
        DatasetShardVerificationOrdersFilter
    },
    server::AppState,
    utils::extractors::{
        json::JsonExtractor,
        path::PathExtractor,
    },
};

#[derive(Debug, Deserialize, ToSchema)]
pub struct OrderCreationRequest {
    pub ds_id: i32,
    pub shard_id: i32,
    pub sample_id: i32,
    pub score: i32,
    pub comment: String,
}

impl Into<NewDatasetShardVerificationOrderDB> for OrderCreationRequest {
    fn into(self) -> NewDatasetShardVerificationOrderDB {
        NewDatasetShardVerificationOrderDB {
            ds_id: self.ds_id,
            shard_id: self.shard_id,
            sample_id: self.sample_id,
            score: self.score,
            comment: self.comment,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct OrderResponse {
    id: i32,
    pub ds_id: i32,
    pub shard_id: i32,
    pub sample_id: i32,
    pub score: i32,
    pub comment: String,
    #[schema(value_type = String)]
    created_at: NaiveDateTime,
    #[schema(value_type = String)]
    updated_at: NaiveDateTime,
}

impl From<DatasetShardVerificationOrderModel> for OrderResponse {
    fn from(order: DatasetShardVerificationOrderModel) -> Self {
        Self {
            id: order.id,
            ds_id: order.ds_id,
            shard_id: order.shard_id,
            sample_id: order.sample_id,
            score: order.score,
            comment: order.comment,
            created_at: order.created_at,
            updated_at: order.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListOrdersResponse {
    orders: Vec<OrderResponse>
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteOrderResponse {
    done: bool
}

#[utoipa::path(
    post,
    path = "/v1/ds_shard_ver_orders",
    request_body = OrderCreationRequest,
    responses(
        (
            status = 200,
            description = "Dataset shard verification order created successfully",
            body = OrderResponse
        ),
    )
)]
#[instrument(skip(state))]
pub async fn create_order(
    State(state): State<AppState>,
    JsonExtractor(new_order): JsonExtractor<OrderCreationRequest>,
) -> Result<Json<OrderResponse>, DatasetShardVerificationOrderError> {
    let new_order = new_order.into();

    let created_order= ds_shard_ver_order_repository::create(
        &state.pg_pool, new_order
    )
        .await
        .map_err(DatasetShardVerificationOrderError::InternalError)?;

    Ok(Json(OrderResponse::from(created_order)))
}

#[utoipa::path(
    get,
    path = "/v1/ds_shard_ver_orders/{id}",
    params(
        ("id", Path, description = "Dataset shard verification order id")
    ),
    responses(
        (
            status = 200,
            description = "Dataset shard verification order query successfully",
            body = OrderResponse
        ),
        (status = NOT_FOUND, description = "Dataset shard verification order not found"),
    )
)]
#[instrument(skip(state))]
pub async fn get_order(
    State(state): State<AppState>,
    PathExtractor(order_id): PathExtractor<i32>,
) -> Result<Json<OrderResponse>, DatasetShardVerificationOrderError> {
    let order: DatasetShardVerificationOrderModel = ds_shard_ver_order_repository::get_by_id(
        &state.pg_pool, order_id
    )
        .await
        .map_err(DatasetShardVerificationOrderError::InternalError)?;

    Ok(Json(OrderResponse::from(order)))
}


#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct OrderSearchQuery {
    /// Dataset id
    pub ds_id: Option<i32>,
    /// Dataset shard id
    pub shard_id: Option<i32>,
    /// Dataset sample id (in shard)
    pub sample_id: Option<i32>,
    /// Order by score, true: desc, false: asc, None / default: do not order by score
    pub order_by_score: Option<bool>,
    /// Skip, default: 0
    pub skip: Option<i64>,
    /// Limit, default: 20
    pub limit: Option<i64>,
}

#[utoipa::path(
    get,
    path = "/v1/ds_shard_ver_orders",
    params(OrderSearchQuery),
    responses(
        (
            status = 200,
            description = "Dataset shard verification order query successfully",
            body = ListOrdersResponse
        ),
        (status = NOT_FOUND, description = "Dataset shard verification order not found"),
    )
)]
#[instrument(skip(state))]
pub async fn list_orders(
    State(state): State<AppState>,
    Query(params): Query<DatasetShardVerificationOrdersFilter>,
) -> Result<Json<ListOrdersResponse>, DatasetShardVerificationOrderError> {
    let orders = ds_shard_ver_order_repository::get_all(
        &state.pg_pool, params
    )
        .await
        .map_err(DatasetShardVerificationOrderError::InternalError)?;

    let orders = orders
        .into_iter()
        .map(OrderResponse::from)
        .collect();

    let orders = ListOrdersResponse {
        orders
    };

    Ok(Json(orders))
}

#[utoipa::path(
    delete,
    path = "/v1/ds_shard_ver_orders/{id}",
    params(
        ("id", Path, description = "Dataset shard verification order id")
    ),
    responses(
        (
            status = 200,
            description = "Dataset shard verification order deletion successfully",
            body = DeleteOrderResponse
        ),
        (status = NOT_FOUND, description = "Dataset shard verification order not found"),
    )
)]
#[instrument(skip(state))]
pub async fn delete_order(
    State(state): State<AppState>,
    PathExtractor(order_id): PathExtractor<i32>,
) -> Result<Json<DeleteOrderResponse>, DatasetShardVerificationOrderError> {
    ds_shard_ver_order_repository::delete_by_id(&state.pg_pool, order_id)
        .await
        .map_err(DatasetShardVerificationOrderError::InternalError)?;

    let res = DeleteOrderResponse {
        done: true
    };
    Ok(Json(res))
}

pub fn ds_shard_ver_orders_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(create_order))
        .route("/", get(list_orders))
        .route("/:id", get(get_order))
        .route("/:id", delete(delete_order))
        .with_state(state)
}
