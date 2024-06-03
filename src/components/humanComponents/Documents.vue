<template>
  <div>
    <CPF /><Greet />
  </div>
</template>

<script lang="ts">
import {defineComponent} from 'vue'
import Greet from "../Greet.vue";

export default defineComponent({
  name: "Documents",
  components: {Greet}
})
</script>

<style scoped>

</style>
