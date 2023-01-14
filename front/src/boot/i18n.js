// import { boot } from 'quasar/wrappers';
// import { createI18n } from 'vue-i18n';
// import messages from 'src/i18n';

// const i18n = createI18n({
//   legacy: false,
//   locale: 'fr',
//   fallbackLocale: 'en',
//   silentTranslationWarn: true,
//   warnHtmlInMessage: 'off',
//   messages,
// });

// export default boot(({ app }) => {
//   app.use(i18n);
// });

// export { i18n };

import { createI18n } from 'vue-i18n'
import messages from 'src/i18n'

export default ({ app }) => {
  // Create I18n instance
  const i18n = createI18n({
    legacy: false,
    locale: 'fr',
    fallbackLocale: 'en',
    silentTranslationWarn: true,
    warnHtmlInMessage: 'off',
    messages,
  });

  // Tell app to use the I18n instance
  app.use(i18n)
}