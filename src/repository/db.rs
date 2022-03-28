use ::fast_log::config::Config;
use fast_log::fast_log;
use rbatis::rbatis::Rbatis;

pub async fn get_db_conn() -> Option<Rbatis> {
    let rb = Rbatis::new();
    // TODO
    rb.link("sqlite://").await.unwrap();

    let log = fast_log::init(Config::new().console()).unwrap();

    Some(rb)
}
