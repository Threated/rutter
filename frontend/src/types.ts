export type User = {
    name: string,
    follows: number,
    follower: number
}

export type Tweet = {
    author: User,
    content: string,
    published: number,
    id: string,
    likes: number
}
