use diesel::prelude::*;
use diesel::mysql::{Mysql, MysqlConnection};
use diesel::connection::Connection;

use r2d2::{self, Pool, Config, PooledConnection };
use r2d2_diesel::ConnectionManager;
use diesel::connection::AnsiTransactionManager;
//use nickel::status::Status::{self};

use dotenv::dotenv;
use std::env;
use std::clone::{self, Clone};


pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let db_url : String = String::from(env::var("DATABASE_URL")
                                .expect("DATABASE_URL must be set"));
    let db_connection = MysqlConnection::establish(&db_url)
                                .expect(&format!("Error connecting to {}",&db_url));

    db_connection
}


/*struct SelfContainedConnection<M, T> {
    // must be in this order to ensure the connection drops before the pool
    //core_renderer: &'a mut CanvasRenderer<'a>,
    conn: &'static mut PooledConnection<T>,
    pool: Arc<Pool<M>>,
}

impl<M, T> SelfContainedConnection<M, T> where M: ConnectionManager, T: ConnectionManager::Connection {
    pub fn new(pool: &Arc<Pool>) -> Result<SelfContainedConnection, GetTimeout> {
        let conn = try!(pool.get());
        let pool = pool.clone();
        Ok(SelfContainedConnection {
            conn: unsafe { mem::transmute(conn) },
            pool: pool
        })
    }
}

impl<M, T> Deref for SelfContainedConnection<M, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &*self.conn
    }
}*/


/*lazy_static! {
    static ref CONNECTION: Pool<ConnectionManager<MysqlConnection>> = {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let config = Config::default();
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        Pool::new(config, manager).expect("Failed to create pool")
    };
}

pub fn connection() -> Pool<ConnectionManager<MysqlConnection>> {
    CONNECTION.clone()
}*/

/*pub fn create_db_pool() -> Pool<ConnectionManager<MysqlConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let config = Config::default();
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::new(config, manager).expect("Failed to create pool.")
}

// DB Items
lazy_static! {
    pub static ref DB_POOL: Pool<ConnectionManager<MysqlConnection>> = create_db_pool();
}

pub struct DB(PooledConnection<ConnectionManager<MysqlConnection>>);

impl DB {
    pub fn conn(&self) -> &MysqlConnection {
        &*self.0
    }
}

impl Connection for MysqlConnection {
    type Backend = Mysql;
    type TransactionManager = AnsiTransactionManager;
    fn establish(database_url: &str) -> ConnectionResult<MysqlConnection>{
        match DB_POOL.get() {
            Ok(conn) => println!("one"),
            Err(e) => println!("none"),
        }
    }
}*/
