import { defineStore } from 'pinia';
import { api } from 'boot/axios';

export const useUserStore = defineStore({
  id: 'userStore',
  state: () => ({
    loading: false,
    user: {},
  }),
  actions: {
    async fetchUser() {
      try {
        this.loading = true;
        const data = (await api.get('/users')).data;
        if (data.length > 0) {
          this.user = data[0];
        }
      } catch (err) {
        // console.log(err);
      } finally {
        this.loading = false;
      }
    },
    async addUser(internal) {
      try {
        this.loading = true;
        this.user = (
          await api.post('/users', {
            name: internal.name,
            gender: internal.gender,
            date_of_birth: internal.date_of_birth,
            height: +internal.height,
            physical_activity_level: +internal.physical_activity_level,
          })
        ).data;
      } catch (err) {
        // console.log(err);
      } finally {
        this.loading = false;
      }
    },
    async updateUser(internal) {
      try {
        this.loading = true;
        this.user = (
          await api.put(`/users/${this.user.id}`, {
            name: internal.name,
            gender: internal.gender,
            date_of_birth: internal.date_of_birth,
            height: +internal.height,
            physical_activity_level: +internal.physical_activity_level,
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
