/// Объект роя
pub struct SwarmObject {
  pub username: String,
  pub version: String,
  pub reader_capacity: usize,
  pub writer_capacity: usize,
}

impl SwarmObject {
  /// Метод создания нового объекта роя
  pub fn create(username: impl Into<String>, version: impl Into<String>) -> Self {
    Self {
      username: username.into(),
      version: version.into(),
      reader_capacity: 45,
      writer_capacity: 45,
    }
  }
}
