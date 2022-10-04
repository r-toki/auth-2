use ulid::Ulid;

pub fn get_new_id() -> String {
    Ulid::new().to_string()
}
