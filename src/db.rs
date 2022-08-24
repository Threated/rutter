use redis::RedisResult;
use redisgraphio::{
    query, AsyncGraphCommands, FromGraphValue, GraphQuery, GraphResponse, GraphStatistic,
};
use rocket::{
    request::{FromRequest, Outcome},
    Request, Build, fairing, Rocket
};
use rocket_db_pools::{deadpool_redis, Connection, Database, Pool};
use uuid::Uuid;

use crate::{auth::User, types::{Tweet, self}};

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

pub async fn db_migrations(rocket: Rocket<Build>) -> fairing::Result {
    if let Some(db) = Db::fetch(&rocket) {
        let connection = &mut db.0
            .get()
            .await
            .expect("Unable to get db connection");
        // Create search idx on username
        connection.graph_query_void(
            "social",
            query!("CALL db.idx.fulltext.createNodeIndex('User', 'name')")
        ).await.expect("Fulltext index error");
        // Create exact match index on username
        connection.graph_query_void(
            "social",
            query!("CREATE INDEX ON :User(name)")
        ).await.expect("Normal index error");
        Ok(rocket)
    } else {
        Err(rocket)
    }
}

impl Redis {
    pub async fn register_user(&mut self, user: &User<'_>) -> bool {
        let res = self
            .graph_query::<()>(query!("\
            MERGE (u:User {name: $name})
            ON CREATE SET u.password = $password, u.follower = 0, u.follows = 0",
                {
                    "name" => user.name, "password" => &user.hash_pw()
                }
            ))
            .await
            .unwrap();
        res.get_statistic(GraphStatistic::NodesCreated) == Some(1.0)
    }

    pub async fn get_pw_by_name(&mut self, name: &str) -> Option<String> {
        self.graph_query::<(Option<String>,)>(query!("\
            MATCH (u:User {name: $name})
            RETURN u.password AS pw",
            {
                "name" => name
            }, true
        ))
        .await
        .unwrap()
        .data
        .pop()?
        .0
    }

    pub async fn follow(&mut self, user: &str, follow: &str) {
        self.graph_query::<()>(query!(
            "\
            MATCH (u:User {name: $user})
            MATCH (f:User {name: $follow})
            MERGE (u)-[:follows]->(f)
            SET f.follower = f.follower + 1, u.follows = u.follows + 1",
            {"user" => user, "follow" => follow}
        ))
        .await
        .unwrap();
    }

    pub async fn unfollow(&mut self, user: &str, follow: &str) {
        self.graph_query::<()>(query!(
            "\
            MATCH (you:User {name: $user})-[f:follows]->(other:User {name: $follow})
            SET other.follower = other.follower - 1, you.follows = u.follows - 1
            DELETE f",
            {"user" => user, "follow" => follow}
        ))
        .await
        .unwrap();
    }

    pub async fn tweet(&mut self, user: &str, tweet: &str) -> Tweet {
        self.graph_query::<Tweet>(query!("\
            MATCH (u:User {name: $user})
            CREATE (u)-[rel:tweets {published: timestamp()}]->(t:Tweet {content: $tweet, id: $id, likes: 0})
            RETURN rel.published, u, t, null, null, false, false, 0
            ",
            {"user" => user, "tweet" => tweet, "id" => Uuid::new_v4().to_string()}
        ))
        .await
        .expect("Database not running")
        .data
        .pop()
        .expect("User not in db")
    }

    pub async fn answer_tweet(&mut self, user: &str, answer: &str, answer_to: &str) -> bool {
        self.graph_query::<()>(query!("\
            MATCH (u:User {name: $user}), (t:Tweet)
            WHERE t.id = $answer_to
            CREATE (u)-[:tweets {published: timestamp()}]->(:Tweet {content: $tweet, id: $id, likes: 0})-[:answer]->(t)",
            {
                "answer_to" => answer_to,
                "id" => Uuid::new_v4().to_string(),
                "tweet" => answer,
                "user" => user
            }
        )).await
        .unwrap()
        .get_statistic(GraphStatistic::NodesCreated) == Some(1.0)
    }

