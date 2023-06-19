pub fn load(key: &str) -> Option<String> {
    let storage = &mut quad_storage::STORAGE.lock().unwrap();
    storage.get(key)
}

pub fn save(key: &str, value: &str) {
    let storage = &mut quad_storage::STORAGE.lock().unwrap();
    storage.set(key, value);
}
