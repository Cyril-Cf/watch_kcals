<template>
  <q-footer class="bg-primary text-white">
    <q-toolbar>
      <q-toolbar-title>{{ title }}</q-toolbar-title>

      <q-space />

      <q-btn-toggle
        v-model="model"
        flat
        stretch
        toggle-color="yellow"
        :options="options"
      />
    </q-toolbar>
  </q-footer>
</template>

<script>
import { ref } from 'vue';

export default {
  setup() {
    const title = ref('Footer Title');

    const model = ref('one');

    const options = [
      { label: 'Contact', value: 'contact' },
      { label: 'Documentation', value: 'documentation' },
      { label: 'TBA', value: 'waiting' },
    ];

    return { title, model, options };
  },
};
</script>
