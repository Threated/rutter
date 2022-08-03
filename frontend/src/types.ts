export type User = {
    name: string,
    follows: number | string,
    follower: number | string
}

export type Tweet = {
        author: User,
        content: string,
        published: number,
        id: string,
        answer_to: User | null,
        retweet_by: User | null,
        likes: number,
        replies: number
        liked: boolean,
        retweeted: boolean,
}

