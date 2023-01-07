import { defineStore } from 'pinia'
import { api } from 'boot/axios'

export const useIngredientCategoryStore = defineStore({
    id: 'ingredientCategoryStore',
    state: () => ({
        loading: false,
        allIngredientCategories: [],
    }),
    actions: {
        async fetchAll() {
            try {
                this.loading = true;
                this.allIngredientCategories = (await api.get('/ingredient_categories')).data
            } catch (err) {
                console.log(err);
            } finally {
                this.loading = false;
            }
        },
    }
})