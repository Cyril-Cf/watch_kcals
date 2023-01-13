import { RouteRecordRaw } from 'vue-router';

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: () => import('layouts/MainLayout.vue'),
    children: [
      { path: 'user-details', component: () => import('pages/PageUserDetails.vue'), name: 'page-user-details' },
      { path: 'ingredients', component: () => import('pages/PageIngredients.vue'), name: 'page-ingredients' },
    ],
  },

  // Always leave this as last one,
  // but you can also remove it
  {
    path: '/:catchAll(.*)*',
    component: () => import('pages/ErrorNotFound.vue'),
  },
];

export default routes;
