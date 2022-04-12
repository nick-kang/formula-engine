use postgres_types::FromSql;

#[derive(Debug, FromSql)]
#[postgres(name = "public_key_type")]
pub enum PublicKeyType {
    #[postgres(name = "TEST")]
    Test,
    #[postgres(name = "PROD")]
    Prod,
}
