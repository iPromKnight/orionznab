use std::sync::LazyLock;
use crate::torznab::types::{Category, Subcategory};

pub static CATEGORIES: LazyLock<Vec<Category>> = LazyLock::new(|| {
    vec![
        Category {
            id: 2000,
            name: "Movies".to_string(),
            subcategories: vec![
                Subcategory { id: 2010, name: "Movies/Foreign".to_string() },
                Subcategory { id: 2020, name: "Movies/Other".to_string() },
                Subcategory { id: 2030, name: "Movies/SD".to_string() },
                Subcategory { id: 2040, name: "Movies/HD".to_string() },
                Subcategory { id: 2045, name: "Movies/UHD".to_string() },
                Subcategory { id: 2050, name: "Movies/BluRay".to_string() },
                Subcategory { id: 2060, name: "Movies/3D".to_string() },
                Subcategory { id: 2070, name: "Movies/DVD".to_string() },
                Subcategory { id: 2080, name: "Movies/WEB-DL".to_string() },
            ],
        },
        Category {
            id: 5000,
            name: "TV".to_string(),
            subcategories: vec![
                Subcategory { id: 5010, name: "TV/WEB-DL".to_string() },
                Subcategory { id: 5020, name: "TV/Foreign".to_string() },
                Subcategory { id: 5030, name: "TV/SD".to_string() },
                Subcategory { id: 5040, name: "TV/HD".to_string() },
                Subcategory { id: 5045, name: "TV/UHD".to_string() },
                Subcategory { id: 5050, name: "TV/Other".to_string() },
                Subcategory { id: 5060, name: "TV/Sport".to_string() },
                Subcategory { id: 5070, name: "TV/Anime".to_string() },
                Subcategory { id: 5080, name: "TV/Documentary".to_string() },
            ],
        },
    ]
});

pub fn get_category_by_name(name: &str) -> Option<&Category> {
    CATEGORIES.iter().find(|c| c.name == name)
}