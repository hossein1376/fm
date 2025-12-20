import { defineComponent, h, PropType } from "vue";

type IconProps = {
  width?: number | string;
  height?: number | string;
  strokeWidth?: number;
  class?: string;
  ariaLabel?: string | null;
};

function svgAttrs(props: IconProps) {
  const w = props.width ?? 20;
  const hgt = props.height ?? w;
  const sw = props.strokeWidth ?? 1.5;
  const aria =
    props.ariaLabel != null
      ? { "aria-label": props.ariaLabel }
      : { "aria-hidden": "true" };
  return {
    width: w,
    height: hgt,
    viewBox: "0 0 24 24",
    fill: "none",
    stroke: "currentColor",
    "stroke-width": String(sw),
    "stroke-linecap": "round",
    "stroke-linejoin": "round",
    class: props.class ?? undefined,
    xmlns: "http://www.w3.org/2000/svg",
    ...aria,
  } as Record<string, any>;
}

export const SunIcon = defineComponent({
  name: "SunIcon",
  props: {
    width: [Number, String] as PropType<number | string>,
    height: [Number, String] as PropType<number | string>,
    strokeWidth: { type: Number as PropType<number>, default: 1.5 },
    class: String,
    ariaLabel: String as PropType<string | null>,
  },
  setup(props: IconProps) {
    return () =>
      h("svg", svgAttrs(props), [
        h("circle", { cx: "12", cy: "12", r: "3.5", fill: "none" }),
        h("path", { d: "M12 2v1.5" }),
        h("path", { d: "M12 20.5V22" }),
        h("path", { d: "M4.22 4.22l1.06 1.06" }),
        h("path", { d: "M18.72 18.72l1.06 1.06" }),
        h("path", { d: "M1.5 12h1.5" }),
        h("path", { d: "M21 12h1.5" }),
        h("path", { d: "M4.22 19.78l1.06-1.06" }),
        h("path", { d: "M18.72 5.28l1.06-1.06" }),
      ]);
  },
});

export const MoonIcon = defineComponent({
  name: "MoonIcon",
  props: {
    width: [Number, String] as PropType<number | string>,
    height: [Number, String] as PropType<number | string>,
    strokeWidth: { type: Number as PropType<number>, default: 1.5 },
    class: String,
    ariaLabel: String as PropType<string | null>,
  },
  setup(props: IconProps) {
    return () =>
      h("svg", svgAttrs(props), [
        h("path", { d: "M21 12.79A9 9 0 1111.21 3 7 7 0 0021 12.79z" }),
      ]);
  },
});

export const RefreshIcon = defineComponent({
  name: "RefreshIcon",
  props: {
    width: [Number, String] as PropType<number | string>,
    height: [Number, String] as PropType<number | string>,
    strokeWidth: { type: Number as PropType<number>, default: 1.5 },
    class: String,
    ariaLabel: String as PropType<string | null>,
  },
  setup(props: IconProps) {
    return () =>
      h("svg", svgAttrs(props), [
        h("path", {
          d: "M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15",
        }),
      ]);
  },
});

export const LogoutIcon = defineComponent({
  name: "LogoutIcon",
  props: {
    width: [Number, String] as PropType<number | string>,
    height: [Number, String] as PropType<number | string>,
    strokeWidth: { type: Number as PropType<number>, default: 1.5 },
    class: String,
    ariaLabel: String as PropType<string | null>,
  },
  setup(props: IconProps) {
    return () =>
      h("svg", svgAttrs(props), [
        h("path", { d: "M9 21H5a2 2 0 01-2-2V5a2 2 0 012-2h4" }),
        h("path", { d: "M16 17l5-5-5-5" }),
        h("path", { d: "M21 12H9" }),
      ]);
  },
});

export const PlusIcon = defineComponent({
  name: "PlusIcon",
  props: {
    width: [Number, String] as PropType<number | string>,
    height: [Number, String] as PropType<number | string>,
    strokeWidth: { type: Number as PropType<number>, default: 1.6 },
    class: String,
    ariaLabel: String as PropType<string | null>,
  },
  setup(props: IconProps) {
    return () =>
      h("svg", svgAttrs(props), [
        h("path", { d: "M12 5v14" }),
        h("path", { d: "M5 12h14" }),
      ]);
  },
});

export const PencilIcon = defineComponent({
  name: "PencilIcon",
  props: {
    width: [Number, String] as PropType<number | string>,
    height: [Number, String] as PropType<number | string>,
    strokeWidth: { type: Number as PropType<number>, default: 1.4 },
    class: String,
    ariaLabel: String as PropType<string | null>,
  },
  setup(props: IconProps) {
    return () =>
      h("svg", svgAttrs(props), [
        h("path", { d: "M3 21l3-1 11-11 1-3L15 3 4 14 3 21z" }),
        h("path", { d: "M14 7l3 3" }),
      ]);
  },
});

