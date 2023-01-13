import { defineStore } from 'pinia';
import { api } from 'boot/axios';

const API_RESSOURCE = '/ingredient_categories';

export const useIngredientCategoryStore = defineStore({
  id: 'ingredientCategoryStore',
  state: () => ({
    loading: false,
    all: [],
    selected: {},
  }),
  actions: {
    changeSelected(object) {
      this.selected = object;
    },
    async fetchAll() {
      try {
        this.loading = true;
        this.all = (await api.get(`${API_RESSOURCE}`)).data;
      } catch (err) {
        console.log(err);
      } finally {
        this.loading = false;
      }
    },
    async fetchOne(id) {
      try {
        this.loading = true;
        this.selected = (await api.get(`${API_RESSOURCE}/${id}`)).data;
      } catch (err) {
        // console.log(err);
      } finally {
        this.loading = false;
      }
    },
    async addOne(req) {
      try {
        this.loading = true;
        this.selected = (await api.post(`${API_RESSOURCE}`, req)).data;
      } catch (err) {
        // console.log(err);
      } finally {
        this.loading = false;
      }
    },
    async update(req) {
      try {
        this.loading = true;
        this.selected = (
          await api.put(`${API_RESSOURCE}/${this.selected[0].id}`, req)
        ).data;
      } catch (err) {
        // console.log(err);
      } finally {
        this.loading = false;
      }
    },
  },
});
