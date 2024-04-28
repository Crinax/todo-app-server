use crate::core::{
    entities::{
        rules::{min_len_string::MinLenString, string_based_id::StringBasedId, BuisnessRule},
        task::Task,
    },
    ports::secondary::task::CreateTaskPort,
};

struct CreateTaskAdapter {}

impl CreateTaskPort for CreateTaskAdapter {
    type CreateTaskPortError = ();

    async fn create_task(
        &self,
        name_base: &MinLenString<1>,
        description_base: &MinLenString<0>,
    ) -> Result<Task, Self::CreateTaskPortError> {
        let id = StringBasedId::parse("1".to_owned()).unwrap();
        let name = name_base.clone();
        let description = description_base.clone();

        // some async logic here :D

        Ok(Task::new(id, name, description))
    }
}

#[tokio::test]
async fn create_task_success_case() {
    let adapter = CreateTaskAdapter {};
    let result = adapter
        .create_task(
            &MinLenString::parse("1234".to_owned()).unwrap(),
            &MinLenString::parse("1234".to_owned()).unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(result.id(), "1");
    assert_eq!(result.name(), "1234");
    assert_eq!(result.description(), "1234");
}
