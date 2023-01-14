<template>
  <div>
    <q-table
      grid
      :title="$t('menu.tabs.ingredient.ingredients')"
      row-key="name"
      :filter="filter"
      hide-header
      :rows="internal"
      :dense="$q.screen.sm"
      :columns="columns"
      class="shadow-0"
      style="table-layout: fixed"
      @row-click="onRowClicked"
    >
      <template v-slot:top-right>
        <q-input
          borderless
          dense
          debounce="300"
          v-model="filter"
          :placeholder="$t('generic.filter')"
        >
          <template v-slot:append>
            <q-icon name="search" />
          </template>
        </q-input>
      </template>
    </q-table>
  </div>
</template>

<script>
import { useI18n } from 'vue-i18n';
import { ref, watch } from 'vue';
import cloneDeep from 'lodash/cloneDeep';
import { useIngredientStore } from 'src/store/modules/ingredient';

export default {
  name: 'IngredientList',
  setup(props, { emit }) {
    const internal = ref([]);
    const { t } = useI18n();
    const ingredientStore = useIngredientStore();

    const commonColumnsAttributes = {
      style: 'max-width: 200px',
      headerStyle: 'max-width: 200px',
      classes: 'ellipsis',
    };

    const refreshInternal = (val) => {
      internal.value = cloneDeep(val);
      internal.value.sort((a, b) => (a.name < b.name ? -1 : 1));
    };

    refreshInternal(ingredientStore.all);

    watch(
      () => ingredientStore.all,
      (newVal) => {
        refreshInternal(newVal);
      }
    );

    const columns = [
      {
        name: 'name',
        // label: t('ingredient.name'),
        align: 'left',
        field: (row) => row.name,
        style: 'width: 20px',
        ...commonColumnsAttributes,
      },
    ];

    const onRowClicked = (evt, row) => {
      ingredientStore.changeSelected(row);
      emit('show-details');
    };

    return {
      t,
      columns,
      internal,
      onRowClicked,
      filter: ref(''),
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
