<template>
  <div class="row justify-center">
    <div class="col-10 q-mt-xl">
      <q-tabs
        v-model="tab"
        dense
        class="text-grey"
        active-color="primary"
        indicator-color="primary"
        align="left"
        narrow-indicator
      >
        <q-tab
          name="ingredients"
          :label="t('menu.tabs.ingredient.ingredients')"
        />
        <q-tab
          name="ingredientsCategories"
          :label="t('menu.tabs.ingredient.ingredientCategories')"
        />
      </q-tabs>
    </div>
    <q-separator />
    <div class="col-12">
      <q-tab-panels v-model="tab">
        <q-tab-panel name="ingredients">
          <div class="row">
            <div class="col-12 col-lg-5">
              <q-btn
                round
                color="primary"
                icon="add"
                @click="dialogNewIngredient = true"
              >
                <q-tooltip>{{
                  t('ingredient.newIngredient')
                }}</q-tooltip>
              </q-btn>
              <q-dialog v-model="dialogNewIngredient">
                <edit-ingredient
                  v-on:close-dialog="dialogNewIngredient = false"
                  :isNew="true"
                />
              </q-dialog>
              <ingredient-list
                v-on:show-details="dialogShowIngredient = true"
              />
              <q-dialog v-model="dialogShowIngredient">
                <edit-ingredient
                  v-on:close-dialog="dialogShowIngredient = false"
                  :isNew="false"
                />
              </q-dialog>
            </div>
          </div>
        </q-tab-panel>
        <q-tab-panel name="ingredientsCategories">
          <div class="row">
            <div class="col-12 col-lg-5">
              <q-btn
                round
                color="primary"
                icon="add"
                @click="dialogNewIngredientCategory = true"
              >
                <q-tooltip>{{
                  t('ingredientCategory.newIngredientCategory')
                }}</q-tooltip>
              </q-btn>
              <q-dialog v-model="dialogNewIngredientCategory">
                <edit-ingredient-category
                  v-on:close-dialog="dialogNewIngredientCategory = false"
                  :isNew="true"
                />
              </q-dialog>
              <ingredient-category-list
                v-on:show-details="dialogShowIngredientCategory = true"
              />
              <q-dialog v-model="dialogShowIngredientCategory">
                <edit-ingredient-category
                  v-on:close-dialog="dialogShowIngredientCategory = false"
                  :isNew="false"
                />
              </q-dialog>
            </div>
          </div>
        </q-tab-panel>
      </q-tab-panels>
    </div>
  </div>
</template>

<script>
import { useI18n } from 'vue-i18n';
import { ref } from 'vue';
import IngredientList from '../components/ingredients/IngredientList.vue';
import EditIngredient from '../components/ingredients/EditIngredient.vue';
import IngredientCategoryList from '../components/ingredients/IngredientCategoryList.vue';
import EditIngredientCategory from '../components/ingredients/EditIngredientCategory.vue';

export default {
  name: 'PageIngredients',
  components: {
    IngredientList,
    EditIngredient,
    IngredientCategoryList,
    EditIngredientCategory,
  },
  setup() {
    const { t } = useI18n();
    return {
      t,
      tab: ref('ingredients'),
      dialogNewIngredient: ref(false),
      dialogShowIngredient: ref(false),
      dialogNewIngredientCategory: ref(false),
      dialogShowIngredientCategory: ref(false),
    };
  },
};
</script>
