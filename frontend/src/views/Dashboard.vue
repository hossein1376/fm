<template>
    <div class="container app-container dashboard-root">
        <!-- Header -->
        <header class="header" role="banner">
            <div class="header-inner">
                <div aria-hidden="true" class="brand-avatar">FM</div>
                <div>
                    <h1 class="app-title">File Manager</h1>
                    <div class="app-subtitle">
                        Manage hosts and files securely
                    </div>
                </div>
            </div>

            <div class="header-actions">
                <div class="user-name">
                    {{ authStore.user?.username }}
                </div>
                <!-- Theme toggle -->
                <Button
                    class="p-button-text"
                    @click="toggleTheme"
                    aria-label="Toggle theme"
                >
                    <component
                        :is="themeVal === 'dark' ? 'SunIcon' : 'MoonIcon'"
                    />
                </Button>
                <Button class="p-button-outlined" @click="refreshHosts">
                    <RefreshIcon /> <span>Refresh</span>
                </Button>
                <Button class="p-button-danger" @click="handleLogout">
                    <LogoutIcon /> <span>Sign out</span>
                </Button>
            </div>
        </header>

        <main class="row main-row">
            <!-- Sidebar -->
            <aside class="sidebar card sidebar-card" aria-label="Hosts">
                <div class="hosts-header">
                    <h2 class="hosts-title">Hosts</h2>
                    <div class="hosts-actions">
                        <Button
                            class="p-button-sm"
                            @click="showAddHostDialog = true"
                        >
                            <PlusIcon /> <span>+ Add</span>
                        </Button>
                        <Button
                            label="≡"
                            class="p-button-sm"
                            @click="toggleCompact"
                        />
                    </div>
                </div>

                <div class="list-card list-card-scroll">
                    <div
                        v-for="host in hostStore.hosts"
                        :key="host.id"
                        class="item host-item"
                        :class="{
                            active: hostStore.currentHost?.id === host.id,
                        }"
                        @click="selectHost(host)"
                        role="button"
                        :aria-pressed="hostStore.currentHost?.id === host.id"
                    >
                        <div class="host-item-left">
                            <div class="host-avatar">
                                {{
                                    host.name
                                        ? host.name.charAt(0).toUpperCase()
                                        : "H"
                                }}
                            </div>
                            <div class="meta">
                                <div class="host-name">
                                    {{ host.name || host.id }}
                                </div>
                                <div class="host-type">
                                    {{ host.host_type?.type || "unknown" }}
                                </div>
                            </div>
                        </div>

                        <div class="host-item-actions">
                            <Button
                                class="p-button-sm"
                                @click.stop="editHost(host)"
                            >
                                <PencilIcon />
                            </Button>
                            <Button
                                class="p-button-sm p-button-danger"
                                @click.stop="deleteHost(host.id)"
                            >
                                <TrashIcon />
                            </Button>
                        </div>
                    </div>

                    <div
                        v-if="!hostStore.hosts.length"
                        class="empty-state empty-state--padded"
                    >
                        No hosts — add one to get started
                    </div>
                </div>
            </aside>

            <!-- Content -->
            <section class="col content-col">
                <div class="content-topbar">
                    <div class="breadcrumb" aria-label="Breadcrumb">
                        <span class="crumb" @click="navigateTo('/')">Root</span>
                        <template v-for="(part, idx) in pathParts" :key="idx">
                            <span class="crumb-sep">/</span>
                            <span
                                class="crumb"
                                @click="navigateTo(getPathUpTo(idx))"
                                >{{ part }}</span
                            >
                        </template>
                    </div>

                    <div class="controls">
                        <InputText
                            v-model="search"
                            placeholder="Search files..."
                            class="search-input"
                        />

                        <Button
                            class="p-button-secondary"
                            @click="showUploadDialog = true"
                            :disabled="!hostStore.currentHost"
                        >
                            <UploadIcon /> <span>Upload</span>
                        </Button>
                        <Button
                            @click="showCreateDirDialog = true"
                            :disabled="!hostStore.currentHost"
                        >
                            <FolderPlusIcon /> <span>New Folder</span>
                        </Button>
                    </div>
                </div>

                <div class="card files-card">
                    <!-- Loading overlay (appears only after a small delay so quick ops don't flash) -->
                    <div
                        v-if="isLoading"
                        class="loading-overlay"
                        role="status"
                        aria-live="polite"
                    >
                        <div class="loading-content" aria-hidden="true">
                            <div class="spinner" />
                            <div class="loading-text">Loading…</div>
                        </div>
                    </div>

                    <div
                        v-if="!hostStore.currentHost"
                        class="empty-state empty-state--large"
                    >
                        Select a host to browse files.
                    </div>

                    <div v-else>
                        <div class="host-header">
                            <div>
                                <div class="host-title">
                                    {{
                                        hostStore.currentHost.name ||
                                        hostStore.currentHost.id
                                    }}
                                </div>
                                <div class="host-subtype">
                                    {{
                                        hostStore.currentHost.host_type?.type ||
                                        "unknown"
                                    }}
                                </div>
                            </div>

                            <div class="host-header-actions">
                                <Button
                                    class="p-button-text"
                                    @click="browseFiles"
                                >
                                    <RefreshIcon /> <span>Reload</span>
                                </Button>
                                <Button
                                    class="p-button-text"
                                    :disabled="currentPath === '/'"
                                    @click="goUp"
                                >
                                    <ChevronUpIcon /> <span>Up</span>
                                </Button>
                            </div>
                        </div>

                        <div
                            v-if="files.length === 0"
                            class="empty-state empty-state--medium"
                        >
                            No files found in this location.
                        </div>

                        <div v-else>
                            <!-- List view -->
                            <div class="file-list">
                                <div class="file-header">
                                    <div class="file-col-icon"></div>
                                    <div class="file-col-name">Name</div>
                                    <div class="file-col-type">Type</div>
                                    <div class="file-col-size">Size</div>
                                    <div class="file-col-modified">
                                        Modified
                                    </div>
                                    <div class="file-col-actions">Actions</div>
                                </div>
                                <article
                                    v-for="file in filteredFiles"
                                    :key="file.path"
                                    class="card file-row"
                                    @click="
                                        file.is_dir && handleFileClick(file)
                                    "
                                    @dblclick="
                                        !file.is_dir &&
                                        handleFileDoubleClick(file)
                                    "
                                >
                                    <div
                                        class="file-icon"
                                        :title="file.is_dir ? 'Folder' : 'File'"
                                    >
                                        <IconFolder
                                            v-if="file.is_dir"
                                            size="18"
                                        />
                                        <IconFile v-else size="18" />
                                    </div>
                                    <div class="file-name">
                                        {{ file.name }}
                                    </div>
                                    <div class="file-type">
                                        {{ file.is_dir ? "Folder" : "File" }}
                                    </div>
                                    <div class="file-size">
                                        {{ formatSize(file.size) }}
                                    </div>
                                    <div class="file-modified">
                                        {{ formatDate(file.modified) }}
                                    </div>
                                    <div class="file-row-actions">
                                        <Button
                                            v-if="!file.is_dir"
                                            class="p-button-sm p-button-secondary"
                                            @click.stop="downloadFile(file)"
                                            aria-label="Download"
                                        >
                                            <UploadIcon />
                                        </Button>
                                        <Button
                                            v-if="file.is_dir"
                                            class="p-button-sm"
                                            @click.stop="openInfo(file)"
                                            aria-label="Info"
                                        >
                                            <InfoIcon />
                                        </Button>
                                        <Button
                                            class="p-button-sm p-button-danger"
                                            @click.stop="deleteFile(file)"
                                        >
                                            <TrashIcon />
                                        </Button>
                                    </div>
                                </article>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </main>

        <!-- Add Host Dialog -->
        <Dialog
            :visible="showAddHostDialog"
            modal
            header="Add Host"
            class="dialog dialog-large"
            @hide="showAddHostDialog = false"
        >
            <form class="form" @submit.prevent="createHost">
                <div class="form-group">
                    <label>Name</label>
                    <InputText v-model="newHost.name" required />
                </div>

                <div class="form-group">
                    <label>Type</label>
                    <select v-model="newHost.type" required>
                        <option value="local">Local</option>
                        <option value="http">HTTP</option>
                        <option value="sftp">SFTP</option>
                    </select>
                </div>

                <div v-if="newHost.type === 'local'" class="form-group">
                    <label>Path</label>
                    <InputText
                        v-model="newHost.path"
                        placeholder="/srv/files"
                    />
                </div>

                <div v-if="newHost.type === 'http'" class="form-group">
                    <label>URL</label>
                    <InputText
                        v-model="newHost.url"
                        placeholder="https://example.com/files"
                    />
                </div>

                <div v-if="newHost.type === 'sftp'">
                    <div class="form-group">
                        <label>Host</label>
                        <InputText v-model="newHost.host" />
                    </div>
                    <div class="form-group">
                        <label>Port</label>
                        <InputText v-model="newHost.port" type="number" />
                    </div>
                    <div class="form-group">
                        <label>Username</label>
                        <InputText v-model="newHost.username" />
                    </div>
                    <div class="form-group">
                        <label>Password</label>
                        <InputText v-model="newHost.password" type="password" />
                    </div>
                </div>

                <div class="modal-actions modal-actions--top">
                    <Button
                        type="submit"
                        label="Create"
                        class="p-button-primary"
                    />
                    <Button
                        label="Cancel"
                        class="p-button-text"
                        @click="showAddHostDialog = false"
                    />
                </div>
            </form>
        </Dialog>

        <!-- Upload Dialog -->
        <Dialog
            :visible="showUploadDialog"
            modal
            header="Upload File"
            class="dialog dialog-medium"
            @hide="showUploadDialog = false"
        >
            <form class="form" @submit.prevent="uploadFile">
                <div class="form-group">
                    <label>File</label>
                    <input type="file" @change="handleFileSelect" />
                </div>

                <div class="modal-actions">
                    <Button
                        type="submit"
                        label="Upload"
                        class="p-button-primary"
                    />
                    <Button
                        label="Cancel"
                        class="p-button-text"
                        @click="showUploadDialog = false"
                    />
                </div>
            </form>
        </Dialog>

        <!-- Create Directory Dialog -->
        <Dialog
            :visible="showCreateDirDialog"
            modal
            header="New Folder"
            class="dialog dialog-small"
            @hide="showCreateDirDialog = false"
        >
            <form class="form" @submit.prevent="createDirectory">
                <div class="form-group">
                    <label>Folder name</label>
                    <InputText v-model="newDirName" required />
                </div>

                <div class="modal-actions">
                    <Button
                        type="submit"
                        label="Create"
                        class="p-button-primary"
                    />
                    <Button
                        label="Cancel"
                        class="p-button-text"
                        @click="showCreateDirDialog = false"
                    />
                </div>
            </form>
        </Dialog>

        <!-- Confirm Delete Dialog -->
        <Dialog
            :visible="showConfirmDialog"
            modal
            header="Confirm"
            class="dialog dialog-small"
            @hide="showConfirmDialog = false"
        >
            <p>{{ confirmMessage }}</p>
            <div class="modal-actions">
                <Button
                    label="Cancel"
                    class="p-button-text"
                    @click="showConfirmDialog = false"
                />
                <Button
                    label="Delete"
                    class="p-button-danger"
                    @click="
                        confirmAction();
                        showConfirmDialog = false;
                    "
                />
            </div>
        </Dialog>

        <!-- File Info Dialog -->
        <Dialog
            v-model:visible="infoVisible"
            class="info-dialog dialog dialog-small"
            modal
            :dismissableMask="true"
            :closable="false"
        >
            <template #header>
                <div
                    class="dialog-header"
                    style="
                        display: flex;
                        align-items: center;
                        justify-content: space-between;
                        width: 100%;
                    "
                >
                    <div class="dialog-title" style="font-weight: 700">
                        {{ infoFile ? infoFile.name : "Info" }}
                    </div>
                    <Button
                        icon="pi pi-times"
                        class="p-button-rounded p-button-danger p-button-sm"
                        @click="infoVisible = false"
                        aria-label="Close info dialog"
                    />
                </div>
            </template>

            <template #content>
                <div class="info-path">{{ infoFile?.path }}</div>
                <dl class="info-dl">
                    <div>
                        <dt class="info-dt">Type</dt>
                        <dd class="info-dd">
                            {{ infoFile?.is_dir ? "Folder" : "File" }}
                        </dd>
                    </div>
                    <div>
                        <dt class="info-dt">Size</dt>
                        <dd class="info-dd">
                            {{ formatSize(infoFile?.size) }}
                        </dd>
                    </div>
                    <div>
                        <dt class="info-dt">Modified</dt>
                        <dd class="info-dd">
                            {{ formatDate(infoFile?.modified) }}
                        </dd>
                    </div>
                    <div>
                        <dt class="info-dt">Path</dt>
                        <dd class="info-dd info-path-dd">
                            {{ infoFile?.path }}
                        </dd>
                    </div>
                </dl>

                <div class="modal-actions info-actions">
                    <button
                        type="button"
                        class="close-btn-red filled"
                        @click="infoVisible = false"
                    >
                        Close
                    </button>
                    <Button
                        v-if="infoFile && !infoFile.is_dir"
                        label="Download"
                        class="p-button-primary"
                        @click="downloadFile(infoFile)"
                    />
                </div>
            </template>
        </Dialog>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
import { useRouter } from "vue-router";
import { useAuthStore } from "../store/auth";
import { useHostStore } from "../store/host";
import api from "../services/api";
import websocket from "../services/websocket";
import themeService from "../services/theme";

// PrimeVue components (Button and InputText are registered globally in main.ts)
// Dialog is imported locally so we can use it in script setup template
import Dialog from "primevue/dialog";

// Icon wrappers
import IconFile from "../components/IconFile.vue";
import IconFolder from "../components/IconFolder.vue";
import { InfoIcon } from "../components/icons/SvgIcons";

// stores
const router = useRouter();
const authStore = useAuthStore();
const hostStore = useHostStore();

// reactive UI state
const currentPath = ref("/");
const files = ref<any[]>([]);
const showAddHostDialog = ref(false);
const showUploadDialog = ref(false);
const showCreateDirDialog = ref(false);
const showConfirmDialog = ref(false);
const confirmMessage = ref("");
const confirmAction = ref<() => void>(() => {});
const selectedFile = ref<File | null>(null);
const newDirName = ref("");
const newHost = ref<any>({
    name: "",
    type: "local",
    path: "",
    url: "",
    host: "",
    port: 22,
    username: "",
    password: "",
});
const infoFile = ref<any | null>(null);
const infoVisible = computed({
    get: () => !!infoFile.value,
    set: (v: boolean) => {
        if (!v) infoFile.value = null;
    },
});
const search = ref("");
const compact = ref(false);

// Loading overlay state with delayed show
const isLoading = ref(false);
let loadingTimer: ReturnType<typeof setTimeout> | null = null;
const loadingDelay = 250; // ms - show overlay only if the request takes longer than this

const startLoadingDelay = () => {
    // Ensure any prior timer is cleared
    if (loadingTimer) {
        clearTimeout(loadingTimer);
        loadingTimer = null;
    }
    // Start a new delayed timer to show the overlay
    loadingTimer = setTimeout(() => {
        isLoading.value = true;
        loadingTimer = null;
    }, loadingDelay);
};

const stopLoadingDelay = () => {
    if (loadingTimer) {
        clearTimeout(loadingTimer);
        loadingTimer = null;
    }
    isLoading.value = false;
};

// theme bindings
const { theme, toggleTheme } = themeService as any;
const themeVal = computed(() => theme?.value ?? "light");

// computed helpers
const pathParts = computed(() => currentPath.value.split("/").filter((p) => p));
const filteredFiles = computed(() => {
    if (!search.value) return files.value;
    const q = search.value.toLowerCase();
    return files.value.filter((f: any) =>
        (f.name || "").toLowerCase().includes(q),
    );
});

// lifecycle
onMounted(async () => {
    await hostStore.fetchHosts();
    websocket.connect();
});

onUnmounted(() => {
    // Clean up any pending timer when component is destroyed
    if (loadingTimer) {
        clearTimeout(loadingTimer);
        loadingTimer = null;
    }
});

// actions
const refreshHosts = async () => {
    await hostStore.fetchHosts();
};

const handleLogout = () => {
    authStore.logout();
    websocket.disconnect();
    router.push("/login");
};

const selectHost = async (host: any) => {
    hostStore.setCurrentHost(host);
    currentPath.value = "/";
    await browseFiles();
};

const browseFiles = async () => {
    if (!hostStore.currentHost) return;
    startLoadingDelay();
    try {
        const response = await api.browseFiles(
            hostStore.currentHost.id,
            currentPath.value,
        );
        files.value = response.files || [];
    } catch (err) {
        console.error("browseFiles error", err);
        files.value = [];
    } finally {
        // ensure overlay/timer cleared regardless of outcome
        stopLoadingDelay();
    }
};

const navigateTo = async (path: string) => {
    currentPath.value = path;
    await browseFiles();
};

const getPathUpTo = (index: number) =>
    "/" + pathParts.value.slice(0, index + 1).join("/");

const handleFileDoubleClick = async (file: any) => {
    if (!file.is_dir) {
        await downloadFile(file);
    }
};

const handleFileClick = async (file: any) => {
    if (file.is_dir) {
        currentPath.value = file.path;
        await browseFiles();
    } else {
        // Single-click on a file opens the info panel for quick actions
        openInfo(file);
    }
};

const createHost = async () => {
    const hostData: any = {
        name: newHost.value.name,
        host_type: { type: newHost.value.type },
        config: {},
    };

    if (newHost.value.type === "local") {
        hostData.config.path = newHost.value.path;
    } else if (newHost.value.type === "http") {
        hostData.config.url = newHost.value.url;
    } else if (newHost.value.type === "sftp") {
        hostData.config.host = newHost.value.host;
        hostData.config.port = newHost.value.port;
        hostData.config.username = newHost.value.username;
        hostData.config.password_encrypted = newHost.value.password;
    }

    const success = await hostStore.createHost(hostData);
    if (success) {
        showAddHostDialog.value = false;
        newHost.value = {
            name: "",
            type: "local",
            path: "",
            url: "",
            host: "",
            port: 22,
            username: "",
            password: "",
        };
    }
};

const editHost = (host: any) => {
    newHost.value = {
        name: host.name || "",
        type: host.host_type?.type || "local",
        path: host.config?.path || "",
        url: host.config?.url || "",
        host: host.config?.host || "",
        port: host.config?.port || 22,
        username: host.config?.username || "",
        password: "",
    };
    showAddHostDialog.value = true;
};

const showConfirm = (message: string, action: () => void) => {
    confirmMessage.value = message;
    confirmAction.value = action;
    showConfirmDialog.value = true;
};

const deleteHost = async (id: string) => {
    showConfirm("Delete this host?", async () => {
        await hostStore.deleteHost(id);
        if (hostStore.currentHost?.id === id) {
            hostStore.setCurrentHost(null);
            files.value = [];
        }
    });
};

const handleFileSelect = (event: any) => {
    selectedFile.value = event.target.files?.[0] ?? null;
};

const uploadFile = async () => {
    if (!selectedFile.value || !hostStore.currentHost) return;
    try {
        const filename = selectedFile.value.name;
        const uploadPath =
            currentPath.value === "/"
                ? `/${filename}`
                : `${currentPath.value}/${filename}`;
        await api.uploadFile(
            hostStore.currentHost.id,
            uploadPath,
            selectedFile.value,
        );
        showUploadDialog.value = false;
        selectedFile.value = null;
        await browseFiles();
    } catch (err) {
        console.error("upload failed", err);
    }
};

const downloadFile = async (file: any) => {
    if (!hostStore.currentHost) return;
    try {
        const blob = await api.downloadFile(
            hostStore.currentHost.id,
            file.path,
        );
        const url = window.URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = file.name;
        document.body.appendChild(a);
        a.click();
        window.URL.revokeObjectURL(url);
        document.body.removeChild(a);
    } catch (err) {
        console.error("download failed", err);
    }
};

const deleteFile = async (file: any) => {
    if (!hostStore.currentHost) return;
    showConfirm(`Delete ${file.name}?`, async () => {
        try {
            await api.deleteFile(hostStore.currentHost.id, file.path);
            await browseFiles();
        } catch (err) {
            console.error("delete failed", err);
        }
    });
};

const createDirectory = async () => {
    if (!hostStore.currentHost || !newDirName.value) return;
    try {
        const dirPath =
            currentPath.value === "/"
                ? `/${newDirName.value}`
                : `${currentPath.value}/${newDirName.value}`;
        await api.createDirectory(hostStore.currentHost.id, dirPath);
        showCreateDirDialog.value = false;
        newDirName.value = "";
        await browseFiles();
    } catch (err) {
        console.error("create dir failed", err);
    }
};

const openInfo = (file: any) => {
    infoFile.value = file;
};

const goUp = async () => {
    if (currentPath.value === "/") return;
    const parts = pathParts.value;
    if (parts.length <= 1) {
        currentPath.value = "/";
    } else {
        currentPath.value = "/" + parts.slice(0, parts.length - 1).join("/");
    }
    await browseFiles();
};

const formatSize = (bytes?: number) => {
    if (bytes == null) return "—";
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + " " + sizes[i];
};

const formatDate = (date?: string | null) => {
    if (!date) return "N/A";
    return new Date(date).toLocaleString();
};

const toggleCompact = () => {
    compact.value = !compact.value;
};
</script>

<style scoped>
.dashboard-root .page-container {
    padding-top: 20px;
    padding-bottom: 48px;
}

/* Header */
.dashboard-root .header {
    border-bottom: 1px solid rgba(15, 23, 42, 0.04);
    margin-bottom: 8px;
}

.dashboard-root .header-inner {
    display: flex;
    align-items: center;
    gap: 14px;
    padding: 12px 0;
    justify-content: space-between;
}

.dashboard-root .brand-avatar {
    width: 44px;
    height: 44px;
    border-radius: 10px;
    display: grid;
    place-items: center;
    background: linear-gradient(135deg, var(--primary), var(--primary-600));
    box-shadow: 0 8px 20px rgba(79, 70, 229, 0.12);
    color: white;
    font-weight: 700;
}

.dashboard-root .app-title {
    margin: 0;
    font-size: 1.125rem;
}

.dashboard-root .app-subtitle {
    color: var(--muted);
    font-size: 0.9rem;
}

.dashboard-root .header-actions {
    display: flex;
    align-items: center;
    gap: 10px;
}

.dashboard-root .user-name {
    color: var(--muted);
    font-weight: 600;
}

.dashboard-root .main-row {
    margin-top: 18px;
    gap: 20px;
    display: flex;
}

.dashboard-root .sidebar {
    width: 300px;
    min-width: 220px;
    max-height: calc(100vh - 140px);
    overflow: auto;
}

.dashboard-root .sidebar-card {
    padding: 8px;
}

.dashboard-root .hosts-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 6px 8px;
    width: 100%;
    box-sizing: border-box;
    gap: 8px;
}