    pub async fn retweet(&mut self, user: &str, tweetid: &str) -> bool {
        self.graph_query::<()>(query!("\
            MATCH (u:User {name: $user})
            MATCH (t:Tweet {id: $id})
            MERGE (u)-[:retweets {published: timestamp()}]->(t)",
            {
                "id" => tweetid,
                "user" => user
            }
        ))
        .await
        .unwrap()
        .get_statistic(GraphStatistic::RelationshipsCreated) == Some(1.0)
    }
    pub async fn unretweet(&mut self, user: &str, tweetid: &str) -> bool {
        self.graph_query::<()>(query!("\
            MATCH (:User {name: $user})-[r:retweets]->(:Tweet {id: $id})
            DELETE r",
            {
                "id" => tweetid,
                "user" => user
            }
        ))
        .await
        .unwrap()
        .get_statistic(GraphStatistic::RelationshipsDeleted) == Some(1.0)
    }

    pub async fn like(&mut self, user: &str, tweetid: &str) -> bool {
        self.graph_query::<()>(query!("\
            MATCH (u:User {name: $user})
            MATCH (t:Tweet {id: $id})
            MERGE (u)-[:likes]->(t)
            ON CREATE SET t.likes = t.likes+1",
            {
                "id" => tweetid,
                "user" => user
            }
        ))
        .await
        .unwrap()
        .get_statistic(GraphStatistic::RelationshipsCreated) == Some(1.0)
    }

    pub async fn unlike(&mut self, user: &str, tweetid: &str) -> bool {
        self.graph_query::<()>(query!("\
            MATCH (:User {name: $user})-[l:likes]->(t:Tweet {id: $id})
            SET t.likes = t.likes-1
            DELETE l",
            {
                "id" => tweetid,
                "user" => user
            }
        ))
        .await
        .unwrap()
        .get_statistic(GraphStatistic::RelationshipsDeleted) == Some(1.0)
    }

    pub async fn get_answers(&mut self, id: &str, user: &str) -> Vec<Tweet> {
        self.graph_query(query!("\
            MATCH (t:Tweet {id: $id})<-[:tweets]-(answer_to:User)
            OPTIONAL MATCH (t)<-[:answer]-(tweets:Tweet)
            WITH *, collect(tweets) AS tweets
            UNWIND tweets AS tweet
            MATCH (o:User)-[rel:tweets]->(tweet)
            OPTIONAL MATCH (u:User {name: $user})
            OPTIONAL MATCH p=(u)-[:likes]->(tweet)
            OPTIONAL MATCH q=(u)-[:retweets]->(tweet)
            OPTIONAL MATCH (:Tweet)-[answers:answer]->(tweet)
            Return rel.published, o, tweet, answer_to, null, exists(p), exists(q), count(answers)",
            {
                "id" => id,
                "user" => user
            }, true
        ))
        .await
        .unwrap()
        .data
    }

    pub async fn delete_user(&mut self, user: &str) {
        self.graph_query::<()>(query!("\
            MATCH (u:User {name: $user})
            DETACH DELETE u",
            {
                "user" => user
            }
        ))
        .await
        .unwrap();
    }

    pub async fn get_user(&mut self, user: &str, viewer: &str) -> Option<(types::User, bool)> {
        self.graph_query(query!("\
            MATCH (u:User {name: $user})
                OPTIONAL MATCH (viewer:User {name: $viewer})
                OPTIONAL MATCH p=(viewer)-[:follows]->(u)
                RETURN u, exists(p)",
                {
                    "user" => user,
                    "viewer" => viewer
                }
            ))
            .await
            .unwrap()
            .data
            .pop()
    }

    pub async fn search_user(&mut self, query: &str) -> Vec<types::User> {
        self.graph_query(query!("\
            CALL db.idx.fulltext.queryNodes('User', $query)
            YIELD node
            RETURN node
            ORDER BY node.follower
            LIMIT 10",
            {
                "query" => format!("{query}*")
            }
        ))
        .await
        .unwrap()
        .data
        .into_iter()
        .map(|(user,)| user)
        .collect()
    }

    pub async fn get_tweet(&mut self, id: &str, viewer: &str) -> Option<Tweet> {
        self.graph_query(query!("\
            MATCH (author:User)-[rel:tweets]->(tweet:Tweet {id: $id})
            OPTIONAL MATCH (viewer:User {name: $viewer})
            OPTIONAL MATCH (tweet)-[:answer]->(:Tweet)<-[:tweets]-(answer_to:User)
            OPTIONAL MATCH p=(viewer)-[:likes]->(tweet)
            OPTIONAL MATCH q=(viewer)-[:retweets]->(tweet)
            OPTIONAL MATCH (:Tweet)-[answers:answer]->(tweet)
            RETURN rel.published, author, tweet, answer_to, null, exists(p), exists(q), count(answers)",
            {
                "id" => id,
                "viewer" => viewer
            }
        ))
        .await
        .unwrap()
        .data
        .pop()
    }

