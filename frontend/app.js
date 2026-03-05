const API = '/api';

function getUsername() {
  return sessionStorage.getItem('testflix_username');
}

function requireLogin() {
  if (!getUsername()) {
    window.location.href = '/login.html';
  }
}

function logout() {
  sessionStorage.removeItem('testflix_username');
  window.location.href = '/login.html';
}

function renderNav(currentPage) {
  const nav = document.createElement('nav');
  const logo = document.createElement('a');
  logo.href = '/index.html';
  logo.className = 'logo';
  logo.textContent = 'TESTFLIX';

  const links = document.createElement('div');
  links.className = 'nav-links';

  if (currentPage !== 'profile') {
    const account = document.createElement('a');
    account.href = '/profile.html';
    account.textContent = 'Account';
    links.appendChild(account);
  }

  const logoutLink = document.createElement('a');
  logoutLink.href = '#';
  logoutLink.textContent = 'Logout';
  logoutLink.addEventListener('click', function(e) {
    e.preventDefault();
    logout();
  });
  links.appendChild(logoutLink);

  nav.appendChild(logo);
  nav.appendChild(links);
  document.body.prepend(nav);
}

function formatDuration(seconds) {
  if (!seconds) return '';
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  if (h > 0) return h + 'h ' + m + 'm';
  return m + 'm';
}
