/// 通用 Repositories，如：角色、权限、用户的关系表等
pub mod model;

mod user_role;

pub use user_role::UserRoleBmc;