    pub async fn get_timeline(&mut self, user: &str) -> Vec<Tweet> {
        self.graph_query(query!("\
            MATCH (user:User {name: $name})
            OPTIONAL MATCH (user)-[tweetRel:tweets]->(tweet:Tweet)
            WITH user, { when: tweetRel.published, author: user, tweet: tweet, retweeted_by: null } AS a
            OPTIONAL MATCH (user)-[tweetRel:retweets]->(tweet:Tweet)<-[:tweets]-(author:User)
            WITH user, a, { when: tweetRel.published, author: author, tweet: tweet, retweeted_by: user } AS b
            OPTIONAL MATCH (user)-[:follows]->(author:User)-[tweetRel:tweets]->(tweet:Tweet)
            WHERE NOT (tweet)-[:answer]->()
            WITH user, a, b, { when: tweetRel.published, author: author, tweet: tweet, retweeted_by: user } AS c
            OPTIONAL MATCH (user)-[:follows]->(retweeted_by:User)-[tweetRel:retweets]->(tweet:Tweet)<-[:tweets]-(author:User)
            WITH user, a, b, c, { when: tweetRel.published, author: author, tweet: tweet, retweeted_by: retweeted_by } AS d
            WITH user, (collect(a)+collect(b)+collect(c)+collect(d)) AS infos
            UNWIND infos AS info
            WITH user, info.tweet AS tweet, info.when AS when, info.author AS author, info.retweeted_by AS retweeted_by
            MATCH (tweet)<-[tweetRel:tweets]-()
            OPTIONAL MATCH (tweet)-[:answer]->(:Tweet)<-[:tweets]-(answer_to:User)
            OPTIONAL MATCH (tweet)<-[replies:answer]-()
            OPTIONAL MATCH liked=(user)-[:likes]->(tweet)
            OPTIONAL MATCH retweeted=(user)-[:retweets]->(tweet)
            RETURN tweetRel.published, author, tweet, answer_to, retweeted_by, exists(liked), exists(retweeted), count(replies)
            ORDER BY when DESC SKIP $skip LIMIT 25",
            {
                "name" => user,
                "skip" => 0
            }, true
        ))
        .await
        .unwrap()
        .data
    }

    pub async fn get_user_tweets(&mut self, user: &str, viewer: &str) -> Vec<Tweet> {
        self.graph_query(query!("\
            MATCH (u:User {name: $user})
            OPTIONAL MATCH (u)-[rel:tweets]-(t:Tweet)
            WITH u, {when: rel.published, author: u, tweet: t, retweeted_by: null} AS a
            OPTIONAL MATCH (u)-[rel:retweets]->(someRetweets)<-[:tweets]-(author:User)
            WITH u, a, {when: rel.published, author: author, tweet: someRetweets, retweeted_by: u} AS b
            WITH u, (collect(a)+collect(b)) AS infos
            UNWIND infos AS info
            WITH u, info.tweet AS tweet, info.author AS author, info.retweeted_by AS retweeted_by, info.when as when
            MATCH (tweet)<-[tweetRel:tweets]-()
            OPTIONAL MATCH (viewer:User {name: $viewer})
            OPTIONAL MATCH (tweet)-[:answer]->(:Tweet)<-[:tweets]-(answer_to:User)
            OPTIONAL MATCH p=(viewer)-[:likes]->(tweet)
            OPTIONAL MATCH q=(viewer)-[:retweets]->(tweet)
            OPTIONAL MATCH (:Tweet)-[answers:answer]->(tweet)
            Return tweetRel.published, author, tweet, answer_to, retweeted_by, exists(p), exists(q), count(answers)
            ORDER BY when DESC SKIP $skip LIMIT 25",
            {
                "user" => user,
                "viewer" => viewer,
                "skip" => 0
            }, true
        )).await.unwrap().data
    }

    #[inline]
    pub async fn graph_query<RT: FromGraphValue>(
        &mut self,
        query: GraphQuery,
    ) -> RedisResult<GraphResponse<RT>> {
        self.connection.graph_query("social", query).await
    }
}
