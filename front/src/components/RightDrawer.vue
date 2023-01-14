<template>
  <q-drawer
    show-if-above
    side="right"
    overlay
    bordered
    no-swipe-open
    no-swipe-close
    no-swipe-backdrop
  >
    <q-scroll-area class="fit">
      <q-list>
        <template v-for="(menuItem, index) in menuItems" :key="index">
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
  </q-drawer>
</template>

<script>
import { ref } from 'vue';

export default {
  name: 'RightDrawer',
  props: {
    menuItems: {
      type: Array,
      required: true,
    },
  },
  setup() {
    return {
      drawer: ref(false),
    };
  },
};
</script>

<style lang="sass" scoped>
.menu-list .q-item
  border-radius: 0 32px 32px 0
</style>
