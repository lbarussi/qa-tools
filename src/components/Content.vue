<template>
  <ALayoutHeader :style="headerStyle">
    QA Tools
  </ALayoutHeader>
  <ALayoutContent :style="contentStyle">
    <slot name="content"></slot>
  </ALayoutContent>
</template>

<script lang="ts">
import {defineComponent} from 'vue'
import type { CSSProperties } from 'vue';

const headerStyle: CSSProperties = {
  textAlign: 'center',
  color: '#fff',
  height: 64,
  paddingInline: 50,
  lineHeight: '64px',
  backgroundColor: '#0f0f0f',
};

const contentStyle: CSSProperties = {
  minHeight: 120,
  lineHeight: '120px',
  backgroundColor: 'transparent',
  padding: "0 24px 24px"
};

export default defineComponent({
  name: "Content",
  data: () => ({
    headerStyle,
    contentStyle
  })
})
</script>

<style scoped>

</style>
