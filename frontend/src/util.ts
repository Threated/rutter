import type { User } from "./types";
import UserHover from "./lib/UserHover.svelte";
import type { SvelteComponent } from "svelte";
import { viewedUser } from "./store";
import { navigate } from "svelte-navigator";

export function hoverUser(element: HTMLElement, user: User) {
    let hoverUser: SvelteComponent = new UserHover({
        props: { user },
        target: element
    });
    element.addEventListener("mouseenter", () => {
        hoverUser.$set({
            isHovered: true
        })
    })
    element.addEventListener("mouseleave", () => {
        hoverUser.$set({
            isHovered: false
        })
    })
    return {
        update(params: User) {
            hoverUser.$set({
                user: params
            })
        }
    }
}

export function readCookie(name: string): string | null {
    var nameEQ = name + "=";
    var ca = document.cookie.split(';');
    for(var i=0;i < ca.length;i++) {
        var c = ca[i];
        while (c.charAt(0)==' ') c = c.substring(1,c.length);
        if (c.indexOf(nameEQ) == 0) return c.substring(nameEQ.length,c.length);
    }
    return null;
}

export function auth_fetch(url: string, options: RequestInit) {
    options.headers = {
        "Content-Type": "application/json",
        "Authorization": `Bearer ${readCookie("jwt")}`,
        ...options.headers
    };
    return fetch(url, options)
}

export function visitUser(user: User) {
    viewedUser.set(user);
    navigate(`/u/${user.name}`);
}
