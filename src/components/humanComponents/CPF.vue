<template>
  <ARow :gutter="[5, 5]">
    <ACol :xs="24" :sm="24" :md="24" :lg="24" :xl="24">
      <AInputSearch placeholder="Validate or generate document" v-model:value="document">
        <template #enterButton>
          <AButton type="primary" @click="generate">Generate</AButton>
        </template>
      </AInputSearch>
    </ACol>
  </ARow>
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
      this.document = await invoke("generate_cpf", {});
      await navigator.clipboard.writeText(String(this.document));
      message.success('Copiado para área de transferencia!');
    },
  }
})
</script>

<style scoped>

</style>
