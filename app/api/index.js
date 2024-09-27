const fs = require('fs');
const path = require('path');
const { get } = require('@vercel/edge-config');
const { kv } = require("@vercel/kv");

function getCookie(name, string) {
    let cookieString= RegExp(name + "=[^;]+").exec(string);
    return decodeURIComponent(!!cookieString ? cookieString.toString().replace(/^[^=]+./,"") : "");
}

function buf2hex(buffer) {
    return [...new Uint8Array(buffer)]
        .map(x => x.toString(16).padStart(2, '0'))
        .join('');
}

module.exports = {
    config: {
        runtime: 'edge'
    },
    GET: async (request) => {
        let url = new URL(request.url);
        let config = await get("wishing-star");

        if (url.searchParams.has("code")) {
            console.log("Got authentication callback code from server");
            let res = await fetch("https://account.equestria.dev/hub/api/rest/oauth2/token", {
                method: "POST",
                headers: {
                    'Authorization': "Basic " + btoa(config['id'] + ":" + config['secret']),
                    'Content-Type': 'application/x-www-form-urlencoded',
                    'Accept': 'application/json'
                },
                body: "grant_type=authorization_code&redirect_uri=" + encodeURIComponent(config['redirect']) + "&code=" + url.searchParams.get("code")
            });
            let data = await res.json();

            if (data["access_token"]) {
                console.log("Got access token from authentication server");
                let res = await fetch("https://account.equestria.dev/hub/api/rest/users/me", {
                    headers: {
                        'Authorization': "Bearer " + data["access_token"],
                        'Accept': 'application/json'
                    }
                });
                let userData = await res.json();
                let allowed = config['allowed'];

                if (!allowed.includes(userData['id'])) {
                    console.log("User is not allowed: " + userData['id']);
                    return new Response(null, {
                        status: 403
                    });
                }

                let token = buf2hex(crypto.getRandomValues(new Uint8Array(128)));
                console.log("Adding session token");
                await kv.set(token, {
                    date: new Date().getTime(),
                    userData
                });

                console.log("Refreshing page");
                return new Response(null, {
                    status: 307,
                    headers: {
                        Location: "/",
                        'Set-Cookie': "wishlist_token=" + token + "; Path=/; HttpOnly; Expires=" + new Date(new Date().getTime() + (86400 * 730000))
                    }
                });
            }
        }

        if (request.headers.has("Cookie")) {
            console.log("Found cookies");
            let cookie = request.headers.get("Cookie");
            let token = getCookie("wishlist_token", cookie);

            if (token.trim() !== "") {
                console.log("Found session token");
                let tokenData = await kv.get(token);

                if (tokenData) {
                    console.log("Fetched session token data");
                    if (new Date().getTime() - new Date(tokenData.date).getTime() >= 604800000) {
                        console.log("Removing expired session");
                        await kv.set(token, {
                            date: 0,
                            userData: null
                        });
                    } else {
                        console.log("Sending page: " + path.join(process.cwd(), "app.html"));
                        return new Response(fs.readFileSync(path.join(process.cwd(), "app.html")));
                    }
                }
            }
        }

        console.log("User does not have a valid session");
        return new Response(null, {
            status: 307,
            headers: {
                Location: "https://account.equestria.dev/hub/api/rest/oauth2/auth?client_id=" + config['id'] + "&response_type=code&redirect_uri=" + encodeURIComponent(config['redirect']) + "&scope=Hub&request_credentials=default&access_type=offline"
            }
        });
    }
}
