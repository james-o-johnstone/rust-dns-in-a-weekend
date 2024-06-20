pub use struct_bytes_derive::ToBytes;

pub trait ToBytes {
  fn to_bytes(&self) -> Vec<u8>;
}

impl ToBytes for u16 {
  fn to_bytes(&self) -> Vec<u8> {
      self.to_be_bytes().to_vec()
  }
}

impl ToBytes for String {
  fn to_bytes(&self) -> Vec<u8> {
      self.as_bytes().to_vec()
  }
}
