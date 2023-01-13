<template>
  <q-card v-if="internal">
    <q-card-section>
      <div class="row justify-between">
        <div class="col-12">
          <q-input
            v-model="internal.name"
            borderless
            :label="t('ingredient.name')"
          />
        </div>
        <div class="col-5">
          <q-select
            v-model="internal.ingredient_category_id"
            :options="ingredientCategoriesOptions"
            map-options
            emit-value
            :label="t('ingredient.ingredientCategory')"
          />
        </div>
      </div>
    </q-card-section>
    <q-card-actions>
      <div class="row flex justify-between">
        <q-btn round color="primary" icon="save" @click="upsert">
          <q-tooltip>{{ btnLabel }}</q-tooltip>
        </q-btn>
      </div>
    </q-card-actions>
    <q-card-actions>
      <div class="row flex justify-between">
        <q-btn
          flat
          unelevated
          color="primary"
          @click="dialogNewIngredientDetail = true"
          v-if="!isNew"
        >
          {{ t('ingredientDetail.newIngredientDetail') }}
        </q-btn>
        <q-dialog v-model="dialogNewIngredientDetail">
          <edit-ingredient-detail
            v-on:close-dialog="dialogNewIngredientDetail = false"
            :isNew="true"
          />
        </q-dialog>
      </div>
    </q-card-actions>
    <q-card-section v-if="!isNew">
      <edit-ingredient-detail
        v-for="ingredientDetail in ingredientDetailStore.all"
        :key="ingredientDetail.id"
        :isNew="false"
        :ingredientDetail="ingredientDetail"
      />
    </q-card-section>
  </q-card>
</template>

<script>
import { useI18n } from 'vue-i18n';
import { ref, watch } from 'vue';
import { useQuasar } from 'quasar';
import cloneDeep from 'lodash/cloneDeep';
import { useIngredientStore } from 'src/store/modules/ingredient';
import { useIngredientCategoryStore } from 'src/store/modules/ingredientCategory';
import EditIngredientDetail from './EditIngredientDetail';
import { useIngredientDetailStore } from 'src/store/modules/ingredientDetail';

export default {
  name: 'EditIngredient',
  props: {
    isNew: {
      type: Boolean,
      required: true,
    },
  },
  components: {
    EditIngredientDetail,
  },
  setup(props, { emit }) {
    const internal = ref({});
    const ingredientStore = useIngredientStore();
    const ingredientCategoryStore = useIngredientCategoryStore();
    const ingredientDetailStore = useIngredientDetailStore();
    const { t } = useI18n();
    const $q = useQuasar();
    const btnLabel = ref('');

    const ingredientCategoriesOptions = ref([]);

    const switchBtnLabel = () => {
      if (!props.isNew) {
        btnLabel.value = t('ingredient.updateIngredient');
      } else {
        btnLabel.value = t('ingredient.createIngredient');
      }
    };
    switchBtnLabel();

    const refreshInternal = (val) => {
      internal.value = cloneDeep(val);
      switchBtnLabel();
    };
    if (!props.isNew) {
      refreshInternal(ingredientStore.selected);
      ingredientDetailStore.fetchAllForOneIngredient(
        ingredientStore.selected.id
      );
    }

    const refreshOptions = (array) => {
      ingredientCategoriesOptions.value = [];
      array.map((ic) => {
        ingredientCategoriesOptions.value.push({
          label: ic[0].name,
          value: ic[0].id,
        });
      });
    };
    refreshOptions(ingredientCategoryStore.all);

    watch(
      () => ingredientCategoryStore.all,
      (newVal) => {
        refreshOptions(newVal);
      }
    );

    watch(
      () => ingredientStore.selected,
      (newVal) => {
        ingredientDetailStore.fetchAllForOneIngredient(newVal.id);
      }
    );

    const upsert = async () => {
      const req = {};
      req.name = internal.value.name;
      req.ingredient_category_id = internal.value.ingredient_category_id;
      if (!props.isNew) {
        await ingredientStore.update(req);
        await ingredientStore.fetchAll();
        emit('close-dialog');
        $q.notify({
          type: 'positive',
          message: `${t('ingredient.ingredientUpdated')}`,
        });
      } else {
        await ingredientStore.addOne(req);
        await ingredientStore.fetchAll();
        emit('close-dialog');
        $q.notify({
          type: 'positive',
          message: `${t('ingredient.ingredientCreated')}`,
        });
      }
    };

    return {
      internal,
      ingredientCategoriesOptions,
      upsert,
      t,
      btnLabel,
      dialogNewIngredientDetail: ref(false),
      ingredientDetailStore,
    };
  },
};
</script>
