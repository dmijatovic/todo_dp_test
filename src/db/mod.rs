use deadpool_postgres::Manager;
use std::env;
use std::time::Duration;
use tokio_postgres::{Config, NoTls};

pub mod todo_item;
pub mod todo_list;

fn new_config() -> Config {
  let mut pg_cfg = tokio_postgres::config::Config::new();
  // get env variables or use defaults
  let host = env::var("PG_HOST").unwrap_or(String::from("localhost"));
  let port: u16 = env::var("PG_PORT")
    .unwrap_or(String::from("5432"))
    .parse()
    .unwrap();
  let user = env::var("PG_USER").unwrap_or(String::from("postgres"));
  let pass = env::var("PG_PASS").unwrap_or(String::from("changeme"));
  let dbname = env::var("PG_DB").unwrap_or(String::from("todo_db"));
  let timeout: u64 = env::var("PG_TIMEOUT")
    .unwrap_or(String::from("5"))
    .parse()
    .unwrap();
  // assign variables to config
  pg_cfg.host(&host);
  pg_cfg.port(port);
  pg_cfg.user(&user);
  pg_cfg.password(&pass);
  pg_cfg.dbname(&dbname);
  pg_cfg.connect_timeout(Duration::new(timeout, 0));
  return pg_cfg;
}

pub type Pool = deadpool_postgres::Pool;

pub async fn create_pool() -> Pool {
  let size: usize = env::var("PG_POOL_SIZE")
    .unwrap_or(String::from("20"))
    .parse()
    .unwrap();
  //create configutation
  let cfg = new_config();
  //create pool manager
  let mgr = Manager::new(cfg, NoTls);
  //create new pool
  Pool::new(mgr, size)
}
