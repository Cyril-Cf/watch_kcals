<template>
  <q-card v-if="internal">
    <q-card-section>
      <div class="row justify-between">
        <div class="col-12">
          <q-select
            v-model="internal.ingredient_detail_type"
            :options="ingredientDetailTypesOptions"
            map-options
            emit-value
            :label="t('ingredientDetail.ingredientDetailType')"
          />
        </div>
      </div>
    </q-card-section>
    <q-card-section>
      <div class="row justify-between">
        <div class="col-5">
          <q-input
            v-model="internal.calories"
            type="number"
            borderless
            :label="t('ingredientDetail.calories')"
          />
        </div>

        <div class="col-5">
          <q-input
            v-model="internal.proteins"
            type="number"
            borderless
            :label="t('ingredientDetail.proteins')"
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
import { useIngredientStore } from 'src/store/modules/ingredient';
import { useIngredientDetailStore } from 'src/store/modules/ingredientDetail';

export default {
  name: 'EditIngredientDetail',
  props: {
    ingredientDetail: {
      type: Object,
      required: false,
    },
    isNew: {
      type: Boolean,
      required: true,
    },
  },
  setup(props, { emit }) {
    const internal = ref({});
    const ingredientDetailStore = useIngredientDetailStore();
    const ingredientStore = useIngredientStore();
    const { t } = useI18n();
    const $q = useQuasar();
    const btnLabel = ref('');
    const ingredientDetailTypesOptions = [
      {
        label: t('ingredientDetail.byGram'),
        value: 'ByGrams',
      },
      {
        label: t('ingredientDetail.byPiece'),
        value: 'ByPiece',
      },
    ];

    const switchBtnLabel = () => {
      if (!props.isNew) {
        btnLabel.value = t('ingredientDetail.updateIngredientDetail');
      } else {
        btnLabel.value = t('ingredientDetail.createIngredientDetail');
      }
    };
    switchBtnLabel();

    const refreshInternal = (val) => {
      internal.value = cloneDeep(val);
      switchBtnLabel();
    };

    if (!props.isNew) {
      refreshInternal(ingredientDetailStore.selected);
    }

    if (props.ingredientDetail) {
        refreshInternal(props.ingredientDetail);
    }

    const upsert = async () => {
      const req = {};
      req.calories = +internal.value.calories;
      req.proteins = +internal.value.proteins;
      req.ingredient_detail_type = internal.value.ingredient_detail_type;
      req.ingredient_id = ingredientStore.selected.id;
      if (!props.isNew) {
        await ingredientDetailStore.update(req, internal.value.id);
        await ingredientDetailStore.fetchAllForOneIngredient(
          ingredientStore.selected.id
        );
        $q.notify({
          type: 'positive',
          message: `${t('ingredientDetail.ingredientDetailUpdated')}`,
        });
      } else {
        await ingredientDetailStore.addOne(req);
        await ingredientDetailStore.fetchAllForOneIngredient(
          ingredientStore.selected.id
        );
        emit('close-dialog');
        $q.notify({
          type: 'positive',
          message: `${t('ingredientDetail.ingredientDetailCreated')}`,
        });
      }
    };

    return {
      internal,
      upsert,
      t,
      btnLabel,
      ingredientDetailTypesOptions,
    };
  },
};
</script>
