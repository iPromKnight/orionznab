use std::sync::LazyLock;
use crate::torznab::types::Limits;

pub static SEARCH_LIMITS: LazyLock<Limits> = LazyLock::new(|| {
    Limits {
        max: 1000,
        default: 50,
    }
});