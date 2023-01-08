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
import { useWeighingStore } from 'src/store/modules/weighing';

export default {
  name: 'WeighingList',
  setup(props, { emit }) {
    const internal = ref({});
    const { t } = useI18n();
    const weighingStore = useWeighingStore();

    const commonColumnsAttributes = {
      style: 'max-width: 200px',
      headerStyle: 'max-width: 200px',
      classes: 'ellipsis',
    };

    const refreshInternal = (val) => {
      internal.value = cloneDeep(val);
      internal.value.sort((a, b) => new Date(a.date) - new Date(b.date));
    };

    refreshInternal(weighingStore.weighings);

    watch(
      () => weighingStore.weighings,
      (newVal) => {
        refreshInternal(newVal);
      }
    );

    const columns = [
      {
        name: 'weight',
        label: t('weighing.weight'),
        align: 'left',
        field: (row) => row.weight,
        style: 'width: 20px',
        ...commonColumnsAttributes,
      },
      {
        name: 'date',
        label: t('weighing.date'),
        align: 'left',
        field: (row) => row.date,
        style: 'width: 20px',
        ...commonColumnsAttributes,
      },
      {
        name: 'body_fat_percentage',
        label: t('weighing.body_fat_percentage'),
        align: 'left',
        field: (row) => row.body_fat_percentage,
        style: 'width: 20px',
        ...commonColumnsAttributes,
      },
      {
        name: 'waist_circumference',
        label: t('weighing.waist_circumference'),
        align: 'left',
        field: (row) => row.waist_circumference,
        style: 'width: 20px',
        ...commonColumnsAttributes,
      },
      {
        name: 'waist_size',
        label: t('weighing.waist_size'),
        align: 'left',
        field: (row) => row.waist_size,
        style: 'width: 20px',
        ...commonColumnsAttributes,
      },
    ];

    const onRowClicked = (evt, row) => {
      weighingStore.changeSelectedWeighing(row);
      emit('weighing-show-details');
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
