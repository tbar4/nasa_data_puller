use dotenv::dotenv;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::engine::remote::ws::Client;


pub struct SurrealConnection {
    host: String,
    port: String,
    namespace: String,
    database: String,
    user: String, 
    password: String,
}

impl Default for SurrealConnection {
    fn default() -> Self {
        SurrealConnection {
            host: dotenv::var("SURREAL_HOST").unwrap(),
            port: dotenv::var("SURREAL_PORT").unwrap(),
            namespace: dotenv::var("SURREAL_NS").unwrap(),
            database: dotenv::var("SURREAL_DB").unwrap(),
            user: dotenv::var("SURREAL_USER").unwrap(),
            password: dotenv::var("SURREAL_PASS").unwrap(),
        }
    }
}

impl SurrealConnection {
    pub async fn init() -> Result<Surreal<Client>, surrealdb::Error> {
        dotenv().ok();

        let surreal_connection = SurrealConnection {
            ..Default::default()
        };

        // Connect to the server
        let host = format!("{}:{}", surreal_connection.host, surreal_connection.port);
        let db = Surreal::new::<Ws>(host).await?;

        // Signin as a namespace, database, or root user
        db.signin(Root {
            username: surreal_connection.user.as_str(),
            password: surreal_connection.password.as_str(),
        })
        .await?;

        // Select a specific namespace / database
        db.use_ns(surreal_connection.namespace).use_db(surreal_connection.database).await?;

        Ok(db)
    }
}