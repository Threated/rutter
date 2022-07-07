import {writable} from "svelte/store";
import { readCookie } from "./main";


export const isAuthenticated = writable<boolean>(!!readCookie("isAuthenticated"));

