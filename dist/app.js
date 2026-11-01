// The walkthrough is illustrative; it does not call models or deploy projects.
const tours = {
  develop: {
    breadcrumb: 'Development',
    content: `<div class="tour-kicker">A GOOD IDEA IS A GREAT PLACE TO START</div>
      <h3>What will you create?</h3>
      <p class="tour-description">A little direction. A team of agents. Something worth building.</p>
      <div class="brief-box"><p>A calm, minimal dashboard for keeping track of my daily habits.<br>Make it feel personal, with a little room to grow.</p><div class="brief-bottom"><span>Your idea, in your words.</span><span class="mock-button">Run agent team ↗</span></div></div>
      <div class="preview-team"><span class="preview-agent"><span class="agent-avatar">P</span>Planner</span><span class="preview-agent"><span class="agent-avatar">D</span>Developer</span><span class="preview-agent"><span class="agent-avatar">R</span>Reviewer</span></div>
      <p class="preview-footnote">Your provider. Your model. One considered workflow.</p>`
  },
  check: {
    breadcrumb: 'Quality checks',
    content: `<div class="tour-kicker">A LITTLE MORE CLARITY BEFORE THE NEXT STEP</div>
      <h3>Take a closer look.</h3>
      <p class="tour-description">Source checks and advisory review, together in one place.</p>
      <div class="check-list"><div><span>Allowed source files</span><span class="check-ok">✓ Passed</span></div><div><span>Duplicate paths & source limits</span><span class="check-ok">✓ Passed</span></div><div><span>Independent agent review</span><span>Advisory findings</span></div><div><span>Runtime & behavior tests</span><span class="check-pending">Not run</span></div></div>
      <p class="preview-footnote">Example report. Static checks do not verify runtime behavior.</p>`
  },
  deploy: {
    breadcrumb: 'Deployments',
    content: `<div class="tour-kicker">SOMETHING YOU MADE. SOMEWHERE TO GO.</div>
      <h3>Ready for the next step?</h3>
      <p class="tour-description">Keep building locally, or share your static website with the world.</p>
      <div class="deploy-options"><div><strong>↓ Export source</strong><p>Your files, a review report, and a manifest. All in one local bundle.</p></div><div><strong>↗ Publish to Vercel</strong><p>Connect your account and confirm before publishing a static website.</p></div></div>
      <p class="preview-footnote">Publishing requires a Vercel token and your explicit confirmation.</p>`
  }
};

const tabs = [...document.querySelectorAll('[data-tour]')];
function selectTour(name, focus = false) {
  const tour = tours[name];
  if (!tour) return;
  for (const tab of tabs) {
    const selected = tab.dataset.tour === name;
    tab.setAttribute('aria-selected', String(selected));
    tab.tabIndex = selected ? 0 : -1;
    if (selected && focus) tab.focus();
  }
  document.querySelector('#tour-panel').setAttribute('aria-labelledby', `tab-${name}`);
  document.querySelector('#tour-breadcrumb').textContent = tour.breadcrumb;
  document.querySelector('#tour-content').innerHTML = tour.content;
  for (const item of document.querySelectorAll('[data-side]')) {
    item.classList.toggle('active', item.dataset.side === name);
  }
}
tabs.forEach((tab, index) => {
  tab.addEventListener('click', () => selectTour(tab.dataset.tour));
  tab.addEventListener('keydown', (event) => {
    let next;
    if (event.key === 'ArrowRight') next = (index + 1) % tabs.length;
    if (event.key === 'ArrowLeft') next = (index + tabs.length - 1) % tabs.length;
    if (event.key === 'Home') next = 0;
    if (event.key === 'End') next = tabs.length - 1;
    if (next !== undefined) {
      event.preventDefault();
      selectTour(tabs[next].dataset.tour, true);
    }
  });
});
selectTour('develop');

const menu = document.querySelector('.menu-toggle');
const navigation = document.querySelector('#navigation');
function closeMenu() {
  menu.setAttribute('aria-expanded', 'false');
  menu.setAttribute('aria-label', 'Open navigation');
  navigation.classList.remove('open');
}
menu.addEventListener('click', () => {
  const open = menu.getAttribute('aria-expanded') !== 'true';
  menu.setAttribute('aria-expanded', String(open));
  menu.setAttribute('aria-label', open ? 'Close navigation' : 'Open navigation');
  navigation.classList.toggle('open', open);
});
navigation.addEventListener('click', (event) => {
  if (event.target.closest('a')) closeMenu();
});
document.addEventListener('keydown', (event) => {
  if (event.key === 'Escape' && menu.getAttribute('aria-expanded') === 'true') {
    closeMenu();
    menu.focus();
  }
});
