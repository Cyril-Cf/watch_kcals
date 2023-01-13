import { defineStore } from 'pinia';
import { api } from 'boot/axios';

const API_RESSOURCE = '/ingredient_details';

export const useIngredientDetailStore = defineStore({
  id: 'ingredientDetailStore',
  state: () => ({
    loading: false,
    all: [],
    selected: {},
  }),
  actions: {
    changeSelected(object) {
      this.selected = object;
    },
    async fetchAllForOneIngredient(ingredientId) {
      try {
        this.loading = true;
        this.all = (await api.get(`${API_RESSOURCE}/ingredients/${ingredientId}`)).data;
        this.all.sort((a, b) => (a.ingredient_detail_type < b.ingredient_detail_type ? -1 : 1))
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
    async update(req, id) {
      try {
        this.loading = true;
        this.selected = (
          await api.put(`${API_RESSOURCE}/${id}`, req)
        ).data;
      } catch (err) {
        // console.log(err);
      } finally {
        this.loading = false;
      }
    },
  },
});