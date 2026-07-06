// 3.	Настройте автоматическое применение миграций:
// o	В src/infrastructure/database.rs создайте функцию для подключения к БД (connection pool).
// o	Создайте функцию для применения миграций через sqlx::migrate!() макрос.
// o	Вызовите эти функции в main.rs при старте сервера.

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

pub async fn run(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    MIGRATOR.run(pool).await
}

pub async fn get_pool(db_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(db_url)
        .await?;
    Ok(pool)
}
