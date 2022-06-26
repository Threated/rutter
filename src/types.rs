use redisgraphio::{FromGraphValue, GraphValue, from_graph_value, Node, PropertyAccess};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Tweet {
    content: String,
    published: u32,
    id: String,
    likes: u32
}

impl FromGraphValue for Tweet {
    fn from_graph_value(value: GraphValue) -> redis::RedisResult<Self> {
        let node: Node = from_graph_value(value)?;
        let (content, published, id, likes) = node.into_property_values()?;
        Ok(Tweet {
            content,
            published,
            id,
            likes
        })
    }
}