export const TrashIcon = defineComponent({
  name: "TrashIcon",
  props: {
    width: [Number, String] as PropType<number | string>,
    height: [Number, String] as PropType<number | string>,
    strokeWidth: { type: Number as PropType<number>, default: 1.4 },
    class: String,
    ariaLabel: String as PropType<string | null>,
  },
  setup(props: IconProps) {
    return () =>
      h("svg", svgAttrs(props), [
        h("polyline", { points: "3 6 5 6 21 6" }),
        h("path", { d: "M19 6l-1 14a2 2 0 01-2 2H8a2 2 0 01-2-2L5 6" }),
        h("path", { d: "M10 11v6" }),
        h("path", { d: "M14 11v6" }),
        h("path", { d: "M9 6V4a1 1 0 011-1h4a1 1 0 011 1v2" }),
      ]);
  },
});

export const UploadIcon = defineComponent({
  name: "UploadIcon",
  props: {
    width: [Number, String] as PropType<number | string>,
    height: [Number, String] as PropType<number | string>,
    strokeWidth: { type: Number as PropType<number>, default: 1.4 },
    class: String,
    ariaLabel: String as PropType<string | null>,
  },
  setup(props: IconProps) {
    return () =>
      h("svg", svgAttrs(props), [
        h("path", { d: "M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4" }),
        h("path", { d: "M17 8l-5-5-5 5" }),
        h("path", { d: "M12 3v12" }),
      ]);
  },
});

export const FolderPlusIcon = defineComponent({
  name: "FolderPlusIcon",
  props: {
    width: [Number, String] as PropType<number | string>,
    height: [Number, String] as PropType<number | string>,
    strokeWidth: { type: Number as PropType<number>, default: 1.4 },
    class: String,
    ariaLabel: String as PropType<string | null>,
  },
  setup(props: IconProps) {
    return () =>
      h("svg", svgAttrs(props), [
        h("path", {
          d: "M3 7a2 2 0 012-2h3l2 2h8a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2V7z",
        }),
        h("path", { d: "M12 10v6" }),
        h("path", { d: "M9 13h6" }),
      ]);
  },
});

export const ChevronUpIcon = defineComponent({
  name: "ChevronUpIcon",
  props: {
    width: [Number, String] as PropType<number | string>,
    height: [Number, String] as PropType<number | string>,
    strokeWidth: { type: Number as PropType<number>, default: 1.4 },
    class: String,
    ariaLabel: String as PropType<string | null>,
  },
  setup(props: IconProps) {
    return () =>
      h("svg", svgAttrs(props), [h("path", { d: "M18 15l-6-6-6 6" })]);
  },
});

/* Grid icon: 4 small squares */
export const GridIcon = defineComponent({
  name: "GridIcon",
  props: {
    width: [Number, String] as PropType<number | string>,
    height: [Number, String] as PropType<number | string>,
    strokeWidth: { type: Number as PropType<number>, default: 1.4 },
    class: String,
    ariaLabel: String as PropType<string | null>,
  },
  setup(props: IconProps) {
    return () =>
      h("svg", svgAttrs(props), [
        h("rect", { x: "3", y: "3", width: "7", height: "7", fill: "none" }),
        h("rect", { x: "14", y: "3", width: "7", height: "7", fill: "none" }),
        h("rect", { x: "3", y: "14", width: "7", height: "7", fill: "none" }),
        h("rect", { x: "14", y: "14", width: "7", height: "7", fill: "none" }),
      ]);
  },
});

/* List icon: three rows with small bullets */
export const ListIcon = defineComponent({
  name: "ListIcon",
  props: {
    width: [Number, String] as PropType<number | string>,
    height: [Number, String] as PropType<number | string>,
    strokeWidth: { type: Number as PropType<number>, default: 1.4 },
    class: String,
    ariaLabel: String as PropType<string | null>,
  },
  setup(props: IconProps) {
    return () =>
      h("svg", svgAttrs(props), [
        h("circle", { cx: "4.5", cy: "6.5", r: "1" }),
        h("path", { d: "M8 6.5h12" }),
        h("circle", { cx: "4.5", cy: "12", r: "1" }),
        h("path", { d: "M8 12h12" }),
        h("circle", { cx: "4.5", cy: "17.5", r: "1" }),
        h("path", { d: "M8 17.5h12" }),
      ]);
  },
});

export const InfoIcon = defineComponent({
  name: "InfoIcon",
  props: {
    width: [Number, String] as PropType<number | string>,
    height: [Number, String] as PropType<number | string>,
    strokeWidth: { type: Number as PropType<number>, default: 1.4 },
    class: String,
    ariaLabel: String as PropType<string | null>,
  },
  setup(props: IconProps) {
    return () =>
      h("svg", svgAttrs(props), [
        h("circle", { cx: "12", cy: "12", r: "10" }),
        h("path", { d: "M12 16v-4" }),
        h("path", { d: "M12 8h.01" }),
      ]);
  },
});

const Icons: Record<string, any> = {
  SunIcon,
  MoonIcon,
  RefreshIcon,
  LogoutIcon,
  PlusIcon,
  PencilIcon,
  TrashIcon,
  UploadIcon,
  FolderPlusIcon,
  ChevronUpIcon,
  GridIcon,
  ListIcon,
  InfoIcon,
};

export default Icons;
