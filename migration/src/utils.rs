use uuid::Uuid;

pub fn default_uuid() -> Option<Box<Uuid>> {
    Some(Box::new(Uuid::new_v4()))
}
