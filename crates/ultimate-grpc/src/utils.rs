use prost_types::FieldMask;
use tonic::{metadata::MetadataMap, Status};
use ultimate::{
  configuration::model::SecurityConf,
  security::{jose::JwtPayload, SecurityUtils},
};

pub fn extract_jwt_payload_from_metadata(
  sc: &SecurityConf,
  metadata: &MetadataMap,
) -> Result<JwtPayload, tonic::Status> {
  let token = extract_token_from_metadata(metadata)?;
  let (payload, _) = SecurityUtils::decrypt_jwt(sc.pwd(), token).map_err(|e| Status::unauthenticated(e.to_string()))?;
  Ok(payload)
}

pub fn extract_token_from_metadata(metadata: &MetadataMap) -> Result<&str, tonic::Status> {
  let auth_header =
    metadata.get("authorization").ok_or_else(|| Status::unauthenticated("Missing authorization header"))?;
  let auth_str = auth_header.to_str().map_err(|_| Status::unauthenticated("Invalid authorization header"))?;
  let offset = if auth_str.starts_with("Bearer ") { 7 } else { 0 };

  Ok(&auth_str[offset..])
}

// 当 paths 为空或者 paths 包含以 path 开头的路径时返回 true，否则返回 false
pub fn field_mask_match_with(field_mask: &FieldMask, path: &str) -> bool {
  field_mask.paths.is_empty() || field_mask.paths.iter().any(|p| p.starts_with(path))
}
