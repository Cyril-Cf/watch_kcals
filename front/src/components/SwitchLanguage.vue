<template>
  <q-btn-dropdown :style="style" no-caps color="primary">
    <template #label>
      <img
        :src="'flags/' + $root.$i18n.locale + '.svg'"
        width="28"
        :alt="$root.$i18n.locale.substr(0, 2).toUpperCase()"
        class="q-mr-md"
      />
      {{ $q.platform.is.mobile ? '' : currentLocale }}
    </template>

    <q-list>
      <q-item
        v-for="(locale, index) in $root.$i18n.availableLocales"
        :key="index"
        v-close-popup
        clickable
        class="items-center"
        @click="setLocale(locale)"
      >
        <q-item-section side>
          <q-icon
            :color="locale === $root.$i18n.locale ? null : 'transparent'"
            name="mdi-check"
          />
        </q-item-section>
        <q-item-label>{{ $t(`menu.lang.${locale}`) }}</q-item-label>
      </q-item>
    </q-list>
  </q-btn-dropdown>
</template>

<script>
import { watch, computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { useQuasar } from 'quasar';
export default {
  name: 'SwitchLanguage',
  setup() {
    const { t, locale } = useI18n({ useScope: 'global' });
    const currentLocale = computed(() => {
      return t(`menu.lang.${locale.value}`);
    });
    const $q = useQuasar();

    watch(locale, (val) => {
      // dynamic import, so loading on demand only
      import(
        /* webpackInclude: /(fr|en-US)\.js$/ */
        'quasar/lang/' + (val === 'en' ? 'en-US' : 'fr')
      ).then((lang) => {
        $q.lang.set(lang.default);
      });
    });

    const setLocale = (lang) => {
      locale.value = lang;
    };

    const style = computed(() => {
      if ($q.platform.is.mobile) {
        return '';
      }
      return 'width: 150px;';
    });
    return {
      setLocale,
      currentLocale,
      style,
    };
  },
};
</script>
