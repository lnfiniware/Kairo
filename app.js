const WIKI_BASE = "https://raw.githubusercontent.com/lnfiniware/Kairo/master/docs/wiki/";

document.addEventListener('DOMContentLoaded', () => {
    const el = document.getElementById('hero-typewriter');
    if (el) {
        const phrases = [
            "Human-readable databases.",
            "Minimal. Fast. Local-first.",
            "Readable schemas.",
            "Built for developers.",
            "Local-first by default.",
            "Fast Rust-powered workflows.",
            "Databases without ORM chaos."
        ];
        typewriter(el, phrases);
    }

    // Default to dark
    if (!document.body.classList.contains('dark')) {
        document.body.classList.add('dark');
        const btn = document.querySelector('.theme-btn');
        if (btn) btn.textContent = 'Light';
    }

    animateBars();
});

function showTab(name) {
    document.querySelectorAll('.tab-content').forEach(el => el.classList.remove('active'));
    document.getElementById(name + '-tab').classList.add('active');

    document.querySelectorAll('.nav-link').forEach(link => {
        link.classList.remove('active');
        const text = link.textContent.trim().toLowerCase();
        if (text === name) link.classList.add('active');
    });

    if (name === 'wiki' && document.getElementById('wiki-body').querySelector('.portal-placeholder')) {
        loadWiki('Home');
    }
    if (name === 'home') {
        setTimeout(animateBars, 50);
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

function loadWiki(name) {
    fetchMarkdown(name, 'wiki-body');
    document.querySelectorAll('#wiki-menu .sidebar-item').forEach(li => {
        li.classList.remove('active');
        if (li.textContent.trim().replace(/ /g, '-') === name || li.textContent.trim() === name) li.classList.add('active');
    });
}

function toggleTheme() {
    const body = document.body;
    const btn = document.querySelector('.theme-btn');
    body.classList.toggle('dark');
    btn.textContent = body.classList.contains('dark') ? 'Light' : 'Dark';
}

function handleWikiSearch(val) {
    const q = val.toLowerCase();
    document.querySelectorAll('#wiki-menu .sidebar-item').forEach(li => {
        li.style.display = li.textContent.toLowerCase().includes(q) ? '' : 'none';
    });
}

// Typewriter
function typewriter(el, phrases) {
    let i = 0, j = 0, deleting = false;
    function tick() {
        const full = phrases[i];
        if (!deleting) {
            j++;
            if (j > full.length) {
                setTimeout(() => { deleting = true; tick(); }, 2200);
                el.innerHTML = full + '<span class="typewriter-cursor">|</span>';
                return;
            }
        } else {
            j--;
            if (j <= 0) {
                deleting = false;
                i = (i + 1) % phrases.length;
                setTimeout(tick, 300);
                el.innerHTML = '<span class="typewriter-cursor">|</span>';
                return;
            }
        }
        el.innerHTML = full.substring(0, j) + '<span class="typewriter-cursor">|</span>';
        setTimeout(tick, deleting ? 30 : 65);
    }
    tick();
}

// Animate benchmark bars
function animateBars() {
    document.querySelectorAll('.benchmark-bar').forEach(bar => {
        const v = bar.getAttribute('data-value');
        bar.style.width = '0%';
        setTimeout(() => { bar.style.width = v + '%'; }, 80);
    });
}
