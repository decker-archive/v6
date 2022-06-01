use rustflake::Snowflake;

pub fn gen_sf() -> i64 {
    let mut snowflake = Snowflake::new(1649325271415, 1, 1);
    snowflake.generate()
}
