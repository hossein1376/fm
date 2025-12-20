<template>
  <component
    :is="iconComponent"
    :width="computedSize"
    :height="computedSize"
    :class="computedClass"
    role="img"
    :aria-label="label"
  />
</template>

<script setup lang="ts">
import { computed, toRefs } from 'vue'
import type { PropType } from 'vue'
import { DocumentIcon as DocumentOutline } from '@heroicons/vue/24/outline'
import { DocumentIcon as DocumentSolid } from '@heroicons/vue/24/solid'

const props = defineProps({
  variant: {
    type: String as PropType<'outline' | 'solid'>,
    default: 'outline'
  },
  size: {
    type: [Number, String] as PropType<number | string>,
    default: 20
  },
  customClass: {
    type: String,
    default: ''
  },
  label: {
    type: String,
    default: 'File'
  }
})

const iconComponent = computed(() =>
  props.variant === 'solid' ? DocumentSolid : DocumentOutline
)

const computedSize = computed(() =>
  typeof props.size === 'number' ? String(props.size) : props.size
)

const computedClass = computed(() => {
  return ['icon-file', props.customClass].filter(Boolean).join(' ')
})

const { label } = toRefs(props)
</script>

<style scoped>
.icon-file {
  display: inline-block;
  vertical-align: middle;
  line-height: 0;
  color: inherit; /* allow icon to be colored via currentColor */
}

.icon-file svg {
  display: block;
  width: 100%;
  height: 100%;
  stroke: currentColor;
  fill: currentColor;
}
</style>
