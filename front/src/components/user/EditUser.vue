<template>
  <q-card>
    <q-card-section>
      <div class="row justify-between">
        <div class="col-5">
          <q-input v-model="internal.name" borderless :label="t('user.name')" />
        </div>
        <div class="col-5">
          <q-select
            v-model="internal.gender"
            :options="genderOptions"
            map-options
            emit-value
            :label="t('user.gender')"
          />
        </div>
      </div>
    </q-card-section>
    <q-card-section>
      <div class="row justify-between">
        <div class="col-3">
          <q-input
            v-model="internal.height"
            type="number"
            borderless
            :label="t('user.height')"
          />
        </div>

        <div class="col-3">
          <q-input
            v-model="internal.physical_activity_level"
            type="number"
            min="1"
            max="5"
            step="1"
            borderless
            :label="t('user.physical_activity_level')"
          >
            <q-tooltip>
              {{ t('user.physical_activity_level_explanation1') }}
              <br />
              {{ t('user.physical_activity_level_explanation2') }}
              <br />
              {{ t('user.physical_activity_level_explanation3') }}
              <br />
              {{ t('user.physical_activity_level_explanation4') }}
              <br />
              {{ t('user.physical_activity_level_explanation5') }}
            </q-tooltip>
          </q-input>
        </div>

        <div class="col-3">
          <q-input
            v-model="internal.date_of_birth"
            filled
            type="date"
            :label="t('user.date_of_birth')"
          />
        </div>
      </div>
    </q-card-section>
    <q-card-actions>
      <div class="row flex justify-center items-center">
        <q-btn round color="primary" icon="save" @click="upsertUser">
          <q-tooltip>{{ btnLabel }}</q-tooltip>
        </q-btn>
      </div>
    </q-card-actions>
  </q-card>
</template>

<script>
import { useI18n } from 'vue-i18n';
import { watch, ref } from 'vue';
import { useUserStore } from 'src/store/modules/user';
import { useQuasar } from 'quasar';
import cloneDeep from 'lodash/cloneDeep';

export default {
  name: 'EditUser',
  setup() {
    const internal = ref({});
    const userStore = useUserStore();
    const { t } = useI18n();
    const $q = useQuasar();
    const btnLabel = ref('');

    const isUserDefined = () => {
      return Object.keys(userStore.user).length > 0;
    };

    const switchBtnLabel = () => {
      if (isUserDefined()) {
        btnLabel.value = t('user.update_user');
      } else {
        btnLabel.value = t('user.create_user');
      }
    };

    const refreshInternal = (val) => {
      internal.value = cloneDeep(val);
      switchBtnLabel();
    };
    refreshInternal(userStore.user);

    watch(
      () => userStore.user,
      (newVal) => {
        refreshInternal(newVal);
      }
    );

    watch(
      () => internal.value.physical_activity_level,
      (newVal) => {
        if (newVal < 1) {
          internal.value.physical_activity_level = 1;
        }
        if (newVal > 5) {
          internal.value.physical_activity_level = 5;
        }
      }
    );

    const genderOptions = [
      {
        label: t('user.genders.male'),
        value: 'Male',
      },
      {
        label: t('user.genders.female'),
        value: 'Female',
      },
    ];

    function upsertUser() {
      if (isUserDefined()) {
        userStore.updateUser(internal.value);
        $q.notify({
          type: 'positive',
          message: `${t('user.user_updated')}`,
        });
      } else {
        userStore.addUser(internal.value);
        $q.notify({
          type: 'positive',
          message: `${t('user.user_created')}`,
        });
      }
    }

    return {
      internal,
      upsertUser,
      t,
      genderOptions,
      btnLabel,
    };
  },
};
</script>
<style scoped lang="scss">
.text-block {
  white-space: pre-line;
}
</style>
