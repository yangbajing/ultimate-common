use ultimate::{configuration::model::DbConfig, ctx::Ctx};

use crate::store::{dbx::new_db_pool_from_config, Dbx};

use crate::{Error, Result};

#[derive(Clone)]
pub struct ModelManager {
    dbx: Dbx,
    session: Option<Ctx>,
}

impl ModelManager {
    /// Constructor
    pub async fn new(db_config: &DbConfig) -> Result<Self> {
        let db_pool = new_db_pool_from_config(db_config)
            .await
            .map_err(|ex| Error::CantCreateModelManagerProvider(ex.to_string()))?;
        let dbx = Dbx::new(db_pool, false)?;
        Ok(ModelManager { dbx, session: None })
    }

    pub fn new_with_txn(&self) -> Result<ModelManager> {
        let dbx = Dbx::new(self.dbx.db().clone(), true)?;
        Ok(ModelManager { dbx, session: self.session.clone() })
    }

    pub fn get_or_new_with_txn(&self) -> Result<ModelManager> {
        if self.dbx().is_txn() {
            Ok(self.clone())
        } else {
            Ok(self.new_with_txn()?)
        }
    }

    pub fn dbx(&self) -> &Dbx {
        &self.dbx
    }

    pub fn session_opt_ref(&self) -> Option<&Ctx> {
        self.session.as_ref()
    }

    pub fn session_ref(&self) -> Result<&Ctx> {
        self.session.as_ref().ok_or(Error::Unauthorized)
    }

    pub fn with_session(mut self, ctx: Ctx) -> Self {
        self.session = Some(ctx);
        self
    }
}
