<template>
  <q-layout view="lHh Lpr lFf">
    <q-header elevated>
      <q-toolbar>
        <q-btn
          flat
          dense
          round
          icon="menu"
          aria-label="Menu"
          @click="toggleLeftDrawer"
        />

        <q-toolbar-title> </q-toolbar-title>
      </q-toolbar>
    </q-header>

    <q-drawer
      v-model="leftDrawerOpen"
      show-if-above
      :width="250"
      :breakpoint="1000"
    >
      <q-scroll-area
        style="
          height: calc(100% - 150px);
          margin-top: 150px;
          border-right: 1px solid #ddd;
        "
      >
        <q-list padding>
          <template v-for="(menuItem, index) in menuList" :key="index">
            <q-item
              clickable
              :to="{ name: menuItem.routeName }"
              :active="$router.currentRoute.value.name === menuItem.routeName"
              v-ripple
            >
              <q-item-section avatar>
                <q-icon :name="menuItem.icon" />
              </q-item-section>
              <q-item-section>
                {{ menuItem.label }}
              </q-item-section>
            </q-item>
            <q-separator :key="'sep' + index" v-if="menuItem.separator" />
          </template>
        </q-list>
      </q-scroll-area>

      <q-img class="absolute-top loggedPanel" style="height: 150px">
        <div class="absolute-bottom bg-transparent">
          <q-avatar size="80px" class="q-mb-sm">
            <q-icon name="person"></q-icon>
          </q-avatar>
          <div class="text-weight-bold">Bonjour {{ userStore.user.name }}</div>
        </div>
      </q-img>
    </q-drawer>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script>
import { ref, onBeforeMount } from 'vue';
import { useI18n } from 'vue-i18n';
import { useUserStore } from 'src/store/modules/user';
import { useWeighingStore } from 'src/store/modules/weighing';

export default {
  name: 'MainLayout',
  setup() {
    const { t } = useI18n();
    const leftDrawerOpen = ref(true);

    const menuList = [
      {
        icon: 'description',
        label: t('menu.user'),
        separator: true,
        routeName: 'page-user-details',
      },
      {
        icon: 'description',
        label: t('menu.ingredients'),
        separator: true,
        routeName: 'page-ingredients',
      },
    ];

    onBeforeMount(async () => {
      const userStore = useUserStore();
      await userStore.fetchUser();
      const weighingStore = useWeighingStore();
      await weighingStore.fetchWeighings();
    });

    const userStore = useUserStore();

    return {
      userStore,
      menuList,
      leftDrawerOpen,
      toggleLeftDrawer() {
        leftDrawerOpen.value = !leftDrawerOpen.value;
      },
      t,
    };
  },
};
</script>
<style lang="sass" scoped>
.loggedPanel
  background-color: $dark

.logout
  color: var(--q-secondary)
  cursor: pointer
</style>
