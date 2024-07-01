function changeOrder() {
    let sort = document.getElementById("sort").value;
    let items = [];

    switch (sort) {
        case "plh":
        case "phl":
            document.getElementById("all-items").style.display = "none";
            document.getElementById("filter-results").style.display = "";

            for (let item of document.getElementsByClassName("list-item-sel")) {
                let node = item.cloneNode(true);
                node.classList.remove("list-item-sel");
                items.push(node);
            }

            items.sort((a, b) => {
                return parseFloat((sort.endsWith("lh") ? a : b).getAttribute("data-price")) -
                    parseFloat((sort.endsWith("lh") ? b : a).getAttribute("data-price"));
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
            if (!confirm("This is a lot of money, make sure you know what you are doing.\n\nOnly spend what you can afford to spend (there is no \"not spending enough\") and make sure to buy only the gifts you want. Please do not ruin yourself just to make gifts.\n\nAre you sure you want to set your budget to " + budget.toFixed(2).replaceAll(".00", ".--") + " " + window.currency + "?")) {
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
            document.getElementById("budget-limit").innerText = left.toFixed(2).replaceAll(".00", ".--") + " " + window.currency + " left";
            document.getElementById("budget-limit").classList.remove("text-danger");
            document.getElementById("budget-limit").classList.remove("text-success");
            document.getElementById("budget-progress").style.width = ((cost / budget) * 100) + "%";
            document.getElementById("budget-progress").classList.remove("bg-success");
            document.getElementById("budget-progress-red").style.width = "0";
        } else if (left < 0) {
            document.getElementById("budget-limit").innerText = Math.abs(left).toFixed(2).replaceAll(".00", ".--") + " " + window.currency + " over";
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

// noinspection JSUnusedGlobalSymbols
function dataExport() {
    let data = btoa(JSON.stringify(localStorage));

    if (confirm("Would you like to copy your user data to the clipboard?")) {
        navigator.clipboard.writeText(data);
        alert("Copied data to your clipboard. Paste it on the other device and use the import option.");
    }
}

// noinspection JSUnusedGlobalSymbols
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

// noinspection JSUnusedGlobalSymbols
function toggleNSFW() {
    if (document.getElementById("nsfw-toggle").innerText === "Show not safe for work items" && confirm("Are you sure you want to show not safe for work (NSFW) items? These items may not be appropriate for everyone.")) {
        document.getElementById("nsfw-toggle").innerText = "Hide not safe for work items";
        document.body.classList.add("show-nsfw");
    } else {
        document.getElementById("nsfw-toggle").innerText = "Show not safe for work items";
        document.body.classList.remove("show-nsfw");
    }
}
