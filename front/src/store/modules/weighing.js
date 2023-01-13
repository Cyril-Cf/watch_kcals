import { defineStore } from 'pinia';
import { api } from 'boot/axios';
import { useUserStore } from 'src/store/modules/user';

const userStore = useUserStore()

export const useWeighingStore = defineStore({
  id: 'weighingStore',
  state: () => ({
    loading: false,
    weighingSelected: {},
    weighings: [],
  }),
  actions: {
    changeSelectedWeighing(weighing) {
      this.weighingSelected = weighing
    },
    async fetchWeighings() {
      try {
        this.loading = true;
        this.weighings = (await api.get('/weighings')).data;
      } catch (err) {
        // console.log(err);
      } finally {
        this.loading = false;
      }
    },
    async fetchWeighing(id) {
      try {
        this.loading = true;
        this.weighingSelected = (await api.get(`/weighings/${id}`)).data;
      } catch (err) {
        // console.log(err);
      } finally {
        this.loading = false;
      }
    },
    async addWeighing(internal) {
      try {
        this.loading = true;
        this.weighingSelected = (
          await api.post('/weighings', {
            date: internal.date,
            body_fat_percentage: +internal.body_fat_percentage,
            waist_circumference: +internal.waist_circumference,
            waist_size: +internal.waist_size,
            weight: +internal.weight,
            user_id: userStore.user.id
          })
        ).data;
      } catch (err) {
        // console.log(err);
      } finally {
        this.loading = false;
      }
    },
    async updateWeighing(internal) {
      try {
        this.loading = true;
        this.weighingSelected = (
          await api.put(`/weighings/${this.weighingSelected.id}`, {
            date: internal.date,
            body_fat_percentage: +internal.body_fat_percentage,
            waist_circumference: +internal.waist_circumference,
            waist_size: +internal.waist_size,
            weight: +internal.weight,
            user_id: userStore.user.id
          })
        ).data;
      } catch (err) {
        // console.log(err);
      } finally {
        this.loading = false;
      }
    },
  },
});
