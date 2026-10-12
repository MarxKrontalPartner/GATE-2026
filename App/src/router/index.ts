import SideDrawerBwd from '@/components/bwd/SideDrawer.vue';
import SideDrawerProjektauswahl from '@/components/projektauswahl/SideDrawer.vue';
import { replaceLettersWithUmlauts, replaceWithDisplayName } from '@/utils/StringSanitizers';
import { createRouter, createWebHistory } from 'vue-router';


const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  scrollBehavior() {
    // always scroll to top
    return { top: 0 }
  },
  routes: [
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
      redirect: { name: 'Untersuchungsstellen' },
      children: [
        {
          path: 'Untersuchungsstellen',
          name: 'Untersuchungsstellen',
          components: { default: () => import('@/views/bwd/UntersuchungsstellenView.vue'), sidedrawer: SideDrawerBwd }
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
