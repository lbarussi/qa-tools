<template>
  <ARow :gutter="[5, 5]">
    <ACol :xs="24" :sm="24" :md="24" :lg="24" :xl="24">
      <GenerateCPF />
    </ACol>
  </ARow>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import GenerateCPF from "./GenerateCPF.vue";

export default defineComponent({
  name: "BaseGenerator",
  components: {
    GenerateCPF,
  }
})
</script>

<style scoped>

</style>
