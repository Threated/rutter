use crate::{auth::Authenticated, db::Redis, responders::JsonRes, types::Tweet};
use rocket::{
    http::Status,
    serde::{json::Json, Deserialize},
};
use uuid::Uuid;

#[derive(Deserialize)]
struct Follow {
    user: String,
}

#[derive(Deserialize)]
pub struct TweetInfo {
    pub content: String,
    pub answer_id: Option<String>,
}

#[post("/follow", format = "json", data = "<follow>")]
async fn follow(follow: Json<Follow>, user: Authenticated, mut db: Redis) -> JsonRes {
    let follow = &*follow.user;
    db.follow(&user.name, follow).await;
    JsonRes::from(format!("{} followed {}", user.name, follow))
}

#[post("/unfollow", format = "json", data = "<unfollow>")]
async fn unfollow(unfollow: Json<Follow>, user: Authenticated, mut db: Redis) -> JsonRes {
    let unfollow = &*unfollow.user;
    db.unfollow(&user.name, unfollow).await;
    JsonRes::from(format!("{} unfollowed {}", user.name, unfollow))
}

#[post("/tweet", format = "json", data = "<tweet>")]
async fn tweet(tweet: Json<TweetInfo>, user: Authenticated, mut db: Redis) -> JsonRes {
    let tweet = &*tweet.content;
    db.tweet(&user.name, tweet).await;
    JsonRes::from(format!("{} tweeted {}", user.name, tweet))
}

#[post("/answer", format = "json", data = "<tweet>")]
async fn answer(tweet: Json<TweetInfo>, user: Authenticated, mut db: Redis) -> JsonRes {
    if tweet.answer_id.is_none() {
        return JsonRes::from((
            Status::UnprocessableEntity,
            "Answer requires id of root tweet",
        ));
    }
    let id = match Uuid::parse_str(tweet.answer_id.as_ref().unwrap()) {
        Err(_) => return JsonRes::from((
            Status::UnprocessableEntity,
            "Invalid id",
        )),
        Ok(id) => id
    };
    if db.answer_tweet(&user.name, &tweet.content, id).await {
        JsonRes::from(format!("{} tweeted {}", user.name, tweet.content))
    } else {
        JsonRes::from((
            Status::NotFound,
            "The tweet answered does not exist",
        ))
    }
}

#[get("/timeline", format = "json")]
async fn timeline(user: Authenticated, mut db: Redis) -> JsonRes<Vec<Tweet>> {
    JsonRes((
        Status::Ok,
        Json(db.get_timeline(&user.name).await)
    ))
}

#[delete("/", format = "json")]
async fn delete(user: Authenticated, mut db: Redis) -> JsonRes {
    db.delete_user(&user.name).await;
    JsonRes::from(format!("Deleted {}", user.name))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![follow, unfollow, tweet, answer, delete, timeline]
}
