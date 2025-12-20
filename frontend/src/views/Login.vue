<template>
    <div class="container" id="login-page" aria-labelledby="login-title">
        <div class="row" style="align-items: stretch; gap: 28px">
            <!-- Left Column: Brand / Illustration / Marketing -->
            <div
                class="col"
                style="
                    flex: 0 0 48%;
                    min-width: 300px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                "
            >
                <div
                    class="card"
                    style="
                        width: 100%;
                        padding: 32px;
                        display: flex;
                        flex-direction: column;
                        gap: 18px;
                    "
                >
                    <div
                        class="kv"
                        style="
                            justify-content: space-between;
                            align-items: center;
                        "
                    >
                        <div
                            style="
                                display: flex;
                                gap: 12px;
                                align-items: center;
                            "
                        >
                            <!-- Small Brand Mark -->
                            <div
                                style="
                                    width: 52px;
                                    height: 52px;
                                    border-radius: 12px;
                                    display: grid;
                                    place-items: center;
                                    background: linear-gradient(
                                        135deg,
                                        var(--primary),
                                        var(--primary-600)
                                    );
                                    box-shadow: 0 8px 24px
                                        rgba(79, 70, 229, 0.12);
                                "
                            >
                                <svg
                                    width="28"
                                    height="28"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    aria-hidden
                                >
                                    <path
                                        d="M4 12h6l2 6 6-12"
                                        stroke="#fff"
                                        stroke-width="1.5"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                    />
                                </svg>
                            </div>
                            <div>
                                <h2 style="margin: 0; font-size: 1.25rem">
                                    File Manager
                                </h2>
                                <p
                                    style="
                                        margin: 0;
                                        color: var(--muted);
                                        font-size: 0.95rem;
                                    "
                                >
                                    Securely browse, upload and manage files
                                    across hosts
                                </p>
                            </div>
                        </div>

                        <div
                            style="display: flex; gap: 8px; align-items: center"
                        >
                            <div
                                class="tag"
                                aria-hidden
                                style="font-size: 0.85rem; padding: 6px 10px"
                            >
                                Beta
                            </div>
                            <Button
                                class="p-button-text"
                                @click="toggleTheme"
                                aria-label="Toggle theme"
                            >
                                <template #icon>
                                    <SunIcon v-if="theme?.value === 'dark'" />
                                    <MoonIcon v-else />
                                </template>
                            </Button>
                        </div>
                    </div>

                    <!-- Illustration -->
                    <div
                        style="
                            display: flex;
                            align-items: center;
                            justify-content: center;
                            padding: 12px 8px;
                        "
                    >
                        <svg
                            width="100%"
                            height="220"
                            viewBox="0 0 600 320"
                            fill="none"
                            role="img"
                            aria-label="Illustration"
                        >
                            <defs>
                                <linearGradient id="g1" x1="0" x2="1">
                                    <stop
                                        offset="0"
                                        stop-color="var(--primary-400)"
                                        stop-opacity="0.9"
                                    />
                                    <stop
                                        offset="1"
                                        stop-color="var(--primary-600)"
                                        stop-opacity="0.9"
                                    />
                                </linearGradient>
                                <linearGradient id="g2" x1="0" x2="1">
                                    <stop
                                        offset="0"
                                        stop-color="var(--accent)"
                                        stop-opacity="0.14"
                                    />
                                    <stop
                                        offset="1"
                                        stop-color="var(--primary-600)"
                                        stop-opacity="0.06"
                                    />
                                </linearGradient>
                            </defs>

                            <rect
                                x="12"
                                y="36"
                                rx="18"
                                width="420"
                                height="220"
                                fill="url(#g2)"
                            />
                            <rect
                                x="48"
                                y="68"
                                rx="12"
                                width="320"
                                height="152"
                                fill="url(#g1)"
                            />
                            <g
                                transform="translate(84,96)"
                                fill="#fff"
                                opacity="0.95"
                            >
                                <rect
                                    x="0"
                                    y="0"
                                    width="64"
                                    height="56"
                                    rx="8"
                                />
                                <rect
                                    x="84"
                                    y="0"
                                    width="216"
                                    height="24"
                                    rx="8"
                                />
                                <rect
                                    x="84"
                                    y="40"
                                    width="216"
                                    height="16"
                                    rx="6"
                                />
                                <rect
                                    x="84"
                                    y="72"
                                    width="160"
                                    height="16"
                                    rx="6"
                                />
                            </g>

                            <circle
                                cx="520"
                                cy="64"
                                r="52"
                                fill="rgba(96,165,250,0.06)"
                            />
                            <circle
                                cx="520"
                                cy="64"
                                r="26"
                                fill="var(--primary-400)"
                            />
                        </svg>
                    </div>

                    <div
                        style="
                            display: flex;
                            flex-direction: column;
                            gap: 8px;
                            margin-top: auto;
                        "
                    >
                        <p style="margin: 0; color: var(--muted)">
                            Manage file systems (local, HTTP, SFTP) from a
                            single beautiful interface. Grant access to teams,
                            track changes and transfer files securely.
                        </p>
                        <div
                            style="
                                display: flex;
                                gap: 8px;
                                align-items: center;
                                flex-wrap: wrap;
                            "
                        >
                            <span class="tag">SFTP</span>
                            <span class="tag">Local FS</span>
                            <span class="tag">HTTP</span>
                            <span
                                style="
                                    color: var(--muted);
                                    font-size: 0.9rem;
                                    margin-left: auto;
                                "
                                >v0.9</span
                            >
                        </div>
                    </div>
                </div>
            </div>

            <!-- Right Column: Auth Card -->
            <div
                class="col"
                style="
                    flex: 0 0 44%;
                    min-width: 300px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                "
            >
                <div
                    class="card"
                    style="width: 100%; padding: 28px; max-width: 420px"
                >
                    <div
                        style="
                            display: flex;
                            align-items: center;
                            justify-content: space-between;
                            margin-bottom: 12px;
                        "
                    >
                        <h2
                            id="login-title"
                            style="font-size: 1.125rem; margin: 0"
                        >
                            Welcome back
                        </h2>
                        <small
                            style="
                                color: var(--muted);
                                font-weight: 600;
                                font-size: 0.9rem;
                            "
                            >Secure sign in</small
                        >
                    </div>

                    <form
                        class="form"
                        @submit.prevent="handleSubmit"
                        novalidate
                    >
                        <div class="form-group">
                            <label for="username">Username</label>
                            <InputText
                                id="username"
                                v-model="username"
                                autocomplete="username"
                                required
                                :disabled="loading"
                                placeholder="e.g. alice@example.com"
                            />
                        </div>

                        <div class="form-group">
                            <label for="password">Password</label>
                            <InputText
                                id="password"
                                v-model="password"
                                type="password"
                                autocomplete="current-password"
                                required
                                :disabled="loading"
                                placeholder="••••••••"
                            />
                        </div>

                        <div
                            style="
                                display: flex;
                                gap: 12px;
                                align-items: center;
                            "
                        >
                            <Button
                                type="submit"
                                class="btn primary full-width"
                                :loading="loading"
                            >
                                <template v-if="loading">Processing…</template>
                                <template v-else>{{
                                    isRegister ? "Create account" : "Sign in"
                                }}</template>
                            </Button>
                        </div>

                        <div
                            style="
                                display: flex;
                                justify-content: space-between;
                                align-items: center;
                                margin-top: 8px;
                            "
                        >
                            <Button
                                type="button"
                                class="btn ghost"
                                @click="toggleMode"
                                :disabled="loading"
                                aria-pressed="false"
                                label="Toggle mode"
                            />
                            <Button
                                type="button"
                                class="btn"
                                @click="fillDemo"
                                :disabled="loading"
                                label="Demo account"
                            />
                        </div>

                        <div v-if="error" class="error" style="margin-top: 8px">
                            {{ error }}
                        </div>

                        <div
                            style="
                                margin-top: 12px;
                                display: flex;
                                justify-content: center;
                                color: var(--muted);
                                font-size: 0.88rem;
                            "
                        >
                            By continuing you agree to our
                            <a
                                href="#terms"
                                style="
                                    color: var(--primary);
                                    text-decoration: underline;
                                "
                                >Terms</a
                            >.
                        </div>
                    </form>
                </div>
            </div>
        </div>

        <!-- Small footer -->
        <div
            style="
                width: 100%;
                max-width: 1100px;
                margin: 18px auto 0;
                display: flex;
                justify-content: center;
                color: var(--muted);
                font-size: 0.9rem;
            "
        >
            <span>© {{ year }} File Manager • Built with care</span>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
