export const routes = [
  {
    name: 'calc',
    path: '/calc',
    component: () => import('../Calculator.vue'),
    meta: {
      pageTitle: 'title.calculator'
    },
  }
]
