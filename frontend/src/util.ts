import type { User } from "./types";
import UserHover from "./lib/UserHover.svelte";
import type { SvelteComponent } from "svelte";

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
}
