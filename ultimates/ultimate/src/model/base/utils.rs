use modql::field::{SeaField, SeaFields};
use sea_query::{DynIden, IntoIden};

use crate::{
  ctx::Session,
  model::{
    base::{CommonIden, DbBmc, TimestampIden},
    Error, Result,
  },
};

/// This method must be called when a model controller intends to create its entity.
pub fn prep_fields_for_create<MC>(mut fields: SeaFields, session: &Session) -> SeaFields
where
  MC: DbBmc,
{
  if MC::has_owner_id() {
    fields.push(SeaField::new(CommonIden::OwnerId.into_iden(), session.uid()));
  }
  if MC::has_creation_timestamps() {
    fields = add_timestamps_for_create(fields, session);
  }

  fields
}

/// This method must be calledwhen a Model Controller plans to update its entity.
pub fn prep_fields_for_update<MC>(mut fields: SeaFields, session: &Session) -> SeaFields
where
  MC: DbBmc,
{
  if MC::has_creation_timestamps() {
    fields = add_timestamps_for_update(fields, session);
  }
  fields
}

fn _exists_in_fields(fields: &[SeaField], iden: DynIden) -> bool {
  // let iden = iden.into_iden();
  fields.iter().any(|f| f.iden == iden)
}

/// Update the timestamps info for create
/// (e.g., cid, ctime, and mid, mtime will be updated with the same values)
fn add_timestamps_for_create(fields: SeaFields, session: &Session) -> SeaFields {
  let mut fields = fields.into_vec();
  if !_exists_in_fields(&fields, TimestampIden::Cid.into_iden()) {
    fields.push(SeaField::new(TimestampIden::Cid, session.uid()));
  }
  if !_exists_in_fields(&fields, TimestampIden::Ctime.into_iden()) {
    fields.push(SeaField::new(TimestampIden::Ctime, *session.req_time()));
  }
  SeaFields::new(fields)
}

/// Update the timestamps info only for update.
/// (.e.g., only mid, mtime will be udpated)
fn add_timestamps_for_update(fields: SeaFields, session: &Session) -> SeaFields {
  let mut fields = fields.into_vec();
  if !_exists_in_fields(&fields, TimestampIden::Mid.into_iden()) {
    fields.push(SeaField::new(TimestampIden::Mid, session.uid()));
  }
  if !_exists_in_fields(&fields, TimestampIden::Mtime.into_iden()) {
    fields.push(SeaField::new(TimestampIden::Mtime, *session.req_time()));
  }
  SeaFields::new(fields)
}

/// 检查 sql execute 语句后受影响的数量
pub fn check_number_of_affected<MC>(expect_n: usize, return_n: u64) -> Result<u64>
where
  MC: DbBmc,
{
  // -- Check result
  if return_n as usize != expect_n {
    Err(Error::EntityNotFound {
      schema: MC::SCHEMA,
      entity: MC::TABLE,
      id: 0.into(), // Using 0 because multiple IDs could be not found, you may want to improve error handling here
    })
  } else {
    Ok(return_n)
  }
}
