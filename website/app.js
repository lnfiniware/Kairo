const GITHUB_RAW_URL = "https://raw.githubusercontent.com/lnfiniware/Kairo/master/docs/wiki/";

function showTab(tabName) {
    // Update buttons
    document.querySelectorAll('.tab-btn').forEach(btn => {
        btn.classList.remove('active');
        if (btn.innerText.toLowerCase() === tabName) btn.classList.add('active');
    });

    // Update content
    document.querySelectorAll('.tab-content').forEach(content => {
        content.classList.remove('active');
    });
    document.getElementById(`${tabName}-tab`).classList.add('active');

    // Show/Hide Search
    const searchContainer = document.getElementById('search-container');
    if (tabName === 'docs' || tabName === 'wiki') {
        searchContainer.style.display = 'block';
    } else {
        searchContainer.style.display = 'none';
    }

    // Load initial content for Wiki/Docs if empty
    if (tabName === 'docs' && document.getElementById('docs-body').innerText.includes('Select')) {
        loadDoc('Getting-Started');
    }
    if (tabName === 'wiki' && document.getElementById('wiki-body').innerText.includes('Loading')) {
        loadWiki('Home');
    }
}

async function loadMarkdown(filename, targetId) {
    const target = document.getElementById(targetId);
    target.innerHTML = `<p class="loading">Fetching ${filename}...</p>`;

    try {
        const response = await fetch(`${GITHUB_RAW_URL}${filename}.md`);
        if (!response.ok) throw new Error("File not found");
        const markdown = await response.text();
        target.innerHTML = marked.parse(markdown);
    } catch (err) {
        target.innerHTML = `<p class="error">Error loading ${filename}. Please try again later.</p>`;
    }
}

function loadDoc(name) {
    loadMarkdown(name, 'docs-body');
    // Highlight active menu item
    document.querySelectorAll('.docs-sidebar li').forEach(li => {
        li.style.color = 'var(--text-secondary)';
        li.style.fontWeight = 'normal';
        if (li.innerText.replace(' ', '-') === name) {
            li.style.color = 'var(--accent-color)';
            li.style.fontWeight = 'bold';
        }
    });
}

function loadWiki(name) {
    loadMarkdown(name, 'wiki-body');
}

function toggleTheme() {
    const body = document.body;
    if (body.classList.contains('light-mode')) {
        body.classList.remove('light-mode');
        body.classList.add('dark-mode');
    } else {
        body.classList.remove('dark-mode');
        body.classList.add('light-mode');
    }
}

function handleSearch(val) {
    const query = val.toLowerCase();
    const activeTab = document.querySelector('.tab-content.active').id;
    
    if (activeTab === 'docs-tab') {
        const items = document.querySelectorAll('.docs-sidebar li');
        items.forEach(item => {
            const text = item.innerText.toLowerCase();
            item.style.display = text.includes(query) ? 'block' : 'none';
        });
    }
}

// Initialize with Home
window.onload = () => {
    // Check system preference
    if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
        toggleTheme();
    }
};
