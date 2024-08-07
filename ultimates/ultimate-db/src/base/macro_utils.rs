/// Convenience macro rules to generate default CRUD functions for a Bmc/Entity.
/// Note: If custom functionality is required, use the code below as foundational
///       code for the custom implementations.
#[macro_export]
macro_rules! generate_common_bmc_fns {
	(
		Bmc: $struct_name:ident,
		Entity: $entity:ty,
		$(ForCreate: $for_create:ty,)?
		$(ForUpdate: $for_update:ty,)?
		$(Filter: $filter:ty,)?
	) => {
		impl $struct_name {
			$(
				pub async fn create(
					mm: &ModelManager,
					entity_c: $for_create,
				) -> ultimate_db::Result<i64> {
					ultimate_db::base::create::<Self, _>(mm, entity_c).await
				}

				pub async fn create_many(
					mm: &ModelManager,
					entity_c: Vec<$for_create>,
				) -> ultimate_db::Result<Vec<i64>> {
					ultimate_db::base::create_many::<Self, _>(mm, entity_c).await
				}
			)?

			$(
				pub async fn insert(
					mm: &ModelManager,
					entity_c: $for_create,
				) -> ultimate_db::Result<()> {
					ultimate_db::base::insert::<Self, _>(mm, entity_c).await
				}

				pub async fn insert_many(
					mm: &ModelManager,
					entity_c: Vec<$for_create>,
				) -> ultimate_db::Result<u64> {
					ultimate_db::base::insert_many::<Self, _>(mm, entity_c).await
				}
			)?

				pub async fn get_by_id(
					mm: &ModelManager,
					id: ultimate_db::Id,
				) -> ultimate_db::Result<$entity> {
					ultimate_db::base::get_by_id::<Self, _>(mm, id).await
				}

			$(
				pub async fn find(
					mm: &ModelManager,
					filter: $filter,
				) -> ultimate_db::Result<Option<$entity>> {
					ultimate_db::base::find::<Self, _, _>(mm, filter).await
				}

				pub async fn list(
					mm: &ModelManager,
					filter: Option<$filter>,
					list_options: Option<modql::filter::ListOptions>,
				) -> ultimate_db::Result<Vec<$entity>> {
					ultimate_db::base::list::<Self, _, _>(mm, filter, list_options).await
				}

				pub async fn count(
					mm: &ModelManager,
					filter: Option<$filter>,
				) -> ultimate_db::Result<i64> {
					ultimate_db::base::count::<Self, _>(mm, filter).await
				}
			)?

			$(
				pub async fn update(
					mm: &ModelManager,
					id: ultimate_db::Id,
					entity_u: $for_update,
				) -> ultimate_db::Result<()> {
					ultimate_db::base::update::<Self, _>(mm, id, entity_u).await
				}
			)?

				pub async fn delete(
					mm: &ModelManager,
					id: ultimate_db::Id,
				) -> ultimate_db::Result<()> {
					ultimate_db::base::delete::<Self>(mm, id).await
				}

				pub async fn delete_many(
					mm: &ModelManager,
					ids: Vec<ultimate_db::Id>,
				) -> ultimate_db::Result<u64> {
					ultimate_db::base::delete_many::<Self>(mm, ids).await
				}
		}
	};
}
