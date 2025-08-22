use {log::log, refinery::config::ConfigDbType, std::env};
mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migration");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let db_host = env::var("DB_HOST")?;
    let db_port = env::var("DB_PORT")?;
    let db_user = env::var("DB_USER")?;
    let db_pass = env::var("DB_PASS")?;
    let db_name = env::var("DB_NAME")?;

    tracing::info!(
        "db_host: {:?}, db_post {:?}, db_user {:?}, db_pass {:?}, db_name {:?}",
        db_host,
        db_pass,
        db_user,
        db_pass,
        db_name,
    );
    let mut conf = refinery::config::Config::new(ConfigDbType::Postgres)
        .set_db_user(&db_user)
        .set_db_pass(&db_pass)
        .set_db_host(&db_host)
        .set_db_port(&db_port)
        .set_db_name(&db_name);
    // Apply embedded migrations using sqlx pool
    let _ = embedded::migrations::runner().run(&mut conf)?;

    tracing::info!("✅ Migrations applied successfully.");

    Ok(())
}
