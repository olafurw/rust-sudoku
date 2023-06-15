pub fn save(value: &str) {
    let storage = &mut quad_storage::STORAGE.lock().unwrap();
    storage.set("save", value);
}
