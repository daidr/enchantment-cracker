export const routes = [
  {
    name: 'cracker',
    path: '/cracker',
    alias: '',
    component: () => import('../Cracker.vue'),
    meta: {
      pageTitle: 'title.cracker'
    },
  }
]
