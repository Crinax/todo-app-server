pub trait GetTaskQuery {
    type Err;
    type Res;

    async fn get_task(&self, id: String) -> Result<Self::Res, Self::Err>;
}
