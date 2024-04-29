use std::collections::HashMap;

use crate::tr::hash::ChooseHash;

#[derive(Default, PartialEq)]
pub struct Query {
    query: HashMap<String, QueryData>,
}
impl Query {
    /// Check Data and add it if is't same
    async fn push(&mut self, name: String, data: QueryData) {
        match self.query.get_mut(&name) {
            Some(e) => {
                if data != *e {
                    *e = data;
                }
            }
            None => {
                self.query.insert(name, data);
            }
        }
    }

    pub fn query(&self) -> &HashMap<String, QueryData> {
        &self.query
    }

    pub fn query_mut(&mut self) -> &mut HashMap<String, QueryData> {
        &mut self.query
    }

    pub fn set_query(&mut self, query: HashMap<String, QueryData>) {
        self.query = query;
    }
}
#[derive(PartialEq)]
pub struct QueryData {
    link: String,
    hash: ChooseHash,
    build: String,
}

impl From<(String, ChooseHash, String)> for QueryData {
    fn from(value: (String, ChooseHash, String)) -> Self {
        Self {
            link: value.0,
            hash: value.1,
            build: value.2,
        }
    }
}
