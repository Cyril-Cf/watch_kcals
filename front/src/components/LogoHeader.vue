<template>
  <div>
    <q-btn no-caps flat :icon="$q.config.logo">
      {{ '&nbsp;' + $q.config.sitename }}
    </q-btn>
    <q-tooltip>{{ $t('menu.slogan') }}</q-tooltip>
  </div>
</template>

<script>
export default {
  name: 'LogoHeader',
  setup() {
    return {};
  },
};
</script>


<style lang="scss">
  .logo
  {
    background-size: cover;
    vertical-align: middle;
  }

  .desktop .logo
  {
    width: 167px;
    background-image: url('../../public/images/yawik_logo-desktop.svg');
  }

  .mobile .logo
  {
    width: 53px;
    background-image: url('../../public/images/yawik_logo-mobile.svg');
  }

</style>