const fs = require('fs');
const child_process = require("node:child_process");
const YAML = require('yaml');

let data;

try {
    data = YAML.parse(fs.readFileSync("./config.yml").toString());
} catch (e) {
    data = YAML.parse(fs.readFileSync("./config.default.yml").toString());
}

fs.writeFileSync("./data/config.json", JSON.stringify(data, null, 2));

let lists = data["custom"] ?? {};

(async () => {
    if (data["steam"]["enable"]) {
        let obj = {};

        for (let entry of Object.entries(data["steam"]["notes"] ?? {})) {
            obj["item-steam-" + entry[0]] = entry[1];
        }

        fs.writeFileSync("./data/notes.json", JSON.stringify(obj, null, 2));

        lists["steam"] = {
            recommend: data['steam']['index'],
            title: data['steam']['name'],
            items: []
        };

        const steam = Object.entries(await (await fetch("https://store.steampowered.com/wishlist/id/" + data['steam']['id'] + "/wishlistdata/")).json())
            .map((i) => {
                i[1]["id"] = i[0];
                return i[1];
            })
            .sort((a, b) => a.priority - b.priority);

        for (let game of steam) {
            // noinspection JSCheckFunctionSignatures
            lists["steam"]["items"].push({
                _id: game["id"],
                image: game["capsule"],
                link: "https://store.steampowered.com/app/" + game["id"],
                name: game["name"],
                score: (((game["win"] ?? 0) + (game["linux"] ?? 0) + parseInt(game["deck_compat"]) + (game["reviews_percent"] / 100)) - (game["prerelease"] ?? 0)) / 5,
                tags: game["tags"],
                price: parseFloat((game["subs"][0] ?? { price: '-100' })["price"]),
                date: game["release_string"],
                source: "Steam",
                unit: "% " + data['currency']
            });
        }
    }

    fs.writeFileSync("./data/lists.json", JSON.stringify(lists, null, 2));
    fs.writeFileSync("./app/app.html", child_process.execSync("php index.php", { cwd: "./public" }).toString());

    if (data['vercel']) {
        child_process.execSync("vercel --prod", { cwd: "./app", stdio: "inherit" });
    }
})();
