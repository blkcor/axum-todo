pub mod category;
pub mod paginate;
/*
通用的sql操作
*/

use deadpool_postgres::{Client, GenericClient};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::{
    types::{FromSqlOwned, ToSql},
    Statement,
};

use crate::{error::AppError, Result};

use self::paginate::Paginate;

/*
    获取statement，用于预处理sql语句
*/
async fn get_stmt(client: &impl GenericClient, sql: &str) -> Result<Statement> {
    client.prepare(sql).await.map_err(AppError::from)
}

/*
查询多条记录
 */
async fn query<T>(
    client: &impl GenericClient,
    sql: &str,
    params: &[&(dyn ToSql + Sync)],
) -> Result<Vec<T>>
where
    T: FromTokioPostgresRow,
{
    let stmt = get_stmt(client, sql).await?;
    let result = client
        .query(&stmt, params)
        .await
        .map_err(AppError::from)?
        .iter()
        .map(|row| T::from_row_ref(row).unwrap())
        .collect::<Vec<T>>();
    Ok(result)
}

/*
    查询单条记录
*/
async fn query_row_opt<T>(
    client: &impl GenericClient,
    sql: &str,
    params: &[&(dyn ToSql + Sync)],
    message: Option<String>,
) -> Result<T>
where
    T: FromTokioPostgresRow,
{
    query(client, sql, params)
        .await?
        .pop()
        .ok_or(AppError::notfound_opt(message))
}

/*
查询单条记录，并指定当记录不存在时，使用的错误信息
*/
async fn query_row_msg<T>(
    client: &impl GenericClient,
    sql: &str,
    params: &[&(dyn ToSql + Sync)],
    msg: &str,
) -> Result<T>
where
    T: FromTokioPostgresRow,
{
    query_row_opt(client, sql, params, Some(msg.to_string())).await
}
/*
查询单条记录，当记录不存在时，使用默认的错误信息
*/
async fn query_row<T>(
    client: &impl GenericClient,
    sql: &str,
    params: &[&(dyn ToSql + Sync)],
) -> Result<T>
where
    T: FromTokioPostgresRow,
{
    query_row_opt(client, sql, params, None).await
}
/// 插入记录并返回指定数据
async fn insert<T>(
    client: &impl GenericClient,
    sql: &str,
    params: &[&(dyn ToSql + Sync)],
    msg: &str,
) -> Result<T>
where
    T: FromTokioPostgresRow,
{
    query_row_msg(client, sql, params, msg).await
}

/*
查询单列数据
*/

async fn query_col<T>(
    client: &impl GenericClient,
    sql: &str,
    params: &[&(dyn ToSql + Sync)],
) -> Result<T>
where
    T: FromSqlOwned,
{
    let stmt = get_stmt(client, sql).await?;
    Ok(client
        .query_one(&stmt, params)
        .await
        .map_err(AppError::from)?
        .get(0))
}

/*
 查询数据个数
*/
async fn count(
    client: &impl GenericClient,
    sql: &str,
    params: &[&(dyn ToSql + Sync)],
) -> Result<i64> {
    query_col(client, sql, params).await
}

/*
执行
*/
async fn execute(
    client: &impl GenericClient,
    sql: &str,
    args: &[&(dyn ToSql + Sync)],
) -> Result<u64> {
    let stmt = get_stmt(client, sql).await?;
    client.execute(&stmt, args).await.map_err(AppError::from)
}

/*
分页查询
*/

const DEFAULT_PAGE_SIZE: u8 = 10;

async fn pagination<T>(
    client: &impl GenericClient,
    sql: &str,
    count_sql: &str,
    params: &[&(dyn ToSql + Sync)],
    page: u32,
) -> Result<Paginate<Vec<T>>>
where
    T: FromTokioPostgresRow,
{
    let data = query(client, sql, params).await?;
    let total_records = count(client, count_sql, params).await?;
    Ok(Paginate::new(page, DEFAULT_PAGE_SIZE, total_records, data))
}

/*
    删除或者恢复记录
*/
async fn del_or_restore(
    client: &Client,
    table: &str,
    id: &(dyn ToSql + Sync),
    is_del: bool,
) -> Result<u64> {
    let sql = format!("UPDATE {} SET is_del=$1 WHERE id=$2", table);
    self::execute(client, &sql, &[&is_del, id]).await
}
