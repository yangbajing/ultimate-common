use ultimate_db::{base::DbBmc, generate_common_bmc_fns};

use super::model::{RoleEntity, RoleFilter, RoleForCreate, RoleForUpdate};

pub struct RoleBmc;
impl DbBmc for RoleBmc {
    const TABLE: &'static str = "role";
    const SCHEMA: &'static str = "iam";
}

generate_common_bmc_fns!(
    Bmc: RoleBmc,
    Entity: RoleEntity,
    ForCreate: RoleForCreate,
    ForUpdate: RoleForUpdate,
    Filter: RoleFilter,
);

#[cfg(test)]
mod test {
    use ultimate::{configuration::model::DbConfig, ctx::Ctx};
    use ultimate_db::ModelManager;

    use super::*;

    static DB_CONF_JSON: &str = r#""#;

    #[tokio::test]
    async fn test_role_bmc() -> anyhow::Result<()> {
        let db_config: DbConfig = serde_json::from_str(DB_CONF_JSON)?;
        let ctx = Ctx::new_root();
        let mm = ModelManager::new(&db_config).await?.with_ctx(ctx.clone());

        let filter = RoleFilter::default();
        let role = RoleBmc::find(&mm, filter).await?;
        println!("Fetch role: {:?}", role);

        Ok(())
    }
}
