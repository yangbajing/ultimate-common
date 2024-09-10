pub mod fruitbox_iam {
  pub mod v1 {
    tonic::include_proto!("fruitbox_iam.v1");

    #[cfg(feature = "tonic-reflection")]
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("fruitbox_iam_descriptor");
  }
}
