import { createApp, defineComponent, h } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import router from "./router";
import "./style.css";
import Icons from "./components/icons/SvgIcons";

/**
 * Resilient bootstrap:
 * - Try to dynamically load PrimeVue and PrimeVue components.
 * - If PrimeVue isn't available, register small fallback components (Button, InputText, Dialog).
 * - Try to import local icon components; if they fail (missing heroicons), register lightweight SVG fallbacks.
 *
 * This lets the app run even in environments where dependencies aren't installed yet.
 */

const app = createApp(App);
const pinia = createPinia();

app.use(pinia);
app.use(router);

// Register SvgIcons globally so templates can reference them by name (e.g. <SunIcon />)
Object.entries(Icons).forEach(([name, comp]: any) => {
  // name is the exported key, ensure it's registered as a component
  app.component(name, comp as any);
});

async function bootstrap() {
  // Fallback implementations -------------------------------------------------
  const FallbackButton = defineComponent({
    name: "FallbackButton",
    props: {
      label: { type: String, default: "" },
      icon: { type: String, default: "" },
      loading: { type: Boolean, default: false },
      // allow arbitrary classes passed from templates
      class: { type: [String, Object, Array], default: "" },
      type: { type: String, default: "button" },
    },
    emits: ["click"],
    setup(props, { emit, slots, attrs }) {
      const onClick = (e: Event) => {
        if (!props.loading) emit("click", e);
      };
      return () =>
        h(
          "button",
          {
            type: (props as any).type || "button",
            class: ["fallback-btn", props.class],
            disabled: props.loading || (attrs as any).disabled,
            onClick,
            ...attrs,
          },
          [
            props.loading
              ? h("span", { style: { marginRight: "8px" } }, "⏳")
              : null,
            props.icon
              ? h(
                  "span",
                  { class: "icon-placeholder", style: { marginRight: "6px" } },
                  props.icon,
                )
              : null,
            slots.default ? slots.default() : props.label,
          ],
        );
    },
  });

  const FallbackInputText = defineComponent({
    name: "FallbackInputText",
    props: {
      modelValue: { type: [String, Number], default: "" },
      type: { type: String, default: "text" },
      placeholder: { type: String, default: "" },
      disabled: { type: Boolean, default: false },
    },
    emits: ["update:modelValue", "input", "change"],
    setup(props, { emit, attrs }) {
      const onInput = (e: Event) => {
        const target = e.target as HTMLInputElement;
        emit("update:modelValue", target.value);
        emit("input", e);
      };
      return () =>
        h("input", {
          value: props.modelValue as any,
          type: props.type,
          placeholder: props.placeholder,
          disabled: props.disabled,
          onInput,
          ...attrs,
        });
    },
  });

  const FallbackDialog = defineComponent({
    name: "FallbackDialog",
    props: {
      visible: { type: Boolean, default: false },
      modal: { type: Boolean, default: true },
      header: { type: String, default: "" },
    },
    emits: ["update:visible", "hide"],
    setup(props, { emit, slots }) {
      const close = () => {
        emit("update:visible", false);
        emit("hide");
      };
      return () =>
        props.visible
          ? h(
              "div",
              {
                class: "fallback-dialog-overlay",
                style: {
                  position: "fixed",
                  inset: "0",
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "center",
                  background: "rgba(0,0,0,0.45)",
                  zIndex: 1100,
                  padding: "20px",
                },
              },
              [
                h(
                  "div",
                  {
                    class: "fallback-dialog",
                    style: {
                      background: "var(--surface, #fff)",
                      color: "var(--text, #000)",
                      borderRadius: "10px",
                      padding: "18px",
                      width: "100%",
                      maxWidth: "720px",
                      boxShadow: "0 10px 30px rgba(0,0,0,0.15)",
                    },
                  },
                  [
                    h(
                      "div",
                      {
                        style: {
                          display: "flex",
                          justifyContent: "space-between",
                          alignItems: "center",
                          marginBottom: "12px",
                        },
                      },
                      [
                        h("div", { style: { fontWeight: 700 } }, props.header),
                        h(
                          "button",
                          {
                            onClick: close,
                            style: {
                              background: "transparent",
                              border: "none",
                              cursor: "pointer",
                              fontSize: "18px",
                            },
                            "aria-label": "Close dialog",
                          },
                          "✕",
                        ),
                      ],
                    ),
                    h("div", {}, slots.default ? slots.default() : undefined),
                  ],
                ),
              ],
            )
          : null;
    },
  });

  // Attempt to dynamically import PrimeVue and PrimeVue components ------------------------
  try {
    // dynamic import PrimeVue base
    const PrimeVueModule = await import("primevue/config");
    const PrimeVue =
      PrimeVueModule && (PrimeVueModule.default || PrimeVueModule);
    if (PrimeVue) {
      app.use(PrimeVue, { ripple: true });
    }

    // Try to import CSS for PrimeVue theme/core if available (fail silently)
    try {
      // @ts-ignore
      await import("primevue/resources/themes/saga-blue/theme.css");
      // @ts-ignore
      await import("primevue/resources/primevue.min.css");
    } catch {
      // ignore missing css modules
    }
    // Try to import heroicons (optional)
    try {
      await import("@heroicons/vue/24/outline");
    } catch {
      // ignore missing heroicons
    }

    // Import and register main PrimeVue components
    const ButtonModule = await import("primevue/button");
    const InputTextModule = await import("primevue/inputtext");
    const DialogModule = await import("primevue/dialog");

    const PButton =
      (ButtonModule && (ButtonModule.default || ButtonModule)) || null;
    const PInputText =
      (InputTextModule && (InputTextModule.default || InputTextModule)) || null;
    const PDialog =
      (DialogModule && (DialogModule.default || DialogModule)) || null;

    if (PButton) app.component("Button", PButton);
    else app.component("Button", FallbackButton);

    if (PInputText) app.component("InputText", PInputText);
    else app.component("InputText", FallbackInputText);

    if (PDialog) app.component("Dialog", PDialog);
    else app.component("Dialog", FallbackDialog);
  } catch {
    // If any of the primevue imports failed, fall back to simple components
    // Register safe fallbacks so templates can still function.
    app.component("Button", FallbackButton);
    app.component("InputText", FallbackInputText);
    app.component("Dialog", FallbackDialog);
  }

  // Try to load local icon components (they may import heroicons); if that fails, register inline fallbacks
  try {
    const IconFileMod = await import("./components/IconFile.vue");
    const IconFolderMod = await import("./components/IconFolder.vue");
    const IconFileComp =
      (IconFileMod && (IconFileMod.default || IconFileMod)) || null;
    const IconFolderComp =
      (IconFolderMod && (IconFolderMod.default || IconFolderMod)) || null;
    if (IconFileComp) app.component("IconFile", IconFileComp);
    else
      app.component(
        "IconFile",
        defineComponent({
          name: "IconFileFallback",
          setup() {
            return () =>
              h(
                "svg",
                {
                  width: 18,
                  height: 18,
                  viewBox: "0 0 24 24",
                  fill: "none",
                  xmlns: "http://www.w3.org/2000/svg",
                },
                [
                  h("path", {
                    d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z",
                    stroke: "currentColor",
                    "stroke-width": "1.4",
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                  }),
                  h("path", {
                    d: "M14 2v6h6",
                    stroke: "currentColor",
                    "stroke-width": "1.4",
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                  }),
                ],
              );
          },
        }),
      );

    if (IconFolderComp) app.component("IconFolder", IconFolderComp);
    else
      app.component(
        "IconFolder",
        defineComponent({
          name: "IconFolderFallback",
          setup() {
            return () =>
              h(
                "svg",
                {
                  width: 18,
                  height: 18,
                  viewBox: "0 0 24 24",
                  fill: "none",
                  xmlns: "http://www.w3.org/2000/svg",
                },
                [
                  h("path", {
                    d: "M3 7a2 2 0 0 1 2-2h4l2 2h6a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V7z",
                    stroke: "currentColor",
                    "stroke-width": "1.4",
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                  }),
                ],
              );
          },
        }),
      );
  } catch {
    // local icon components not available; register simple inline SVG fallbacks
    app.component(
      "IconFile",
      defineComponent({
        name: "IconFileFallback",
        setup() {
          return () =>
            h(
              "svg",
              {
                width: 18,
                height: 18,
                viewBox: "0 0 24 24",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
              },
              [
                h("path", {
                  d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z",
                  stroke: "currentColor",
                  "stroke-width": "1.4",
                  "stroke-linecap": "round",
                  "stroke-linejoin": "round",
                }),
                h("path", {
                  d: "M14 2v6h6",
                  stroke: "currentColor",
                  "stroke-width": "1.4",
                  "stroke-linecap": "round",
                  "stroke-linejoin": "round",
                }),
              ],
            );
        },
      }),
    );

    app.component(
      "IconFolder",
      defineComponent({
        name: "IconFolderFallback",
        setup() {
          return () =>
            h(
              "svg",
              {
                width: 18,
                height: 18,
                viewBox: "0 0 24 24",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
              },
              [
                h("path", {
                  d: "M3 7a2 2 0 0 1 2-2h4l2 2h6a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V7z",
                  stroke: "currentColor",
                  "stroke-width": "1.4",
                  "stroke-linecap": "round",
                  "stroke-linejoin": "round",
                }),
              ],
            );
        },
      }),
    );
  }

  // Finalize app mount -------------------------------------------------------
  // If PrimeVue is available we already registered its components; otherwise fallback components are registered.
  // Mount the app after registration so components are available immediately.
  app.mount("#app");
}

// Start the bootstrap process but don't block the top-level execution.
// bootstrap will register components and mount the app.
bootstrap().catch(() => {
  // If bootstrap fails for some unexpected reason, ensure the app still mounts with fallbacks.
  // Register minimal fallbacks if not already registered.
  try {
    // Only register fallbacks if not registered yet
    if (!app._context.components["Button"]) {
      app.component(
        "Button",
        defineComponent({
          name: "QuickButton",
          props: ["label"],
          setup(props) {
            return () => h("button", props.label || "Button");
          },
        }),
      );
    }
    if (!app._context.components["InputText"]) {
      app.component(
        "InputText",
        defineComponent({
          name: "QuickInput",
          props: ["modelValue"],
          emits: ["update:modelValue"],
          setup(props, { emit }) {
            return () =>
              h("input", {
                value: (props as any).modelValue,
                onInput: (e: Event) =>
                  emit(
                    "update:modelValue",
                    (e.target as HTMLInputElement).value,
                  ),
              });
          },
        }),
      );
    }
  } catch {
    // ignore
  }
  // ensure the app is mounted in any case
  try {
    app.mount("#app");
  } catch {
    // ignore final mount errors here - the environment may not have DOM.
  }
});
