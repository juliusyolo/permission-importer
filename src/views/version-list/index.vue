<script lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { defineComponent, reactive } from "vue";

export default defineComponent({
    name:'version-list',
    setup(){
        const columns = [{
      title: 'Name',
      dataIndex: 'name',
    }, {
      title: 'Salary',
      dataIndex: 'salary',
    }, {
      title: 'Address',
      dataIndex: 'address',
    }, {
      title: 'Email',
      dataIndex: 'email',
    }, {
      title: 'Optional',
      slotName: 'optional'
    }];
    const data = reactive([{
      key: '1',
      name: 'Jane Doe',
      salary: 23000,
      address: '32 Park Road, London',
      email: 'jane.doe@example.com'
    }, {
      key: '2',
      name: 'Alisa Ross',
      salary: 25000,
      address: '35 Park Road, London',
      email: 'alisa.ross@example.com'
    }, {
      key: '3',
      name: 'Kevin Sandra',
      salary: 22000,
      address: '31 Park Road, London',
      email: 'kevin.sandra@example.com'
    }, {
      key: '4',
      name: 'Ed Hellen',
      salary: 17000,
      address: '42 Park Road, London',
      email: 'ed.hellen@example.com'
    }, {
      key: '5',
      name: 'William Smith',
      salary: 27000,
      address: '62 Park Road, London',
      email: 'william.smith@example.com'
    }]);
        return {
            columns,
            data
        }
    }
})

</script>

<template>
  <a-table :columns="columns" :data="data">
    <template #optional="{ record }">
      <a-button @click="$modal.info({ title:'Name', content:record.name })">view</a-button>
    </template>
  </a-table>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>