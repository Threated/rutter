import {writable} from "svelte/store";
import { readCookie } from "./util";
import type { Tweet, User } from "./types";


export const isAuthenticated = writable<boolean>(!!readCookie("jwt"));

export const viewedUser = writable<User | null>(null);
export const viewedTweet = writable<Tweet | null>(null);
