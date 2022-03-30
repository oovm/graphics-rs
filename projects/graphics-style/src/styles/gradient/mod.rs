use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]

pub struct Gradient {
    shared: Arc<palette::Gradient>,
}
