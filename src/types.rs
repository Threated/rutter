use redisgraphio::{FromGraphValue, GraphValue, from_graph_value, Node, PropertyAccess};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    follows: u32,
    follower: u32
}

#[derive(Serialize, Deserialize)]
pub struct Tweet {
    author: User,
    content: String,
    published: u32,
    id: String,
    likes: u32
}

impl FromGraphValue for User {
    fn from_graph_value(value: GraphValue) -> redis::RedisResult<Self> {
        let author: Node = from_graph_value(value)?;
        let (name, _, follower, follows): (_, (), _, _) = author.into_property_values()?;
        Ok(User {
            name,
            follower,
            follows
        })
    }
}

impl FromGraphValue for Tweet {
    fn from_graph_value(value: GraphValue) -> redis::RedisResult<Self> {
        let (author, tweet): (User, Node) = from_graph_value(value)?;
        let (content, published, id, likes) = tweet.into_property_values()?;
        Ok(Tweet {
            content,
            author,
            published,
            id,
            likes
        })
    }
}
