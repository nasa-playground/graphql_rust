use crate::contriview::ContriView;
use chrono::*;
use juniper::FieldResult;
use juniper::RootNode;
use reqwest::*;

pub struct Query;

graphql_object!(Query: () |&self| {
    field contriview(&executor, username: String) -> FieldResult<ContriView> {
        let url = format!("https://github.com/users/{}/contributions", username);

        let mut resp = Client::new().get(&url).send().unwrap();
        let html = resp.text().unwrap();
        let date = Local::today();

        Ok(ContriView::from_html(&html, date).unwrap())
    }
});

pub type Schema = RootNode<'static, Query, juniper::EmptyMutation<()>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, juniper::EmptyMutation::new())
}
