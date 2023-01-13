<template>
  <div>
    <q-table
      :rows="internal"
      :dense="$q.screen.sm"
      :grid="$q.screen.xs"
      :columns="columns"
      class="shadow-0"
      style="table-layout: fixed"
      @row-click="onRowClicked"
    >
    </q-table>
  </div>
</template>

<script>
import { useI18n } from 'vue-i18n';
import { ref, watch } from 'vue';
import cloneDeep from 'lodash/cloneDeep';
import { useIngredientCategoryStore } from 'src/store/modules/ingredientCategory';

export default {
  name: 'IngredientCategoryList',
  setup(props, { emit }) {
    const internal = ref([]);
    const { t } = useI18n();
    const ingredientCategoryStore = useIngredientCategoryStore();

    const commonColumnsAttributes = {
      style: 'max-width: 200px',
      headerStyle: 'max-width: 200px',
      classes: 'ellipsis',
    };

    const refreshInternal = (val) => {
      internal.value = cloneDeep(val);
      internal.value.sort((a, b) => (a[0].name < b[0].name ? -1 : 1));
    };

    refreshInternal(ingredientCategoryStore.all);

    watch(
      () => ingredientCategoryStore.all,
      (newVal) => {
        refreshInternal(newVal);
      }
    );

    const columns = [
      {
        name: 'name',
        label: t('ingredientCategory.name'),
        align: 'left',
        field: (row) => row[0].name,
        style: 'width: 20px',
        ...commonColumnsAttributes,
      },
    ];

    const onRowClicked = (evt, row) => {
      ingredientCategoryStore.changeSelected(row);
      emit('show-details');
    };

    return {
      t,
      columns,
      internal,
      onRowClicked,
    };
  },
};
</script>

<style scoped lang="scss">
.q-table__container {
  .q-img {
    width: 30%;
    margin-right: 20px;

    img {
      max-height: 100px;
      width: auto;
    }
  }
}
</style>
