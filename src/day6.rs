use axum::{debug_handler, Json};
use serde::Serialize;

#[derive(Serialize, Default)]
pub struct Occurence {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelf_with_no_elf: usize,
}

#[debug_handler]
pub async fn count_elfs(data: String) -> Json<Occurence> {
    let mut occ = Occurence::default();
    let search = b"elf on a shelf";
    occ.elf = data.matches("elf").count();
    occ.elf_on_a_shelf = data
        .clone()
        .into_bytes()
        .windows(search.len())
        .filter(|s| s == search)
        .count();
    occ.shelf_with_no_elf = data.matches("shelf").count() - occ.elf_on_a_shelf;

    Json(occ)
}
