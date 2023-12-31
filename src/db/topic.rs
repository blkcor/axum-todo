use std::time;

use deadpool_postgres::Client;

use crate::{
    form, md,
    model::{TopicEditData, TopicID, TopicList},
    Result,
};

use super::{paginate::Paginate, DEFAULT_PAGE_SIZE};

/// 创建文章
pub async fn create(client: &Client, frm: &form::CreateTopic) -> Result<TopicID> {
    let html = md2html(&frm.markdown);
    let dateline = time::SystemTime::now();
    super::insert(
        client,
        "INSERT INTO topics (title,category_id, summary, markdown, html, hit, dateline, is_del) VALUES ($1, $2, $3, $4, $5, 0, $6, false) RETURNING id",
        &[
            &frm.title,
            &frm.category_id,
            &frm.summary,
            &frm.markdown,
            &html,
            &dateline,
        ],
        "添加文章失败",
    )
    .await
}

/// 文章列表
pub async fn list(client: &Client, page: u32) -> Result<Paginate<Vec<TopicList>>> {
    let sql=format!("SELECT id,title,category_id,summary,hit,dateline,is_del,category_name FROM v_topic_cat_list WHERE is_del=false ORDER BY id DESC LIMIT {} OFFSET {}", DEFAULT_PAGE_SIZE, DEFAULT_PAGE_SIZE as u32 * page);
    let count_sql = "SELECT COUNT(*) FROM v_topic_cat_list WHERE is_del=false";
    super::pagination(client, &sql, count_sql, &[], page).await
}

/// 更新文章
pub async fn update(client: &Client, frm: &form::EditTopic, id: i64) -> Result<bool> {
    let html = md2html(&frm.markdown);
    let sql =
        "UPDATE topics SET title=$1,category_id=$2,summary=$3,markdown=$4,html=$5 WHERE id=$6";
    let n = super::execute(
        client,
        sql,
        &[
            &frm.title,
            &frm.category_id,
            &frm.summary,
            &frm.markdown,
            &html,
            &id,
        ],
    )
    .await?;
    Ok(n > 0)
}

/// 查找用于修改的文章数据
pub async fn find2edit(client: &Client, id: i64) -> Result<TopicEditData> {
    let sql = "SELECT id,title,category_id,summary,markdown FROM topics WHERE id=$1";
    super::query_row(client, sql, &[&id]).await
}

//删除或者还原文章
pub async fn del_or_restore(client: &Client, id: i64, is_del: bool) -> Result<bool> {
    let n = super::del_or_restore(client, "topic", &id, is_del).await?;
    Ok(n > 0)
}

/// markdown -> html
fn md2html(markdown: &str) -> String {
    md::to_html(markdown)
}
