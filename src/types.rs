use redisgraphio::{FromGraphValue, GraphValue, from_graph_value, Node, PropertyAccess};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    name: String,
    follows: u64,
    follower: u64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tweet {
    author: User,
    content: String,
    published: u64,
    id: String,
    likes: u64,
    liked: bool,
    retweeted: bool,
    replies: u64
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
        let (author, tweet, liked, retweeted, replies): (_, Node, _, _, _) = from_graph_value(value)?;
        let (content, published, id, likes) = tweet.into_property_values()?;
        Ok(Tweet {
            content,
            author,
            published,
            id,
            likes,
            liked,
            retweeted,
            replies
        })
    }
}
