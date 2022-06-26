use redisgraphio::{FromGraphValue, GraphValue, from_graph_value, Node, PropertyAccess};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Tweet {
    content: String,

}

impl FromGraphValue for Tweet {
    fn from_graph_value(value: GraphValue) -> redis::RedisResult<Self> {
        let node: Node = from_graph_value(value)?;
        let (content, ) = node.into_property_values()?;
        Ok(Tweet {
            content
        })
    }
}
