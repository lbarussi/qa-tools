<template>
  <div style="display: flex">
    <AInput v-model:value="document" size="large">
    </AInput>
    <div style="margin-left: 5px">
      <AButton type="primary" @click="generate" size="large" :disabled="document === ''">
        Gerar CPF
      </AButton>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import { invoke } from "@tauri-apps/api/tauri";
import { message } from "ant-design-vue";

export default defineComponent({
  name: "CPF",
  data: () => ({
    document: null as null | string,
  }),
  methods: {
    async generate() {
      this.document = await invoke("generate_cpf");
      await navigator.clipboard.writeText(String(this.document));
      message.success('Copiado para área de transferencia!');
    },
  }
})
</script>

<style scoped>

</style>
