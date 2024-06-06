<template>
  <div style="display: flex;">
    <AInput placeholder="Validar CPF" v-model:value="document" @paste="paste" size="large">
      <template #prefix v-if="isValid">
        <IconRosetteDiscountCheckFilled style="color: green" size="25" />
      </template>
    </AInput>
    <div style="margin-left: 5px">
      <AButton type="primary" @click="validate" size="large" :disabled="!documentIsValid">
        Validar CPF
      </AButton>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, h } from 'vue'
import { invoke } from "@tauri-apps/api/tauri";
import { IconRosetteDiscountCheckFilled } from '@tabler/icons-vue';
import { message } from "ant-design-vue";

export default defineComponent({
  name: "ValidateCPF",
  components: {
    IconRosetteDiscountCheckFilled
  },
  data: () => ({
    document: null as null | string,
    isValid: false as boolean,
  }),
  computed: {
    documentIsValid(): Boolean {
      if (this.document === null || this.document === "") {
        return false;
      }

      return this.getDocumentCleared?.length === 11;
    },
    getDocumentCleared(): string | undefined {
      return this.document?.replaceAll(/\D/g, "")
    }
  },
  methods: {
    h,
    async validate() {
      const response: boolean = await invoke("validate_cpf", { document: this.getDocumentCleared });

      if (response) {
        this.isValid = response;
        message.success('CPF Verdadeiro!');

        setTimeout(() => {
          this.isValid = false
        }, 3000)
      } else {
        message.error('CPF Inválido!');
      }
    },
    async paste() {
      setTimeout(async () => {
        if (this.documentIsValid) {
          await this.validate();
        } else {
          message.error('CPF Inválido!');
        }
      }, 100)
    },
  }
})
</script>
