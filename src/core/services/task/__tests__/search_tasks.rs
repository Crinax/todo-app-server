use crate::core::{
    entities::{
        rules::{min_len_string::MinLenString, string_based_id::StringBasedId, BusinessRule},
        task::Task,
        window::{Id, Window},
    },
    ports::secondary::task::SearchTasksByNamePort,
};

struct SearchTasksAdapter {}

impl SearchTasksByNamePort for SearchTasksAdapter {
    type SearchTasksByNamePortError = ();

    async fn search_tasks_by_name(
        &self,
        text: &MinLenString<1>,
    ) -> Result<Window<Task>, Self::SearchTasksByNamePortError> {
        let id = StringBasedId::parse("1".to_owned()).unwrap();
        let name = MinLenString::parse("1234".to_owned()).unwrap();
        let description = MinLenString::parse("1234".to_owned()).unwrap();

        let task = Task::new(id, name, description);
        let result = [task];

        Ok(result
            .iter()
            .filter(|task| task.name().eq(text))
            .cloned()
            .collect())
    }
}

#[tokio::test]
async fn search_tasks_by_name_found_task_case() {
    let adapter = SearchTasksAdapter {};
    let result = adapter
        .search_tasks_by_name(&MinLenString::parse("1234".to_owned()).unwrap())
        .await
        .unwrap();

    assert_eq!(result.len(), 1);

    let task = result.collection().first().unwrap();

    assert_eq!(task.name(), &"1234".parse().unwrap());
    assert_eq!(task.id(), &"1".parse().unwrap());
    assert_eq!(task.description(), &"1234".parse().unwrap());
}

#[tokio::test]
async fn search_tasks_by_name_not_found_task_case() {
    let adapter = SearchTasksAdapter {};
    let result = adapter
        .search_tasks_by_name(&MinLenString::parse("12345".to_owned()).unwrap())
        .await
        .unwrap();

    assert_eq!(result.len(), 0);
}
