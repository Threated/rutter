# Rutter
This is a project is a Twitter clone which implements all the basic Twitter features like tweeting, retweeting, liking and following other users. It can also serve at Timeline like where you see all the Tweets from the people you follow. You can discover users by searching for them or seeing their tweets on you Timeline.

## Implementation
The frontend is written in [Svelte](https://svelte.dev/) using [svelte-navigator](https://github.com/mefechoel/svelte-navigator) to create a SPA which interfaces with a REST api to fetch all the requiered information.\
Authorization is handled with [JWT](https://jwt.io/)s which are served by the Rust framework [Rocket](https://rocket.rs). This can also serves the frontend, but for development the frontend can be run as its own Microservice via [Node](https://nodejs.org/) to enable HMR see [Local Developement](https://github.com/Threated/rutter#Local-developement).
The server uses a [Redisgraph](https://redis.io/docs/stack/graph/) Database to store all tweets and users. The server interfaces with the [redisgraphio](https://github.com/Threated/redisgraphio) crate which I wrote to enable simple graph queries to access the database.

## Run with Docker

    docker compose up
This will serve the app at [localhost:8000](http://localhost:8000)

## Dev requirements

- Docker
- Rust
- cargo
- npm


## Local developement
Run the database:

    docker run -p 6379:6379 -d redislabs/redisgraph:latest

Run the rust backend server:

    cargo run

Setup the frontend in a new terminal:

    cd frontend
    npm install

To get HMR run:

    npm run dev
This will serve the frontend at [localhost:3000](http://localhost:3000) and enable HMR.

If you don't need HMR or want to test the production enviroment behavior build the static html:

    npm run build
Now once we run `cargo run` in the root directory again the Rust server will serve the single page application at [localhost:8000](http://localhost:8000) without requireing a node server


