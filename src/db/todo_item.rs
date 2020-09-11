use deadpool_postgres::{Pool, PoolError};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
// use tokio_postgres::row::Row;
use tokio_postgres::types::ToSql;

#[derive(Serialize, Deserialize, PostgresMapper, Debug)]
#[pg_mapper(table = "todo_item")]
pub struct TodoItem {
  pub id: i32,
  pub list_id: i32,
  pub title: String,
  pub checked: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewTodoItem {
  pub list_id: i32,
  pub title: String,
  pub checked: bool,
}

pub async fn get_todo_items(pool: &Pool, list_id: i32) -> Result<Vec<TodoItem>, PoolError> {
  // create raw sql statement to execute
  let raw_sql = "SELECT * FROM todo_item WHERE list_id=$1;";
  // execute query
  let rows = todo_item_query(pool, raw_sql, &[&list_id]).await?;
  // return rowns
  Ok(rows)
}

pub async fn get_todo_item(pool: &Pool, uid: i32) -> Result<Vec<TodoItem>, PoolError> {
  //sql to run
  let raw_sql = "SELECT * FROM todo_item WHERE id=$1;";
  //execute query with params
  let row = todo_item_query(pool, raw_sql, &[&uid]).await?;
  //return row
  Ok(row)
}

pub async fn add_todo_item(pool: &Pool, item: &NewTodoItem) -> Result<Vec<TodoItem>, PoolError> {
  //sql to run
  let raw_sql = "INSERT INTO todo_item (list_id,title,checked) VALUES($1,$2,$3) RETURNING id,list_id,title,checked;";
  //execute query with params
  let row = todo_item_query(pool, raw_sql, &[&item.list_id, &item.title, &item.checked]).await?;
  //return row
  Ok(row)
}

pub async fn update_todo_item(pool: &Pool, item: &TodoItem) -> Result<Vec<TodoItem>, PoolError> {
  //sql to run
  let raw_sql = "UPDATE todo_item SET list_id=$1, title=$2, checked=$3 WHERE id=$4 RETURNING id,list_id,title,checked;";
  //execute query with params
  let row = todo_item_query(
    pool,
    raw_sql,
    &[&item.list_id, &item.title, &item.checked, &item.id],
  )
  .await?;
  //return row
  Ok(row)
}

pub async fn delete_todo_item(pool: &Pool, id: i32) -> Result<Vec<TodoItem>, PoolError> {
  //sql to run
  let raw_sql = "DELETE from todo_item WHERE id=$1 RETURNING id,list_id,title,checked;";
  //execute query with params
  let row = todo_item_query(pool, raw_sql, &[&id]).await?;
  //return row
  if row.len() == 0 {
    warn!("Nothing DELETED!!! Check id");
  }
  Ok(row)
}

// Runs all tokio_postgres queries for todo items.
// Provide pool, raw sql string and the list of parameters
// to be used in raw_sql. If no parameters provide refference
// to empty list/array like this &[].
async fn todo_item_query(
  pool: &Pool,
  raw_sql: &str,
  args: &'_ [&'_ (dyn ToSql + Sync)],
) -> Result<Vec<TodoItem>, PoolError> {
  // get connection from pool
  let cnn = pool.get().await?;

  // prepare returns tokio_postgres::Error
  // `RunError` impl `From<std::error::Error>`
  let sql = cnn.prepare(raw_sql).await?;

  // execute query and collect results
  let rows = cnn
    .query(&sql, args)
    .await?
    .iter()
    .map(|row| TodoItem::from_row_ref(row).unwrap())
    .collect::<Vec<TodoItem>>();

  Ok(rows)
}
