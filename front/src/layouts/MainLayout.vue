<template>
  <q-layout view="lHh Lpr lFf">
    <q-header v-if="showToolbar" reveal elevated>
      <q-toolbar>
        <logo-header v-if="showToolbar" />
        <q-toolbar-title>
          <bread-crumb :menuItems="menuItems" v-if="!$q.platform.is.mobile" />
        </q-toolbar-title>

        <q-toggle
          v-model="dark"
          :color="$q.dark.mode ? 'primary' : 'secondary'"
          icon-color="secondary"
          :icon="$q.dark.mode ? 'mdi-weather-night' : 'mdi-weather-sunny'"
          size="lg"
          keep-color
          @click="$q.dark.toggle()"
        >
          <q-tooltip v-if="!$q.platform.is.mobile">{{ t('menu.darkMode') }}</q-tooltip>
        </q-toggle>

        <q-separator spaced vertical />

        <switch-language v-if="!$q.platform.is.mobile" class="q-mx-auto" />

        <q-separator
          v-if="!$q.platform.is.mobile"
          spaced
          vertical
          color="transparent"
        />

        <q-separator spaced vertical />

        <q-btn
          dense
          flat
          round
          :icon="showDrawer ? 'mdi-menu' : 'mdi-menu-open'"
          @click="showDrawer = !showDrawer"
        />
      </q-toolbar>

      <right-drawer v-model="showDrawer" :menuItems="menuItems" />
    </q-header>
    <q-page-container>
      <router-view />
    </q-page-container>
    <page-footer v-if="showFooter" />
  </q-layout>
</template>

<script>
import { ref, onBeforeMount } from 'vue';
import { useI18n } from 'vue-i18n';
import { useUserStore } from 'src/store/modules/user';
import { useWeighingStore } from 'src/store/modules/weighing';
import { useIngredientStore } from 'src/store/modules/ingredient';
import { useIngredientCategoryStore } from 'src/store/modules/ingredientCategory';
import PageFooter from '../components/PageFooter';
import RightDrawer from '../components/RightDrawer.vue';
import LogoHeader from '../components/LogoHeader';
import SwitchLanguage from '../components/SwitchLanguage';
import BreadCrumb from '../components/BreadCrumb';

export default {
  name: 'MainLayout',
  components: {
    PageFooter,
    RightDrawer,
    SwitchLanguage,
    LogoHeader,
    BreadCrumb,
  },
  setup() {
    const { t } = useI18n();
    const leftDrawerOpen = ref(true);
    const showDrawer = ref(true);
    const showToolbar = ref(true);
    const showFooter = ref(true);
    const dark = ref(true);

    onBeforeMount(async () => {
      const userStore = useUserStore();
      await userStore.fetchUser();
      const weighingStore = useWeighingStore();
      await weighingStore.fetchWeighings();
      const ingredientStore = useIngredientStore();
      await ingredientStore.fetchAll();
      const ingredientCategoryStore = useIngredientCategoryStore();
      await ingredientCategoryStore.fetchAll();
    });

    const userStore = useUserStore();

    const menuItems = [
      {
        icon: 'dashboard',
        label: t('menu.dashboard'),
        separator: true,
        routeName: 'page-user-dashboard',
      },
      {
        icon: 'account_circle',
        label: t('menu.user'),
        separator: true,
        routeName: 'page-user-details',
      },
      {
        icon: 'monitor_weight',
        label: t('menu.weighings'),
        separator: true,
        routeName: 'page-user-weighings',
      },
      {
        icon: 'restaurant',
        label: t('menu.mealDeclarations'),
        separator: true,
        routeName: 'page-mealDeclarations',
      },
      {
        icon: 'kitchen',
        label: t('menu.ingredients'),
        separator: true,
        routeName: 'page-ingredients',
      },
      {
        icon: 'dinner_dining',
        label: t('menu.recipes'),
        separator: true,
        routeName: 'page-recipes',
      },
    ];

    return {
      showDrawer,
      userStore,
      leftDrawerOpen,
      toggleLeftDrawer() {
        leftDrawerOpen.value = !leftDrawerOpen.value;
      },
      t,
      showToolbar,
      showFooter,
      dark,
      menuItems,
    };
  },
};
</script>
