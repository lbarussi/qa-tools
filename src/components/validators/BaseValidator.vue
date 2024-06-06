<template>
  <ARow :gutter="[5, 5]">
    <ACol :xs="24" :sm="24" :md="24" :lg="24" :xl="24">
      <ValidateCPF />
    </ACol>
  </ARow>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import ValidateCPF from "../validators/ValidateCPF.vue";

export default defineComponent({
  name: "BaseValidator",
  components: { ValidateCPF }
})
</script>

<style scoped>

</style>
