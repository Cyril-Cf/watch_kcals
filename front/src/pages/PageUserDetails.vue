<template>
  <div class="row justify-center">
    <div class="col-10 q-mt-xl">
      <edit-user />
    </div>
    <q-btn flat unelevated color="primary" @click="dialogNewWeighing = true">
      {{ t('weighing.newWeighing') }}
    </q-btn>
    <q-dialog v-model="dialogNewWeighing">
      <edit-weighing v-on:close-dialog="dialogNewWeighing = false" :isNew="true" />
    </q-dialog>
    <weighing-list v-on:weighing-show-details="dialogShowWeighing = true" />
    <q-dialog v-model="dialogShowWeighing">
      <edit-weighing v-on:close-dialog="dialogShowWeighing = false" :isNew="false" />
    </q-dialog>
  </div>
</template>

<script>
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import EditUser from '../components/user/EditUser.vue';
import EditWeighing from '../components/user/EditWeighing.vue';
import WeighingList from '../components/user/WeighingList.vue';

export default {
  name: 'UserDetails',
  components: {
    EditUser,
    EditWeighing,
    WeighingList,
  },
  setup() {
    const { t } = useI18n();
    let dialogNewWeighing = ref(false);
    let dialogShowWeighing = ref(false);
    return {
      dialogNewWeighing,
      dialogShowWeighing,
      t,
    };
  },
};
</script>
