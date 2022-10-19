export const routes = [
  {
    name: "about",
    path: '/about',
    component: () => import('../About.vue'),
    meta: {
      pageTitle: 'title.about'
    },
  }
]
