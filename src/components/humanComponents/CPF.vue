<template>
  <ARow :gutter="5">
    <ACol :xs="24" :sm="24" :md="24" :lg="24" :xl="24">
      <a-input-search placeholder="Validate or generate document" v-model:value="document">
        <template #enterButton>
          <a-button type="primary" @click="generate">Generate</a-button>
        </template>
      </a-input-search>
    </ACol>
  </ARow>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import { invoke } from "@tauri-apps/api/tauri";

export default defineComponent({
  name: "CPF",
  data: () => ({
    document: null as null | string,
  }),
  methods: {
    async generate() {
      try {
        this.document = await invoke("generate_cpf", {});
      } catch (exception) {
        // TODO: Remove this
        console.error(exception)
      }
    },
  }
})
</script>

<style scoped>

</style>
