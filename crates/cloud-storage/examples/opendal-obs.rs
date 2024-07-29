use anyhow::Result;
use opendal::services::Obs;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
  // create backend builder
  let mut builder = Obs::default();

  // set the storage bucket for OpenDAL
  builder.bucket("file-009");
  builder.endpoint("obs.cn-southwest-2.myhuaweicloud.com");
  // Set the access_key_id and secret_access_key.
  //
  // OpenDAL will try load credential from the env.
  // If credential not set and no valid credential in env, OpenDAL will
  // send request without signing like anonymous user.
  builder.access_key_id("HGSSSWHAXMKCPFNCWVY3");
  builder.secret_access_key("r7gFLoIGBAvrl4mQhGu8Lnlu2IfNTLgC4vwtbglF");

  let op: Operator = Operator::new(builder)?.finish();

  let file1 = "sqls/2023-04-24/pgsql-approval-2023-04-24.tar.bz2";
  let meta = op.stat(file1).await?;
  println!("Metadata is {meta:?}");

  // 当前 opendal 不支持
  // let file1_to = "sqls/2023-04-24/pgsql-approval-2023-04-24.tar.bz2.bak";
  // op.remo(file1, file1_to).await?;

  Ok(())
}
