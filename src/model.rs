use serde::Serialize;
use tokio_pg_mapper_derive::PostgresMapper;

///分类
#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table = "categories")]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub is_del: bool,
}

///分类ID 这个结构体是为了对新插入的ID单独定义一个结构体，插入数据的时候能够获取到返回的ID
#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table = "categories")]
pub struct CategoryID {
    pub id: i32,
}
