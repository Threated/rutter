import {writable} from "svelte/store";
import { readCookie } from "./util";
import type { Tweet, User } from "./types";


export const isAuthenticated = writable<boolean>(!!readCookie("jwt"));

// Decode jwt data and save it to store
let b64 = readCookie("jwt")?.split(".")[1];
let username = "";   
if (b64) {
    username = JSON.parse(atob(b64)).name;
}
export const loggedinUsername = writable<String>(username);

export const viewedUser = writable<User | null>(null);
export const viewedTweet = writable<Tweet | null>(null);
