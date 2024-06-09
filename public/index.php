<?php

$notes = json_decode(file_get_contents("../data/notes.json"), true);
$lists = json_decode(file_get_contents("../data/lists.json"), true);
$config = json_decode(file_get_contents("../data/config.json"), true);

?>
<!doctype html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport"
          content="width=device-width, user-scalable=no, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="ie=edge">
    <title><?= $config["name"] ?>'s Wishing Star</title>
    <link href="/assets/bootstrap.min.css" rel="stylesheet">
    <script src="/assets/bootstrap.bundle.min.js"></script>

    <link rel="shortcut icon" href="/assets/logo.png" type="image/png">
    <style>
        body:not(.show-nsfw) .list-item-nsfw, body:not(.show-nsfw) .list-container-nsfw {
            display: none;
        }

        .list-item {
            display: grid;
            grid-template-columns: 8px 1fr 3fr 150px;
            grid-gap: 20px;
            border-radius: 0 !important;
        }

        @media (max-width: 700px) {
            .list-item {
                grid-template-columns: 1fr !important;
            }
        }

        .custom-badge {
            background-color: rgba(var(--custom-badge-base, (0, 0, 0)), 1);
        }

        @media (prefers-color-scheme: dark) {
            .custom-badge {
                background-color: rgba(var(--custom-badge-base, (0, 0, 0)), .5);
            }
        }

        #filter-results input {
            display: none;
        }
    </style>
