import './app.css'
import App from './App.svelte'
import { isAuthenticated } from './store'

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

const app = new App({
  target: document.getElementById('app')
})


if (readCookie("session_id")) {
    console.log("asdfsdfasdfasdfasdfadsfasdfasdfasdfasdfsdfaf")
    isAuthenticated.set(true)
}

// https://github.com/LoginRadius/engineering-blog-samples/tree/master/Svelte/SvelteAuthApp/src

export default app

