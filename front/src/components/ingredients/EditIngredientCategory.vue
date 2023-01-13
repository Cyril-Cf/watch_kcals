<template>
  <q-card v-if="internal">
    <q-card-section>
      <div class="row justify-between">
        <div class="col-12">
          <q-input
            v-model="internal[0].name"
            borderless
            :label="t('ingredientCategory.name')"
          />
        </div>
      </div>
    </q-card-section>
    <q-card-actions>
      <div class="row flex justify-center items-center">
        <q-btn flat unelevated color="primary" @click="upsert">
          {{ btnLabel }}
        </q-btn>
      </div>
    </q-card-actions>
  </q-card>
</template>

<script>
import { useI18n } from 'vue-i18n';
import { ref } from 'vue';
import { useQuasar } from 'quasar';
import cloneDeep from 'lodash/cloneDeep';
import { useIngredientCategoryStore } from 'src/store/modules/ingredientCategory';

export default {
  name: 'EditIngredientCategory',
  props: {
    isNew: {
      type: Boolean,
      required: true,
    },
  },
  setup(props, { emit }) {
    const internal = ref([{}]);
    const ingredientCategoryStore = useIngredientCategoryStore();
    const { t } = useI18n();
    const $q = useQuasar();
    const btnLabel = ref('');

    const switchBtnLabel = () => {
      if (!props.isNew) {
        btnLabel.value = t('ingredientCategory.updateIngredientCategory');
      } else {
        btnLabel.value = t('ingredientCategory.createIngredientCategory');
      }
    };
    switchBtnLabel();

    const refreshInternal = (val) => {
      internal.value = cloneDeep(val);
      switchBtnLabel();
    };

    if (!props.isNew) {
      refreshInternal(ingredientCategoryStore.selected);
    }

    const upsert = async () => {
      const req = {};
      req.name = internal.value[0].name;
      if (!props.isNew) {
        await ingredientCategoryStore.update(req);
        await ingredientCategoryStore.fetchAll();
        emit('close-dialog');
        $q.notify({
          type: 'positive',
          message: `${t('ingredientCategory.ingredientCategoryUpdated')}`,
        });
      } else {
        await ingredientCategoryStore.addOne(req);
        await ingredientCategoryStore.fetchAll();
        emit('close-dialog');
        $q.notify({
          type: 'positive',
          message: `${t('ingredientCategory.ingredientCategoryCreated')}`,
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