</head>
<body data-bs-theme="light">
<div class="container">
    <br><br>
    <h1><?= $config["name"] ?>'s Wishing Star</h1>

    <div style="display: grid; grid-template-columns: 1fr 1fr; grid-gap: 10px;">
        <div>
            <!--suppress HtmlFormInputWithoutLabel -->
            <select onchange="changeOrder();" class="form-select" id="sort">
                <option value="cro">Sort by <?= $config["name"] ?>'s order</option>
                <option value="cnt">Sort by <?= $config["name"] ?>'s order (no categories)</option>
                <option value="plh">Sort by price (low to high)</option>
                <option value="phl">Sort by price (high to low)</option>
                <option value="alh">Sort by Predicted appreciation (low to high)</option>
                <option value="ahl">Sort by Predicted appreciation (high to low)</option>
                <option value="rec">Sort by recommendation</option>
            </select>
            <span style="margin-bottom: 30px; margin-top: 10px; display: inline-block;" class="small text-muted">Revert to "Sort by <?= $config["name"] ?>'s order" to check items and add them to your list.</span>
        </div>
        <!--suppress HtmlFormInputWithoutLabel -->
        <input onchange="updateBudget();" type="number" style="margin-bottom: 30px; height: 38px;" placeholder="Enter your budget here" id="budget" class="form-control">
    </div>

    <div id="budget-outer" style="margin-bottom: 30px; display: none;">
        <h3>Your budget</h3>
        <p style="display: grid; grid-template-columns: 1fr max-content; margin-bottom: 10px;">
            <span>You allocated <span id="budget-value">0.00</span> <?= $config["currency"] ?> and are currently using <span id="budget-usage">0.00</span> <?= $config["currency"] ?> (<span id="budget-usage-percent"></span> %)</span>
            <span><b id="budget-limit">0.00 <?= $config["currency"] ?> left</b></span>
        </p>

        <div class="progress" style="margin-bottom: 10px;">
            <div class="progress-bar" id="budget-progress"></div>
            <div class="progress-bar bg-danger" id="budget-progress-red"></div>
        </div>

        <div style="display: grid; grid-template-columns: 1fr 1fr;">
            <div class="form-check" style="margin-bottom: 10px;">
                <input class="form-check-input" type="checkbox" id="show-only-selected" onchange="updateBudget()">
                <label class="form-check-label" for="show-only-selected">
                    Only show checked items
                </label>
            </div>
            <div class="form-check" style="margin-bottom: 10px;">
                <input class="form-check-input" type="checkbox" id="hide-oob" onchange="updateBudget()">
                <label class="form-check-label" for="hide-oob">
                    Hide out of budget items
                </label>
            </div>
        </div>
    </div>

    <?php if ($config["faq"]): ?>
    <h3>Frequently asked questions</h3>
    <div class="accordion" id="faq" style="margin-bottom: 15px;">
        <div class="accordion-item">
            <h2 class="accordion-header">
                <button class="accordion-button collapsed" type="button" data-bs-toggle="collapse" data-bs-target="#faq-1">
                    What is the "Predicted appreciation"?
                </button>
            </h2>
            <div id="faq-1" class="accordion-collapse collapse" data-bs-parent="#faq">
                <div class="accordion-body">
                    For some items, such as Steam games, a score is calculated based on user ratings and compatibility with the hardware we own (in the case of Steam games) or similarity with other products we own (in other cases).
                    Therefore, this score indicates how likely we are to like a specific product if we do acquire it.
                </div>
            </div>
        </div>
        <div class="accordion-item">
            <h2 class="accordion-header">
                <button class="accordion-button collapsed" type="button" data-bs-toggle="collapse" data-bs-target="#faq-2">
                    What do "Most Wanted" and "Recommended" mean?
                </button>
            </h2>
            <div id="faq-2" class="accordion-collapse collapse" data-bs-parent="#faq">
                <div class="accordion-body">
                    <p>The "Most Wanted" badge shows for an item that's at the top of our list when sorted by our own priority.
                        It is especially useful to see what we want the most and base your purchase decision on this if you have no idea.</p>
                    <div>The "Recommended" badge uses an algorithm to recommend the best items to buy according to your budget. It is a great way to know what to give us if you are on a very low budget or you don't know what to buy. Also check out the "Sort by recommendation" option.</div>
                </div>
            </div>
        </div>
        <div class="accordion-item">
            <h2 class="accordion-header">
                <button class="accordion-button collapsed" type="button" data-bs-toggle="collapse" data-bs-target="#faq-3">
                    Can I get items in other places?
                </button>
            </h2>
            <div id="faq-3" class="accordion-collapse collapse" data-bs-parent="#faq">
                <div class="accordion-body">
                    For Steam games, they must be purchased from Steam (so that they can work out-of-the-box on Steam Deck).
                    For other items, the same item can be purchased from another place without asking us, and a different (but similar, e.g. cheaper) item can be purchased from another place after asking us.
                </div>
            </div>
        </div>
        <div class="accordion-item">
            <h2 class="accordion-header">
                <button class="accordion-button collapsed" type="button" data-bs-toggle="collapse" data-bs-target="#faq-4">
                    How should I pay for these items?
                </button>
            </h2>
            <div id="faq-4" class="accordion-collapse collapse" data-bs-parent="#faq">
                <div class="accordion-body">
                    <p>
                        For items that are available on Steam, Amazon, Google Play, Apple stores and other compatible platforms, a gift card would be preferred to prevent issues with delivery.
                        Steam games must be purchased with gift cards due to regional restrictions.
                        For items that cannot be bought with a gift card, you will have to order them yourself and enter our address.
                    </p>
                    <div>
                        You can also order items yourself and ship them to yourself, then send them to us.
                        That way, you can bundle multiple items together and/or add personal items or notes.
                    </div>
                </div>
            </div>
        </div>
    </div>
    <?php endif; ?>

    <?php if ($config["export"]): ?>
    <div class="small text-muted" style="margin-bottom: 15px;">
        All the data associated with your settings and budget on this page is saved on this browser only and is not sent to Equestria.dev or to your other devices.
        If you would like to transfer this data to other devices or back it up, you will need to <a href="#" onclick="dataImport();">import</a> or <a href="#" onclick="dataExport();">export</a> it.
    </div>
    <?php endif; ?>

    <p>
        <a onclick="toggleNSFW();" id="nsfw-toggle" href="#">Show not safe for work items</a>
    </p>

    <?php if (isset($config["notice"])): ?>
    <div style="margin-bottom: 30px;" class="alert alert-warning"><?= $config["notice"] ?></div>
    <?php endif; ?>

    <div id="filter-results" style="display: none; margin-bottom: 30px;" class="list-group"></div>

    <div id="all-items">
        <?php foreach ($lists as $list_name => $list): ?>
            <div class="list-container <?= $list["nsfw"] ? "list-container-nsfw" : "" ?>">
                <h3><?= $list["name"] ?? $list["title"] ?></h3>
                <div id="list-<?= $list_name ?>" class="list-group" style="margin-bottom: 30px;">
                    <?php foreach ($list["items"] as $item): $index = $item["_id"] ?? md5($item["name"]) ?>
                        <a data-recommend="<?= $list["recommend"] ?? -1 ?>" data-list="<?= $list_name ?>" id="item-<?= $list_name ?>-<?= $index ?>" data-score="<?= $item["score"] ?? -1 ?>" data-price="<?php

                        $price = -200;

                        if (isset($item["links"])) {
                            $avg = array_sum(array_map(function ($i) {
                                    return $i["price"] ?? 0;
                                }, $item["links"])) / count($item["links"]);
                            $price = $avg;
                        } else {
                            $price = $item["price"] ?? -200;
                        }

                        echo($price);

                        ?>" <?php if (isset($item["link"])): ?>href="<?= $item["link"] ?>" target="_blank"
                           class="list-item list-item-sel list-group-item list-group-item-action <?= $list["nsfw"] ? "list-item-nsfw" : "" ?>"
                           <?php else: ?>class="list-item list-item-sel list-group-item <?= $list["nsfw"] ? "list-item-nsfw" : "" ?>"<?php endif ?>>
                            <div style="display: flex; align-items: center; justify-content: center;">
                                <!--suppress HtmlFormInputWithoutLabel -->
                                <input class="form-check-input item-select" onchange="updateBudget();" type="checkbox" value="" data-price="<?= $price ?>" disabled>
                            </div>
                            <div>
                                <div class="bg-body-tertiary" style="aspect-ratio: 292/136; border-radius: 0.375rem; background-repeat: no-repeat; background-size: contain; background-position: center; <?php if (isset($item["image"])): ?>background-image: url('<?= $item["image"] ?>');<?php endif; ?>"></div>
                            </div>
                            <div <?php if (!isset($item["links"])): ?>style="display: flex; align-items: center;" <?php else: ?>style="margin-top: 10px; margin-bottom: 10px;"<?php endif; ?>>
                                <div>
                                    <div>
                                        <span id="badge-budget-<?= "item-$list_name-$index" ?>" class="badge-budget badge custom-badge" style="margin-bottom: 10px; display: none; --custom-badge-base: 32, 201, 151;">Recommended</span>
                                        <span class="badge-most badge custom-badge" style="display: <?php if ($index === 0): ?>inline-block<?php else: ?>none<?php endif; ?>; margin-bottom: 10px; --custom-badge-base: 111, 66, 193;">Most Wanted</span>
                                    </div>
                                    <h5><?php if (isset($item["name"])): ?><?= $item["name"] ?><?php else: ?>(Unnamed)<?php endif; ?></h5>
                                    <div>
                                        <?php if (isset($item["score"]) || isset($item["note"])): ?><div class="text-muted"><?php if (isset($item["note"]) || isset($notes["item-$list_name-$index"])): ?><i>"<?= $item["note"] ?? $notes["item-$list_name-$index"] ?>"</i><?php endif; ?><?php if (isset($item["score"])): ?><?php if (isset($item["note"]) || isset($notes["item-$list_name-$index"])): ?> · <?php endif; ?>Predicted appreciation: <?= round(($item["score"] / 5) * 100, 1) ?>%<?php endif; ?></div><?php endif; ?>
                                        <?php if (isset($item["tags"])): ?><div><?= implode(" · ", $item["tags"]) ?></div><?php endif; ?>
                                        <?php if (isset($item["links"])): ?>
                                            <div class="list-group" style="margin-top: 10px;">
                                                <?php foreach ($item["links"] as $link): ?>
                                                    <div onclick="window.open(`<?= $link["link"] ?? "#" ?>`);" style="cursor: pointer;" class="list-group-item list-group-item-action">
                                                        <div style="display: grid; grid-template-columns: 1fr max-content;">
                                                            <div><?= $link["name"] ?? "Option" ?></div>
                                                            <div>
                                                                <?php if (isset($link["price"])): ?>
                                                                    <b><?= str_replace("%", str_replace(".00", ".--", number_format($link["price"] / 100, 2)), ($item["unit"] ?? "% " . $config["currency"])) ?></b>
                                                                <?php endif; ?>
                                                                <?php if (isset($link["source"])): ?>
                                                                    <?php if (isset($link["price"])): ?> · <?php endif; ?>
                                                                    <span><?= $link["source"] ?> ↗</span>
                                                                <?php endif; ?>
                                                            </div>
                                                        </div>
                                                    </div>
                                                <?php endforeach; ?>
                                            </div>
                                        <?php endif; ?>
                                    </div>
                                </div>
                            </div>
                            <div style="display: flex; align-items: center; justify-content: end;">
                                <div style="text-align: right;">
                                    <?php if (isset($item["price"])): ?>
                                        <h4 style="text-align: right;">
                                            <?php if ($item["price"] > 0): ?>
                                                <?= str_replace("%", str_replace(".00", ".--", number_format($item["price"] / 100, 2)), ($item["unit"] ?? "% " . $config["currency"])) ?>
                                            <?php else: ?>
                                                <span class="text-muted"><?= $item["date"] ?></span>
                                            <?php endif; ?>
                                        </h4>
                                    <?php endif; ?>
                                    <?php if (isset($item["source"])): ?><div><?= $item["source"] ?> ↗</div><?php endif; ?>
                                </div>
                            </div>
                        </a>
                    <?php endforeach; ?>
                </div>
            </div>
        <?php endforeach; ?>
    </div>

    <br><br>

    <!--suppress JSCheckFunctionSignatures -->
    <script>
        function changeOrder() {
            let sort = document.getElementById("sort").value;
            let items = [];

            switch (sort) {
                case "plh":
                case "phl":
                case "alh":
                case "ahl":
                    document.getElementById("all-items").style.display = "none";
                    document.getElementById("filter-results").style.display = "";

                    for (let item of document.getElementsByClassName("list-item-sel")) {
                        let node = item.cloneNode(true);
                        node.classList.remove("list-item-sel");
                        items.push(node);
                    }

                    items.sort((a, b) => {
                        return parseFloat((sort.endsWith("lh") ? a : b).getAttribute(sort.startsWith("p") ? "data-price" : "data-score")) -
                            parseFloat((sort.endsWith("lh") ? b : a).getAttribute(sort.startsWith("p") ? "data-price" : "data-score"));
                    });

                    document.getElementById("filter-results").innerHTML = items.map(i => i.outerHTML).join("");

                    break;

                case "cnt":
                    document.getElementById("all-items").style.display = "none";
                    document.getElementById("filter-results").style.display = "";

                    for (let item of document.getElementsByClassName("list-item-sel")) {
                        let node = item.cloneNode(true);
                        node.classList.remove("list-item-sel");
                        items.push(node);
                    }

                    document.getElementById("filter-results").innerHTML = items.map(i => i.outerHTML).join("");

                    break;

                case "rec":
                    document.getElementById("all-items").style.display = "none";
                    document.getElementById("filter-results").style.display = "";

                    for (let item of document.getElementsByClassName("list-item-sel")) {
                        let node = item.cloneNode(true);
                        node.classList.remove("list-item-sel");
                        items.push(node);
                    }

                    items.sort((a, b) => {
                        let sA = 0;
                        if (a.getElementsByClassName("badge-budget")[0].style.display !== "none") sA += 2;
                        if (a.getElementsByClassName("badge-most")[0].style.display !== "none") sA++;

                        let sB = 0;
                        if (b.getElementsByClassName("badge-budget")[0].style.display !== "none") sB += 2;
                        if (b.getElementsByClassName("badge-most")[0].style.display !== "none") sB++;

                        return sB - sA;
                    });

                    document.getElementById("filter-results").innerHTML = items.map(i => i.outerHTML).join("");

                    break;

                case "cro":
                    document.getElementById("all-items").style.display = "";
                    document.getElementById("filter-results").style.display = "none";
                    break;
            }

            document.getElementById("sort").blur();
            save();
        }

        function makePositive(n) {
            if (n > 0) {
                return n;
            } else {
                return 0;
            }
        }

        function updateBudget(restore) {
            Array.from(document.getElementsByClassName("item-select")).map(i => i.blur());
            document.getElementById("budget").blur();
            document.getElementById("show-only-selected").blur();
            document.getElementById("hide-oob").blur();

            let budget = parseFloat(document.getElementById("budget").value);
            if (isNaN(budget)) budget = 0;

            if (parseFloat(document.getElementById("budget-value").innerText) !== budget && !restore) {
                if (budget >= 50) {
                    if (!confirm("This is a lot of money, make sure you know what you are doing.\n\nOnly spend what you can afford to spend (there is no \"not spending enough\") and make sure to buy only the gifts you want. Please do not ruin yourself just to make gifts.\n\nAre you sure you want to set your budget to " + budget.toFixed(2).replaceAll(".00", ".--") + " <?= $config["currency"] ?>?")) {
                        document.getElementById("budget").value = "0";
                        updateBudget();
                        return;
                    }
                }

                location.reload();
            }

            let items = [];
            let cost = 0;

            for (let item of document.getElementsByClassName("list-item-sel")) {
                if (item.getElementsByTagName("input")[0].checked) {
                    item.classList.add("item-active");
                    items.push(item);
                    cost += parseInt(makePositive(item.getAttribute("data-price"))) / 100;
                    item.style.opacity = "1";
                } else {
                    item.classList.remove("item-active");
                    item.style.opacity = "0.75";
                }
            }

            let left = budget - cost;

            if (budget > 0) {
                document.getElementById("budget").value = budget.toFixed(2);

                if (document.getElementById("budget-outer").style.display === "none") {
                    document.getElementById("budget-outer").style.display = "";
                    Array.from(document.getElementsByClassName("item-select")).map(i => { i.checked = false; i.disabled = false; });
                }

                if (document.getElementById("sort").disabled && !document.getElementById("show-only-selected").checked) {
                    document.getElementById("sort").disabled = false;
                    document.getElementById("sort").value = "cro";
                    changeOrder();
                } else if (document.getElementById("show-only-selected").checked) {
                    document.getElementById("sort").disabled = true;
                    document.getElementById("sort").value = "cnt";
                    changeOrder();

                    document.getElementById("all-items").style.display = "none";
                    document.getElementById("filter-results").style.display = "";

                    let items = [];

                    for (let item of document.getElementsByClassName("list-item-sel")) {
                        let node = item.cloneNode(true);
                        node.classList.remove("list-item-sel");
                        node.classList.remove("item-active");
                        node.getElementsByTagName("input")[0].disabled = true;
                        node.getElementsByTagName("input")[0].checked = true;
                        if (item.classList.contains("item-active")) items.push(node);
                        node.style.opacity = "1";
                    }

                    document.getElementById("filter-results").innerHTML = items.map(i => i.outerHTML).join("");
                } else if (document.getElementById("hide-oob").checked) {
                    Array.from(document.getElementsByClassName("list-item-sel")).map(i => { i.style.display = ""; i.classList.add("list-group-item"); });

                    for (let item of document.getElementsByClassName("list-item-sel")) {
                        if (parseFloat(makePositive(item.getAttribute("data-price"))) / 100 > left && !item.classList.contains("item-active")) {
                            item.style.display = "none";
                            item.classList.remove("list-group-item");
                        }
                    }
                } else {
                    Array.from(document.getElementsByClassName("list-item-sel")).map(i => { i.style.display = ""; i.classList.add("list-group-item"); });
                }

                document.getElementById("budget-value").innerText = budget.toFixed(2).replaceAll(".00", ".--");

                localStorage.setItem("items", items.map(i => i.id).join(","));

                document.getElementById("budget-usage").innerText = cost.toFixed(2).replaceAll(".00", ".--");
                document.getElementById("budget-usage-percent").innerText = ((cost / budget) * 100).toFixed();

                left = parseFloat(left.toFixed(2));

                if (left > 0) {
                    document.getElementById("budget-limit").innerText = left.toFixed(2).replaceAll(".00", ".--") + " <?= $config["currency"] ?> left";
                    document.getElementById("budget-limit").classList.remove("text-danger");
                    document.getElementById("budget-limit").classList.remove("text-success");
                    document.getElementById("budget-progress").style.width = ((cost / budget) * 100) + "%";
                    document.getElementById("budget-progress").classList.remove("bg-success");
                    document.getElementById("budget-progress-red").style.width = "0";
                } else if (left < 0) {
                    document.getElementById("budget-limit").innerText = Math.abs(left).toFixed(2).replaceAll(".00", ".--") + " <?= $config["currency"] ?> over";
                    document.getElementById("budget-limit").classList.add("text-danger");
                    document.getElementById("budget-progress").style.width = "0";
                    document.getElementById("budget-progress").classList.remove("bg-success");
                    document.getElementById("budget-progress-red").style.width = "100%";
                } else {
                    document.getElementById("budget-limit").innerText = "Used up";
                    document.getElementById("budget-limit").classList.add("text-success");
                    document.getElementById("budget-progress").style.width = "100%";
                    document.getElementById("budget-progress").classList.add("bg-success");
                }

                Array.from(document.getElementsByClassName("list-item-sel")).map(i => { i.getElementsByClassName("badge-budget")[0].style.display = "none"; });

                let leftOver = budget;
                let eligible = Array.from(document.getElementsByClassName("list-item-sel")).sort((a, b) => {
                    return parseInt(b.getAttribute("data-recommend")) - parseInt(a.getAttribute("data-recommend"));
                });

                for (let item of eligible) {
                    let itemPrice = makePositive(parseInt(item.getAttribute("data-price")) / 100);
                    console.log(item, itemPrice, leftOver, !(itemPrice > leftOver));
                    if (itemPrice > leftOver) continue;

                    leftOver -= itemPrice;
                    document.getElementById("badge-budget-" + item.id).style.display = "inline-block";
                }
            } else {
                document.getElementById("budget-outer").style.display = "none";
                Array.from(document.getElementsByClassName("item-select")).map(i => { i.checked = false; i.disabled = true; });
                Array.from(document.getElementsByClassName("list-item-sel")).map(i => { i.classList.remove("item-active"); i.style.opacity = "1"; i.style.display = ""; i.classList.add("list-group-item"); i.getElementsByClassName("badge-budget")[0].style.display = "none"; });
                document.getElementById("budget").value = "";
                document.getElementById("budget-value").innerText = "";
            }

            save();
            if (!restore) {
                changeOrder();
                let order = document.getElementById("sort").value;
                document.getElementById("sort").value = "cro";
                changeOrder();
                document.getElementById("sort").value = order;
                changeOrder();
            }
        }

        <?php if ($config["export"]): ?>
        function dataExport() {
            let data = btoa(JSON.stringify(localStorage));

            if (confirm("Would you like to copy your user data to the clipboard?")) {
                navigator.clipboard.writeText(data);
                alert("Copied data to your clipboard. Paste it on the other device and use the import option.");
            }
        }

        function dataImport() {
            let data = prompt("Paste the data you exported from the other device below:");

            if (data && data.trim() !== "") {
                data = JSON.parse(atob(data));
                localStorage.clear();

                for (let item of Object.entries(data)) {
                    localStorage.setItem(item[0], item[1]);
                }

                alert("Successfully imported data.");
                location.reload();
            }
        }
        <?php endif; ?>

        function save() {
            localStorage.setItem("sort", document.getElementById("sort").value);
            localStorage.setItem("budget", document.getElementById("budget").value);
            localStorage.setItem("show-only-selected", document.getElementById("show-only-selected").checked ? "1" : "0");
            localStorage.setItem("hide-oob", document.getElementById("hide-oob").checked ? "1" : "0");
        }

        document.getElementById("sort").value = localStorage.getItem("sort") ?? "cro";
        document.getElementById("budget").value = localStorage.getItem("budget") ?? "";
        document.getElementById("show-only-selected").checked = localStorage.getItem("show-only-selected") === "1" ?? false;
        document.getElementById("hide-oob").checked = localStorage.getItem("hide-oob") === "1" ?? false;

        let selected = (localStorage.getItem("items") ?? "").split(",").filter(i => i && i.trim() !== "");
        console.log(selected);

        updateBudget(true);

        for (let item of selected) {
            try {
                document.getElementById(item).getElementsByTagName("input")[0].checked = true;
            } catch (e) {
                console.error(e);
            }
        }

        save(); updateBudget(true);

        window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
            document.body.setAttribute("data-bs-theme", e.matches ? "dark" : "light");
        });

        if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
            document.body.setAttribute("data-bs-theme", "dark");
        } else {
            document.body.setAttribute("data-bs-theme", "light");
        }

        changeOrder();

        function toggleNSFW() {
            if (document.getElementById("nsfw-toggle").innerText === "Show not safe for work items" && confirm("Are you sure you want to show not safe for work (NSFW) items? These items may not be appropriate for everyone.")) {
                document.getElementById("nsfw-toggle").innerText = "Hide not safe for work items";
                document.body.classList.add("show-nsfw");
            } else {
                document.getElementById("nsfw-toggle").innerText = "Show not safe for work items";
                document.body.classList.remove("show-nsfw");
            }
        }
    </script>
</div>
</body>
</html>
