export const routes = [
  {
    path: '/cracker',
    alias: '',
    component: () => import('../Cracker.vue'),
    meta: {
      pageTitle: 'Cracker'
    },
  }
]
