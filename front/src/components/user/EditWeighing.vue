<template>
  <q-card v-if="internal">
    <q-card-section>
      <div class="row justify-between">
        <div class="col-5">
          <q-input
            v-model="internal.weight"
            type="number"
            borderless
            :label="t('weighing.weight')"
          />
        </div>
        <div class="col-5">
          <q-input
            v-model="internal.body_fat_percentage"
            type="number"
            borderless
            :label="t('weighing.body_fat_percentage')"
          />
        </div>
      </div>
    </q-card-section>
    <q-card-section>
      <div class="row justify-between">
        <div class="col-3">
          <q-input
            v-model="internal.waist_circumference"
            type="number"
            borderless
            :label="t('weighing.waist_circumference')"
          />
        </div>

        <div class="col-3">
          <q-input
            v-model="internal.waist_size"
            type="number"
            borderless
            :label="t('weighing.waist_size')"
          />
        </div>

        <div class="col-3">
          <q-input
            v-model="internal.date"
            filled
            type="date"
            :label="t('weighing.date')"
          />
        </div>
      </div>
    </q-card-section>
    <q-card-actions>
      <div class="row flex justify-center items-center">
        <q-btn round color="primary" icon="save" @click="upsert">
          <q-tooltip>{{ btnLabel }}</q-tooltip>
        </q-btn>
      </div>
    </q-card-actions>
  </q-card>
</template>

<script>
import { useI18n } from 'vue-i18n';
import { ref } from 'vue';
import { useWeighingStore } from 'src/store/modules/weighing';
import { useQuasar } from 'quasar';
import cloneDeep from 'lodash/cloneDeep';

export default {
  name: 'EditWeighing',
  props: {
    isNew: {
      type: Boolean,
      required: true,
    },
  },
  setup(props, { emit }) {
    const internal = ref({});
    const weighingStore = useWeighingStore();
    const { t } = useI18n();
    const $q = useQuasar();
    const btnLabel = ref('');

    const switchBtnLabel = () => {
      if (!props.isNew) {
        btnLabel.value = t('weighing.update_weighing');
      } else {
        btnLabel.value = t('weighing.create_weighing');
      }
    };
    switchBtnLabel();

    const refreshInternal = (val) => {
      internal.value = cloneDeep(val);
      switchBtnLabel();
    };

    if (!props.isNew) {
      refreshInternal(weighingStore.weighingSelected);
    }

    const upsert = async () => {
      if (!props.isNew) {
        await weighingStore.updateWeighing(internal.value);
        await weighingStore.fetchWeighings();
        emit('close-dialog');
        $q.notify({
          type: 'positive',
          message: `${t('weighing.weighing_updated')}`,
        });
      } else {
        await weighingStore.addWeighing(internal.value);
        await weighingStore.fetchWeighings();
        emit('close-dialog');
        $q.notify({
          type: 'positive',
          message: `${t('weighing.weighing_created')}`,
        });
      }
    };

    return {
      internal,
      upsert,
      t,
      btnLabel,
    };
  },
};
</script>
