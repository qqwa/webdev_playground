const urls_div = document.getElementById('urls');

async function getUrls() {
    const reponse = await fetch("/api/urls");
    const urls = await reponse.json();
    return urls
}

function insertUrls(urls, target) {
    const table = document.createElement('table');
    table.appendChild(createTHead());

    for (let url of urls) {
        table.appendChild(createRow(url))
    }

    target.replaceChildren(table);
    setTimeout(poll, 2000);
}

function createTHead() {
    const tHead = document.createElement('tHead');
    tHead.classList.add('text-left');
    tHead.innerHTML = '<tr><th>Short Url</th><th>Long Url</th><th>Clicks</th></tr>';
    return tHead;
}

function createRow(url) {
    const tr = document.createElement('tr');
    const short_url = document.createElement('td');
    short_url.innerHTML = `<a class="text-blue-700" href="/l/${url.short_url}">${url.short_url}</a>`;
    const long_url = document.createElement('td');
    long_url.innerHTML = `<a class="text-blue-700" href="/l/${url.long_url}">${url.long_url}</a>`;
    const counter = document.createElement('td');
    counter.innerText = url.counter;
    tr.appendChild(short_url);
    tr.appendChild(long_url);
    tr.appendChild(counter);
    return tr;
}

function poll() {
    const urlsPromise = getUrls();
    urlsPromise.then((urls) => {
        insertUrls(urls, urls_div);
    })
}

poll();
