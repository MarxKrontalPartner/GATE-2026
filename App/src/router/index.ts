import SideDrawerBwd from '@/components/bwd/SideDrawer.vue';
import SideDrawerProjektauswahl from '@/components/projektauswahl/SideDrawer.vue';
import { replaceLettersWithUmlauts, replaceWithDisplayName } from '@/utils/StringSanitizers';
import { isLoggedIn } from '@/services/auth';
import { createRouter, createWebHistory } from 'vue-router';


const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  scrollBehavior() {
    // always scroll to top
    return { top: 0 }
  },
  routes: [
    {
  path: '/login',
  name: 'login',
  component: () => import('@/views/LoginView.vue')
  }, 
    {
      path: '/',
      name: 'index',
      redirect: { name: 'bauwerkswahl' }
    },
    {
      path: '/listenansicht',
      name: 'bauwerkswahl',
      components: { default: () => import('@/views/bauwerksverwaltung/BauwerkswahlView.vue'), sidedrawer: SideDrawerProjektauswahl }
    },
    {
      path: `/:bauwerkId(\\d+)/bwd`,
      name: 'bwd',
      redirect: { name: 'BauwerkUebersicht' },
      children: [
        {
          path: 'Uebersicht',
          name: 'BauwerkUebersicht',
          components: { default: () => import('@/views/bwd/BauwerkUebersichtView.vue'), sidedrawer: SideDrawerBwd }
        },
        {
          path: 'Projekt/:projektId(\\d+)',
          name: 'BauwerkProjekt',
          components: { default: () => import('@/views/bwd/BauwerkProjektView.vue'), sidedrawer: SideDrawerBwd }
        },
        {
          path: 'Untersuchungsstellen',
          name: 'Untersuchungsstellen',
          components: { default: () => import('@/views/bwd/UntersuchungsstellenView.vue'), sidedrawer: SideDrawerBwd }
        },
        {
          path: 'Untersuchungsstelle/:stelleId(\\d+)',
          name: 'UntersuchungsstelleDetail',
          components: { default: () => import('@/views/bwd/UntersuchungsstelleDetailView.vue'), sidedrawer: SideDrawerBwd }
        },
        {
          path: `VorOrtUntersuchung`,
          name: 'VorOrtUntersuchung',
          components: { default: () => import('@/views/bwd/VorOrtUntersuchung.vue'), sidedrawer: SideDrawerBwd }
        },
        {
          path: `Laboruntersuchung`,
          name: 'Laboruntersuchung',
          components: { default: () => import('@/views/bwd/Laboruntersuchung.vue'), sidedrawer: SideDrawerBwd }
        },
        {
          path: 'Untersuchungsverfahren/:art/:untersuchung/:id',
          name: 'UntersuchungsverfahrenDetail',
          components: { default: () => import('@/views/bwd/UntersuchungsverfahreDetailView.vue'), sidedrawer: SideDrawerBwd }
        },
      ]
    },

  ]
})

// Every page except the login page needs a login.
// The stored tokens count as logged in, so the app also opens offline.
router.beforeEach((to) => {
  const loggedIn = isLoggedIn();
  if (to.name !== 'login' && !loggedIn) {
    return { name: 'login', query: to.fullPath !== '/' ? { redirect: to.fullPath } : {} };
  }
  if (to.name === 'login' && loggedIn) {
    return { name: 'index' };
  }
});

router.beforeEach(async (to) => {
  // Set title of the document
  let destinationTitle = to.path.split('/').pop();
  if (!destinationTitle) {
    document.title = 'MKP Diagnostik';
  } else {
    destinationTitle = destinationTitle.charAt(0).toUpperCase() + destinationTitle.slice(1);
    destinationTitle = replaceLettersWithUmlauts(destinationTitle);
    destinationTitle = replaceWithDisplayName(destinationTitle);
    document.title = destinationTitle + ' - MKP Diagnostik';
  }
});

export default router