import { useAuthStore } from "../store/auth";
import themeService from "../services/theme";

const router = useRouter();
const authStore = useAuthStore();

const username = ref("");
const password = ref("");
const isRegister = ref(false);
const loading = ref(false);
const error = ref("");

const year = new Date().getFullYear();

const handleSubmit = async () => {
    error.value = "";
    loading.value = true;

    try {
        let success = false;
        if (isRegister.value) {
            success = await authStore.register(
                username.value.trim(),
                password.value,
            );
        } else {
            success = await authStore.login(
                username.value.trim(),
                password.value,
            );
        }

        if (success) {
            // after successful auth, navigate to dashboard
            router.push("/");
        } else {
            error.value = isRegister.value
                ? "Registration failed (username may be taken)"
                : "Invalid username or password";
        }
    } catch (err: any) {
        // surface a friendly message
        error.value = err?.message || "An unexpected error occurred";
    } finally {
        loading.value = false;
    }
};

const toggleMode = () => {
    isRegister.value = !isRegister.value;
    error.value = "";
};

// Helpful quick demo fill
const fillDemo = () => {
    username.value = "demo";
    password.value = "demo";
};

const { theme, toggleTheme } = themeService;
</script>

<style scoped>
/* Scoped tweaks for the Login page to complement global design */
#login-page {
    width: 100%;
    margin-top: 12px;
    margin-bottom: 12px;
}

@media (max-width: 900px) {
    .row {
        gap: 14px !important;
    }

    #login-page .row {
        flex-direction: column;
        align-items: stretch;
    }

    #login-page .col {
        flex: 1 1 auto !important;
    }
}

/* Slightly tighter card look for auth */
.card {
    border-radius: 14px;
}

/* Buttons override for ghost behavior */
.btn.ghost {
    background: transparent;
    color: var(--muted);
    border: 1px solid rgba(15, 23, 42, 0.04);
    padding: 8px 12px;
}

/* Improve focus outlines for keyboard users */
input:focus,
button:focus {
    outline: 3px solid color-mix(in srgb, var(--primary) 30%, transparent);
    outline-offset: 2px;
}

/* small link style inside form */
a {
    color: var(--primary);
    text-decoration: none;
}
a:hover {
    text-decoration: underline;
}
</style>
