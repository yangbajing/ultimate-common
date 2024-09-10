use ultimate_db::{base::DbBmc, generate_common_bmc_fns};

use super::{Permission, PermissionFilter, PermissionForCreate, PermissionForUpdate};

pub struct PermissionBmc;
impl DbBmc for PermissionBmc {
  const SCHEMA: &'static str = "iam";
  const TABLE: &'static str = "permission";
}

generate_common_bmc_fns!(
  Bmc: PermissionBmc,
  Entity: Permission,
  ForCreate: PermissionForCreate,
  ForUpdate: PermissionForUpdate,
  Filter: PermissionFilter,
);