.dashboard-root .hosts-title {
    font-size: 1rem;
    margin: 0;
}

.dashboard-root .hosts-actions {
    display: flex;
    gap: 8px;
    align-items: center;
    white-space: nowrap; /* prevent buttons from wrapping */
}

.dashboard-root .list-card-scroll {
    margin-top: 8px;
    overflow: auto;
}

.dashboard-root .host-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 12px;
    gap: 12px;
    transition:
        background 0.15s ease,
        transform 0.12s ease;
}

.dashboard-root .host-item:hover {
    transform: translateY(-1px);
}

.dashboard-root .host-item-left {
    display: flex;
    align-items: center;
    gap: 12px;
}

.dashboard-root .host-avatar {
    width: 44px;
    height: 44px;
    border-radius: 8px;
    display: grid;
    place-items: center;
    background: linear-gradient(180deg, var(--primary-400), var(--primary-600));
    color: white;
    font-weight: 700;
}

.dashboard-root .host-name {
    font-weight: 600;
}

.dashboard-root .host-type {
    color: var(--muted);
    font-size: 0.85rem;
}

.dashboard-root .host-item-actions {
    display: flex;
    gap: 8px;
    align-items: center;
}

.dashboard-root .list-card .item.active {
    background: color-mix(in srgb, var(--primary-400) 8%, transparent);
    border-left: 3px solid var(--primary);
}

