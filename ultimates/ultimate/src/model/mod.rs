pub mod acs;
pub mod auth;
pub mod base;
mod error;
pub mod modql_utils;
pub mod store;

use crate::ctx::Session;
use crate::model::store::dbx::{new_db_pool_from_config, Dbx};
use crate::model::store::DbConfig;

pub use self::error::{Error, Result};

#[derive(Clone)]
pub struct ModelManager {
  dbx: Dbx,
  session: Option<Session>,
}

impl ModelManager {
  /// Constructor
  pub async fn new(db_config: &DbConfig) -> Result<Self> {
    let db_pool =
      new_db_pool_from_config(db_config).await.map_err(|ex| Error::CantCreateModelManagerProvider(ex.to_string()))?;
    let dbx = Dbx::new(db_pool, false)?;
    Ok(ModelManager { dbx, session: None })
  }

  pub fn new_with_txn(&self) -> Result<ModelManager> {
    let dbx = Dbx::new(self.dbx.db().clone(), true)?;
    Ok(ModelManager { dbx, session: self.session.clone() })
  }

  pub fn dbx(&self) -> &Dbx {
    &self.dbx
  }

  pub fn session_opt_ref(&self) -> Option<&Session> {
    self.session.as_ref()
  }

  pub fn session_ref(&self) -> Result<&Session> {
    self.session.as_ref().ok_or(Error::Unauthorized)
  }

  pub fn with_session(mut self, ctx: Session) -> Self {
    self.session = Some(ctx);
    self
  }
}

#[derive(Clone)]
pub struct DbState {
  mm: ModelManager,
}

impl DbState {
  pub async fn from_config(db: &DbConfig) -> Result<Self> {
    let mm = ModelManager::new(db).await?;
    Ok(DbState { mm })
  }

  pub fn mm(&self) -> &ModelManager {
    &self.mm
  }
}
