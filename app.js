const WIKI_BASE = "https://raw.githubusercontent.com/lnfiniware/Kairo/master/docs/wiki/";

function showTab(name) {
    document.querySelectorAll('.tab-content').forEach(el => el.classList.remove('active'));
    document.getElementById(name + '-tab').classList.add('active');

    document.querySelectorAll('.nav-link').forEach(link => {
        link.classList.remove('active');
        const text = link.textContent.toLowerCase();
        if (text === name || (name === 'docs' && text === 'documentation')) {
            link.classList.add('active');
        }
    });

    if (name === 'docs' && document.getElementById('docs-body').querySelector('.portal-placeholder')) {
        loadDoc('Getting-Started');
    }
    if (name === 'wiki' && document.getElementById('wiki-body').querySelector('.portal-placeholder')) {
        loadWiki('Home');
    }
}

async function fetchMarkdown(filename, targetId) {
    const target = document.getElementById(targetId);
    target.innerHTML = '<p style="color:var(--text-light)">Loading...</p>';
    try {
        const res = await fetch(WIKI_BASE + filename + '.md');
        if (!res.ok) throw new Error('not found');
        const md = await res.text();
        target.innerHTML = marked.parse(md);
    } catch (e) {
        target.innerHTML = '<p style="color:var(--text-light)">Could not load ' + filename + '.</p>';
    }
}

function loadDoc(name) {
    fetchMarkdown(name, 'docs-body');
    document.querySelectorAll('#docs-menu .sidebar-item').forEach(li => {
        li.classList.remove('active');
        if (li.textContent.replace(/ /g, '-') === name) li.classList.add('active');
    });
}

function loadWiki(name) {
    fetchMarkdown(name, 'wiki-body');
    document.querySelectorAll('#wiki-menu .sidebar-item').forEach(li => {
        li.classList.remove('active');
        if (li.textContent.replace(/ /g, '-') === name || li.textContent === name) li.classList.add('active');
    });
}

function toggleTheme() {
    const body = document.body;
    const btn = document.querySelector('.theme-btn');
    body.classList.toggle('dark');
    btn.textContent = body.classList.contains('dark') ? 'Light' : 'Dark';
}

function handleSearch(val) {
    const q = val.toLowerCase();
    document.querySelectorAll('#docs-menu .sidebar-item').forEach(li => {
        li.style.display = li.textContent.toLowerCase().includes(q) ? '' : 'none';
    });
}

function handleWikiSearch(val) {
    const q = val.toLowerCase();
    document.querySelectorAll('#wiki-menu .sidebar-item').forEach(li => {
        li.style.display = li.textContent.toLowerCase().includes(q) ? '' : 'none';
    });
}
