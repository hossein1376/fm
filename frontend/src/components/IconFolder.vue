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
import { computed, toRefs } from "vue";
import type { PropType } from "vue";
import { FolderIcon as FolderOutline } from "@heroicons/vue/24/outline";
import { FolderIcon as FolderSolid } from "@heroicons/vue/24/solid";

/**
 * IconFolder
 *
 * A small wrapper around Heroicons' Folder icon that supports:
 *  - variant: 'outline' | 'solid'
 *  - size: number | string (px by default when number provided)
 *  - customClass: additional classes to apply
 *  - label: accessible label for screen readers
 *
 * Usage:
 * <IconFolder variant="solid" :size="20" customClass="text-indigo-500" label="Project folder" />
 */

const props = defineProps({
    variant: {
        type: String as PropType<"outline" | "solid">,
        default: "outline",
    },
    size: {
        type: [Number, String] as PropType<number | string>,
        default: 20,
    },
    customClass: {
        type: String,
        default: "",
    },
    label: {
        type: String,
        default: "Folder",
    },
});

const iconComponent = computed(() =>
    props.variant === "solid" ? FolderSolid : FolderOutline,
);

const computedSize = computed(() =>
    typeof props.size === "number" ? String(props.size) : props.size,
);

const computedClass = computed(() => {
    // keep a base classname to scope styling if necessary
    return ["icon-folder", props.customClass].filter(Boolean).join(" ");
});

const { label } = toRefs(props);
</script>

<style scoped>
.icon-folder {
    display: inline-block;
    vertical-align: middle;
    line-height: 0;
    /* allow coloring via currentColor */
    color: inherit;
}

/* Ensure the SVG fits the container and inherits color */
.icon-folder svg {
    display: block;
    width: 100%;
    height: 100%;
    stroke: currentColor;
    fill: currentColor;
}
</style>
