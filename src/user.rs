use crate::{auth::Authenticated, db::Redis, responders::JsonRes, types::{Tweet, self}};
use rocket::{
    http::Status,
    serde::{
        json::{serde_json::json, Json},
        Deserialize,
    },
};

#[derive(Deserialize)]
pub struct TweetInfo {
    pub content: String,
    pub id: Option<String>,
}

#[post("/follow/<follow>", format = "json")]
async fn follow(follow: String, user: Authenticated, mut db: Redis) -> JsonRes {
    if follow == user.name {
        return JsonRes::from((Status::UnprocessableEntity, "Cant follow yourself"))
    }
    db.follow(&user.name, &follow).await;
    JsonRes::from(format!("{} followed {}", user.name, follow))
}

#[post("/unfollow/<unfollow>", format = "json")]
async fn unfollow(unfollow: String, user: Authenticated, mut db: Redis) -> JsonRes {
    db.unfollow(&user.name, &unfollow).await;
    JsonRes::from(format!("{} unfollowed {}", user.name, unfollow))
}

#[post("/retweet?<id>", format = "json")]
async fn retweet(id: &str, user: Authenticated, mut db: Redis) -> JsonRes {
    JsonRes::from((
        Status::Ok,
        json!( {"success": db.retweet(&user.name, id).await } ),
    ))
}

#[post("/unretweet?<id>", format = "json")]
async fn unretweet(id: &str, user: Authenticated, mut db: Redis) -> JsonRes {
    JsonRes::from((
        Status::Ok,
        json!( {"success": db.unretweet(&user.name, id).await } ),
    ))
}
#[post("/like?<id>", format = "json")]
async fn like(id: &str, user: Authenticated, mut db: Redis) -> JsonRes {
    JsonRes::from((
        Status::Ok,
        json!( {"success": db.like(&user.name, id).await } ),
    ))
}

#[post("/unlike?<id>", format = "json")]
async fn unlike(id: &str, user: Authenticated, mut db: Redis) -> JsonRes {
    JsonRes::from((
        Status::Ok,
        json!( {"success": db.unlike(&user.name, id).await } ),
    ))
}

#[post("/tweet", format = "json", data = "<tweet>")]
async fn tweet(tweet: Json<TweetInfo>, user: Authenticated, mut db: Redis) -> JsonRes<Tweet> {
    let tweet = &*tweet.content;
    JsonRes((Status::Ok, Json(db.tweet(&user.name, tweet).await)))
}

#[post("/answer", format = "json", data = "<tweet>")]
async fn answer(tweet: Json<TweetInfo>, user: Authenticated, mut db: Redis) -> JsonRes {
    if tweet.id.is_none() {
        return JsonRes::from((
            Status::UnprocessableEntity,
            "Answer requires id of root tweet",
        ));
    }
    if db
        .answer_tweet(&user.name, &tweet.content, tweet.id.as_ref().unwrap())
        .await
    {
        JsonRes::from(format!("{} tweeted {}", user.name, tweet.content))
    } else {
        JsonRes::from((Status::NotFound, "The tweet answered does not exist"))
    }
}

#[get("/answers?<id>", format = "json", rank = 1)]
async fn answers(id: &str, user: Authenticated, mut db: Redis) -> JsonRes<Vec<Tweet>> {
    JsonRes((Status::Ok, Json(db.get_answers(id, &user.name).await)))
}

#[get("/answers?<id>", format = "json", rank = 2)]
async fn answers_no_auth(id: &str, mut db: Redis) -> JsonRes<Vec<Tweet>> {
    JsonRes((Status::Ok, Json(db.get_answers(id, &"").await)))
}

#[get("/tweets/<user>", format = "json", rank = 1)]
async fn user(user: &str, viewer: Authenticated, mut db: Redis) -> JsonRes<Vec<Tweet>> {
    JsonRes((
        Status::Ok,
        Json(db.get_user_tweets(user, &viewer.name).await),
    ))
}

#[get("/tweets/<user>", format = "json", rank = 2)]
async fn user_no_auth(user: &str, mut db: Redis) -> JsonRes<Vec<Tweet>> {
    JsonRes((Status::Ok, Json(db.get_user_tweets(user, &"").await)))
}

#[get("/timeline", format = "json")]
async fn timeline(user: Authenticated, mut db: Redis) -> JsonRes<Vec<Tweet>> {
    JsonRes((Status::Ok, Json(db.get_timeline(&user.name).await)))
}

#[get("/info/<username>", format = "json", rank=2)]
async fn info(username: &str, mut db: Redis) -> JsonRes<(types::User, bool)> {
    let user = db.get_user(username, "").await;
    if let Some(user) = user {
        JsonRes((Status::Ok, Json(user)))
    } else {
        JsonRes((Status::NotFound, Json((types::User::default(), false))))
    }
}

#[get("/info/<username>", format = "json", rank=1)]
async fn info_auth(username: &str, viewer: Authenticated, mut db: Redis) -> JsonRes<(types::User, bool)> {
    let user = db.get_user(username, &viewer.name).await;
    if let Some(user) = user {
        JsonRes((Status::Ok, Json(user)))
    } else {
        JsonRes((Status::NotFound, Json((types::User::default(), false))))
    }
}

#[get("/tweet?<id>", format = "json", rank = 2)]
async fn get_tweet(id: &str, mut db: Redis) -> JsonRes<Tweet> {
    let tweet = db.get_tweet(id, "").await;
    if let Some(tweet) = tweet {
        JsonRes((Status::Ok, Json(tweet)))
    } else {
        JsonRes((Status::NotFound, Json(Tweet::default())))
    }
}

#[get("/tweet?<id>", format = "json", rank = 1)]
async fn get_tweet_auth(id: &str, user: Authenticated, mut db: Redis) -> JsonRes<Tweet> {
    let tweet = db.get_tweet(id, &user.name).await;
    if let Some(tweet) = tweet {
        JsonRes((Status::Ok, Json(tweet)))
    } else {
        JsonRes((Status::NotFound, Json(Tweet::default())))
    }
}

#[delete("/", format = "json")]
async fn delete(user: Authenticated, mut db: Redis) -> JsonRes {
    db.delete_user(&user.name).await;
    JsonRes::from(format!("Deleted {}", user.name))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        follow,
        unfollow,
        tweet,
        answer,
        delete,
        timeline,
        like,
        unlike,
        retweet,
        unretweet,
        answers,
        answers_no_auth,
        user,
        user_no_auth,
        info,
        info_auth,
        get_tweet,
        get_tweet_auth,
    ]
}
