use redis::RedisResult;
use rocket::{
    request::{FromRequest, Outcome},
    Request
};
use rocket_db_pools::{deadpool_redis, Connection, Database, Pool};
use uuid::Uuid;
use redisgraphio::{AsyncGraphCommands, query, GraphQuery, GraphResponse, FromGraphValue, GraphStatistic};


use crate::{auth::User, types::Tweet};

#[derive(Database)]
#[database("redis")]
pub struct Db(deadpool_redis::Pool);

/// Database Connection Wrapper around `Connection<Db>`
/// Implements the actual redisgraph and redisearch commands used to query the data
pub struct Redis {
    pub connection: Connection<Db>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Redis {
    type Error = Option<<<Db as Database>::Pool as Pool>::Error>;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match Connection::<Db>::from_request(request).await {
            Outcome::Success(connection) => Outcome::Success(Redis { connection }),
            Outcome::Failure(x) => Outcome::Failure(x),
            Outcome::Forward(x) => Outcome::Forward(x),
        }
    }
}

impl Redis {
    pub async fn register_user(&mut self, user: &User<'_>) -> bool {
        let res = self.graph_query::<()>(query!("\
            MERGE (u:User {name: $name})
            ON CREATE SET u.password = $password, u.follower = 0, u.follows = 0",
            {
                "name" => user.name, "password" => &user.hash_pw()
            }
        )).await.unwrap();
        res.get_statistic(GraphStatistic::NodesCreated) == Some(1.0)
    }

    pub async fn get_pw_by_name(&mut self, name: &str) -> Option<String> {
        self.graph_query::<(Option<String>,)>(query!("\
            MATCH (u:User {name: $name})
            RETURN u.password AS pw",
            {
                "name" => name
            }, true
        )).await.unwrap().data.pop()?.0
    }

    pub async fn follow(&mut self, user: &str, follow: &str) {
        self.graph_query::<()>(query!(
            "\
            MATCH (u:User {name: $user})
            MATCH (f:User {name: $follow})
            MERGE (u)-[:follows]->(f)
            SET f.follower = f.follower + 1, u.follows = u.follows + 1",
            {"user" => user, "follow" => follow}
        )).await.unwrap();
    }

    pub async fn unfollow(&mut self, user: &str, follow: &str) {
        self.graph_query::<()>(query!(
            "\
            MATCH (you:User {name: $user})-[f:follows]->(other:User {name: $follow})
            SET other.follower = other.follower - 1, you.follows = u.follows - 1
            DELETE f",
            {"user" => user, "follow" => follow}
        )).await.unwrap();
    }

    pub async fn tweet(&mut self, user: &str, tweet: &str) {
        self.graph_query::<()>(query!("\
            MATCH (u:User {name: $user})
            CREATE (u)-[:tweets]->(:Tweet {content: $tweet, published: timestamp(), id: $id, likes: 0})",
            {"user" => user, "tweet" => tweet, "id" => Uuid::new_v4().to_string()}
        )).await.unwrap();
    }

    pub async fn answer_tweet(&mut self, user: &str, answer: &str, answer_to: Uuid) -> bool{
        let res = self.graph_query::<()>(query!("\
            MATCH (u:User {name: $user}), (t:Tweet)
            WHERE t.id = $answer_to
            CREATE (u)-[:tweets]->(:Tweet {content: $tweet, published: timestamp(), id: $id, likes: 0})<-[:answer]-(t)",
            {
                "answer_to" => answer_to.to_string(),
                "id" => Uuid::new_v4().to_string(),
                "answer" => answer,
                "user" => user
            }
        )).await.unwrap();
        res.get_statistic(GraphStatistic::NodesCreated) == Some(1.0)
    }

    pub async fn get_tweet_by_id(&mut self, id: Uuid) -> Option<(String, Tweet)> {
        self.graph_query(query!("\
            MATCH (u)-[:tweets]->(t:Tweet {id: $id})
            RETURN u.name, t",
            {"id" => id.to_string()}, true
        )).await.ok()?.data.pop()
    }

    pub async fn delete_user(&mut self, user: &str) {
        self.graph_query::<()>(query!("\
            MATCH (u:User {name: $user})
            DETACH DELETE u",
        {"user" => user}
        )).await.unwrap();
    }

    pub async fn get_user(&mut self, user: &str) -> Option<(String, Vec<Tweet>)> {
        self.graph_query(query!("\
            MATCH (u:User {name: $user})
            OPTIONAL MATCH (u)-[:tweets]->(t:Tweet)
            WHERE NOT (t)-[:answer]->(:Tweet)
            WITH u, t ORDER BY t.published DESC LIMIT 10
            RETURN u.name, t AS tweets",
            {
                "user" => user        
            }, true
        )).await.ok()?.data.pop()
    }

    #[inline]
    pub async fn graph_query<RT: FromGraphValue>(&mut self, query: GraphQuery) -> RedisResult<GraphResponse<RT>> {
        self.connection.graph_query("social", query).await
    }
}
