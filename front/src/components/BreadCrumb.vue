<template>
  <div class="q-pa-md">
    <q-toolbar>
      <q-breadcrumbs active-color="secondary" style="font-size: 16px">
        <q-breadcrumbs-el label="Home" icon="home" :to="home" />
        <q-breadcrumbs-el :label="route" :icon="icon" :to="$route.path" />
      </q-breadcrumbs>
    </q-toolbar>
  </div>
</template>

<script>
import { computed } from 'vue';
import { useRoute } from 'vue-router';
export default {
  name: 'LogoHeader',
  props: {
    menuItems: {
      type: Array,
      required: true,
    },
  },
  setup(props) {
    const route = computed(() => {
      let route = '';
      props.menuItems.forEach((item) => {
        if (item.routeName === useRoute().name) {
          route = item.label;
        }
      });
      return route;
    });
    const icon = computed(() => {
      let icon = '';
      props.menuItems.forEach((item) => {
        if (item.routeName === useRoute().name) {
          icon = item.icon;
        }
      });
      return icon;
    });
    const home = process.env.VUE_ROUTER_BASE;
    return {
      home,
      route,
      icon,
    };
  },
};
</script>
