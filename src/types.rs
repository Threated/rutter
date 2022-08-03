use redisgraphio::{FromGraphValue, GraphValue, from_graph_value, Node, PropertyAccess};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct User {
    name: String,
    follows: u64,
    follower: u64
}



#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Tweet {
    author: User,
    content: String,
    id: String,
    published: u64,
    likes: u64,
    replies: u64,
    answer_to: Option<User>,
    retweet_by: Option<User>,
    liked: bool,
    retweeted: bool
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
        let (published, author, tweet, answer_to, retweet_by, liked, retweeted, replies): (_, _, Node, _, _, _, _, _) = from_graph_value(value)?;
        let (content, id, likes) = tweet.into_property_values()?;
        Ok(Tweet {
            content,
            author,
            published,
            id,
            likes,
            answer_to,
            retweet_by,
            liked,
            retweeted,
            replies
        })
    }
}