/* Content column */
.dashboard-root .content-col {
    min-width: 0;
    flex: 1;
}

/* Top controls */
.dashboard-root .content-topbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 12px;
    margin-bottom: 12px;
}

.dashboard-root .controls {
    display: flex;
    gap: 10px;
    align-items: center;
}

.dashboard-root .search-input {
    width: 220px;
}

/* Files card */
.dashboard-root .files-card {
    padding: 16px;
    position: relative;
}

/* Loading overlay */
.dashboard-root .loading-overlay {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    background: rgba(255, 255, 255, 0.6);
    backdrop-filter: blur(2px);
}

[data-theme="dark"] .dashboard-root .loading-overlay,
.dark .dashboard-root .loading-overlay {
    background: rgba(8, 10, 18, 0.6);
}

.dashboard-root .loading-content {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    border-radius: 10px;
    background: rgba(255, 255, 255, 0.9);
    box-shadow: 0 6px 20px rgba(2, 6, 23, 0.08);
    color: var(--text, #000);
}

[data-theme="dark"] .dashboard-root .loading-content,
.dark .dashboard-root .loading-content {
    background: rgba(11, 18, 32, 0.85);
    color: var(--text, #fff);
}

.dashboard-root .spinner {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    border: 4px solid rgba(0, 0, 0, 0.08);
    border-top-color: var(--primary, #4f46e5);
    animation: spin 1s linear infinite;
}

[data-theme="dark"] .dashboard-root .spinner,
.dark .dashboard-root .spinner {
    border: 4px solid rgba(255, 255, 255, 0.08);
}

.dashboard-root .loading-text {
    font-weight: 600;
    font-size: 0.95rem;
}

/* Host header (title + actions) */
.dashboard-root .host-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
    gap: 12px;
    width: 100%;
    box-sizing: border-box;
}

/* Ensure the title column can shrink and keep actions on the right */
.dashboard-root .host-header > div:first-child {
    flex: 1 1 auto;
    min-width: 0; /* allow text truncation if needed */
}

/* Actions area on the right (reload / up) */
.dashboard-root .host-header-actions {
    display: flex;
    gap: 8px;
    align-items: center;
    white-space: nowrap; /* keep buttons on one line */
}

/* Title and subtype */
.dashboard-root .host-title {
    font-weight: 700;
}

.dashboard-root .host-subtype,
.dashboard-root .host-subtitle {
    color: var(--muted);
    font-size: 0.9rem;
    margin-top: 4px;
}

.dashboard-root .files-empty {
    height: 160px;
}

/* List view as table */
.dashboard-root .file-list {
    display: flex;
    flex-direction: column;
}

.dashboard-root .file-header {
    display: grid;
    grid-template-columns: 40px 1fr 100px 100px 150px 120px;
    gap: 12px;
    padding: 8px 12px;
    font-weight: 600;
    color: var(--muted);
    border-bottom: 1px solid var(--border);
    font-size: 0.9rem;
}

.dashboard-root .file-row {
    display: grid;
    grid-template-columns: 40px 1fr 100px 100px 150px 120px;
    gap: 12px;
    padding: 12px;
    align-items: center;
    cursor: default;
    border-radius: 8px;
    transition: background 0.15s ease;
}

.dashboard-root .file-row:hover {
    background: color-mix(in srgb, var(--primary-400) 5%, transparent);
}

.dashboard-root .file-name {
    font-weight: 600;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.dashboard-root .file-type,
.dashboard-root .file-size,
.dashboard-root .file-modified {
    color: var(--muted);
    font-size: 0.85rem;
}

.dashboard-root .file-row-actions {
    display: flex;
    gap: 8px;
    align-items: center;
    justify-content: flex-end;
}

/* Dialog sizes */
.dashboard-root .dialog {
    /* primevue dialog uses inline width normally; we set classes to provide consistent widths */
}

/* normalized dialog sizes used across the app */
.dashboard-root .dialog-small {
    width: 420px;
}

.dashboard-root .dialog-medium {
    width: 560px;
}

.dashboard-root .dialog-large {
    width: 720px;
}

/* Modal / form layout */
.dashboard-root .modal .modal-content {
    max-width: 720px;
}

.dashboard-root .modal-actions {
    display: flex;
    gap: 10px;
    justify-content: flex-end;
    margin-top: 12px;
}

.dashboard-root .modal-actions--top {
    margin-top: 14px;
}

/* Info dialog dark-mode compatibility and close button styling */
/* Apply the app's surface/text variables so dialog follows theme colors */
.dashboard-root .info-dialog,
.dashboard-root .info-dialog .p-dialog,
.dashboard-root .info-dialog .p-dialog-content,
.dashboard-root .info-dialog .p-dialog-titlebar,
.dashboard-root .info-dialog .p-dialog-header,
.dashboard-root .fallback-dialog {
    background: var(--surface, #fff);
    color: var(--text, #000);
}

/* If a data-theme attribute is used for dark mode, ensure dialogs flip */
[data-theme="dark"] .dashboard-root .info-dialog,
[data-theme="dark"] .dashboard-root .info-dialog .p-dialog,
[data-theme="dark"] .dashboard-root .info-dialog .p-dialog-content,
[data-theme="dark"] .dashboard-root .fallback-dialog {
    background: var(--surface, #0b1220);
    color: var(--text, #fff);
}

/* Generic fallback for when dark mode toggles set a .dark class on root */
.dark .dashboard-root .info-dialog,
.dark .dashboard-root .info-dialog .p-dialog,
.dark .dashboard-root .info-dialog .p-dialog-content,
.dark .dashboard-root .fallback-dialog {
    background: var(--surface, #0b1220);
    color: var(--text, #fff);
}

/* Ensure dialog internals inherit text color */
.dashboard-root .info-dialog .p-dialog-content *,
.dashboard-root .fallback-dialog * {
    color: inherit;
}

/* Close button: make clearly red across both PrimeVue buttons and fallbacks */
.dashboard-root .close-btn-red {
    color: var(--danger, #dc2626) !important;
    background: transparent !important;
    border-color: transparent !important;
}

/* When we need filled destructive action style */
.dashboard-root .close-btn-red.filled {
    /* Ensure solid, non-translucent background */
    background-image: none !important;
    background-color: var(--danger, #dc2626) !important;
    color: #ffffff !important;
    border: 1px solid rgba(0, 0, 0, 0.12) !important;
    box-shadow: 0 6px 18px rgba(239, 68, 68, 0.18) !important;
    opacity: 1 !important;

    /* Prevent any blending/filters from making it translucent */
    mix-blend-mode: normal !important;
    filter: none !important;
    -webkit-filter: none !important;
    backdrop-filter: none !important;
    -webkit-backdrop-filter: none !important;
    isolation: isolate !important;

    /* Ensure it sits above any overlay or icon that might render on top */
    position: relative !important;
    z-index: 1101 !important;

    /* Tighten visual: stronger border for contrast */
    border-radius: 6px !important;
    padding: 8px 12px !important;
}

/* Info dialog internals */
.dashboard-root .info-path {
    color: var(--muted);
    margin-bottom: 12px;
}

.dashboard-root .info-dl {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 8px;
}

.dashboard-root .info-dt {
    font-weight: 600;
}

.dashboard-root .info-dd {
    margin: 0;
    color: var(--muted);
}

.dashboard-root .info-path-dd {
    word-break: break-all;
}

/* Ensure SVG icons in host header actions have proper color in dark mode */
[data-theme="dark"] .dashboard-root .host-header-actions button svg,
.dark .dashboard-root .host-header-actions button svg {
    color: var(--text);
}

/* responsive */
@keyframes spin {
    to {
        transform: rotate(360deg);
    }
}

@media (max-width: 900px) {
    .dashboard-root .sidebar {
        width: 100%;
        order: 2;
    }
    .dashboard-root main.row {
        flex-direction: column;
    }
}
</style>
