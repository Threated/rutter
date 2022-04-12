
use redis::from_redis_value;
use rocket::{Request, request::{FromRequest, Outcome}};
use rocket_db_pools::{Database, deadpool_redis, Connection, Pool};
use redis_graph::{GraphResultSet, GraphValue::Scalar};

use crate::auth::User;

#[derive(Database)]
#[database("redis")]
pub struct Db(deadpool_redis::Pool);

/// Database Connection Wrapper around `Connection<Db>`
/// Implements the actual redisgraph and redisearch commands used to query the data
pub struct Redis {
    pub connection: Connection<Db>
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Redis {

    type Error = Option<<<Db as Database>::Pool as Pool>::Error>;

    async fn from_request(request: & 'r Request<'_>) ->  Outcome<Self, Self::Error> {
        match Connection::<Db>::from_request(request).await {
            Outcome::Success(connection) => Outcome::Success(Redis {connection}),
            Outcome::Failure(x) => Outcome::Failure(x),
            Outcome::Forward(x) => Outcome::Forward(x),
        }
    }

}

impl Redis {
    pub async fn register_user(&mut self, user: &User<'_>) -> bool {
        let res = self.graph_query(
            "\
            MERGE (u:User {name: $name})
            ON CREATE SET u.password = $password", 
            &[
                ("name", user.name),
                ("password", &user.hash_pw())
            ]
        ).await;
        res.metadata.contains(&"Nodes created: 1".to_string())
    }

    pub async fn get_pw_by_name(&mut self, name: &str) -> Option<String> {
        let res = self.graph_query(
            "\
            MATCH (u:User {name: $name})
            RETURN u.password AS pw",
            &[("name", name)]
        ).await;
        match res.data.get(0)?.data.get("pw")? {
            Scalar(pw) => from_redis_value(pw).ok(),
            _ => None,
        }
    }

    pub async fn follow(&mut self, user: &str, follow: &str) {
        self.graph_query(
            "\
            MATCH (u:User {name: $user})
            MATCH (f:User {name: $follow})
            MERGE (u)-[:follows]->(f)",
            &[
                ("user", user),
                ("follow", follow)
            ]
        ).await;
    }

    pub async fn tweet(&mut self, user: &str, tweet: &str) {
        self.graph_query(
            "\
            MATCH (u:User {name: $user})
            CREATE (u)-[:tweets]->(:Tweet {content: $tweet, published: timestamp()})",
            &[
                ("user", user),
                ("tweet", tweet)
            ]
        ).await;
    }

    pub async fn answer_tweet(&mut self, user: &str, answer: &str, answer_to: i32) {
        let command = format!("\
            MATCH (u:User {{name: $user}}), (t:Tweet)
            WHERE id(t) = {}
            CREATE (u)-[:tweets]->(:Tweet {{content: $tweet, published: timestamp()}})<-[:answer]-(t)",
            answer_to
        );
        self.graph_query(
            &command,
            &[
                ("user", user),
                ("tweet", answer),
            ]
        ).await;
    }

    pub async fn unfollow(&mut self, user: &str, follow: &str) {
        self.graph_query("\
            MATCH (:User {name: $name})-[f:follows]->(:User {name: $follow})
            DELETE f",
            &[
                ("user", user),
                ("follow", follow)
            ]
        ).await;
    }

    async fn graph_query(&mut self, command: &str, params: &[(&str, &str)]) -> GraphResultSet {
        redis::cmd("GRAPH.QUERY")
            .arg("social")
            .arg(parse_params(params) + command)
            .query_async(&mut *self.connection).await.unwrap()
    }
}

fn parse_params(params: &[(&str, &str)]) -> String {
    if params.is_empty() {
        return String::new()
    }
    const CYPHER: &str = "CYPHER ";
    let mut prepend = String::with_capacity(
        CYPHER.len()
        + params.iter().map(|(k, v)| k.len() + v.len() + 4).sum::<usize>());
    prepend.push_str(CYPHER);
    params.iter().for_each(|(key, value)| prepend.push_str(&format!("{}='{}' ", key, value)));
    prepend
}
