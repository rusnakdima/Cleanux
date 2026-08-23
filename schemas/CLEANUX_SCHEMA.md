# Cleanux SDUI Schema — All 10 Pages

## Schema Structure
Each page follows the SDUI schema format:
```json
{
  "id": "page-id",
  "title": "Page Title",
  "route": "/route",
  "layout": "stacked|sidebar|grid",
  "data_binding": "handler_id",
  "elements": [...]
}
```

## Page Route Map
| Page | Route | Handler(s) | Data Entities |
|------|-------|------------|---------------|
| Home | `/` | — | Static |
| Dashboard | `/dashboard` | `get_system_stats`, `get_memory_info` | SystemStats, MemoryInfo |
| Cleaner | `/cleaner` | `get_junk_summary`, `scan_cache`, `scan_trash`, `scan_logs`, `scan_large_files`, `scan_duplicates`, `clean_items` | JunkSummary, JunkItem |
| Automation | `/automation` | `list_recipes`, `create_recipe`, `delete_recipe`, `trigger_recipe` | AutomationRecipe |
| System | `/system` | `get_system_stats`, `get_memory_info`, `get_cpu_temp`, `get_gpu_temp`, `get_processes` | SystemStats, ProcessMemory |
| Storage | `/storage` | `scan_directory`, `get_directory_size`, `find_duplicates`, `list_backups`, `scan_browser_caches` | DirectoryInfo, BackupInfo |
| Profiles | `/profiles` | CRUD for CleaningProfile | CleaningProfile |
| Settings | `/settings` | — | AppSettings |
| Reports | `/reports` | CRUD for CleaningReport, ExecutionHistory | CleaningReport, ExecutionHistory |
| Logs | `/logs` | `find_empty_directories`, journal handlers | LogInfo |

---

## PAGE 1: Home (`/`)

```json
{
  "id": "home-page",
  "title": "Home",
  "route": "/",
  "layout": "stacked",
  "elements": [
    {
      "id": "hero-section",
      "component": "div",
      "props": { "classes": "text-center py-20" },
      "children": [
        {
          "id": "hero-title",
          "component": "text",
          "props": { "text": "Welcome to Cleanux", "classes": "text-4xl font-bold mb-4" }
        },
        {
          "id": "hero-subtitle",
          "component": "text",
          "props": { "text": "Your system cleanup and optimization tool", "classes": "text-gray-600 mb-8 max-w-md mx-auto" }
        },
        {
          "id": "feature-grid",
          "component": "div",
          "props": { "classes": "grid grid-cols-1 md:grid-cols-3 gap-6 max-w-4xl mx-auto mt-12" },
          "children": [
            {
              "id": "feature-dashboard",
              "component": "card",
              "props": { "classes": "p-6" },
              "children": [
                { "id": "dash-title", "component": "text", "props": { "text": "Dashboard", "classes": "text-xl font-semibold mb-2" } },
                { "id": "dash-desc", "component": "text", "props": { "text": "View system health and quick actions", "classes": "text-gray-600 mb-4" } },
                {
                  "id": "dash-link",
                  "component": "action-button",
                  "props": { "label": "Go →", "action": "navigate", "route": "/dashboard", "classes": "text-blue-600 hover:text-blue-800 font-medium" }
                }
              ]
            },
            {
              "id": "feature-cleaner",
              "component": "card",
              "props": { "classes": "p-6" },
              "children": [
                { "id": "clean-title", "component": "text", "props": { "text": "Cleaner", "classes": "text-xl font-semibold mb-2" } },
                { "id": "clean-desc", "component": "text", "props": { "text": "Clean cache, trash, and system junk", "classes": "text-gray-600 mb-4" } },
                {
                  "id": "clean-link",
                  "component": "action-button",
                  "props": { "label": "Go →", "action": "navigate", "route": "/cleaner", "classes": "text-blue-600 hover:text-blue-800 font-medium" }
                }
              ]
            },
            {
              "id": "feature-automation",
              "component": "card",
              "props": { "classes": "p-6" },
              "children": [
                { "id": "auto-title", "component": "text", "props": { "text": "Automation", "classes": "text-xl font-semibold mb-2" } },
                { "id": "auto-desc", "component": "text", "props": { "text": "Set up automated cleaning recipes", "classes": "text-gray-600 mb-4" } },
                {
                  "id": "auto-link",
                  "component": "action-button",
                  "props": { "label": "Go →", "action": "navigate", "route": "/automation", "classes": "text-blue-600 hover:text-blue-800 font-medium" }
                }
              ]
            }
          ]
        }
      ]
    }
  ]
}
```

---

## PAGE 2: Dashboard (`/dashboard`)

**Data source**: `get_system_stats` → `SystemStats`; `get_memory_info` → `MemoryInfo`

```json
{
  "id": "dashboard-page",
  "title": "Dashboard",
  "route": "/dashboard",
  "layout": "stacked",
  "data_binding": "get_system_stats",
  "elements": [
    {
      "id": "stats-grid",
      "component": "div",
      "props": { "classes": "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4" },
      "children": [
        {
          "id": "memory-card",
          "component": "card",
          "props": { "classes": "p-4" },
          "children": [
            { "id": "memory-label", "component": "text", "props": { "text": "Memory Used", "classes": "text-sm text-gray-500" } },
            { "id": "memory-value", "component": "text", "props": { "bind": "memory_used_gb", "format": "{value} / {total} GB", "classes": "text-2xl font-bold" } },
            { "id": "memory-trend", "component": "badge", "props": { "bind": "memory_trend", "variant": "neutral" } }
          ]
        },
        {
          "id": "cpu-card",
          "component": "card",
          "props": { "classes": "p-4" },
          "children": [
            { "id": "cpu-label", "component": "text", "props": { "text": "CPU Usage", "classes": "text-sm text-gray-500" } },
            { "id": "cpu-value", "component": "text", "props": { "bind": "cpu_percent", "format": "{value}%", "classes": "text-2xl font-bold" } },
            { "id": "cpu-trend", "component": "badge", "props": { "bind": "cpu_trend", "variant": "neutral" } }
          ]
        },
        {
          "id": "storage-card",
          "component": "card",
          "props": { "classes": "p-4" },
          "children": [
            { "id": "storage-label", "component": "text", "props": { "text": "Storage", "classes": "text-sm text-gray-500" } },
            { "id": "storage-value", "component": "text", "props": { "bind": "storage_used_gb", "format": "{value} / {total} GB", "classes": "text-2xl font-bold" } },
            { "id": "storage-bar", "component": "progress", "props": { "bind": "storage_percent", "max": 100 } },
            { "id": "storage-trend", "component": "badge", "props": { "bind": "storage_trend", "variant": "neutral" } }
          ]
        },
        {
          "id": "temp-card",
          "component": "card",
          "props": { "classes": "p-4" },
          "children": [
            { "id": "temp-label", "component": "text", "props": { "text": "Temperature", "classes": "text-sm text-gray-500" } },
            { "id": "temp-value", "component": "text", "props": { "bind": "cpu_temp_c", "format": "{value}°C", "classes": "text-2xl font-bold" } },
            { "id": "temp-status", "component": "badge", "props": { "bind": "temp_status", "variant": "success|warning|danger" } }
          ]
        }
      ]
    },
    {
      "id": "quick-actions",
      "component": "card",
      "props": { "classes": "p-6" },
      "children": [
        { "id": "qa-title", "component": "text", "props": { "text": "Quick Actions", "classes": "text-lg font-semibold mb-4" } },
        {
          "id": "qa-buttons",
          "component": "div",
          "props": { "classes": "flex flex-wrap gap-3" },
          "children": [
            { "id": "btn-optimize", "component": "button", "props": { "label": "Optimize Memory", "action": "optimize-memory", "variant": "primary" } },
            { "id": "btn-scan", "component": "button", "props": { "label": "Scan System", "action": "scan-system", "variant": "secondary" } },
            { "id": "btn-refresh", "component": "button", "props": { "label": "Refresh Stats", "action": "refresh-stats", "variant": "outline" } }
          ]
        }
      ]
    },
    {
      "id": "system-status",
      "component": "card",
      "props": { "classes": "p-6" },
      "children": [
        { "id": "ss-title", "component": "text", "props": { "text": "System Status", "classes": "text-lg font-semibold mb-4" } },
        {
          "id": "ss-rows",
          "component": "div",
          "props": { "classes": "space-y-3" },
          "children": [
            { "id": "ss-last-cleanup", "component": "div", "props": { "classes": "flex justify-between" }, "children": [
              { "id": "slc-label", "component": "text", "props": { "text": "Last Cleanup", "classes": "text-gray-600" } },
              { "id": "slc-value", "component": "text", "props": { "bind": "last_cleanup", "classes": "font-medium" } }
            ]},
            { "id": "ss-active-recipes", "component": "div", "props": { "classes": "flex justify-between" }, "children": [
              { "id": "sar-label", "component": "text", "props": { "text": "Active Recipes", "classes": "text-gray-600" } },
              { "id": "sar-value", "component": "text", "props": { "bind": "active_recipes_count", "classes": "font-medium" } }
            ]},
            { "id": "ss-health-score", "component": "div", "props": { "classes": "flex justify-between" }, "children": [
              { "id": "shs-label", "component": "text", "props": { "text": "Health Score", "classes": "text-gray-600" } },
              { "id": "shs-value", "component": "badge", "props": { "bind": "health_score", "variant": "success|warning|danger" } }
            ]}
          ]
        }
      ]
    }
  ]
}
```

**KAS bindings needed**:
- `get_system_stats` → `SystemStats { cpu_percent, memory_used_mb, memory_total_mb, storage_used_gb, storage_total_gb, cpu_temp_c }`
- `get_memory_info` → `MemoryInfo { used, total, swap_used, swap_total }`

---

## PAGE 3: Cleaner (`/cleaner`)

**Data source**: `get_junk_summary` → `JunkSummary`; `scan_cache/trash/logs/large_files/duplicates`

```json
{
  "id": "cleaner-page",
  "title": "System Cleaner",
  "route": "/cleaner",
  "layout": "stacked",
  "data_binding": "get_junk_summary",
  "elements": [
    {
      "id": "scan-header",
      "component": "div",
      "props": { "classes": "flex justify-between items-center mb-6" },
      "children": [
        { "id": "sh-title", "component": "text", "props": { "text": "Junk Categories", "classes": "text-lg font-semibold" } },
        { "id": "btn-scan-all", "component": "button", "props": { "label": "Scan All", "action": "scan-all", "variant": "primary" } }
      ]
    },
    {
      "id": "junk-grid",
      "component": "div",
      "props": { "classes": "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6" },
      "children": [
        {
          "id": "card-cache",
          "component": "card",
          "props": { "title": "Cache Cleaner", "classes": "p-6" },
          "children": [
            { "id": "cache-desc", "component": "text", "props": { "text": "Remove application cached files", "classes": "text-gray-600 text-sm mb-4" } },
            { "id": "cache-size", "component": "text", "props": { "bind": "cache_size", "format": "{value}", "classes": "text-2xl font-bold" } },
            {
              "id": "cache-actions",
              "component": "div",
              "props": { "classes": "flex items-center justify-between mt-4" },
              "children": [
                { "id": "btn-scan-cache", "component": "button", "props": { "label": "Scan", "action": "scan_cache", "variant": "ghost", "size": "sm" } },
                { "id": "btn-clean-cache", "component": "button", "props": { "label": "Clean", "action": "clean_cache", "variant": "primary", "size": "sm" } }
              ]
            }
          ]
        },
        {
          "id": "card-trash",
          "component": "card",
          "props": { "title": "Trash", "classes": "p-6" },
          "children": [
            { "id": "trash-desc", "component": "text", "props": { "text": "Empty the trash bin", "classes": "text-gray-600 text-sm mb-4" } },
            { "id": "trash-size", "component": "text", "props": { "bind": "trash_size", "format": "{value}", "classes": "text-2xl font-bold" } },
            {
              "id": "trash-actions",
              "component": "div",
              "props": { "classes": "flex items-center justify-between mt-4" },
              "children": [
                { "id": "btn-scan-trash", "component": "button", "props": { "label": "Scan", "action": "scan_trash", "variant": "ghost", "size": "sm" } },
                { "id": "btn-clean-trash", "component": "button", "props": { "label": "Clean", "action": "clean_trash", "variant": "primary", "size": "sm" } }
              ]
            }
          ]
        },
        {
          "id": "card-logs",
          "component": "card",
          "props": { "title": "Logs", "classes": "p-6" },
          "children": [
            { "id": "logs-desc", "component": "text", "props": { "text": "Clean old log files", "classes": "text-gray-600 text-sm mb-4" } },
            { "id": "logs-size", "component": "text", "props": { "bind": "logs_size", "format": "{value}", "classes": "text-2xl font-bold" } },
            {
              "id": "logs-actions",
              "component": "div",
              "props": { "classes": "flex items-center justify-between mt-4" },
              "children": [
                { "id": "btn-scan-logs", "component": "button", "props": { "label": "Scan", "action": "scan_logs", "variant": "ghost", "size": "sm" } },
                { "id": "btn-clean-logs", "component": "button", "props": { "label": "Clean", "action": "clean_logs", "variant": "primary", "size": "sm" } }
              ]
            }
          ]
        },
        {
          "id": "card-temp",
          "component": "card",
          "props": { "title": "System Temp", "classes": "p-6" },
          "children": [
            { "id": "temp-desc", "component": "text", "props": { "text": "Remove temporary files", "classes": "text-gray-600 text-sm mb-4" } },
            { "id": "temp-size", "component": "text", "props": { "bind": "temp_size", "format": "{value}", "classes": "text-2xl font-bold" } },
            {
              "id": "temp-actions",
              "component": "div",
              "props": { "classes": "flex items-center justify-between mt-4" },
              "children": [
                { "id": "btn-scan-temp", "component": "button", "props": { "label": "Scan", "action": "scan_temp", "variant": "ghost", "size": "sm" } },
                { "id": "btn-clean-temp", "component": "button", "props": { "label": "Clean", "action": "clean_temp", "variant": "primary", "size": "sm" } }
              ]
            }
          ]
        },
        {
          "id": "card-browser",
          "component": "card",
          "props": { "title": "Browser Cache", "classes": "p-6" },
          "children": [
            { "id": "browser-desc", "component": "text", "props": { "text": "Clean browser cache files", "classes": "text-gray-600 text-sm mb-4" } },
            { "id": "browser-size", "component": "text", "props": { "bind": "browser_size", "format": "{value}", "classes": "text-2xl font-bold" } },
            {
              "id": "browser-actions",
              "component": "div",
              "props": { "classes": "flex items-center justify-between mt-4" },
              "children": [
                { "id": "btn-scan-browser", "component": "button", "props": { "label": "Scan", "action": "scan_browser", "variant": "ghost", "size": "sm" } },
                { "id": "btn-clean-browser", "component": "button", "props": { "label": "Clean", "action": "clean_browser", "variant": "primary", "size": "sm" } }
              ]
            }
          ]
        },
        {
          "id": "card-packages",
          "component": "card",
          "props": { "title": "Package Cache", "classes": "p-6" },
          "children": [
            { "id": "pkg-desc", "component": "text", "props": { "text": "Remove package manager cache", "classes": "text-gray-600 text-sm mb-4" } },
            { "id": "pkg-size", "component": "text", "props": { "bind": "package_size", "format": "{value}", "classes": "text-2xl font-bold" } },
            {
              "id": "pkg-actions",
              "component": "div",
              "props": { "classes": "flex items-center justify-between mt-4" },
              "children": [
                { "id": "btn-scan-pkg", "component": "button", "props": { "label": "Scan", "action": "scan_packages", "variant": "ghost", "size": "sm" } },
                { "id": "btn-clean-pkg", "component": "button", "props": { "label": "Clean", "action": "clean_packages", "variant": "primary", "size": "sm" } }
              ]
            }
          ]
        }
      ]
    },
    {
      "id": "scan-progress",
      "component": "div",
      "props": { "classes": "mt-6" },
      "children": [
        { "id": "sp-bar", "component": "progress", "props": { "bind": "scan_progress", "max": 100, "show_label": true } },
        { "id": "sp-status", "component": "text", "props": { "bind": "scan_status", "classes": "text-sm text-gray-500 mt-2" } }
      ]
    }
  ]
}
```

**KAS handlers needed**:
- `get_junk_summary` → `JunkSummary { cache_bytes, trash_bytes, logs_bytes, temp_bytes, browser_bytes, package_bytes }`
- `scan_cache`, `scan_trash`, `scan_logs`, `scan_large_files`, `scan_duplicates` → `Vec<JunkItem>`
- `clean_items` → `CleaningReport`

---

## PAGE 4: Automation (`/automation`)

**Data source**: `list_recipes`, `create_recipe`, `delete_recipe`, `trigger_recipe`

```json
{
  "id": "automation-page",
  "title": "Automation",
  "route": "/automation",
  "layout": "stacked",
  "data_binding": "list_recipes",
  "elements": [
    {
      "id": "page-header",
      "component": "div",
      "props": { "classes": "flex justify-between items-center mb-6" },
      "children": [
        { "id": "ph-title", "component": "text", "props": { "text": "Automation Recipes", "classes": "text-lg font-semibold" } },
        { "id": "btn-new", "component": "button", "props": { "label": "New Recipe", "action": "create-recipe", "variant": "primary" } }
      ]
    },
    {
      "id": "recipes-grid",
      "component": "div",
      "props": { "classes": "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4" },
      "children": [
        {
          "id": "recipe-card-template",
          "component": "card",
          "props": { "bind": "recipe.id", "classes": "p-6" },
          "children": [
            { "id": "rc-title", "component": "text", "props": { "bind": "recipe.name", "classes": "text-xl font-semibold mb-2" } },
            { "id": "rc-desc", "component": "text", "props": { "bind": "recipe.description", "classes": "text-gray-600 text-sm mb-4" } },
            { "id": "rc-trigger", "component": "text", "props": { "bind": "recipe.trigger", "classes": "text-xs text-gray-500 mb-4" } },
            {
              "id": "rc-footer",
              "component": "div",
              "props": { "classes": "flex items-center justify-between" },
              "children": [
                {
                  "id": "rc-status",
                  "component": "badge",
                  "props": { "bind": "recipe.enabled", "variant_map": { "true": "success", "false": "neutral" }, "label_map": { "true": "Enabled", "false": "Disabled" } }
                },
                {
                  "id": "rc-actions",
                  "component": "div",
                  "props": { "classes": "flex gap-2" },
                  "children": [
                    { "id": "rc-btn-edit", "component": "button", "props": { "label": "Edit", "action": "edit-recipe", "bind": "recipe.id", "variant": "ghost", "size": "sm" } },
                    { "id": "rc-btn-run", "component": "button", "props": { "label": "Run", "action": "trigger-recipe", "bind": "recipe.id", "variant": "primary", "size": "sm" } }
                  ]
                }
              ]
            }
          ]
        }
      ]
    },
    {
      "id": "empty-state",
      "component": "div",
      "props": { "classes": "text-center py-12" },
      "children": [
        { "id": "es-icon", "component": "icon", "props": { "icon": "calendar", "classes": "w-12 h-12 mx-auto text-gray-400" } },
        { "id": "es-title", "component": "text", "props": { "text": "No automation recipes yet", "classes": "text-lg font-medium mt-4" } },
        { "id": "es-desc", "component": "text", "props": { "text": "Create your first recipe to automate cleaning tasks", "classes": "text-gray-500 mt-2" } },
        { "id": "es-btn", "component": "button", "props": { "label": "Create Recipe", "action": "create-recipe", "variant": "primary" } }
      ]
    }
  ]
}
```

**KAS handlers needed**:
- `list_recipes` → `Vec<AutomationRecipe>`
- `create_recipe` (POST) → `AutomationRecipe`
- `delete_recipe` (DELETE) → `()`
- `trigger_recipe` (POST) → `CleaningReport`

---

## PAGE 5: System (`/system`)

**Data source**: `get_system_stats`, `get_processes`, `get_cpu_temp`, `get_gpu_temp`

```json
{
  "id": "system-page",
  "title": "System Information",
  "route": "/system",
  "layout": "grid",
  "data_binding": "get_system_stats",
  "elements": [
    {
      "id": "kernel-card",
      "component": "card",
      "props": { "title": "Kernel", "classes": "p-6" },
      "children": [
        { "id": "kc-version", "component": "div", "props": { "classes": "flex justify-between text-sm" }, "children": [
          { "id": "kc-label", "component": "text", "props": { "text": "Current Kernel", "classes": "text-gray-600" } },
          { "id": "kc-value", "component": "text", "props": { "bind": "kernel_version", "classes": "font-medium" } }
        ]},
        { "id": "kc-btn", "component": "button", "props": { "label": "Clean Old Kernels", "action": "clean-kernels", "variant": "secondary", "size": "sm" } }
      ]
    },
    {
      "id": "services-card",
      "component": "card",
      "props": { "title": "Services", "classes": "p-6" },
      "children": [
        { "id": "svc-running", "component": "div", "props": { "classes": "flex justify-between text-sm" }, "children": [
          { "id": "svc-run-label", "component": "text", "props": { "text": "Running Services", "classes": "text-gray-600" } },
          { "id": "svc-run-value", "component": "text", "props": { "bind": "running_services", "classes": "font-medium text-green-600" } }
        ]},
        { "id": "svc-stopped", "component": "div", "props": { "classes": "flex justify-between text-sm" }, "children": [
          { "id": "svc-stop-label", "component": "text", "props": { "text": "Stopped Services", "classes": "text-gray-600" } },
          { "id": "svc-stop-value", "component": "text", "props": { "bind": "stopped_services", "classes": "font-medium" } }
        ]},
        { "id": "svc-btn", "component": "button", "props": { "label": "Manage Services", "action": "manage-services", "variant": "secondary", "size": "sm" } }
      ]
    },
    {
      "id": "processes-card",
      "component": "card",
      "props": { "title": "Processes", "classes": "p-6" },
      "children": [
        { "id": "proc-total", "component": "div", "props": { "classes": "flex justify-between text-sm" }, "children": [
          { "id": "proc-total-label", "component": "text", "props": { "text": "Total Processes", "classes": "text-gray-600" } },
          { "id": "proc-total-value", "component": "text", "props": { "bind": "total_processes", "classes": "font-medium" } }
        ]},
        { "id": "proc-running", "component": "div", "props": { "classes": "flex justify-between text-sm" }, "children": [
          { "id": "proc-run-label", "component": "text", "props": { "text": "Running", "classes": "text-gray-600" } },
          { "id": "proc-run-value", "component": "text", "props": { "bind": "running_processes", "classes": "font-medium text-green-600" } }
        ]},
        { "id": "proc-btn", "component": "button", "props": { "label": "View Processes", "action": "view-processes", "variant": "secondary", "size": "sm" } }
      ]
    },
    {
      "id": "power-card",
      "component": "card",
      "props": { "title": "Power", "classes": "p-6" },
      "children": [
        { "id": "pw-profile", "component": "div", "props": { "classes": "flex justify-between text-sm" }, "children": [
          { "id": "pw-profile-label", "component": "text", "props": { "text": "Power Profile", "classes": "text-gray-600" } },
          { "id": "pw-profile-value", "component": "text", "props": { "bind": "power_profile", "classes": "font-medium" } }
        ]},
        { "id": "pw-battery", "component": "div", "props": { "classes": "flex justify-between text-sm" }, "children": [
          { "id": "pw-bat-label", "component": "text", "props": { "text": "Battery", "classes": "text-gray-600" } },
          { "id": "pw-bat-value", "component": "text", "props": { "bind": "battery_percent", "format": "{value}%", "classes": "font-medium" } }
        ]},
        { "id": "pw-btn", "component": "button", "props": { "label": "Power Settings", "action": "power-settings", "variant": "secondary", "size": "sm" } }
      ]
    },
    {
      "id": "cpu-detail-card",
      "component": "card",
      "props": { "title": "CPU", "classes": "p-6 col-span-2" },
      "children": [
        { "id": "cpu-name", "component": "text", "props": { "bind": "cpu_name", "classes": "font-medium mb-4" } },
        { "id": "cpu-usage-bar", "component": "progress", "props": { "bind": "cpu_percent", "max": 100, "show_label": true } }
      ]
    },
    {
      "id": "memory-detail-card",
      "component": "card",
      "props": { "title": "Memory", "classes": "p-6 col-span-2" },
      "children": [
        { "id": "mem-usage-bar", "component": "progress", "props": { "bind": "memory_percent", "max": 100, "show_label": true } },
        { "id": "mem-detail", "component": "text", "props": { "bind": "memory_detail", "format": "{used} GB used of {total} GB", "classes": "text-sm text-gray-500 mt-2" } }
      ]
    }
  ]
}
```

**KAS handlers needed**:
- `get_system_stats` → `SystemStats`
- `get_processes` → `Vec<ProcessMemory>` (paginated)
- `get_cpu_temp`, `get_gpu_temp` → `f32`

---

## PAGE 6: Storage (`/storage`)

**Data source**: `scan_directory`, `find_duplicates`, `list_backups`, `scan_browser_caches`

```json
{
  "id": "storage-page",
  "title": "Storage Management",
  "route": "/storage",
  "layout": "stacked",
  "elements": [
    {
      "id": "disk-usage-card",
      "component": "card",
      "props": { "title": "Disk Usage", "classes": "p-6" },
      "children": [
        { "id": "du-bar", "component": "progress", "props": { "bind": "disk_percent", "max": 100 } },
        {
          "id": "du-labels",
          "component": "div",
          "props": { "classes": "flex justify-between mt-2 text-sm" },
          "children": [
            { "id": "du-used", "component": "text", "props": { "bind": "disk_used_gb", "format": "Used: {value} GB", "classes": "text-gray-600" } },
            { "id": "du-total", "component": "text", "props": { "bind": "disk_total_gb", "format": "Total: {value} GB", "classes": "text-gray-600" } }
          ]
        }
      ]
    },
    {
      "id": "largest-dirs-card",
      "component": "card",
      "props": { "title": "Largest Directories", "classes": "p-6" },
      "children": [
        {
          "id": "ld-list",
          "component": "div",
          "props": { "classes": "space-y-2 text-sm" },
          "children": [
            {
              "id": "ld-row-template",
              "component": "div",
              "props": { "bind": "dir", "classes": "flex justify-between" },
              "children": [
                { "id": "ld-path", "component": "text", "props": { "bind": "dir.path", "classes": "text-gray-900 dark:text-white" } },
                { "id": "ld-size", "component": "text", "props": { "bind": "dir.size_gb", "format": "{value} GB", "classes": "font-medium" } }
              ]
            }
          ]
        }
      ]
    },
    {
      "id": "duplicates-card",
      "component": "card",
      "props": { "title": "Duplicate Files", "classes": "p-6" },
      "children": [
        { "id": "dup-summary", "component": "text", "props": { "bind": "duplicates_summary", "classes": "text-gray-600 text-sm mb-4" } },
        {
          "id": "dup-btn",
          "component": "button",
          "props": { "label": "Review Duplicates", "action": "review-duplicates", "variant": "secondary", "size": "sm" }
        }
      ]
    },
    {
      "id": "browser-cache-card",
      "component": "card",
      "props": { "title": "Browser Caches", "classes": "p-6" },
      "children": [
        {
          "id": "bc-list",
          "component": "div",
          "props": { "classes": "space-y-2 text-sm" },
          "children": [
            {
              "id": "bc-row-template",
              "component": "div",
              "props": { "bind": "browser" },
              "children": [
                { "id": "bc-name", "component": "text", "props": { "bind": "browser.name", "classes": "text-gray-900 dark:text-white" } },
                { "id": "bc-size", "component": "text", "props": { "bind": "browser.size", "format": "{value}", "classes": "font-medium" } }
              ]
            }
          ]
        },
        { "id": "bc-btn", "component": "button", "props": { "label": "Clean Browser Caches", "action": "clean-browser-caches", "variant": "outline", "size": "sm" } }
      ]
    },
    {
      "id": "backups-card",
      "component": "card",
      "props": { "title": "Backups", "classes": "p-6" },
      "children": [
        {
          "id": "backup-list",
          "component": "div",
          "props": { "classes": "space-y-3" },
          "children": [
            {
              "id": "backup-row-template",
              "component": "div",
              "props": { "bind": "backup" },
              "children": [
                { "id": "backup-name", "component": "text", "props": { "bind": "backup.name", "classes": "font-medium" } },
                { "id": "backup-date", "component": "text", "props": { "bind": "backup.date", "classes": "text-sm text-gray-500" } },
                { "id": "backup-size", "component": "text", "props": { "bind": "backup.size", "format": "{value}", "classes": "text-sm" } }
              ]
            }
          ]
        },
        { "id": "backup-btn", "component": "button", "props": { "label": "Create Backup", "action": "create-backup", "variant": "primary", "size": "sm" } }
      ]
    }
  ]
}
```

**KAS handlers needed**:
- `scan_directory` → `DirectoryInfo`
- `find_duplicates` → `Vec<Value>`
- `list_backups` → `Vec<BackupInfo>`
- `scan_browser_caches` → `Vec<Value>`

---

## PAGE 7: Profiles (`/profiles`)

**Data source**: CRUD for `CleaningProfile` via KAS

```json
{
  "id": "profiles-page",
  "title": "Cleaning Profiles",
  "route": "/profiles",
  "layout": "stacked",
  "data_binding": "list_profiles",
  "elements": [
    {
      "id": "page-header",
      "component": "div",
      "props": { "classes": "flex justify-between items-center mb-6" },
      "children": [
        { "id": "ph-desc", "component": "text", "props": { "text": "Create and manage cleaning profiles", "classes": "text-gray-600" } },
        { "id": "btn-new", "component": "button", "props": { "label": "New Profile", "action": "create-profile", "variant": "primary" } }
      ]
    },
    {
      "id": "profiles-grid",
      "component": "div",
      "props": { "classes": "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4" },
      "children": [
        {
          "id": "profile-card-template",
          "component": "card",
          "props": { "bind": "profile.id", "classes": "p-6" },
          "children": [
            { "id": "pc-name", "component": "text", "props": { "bind": "profile.name", "classes": "text-xl font-semibold mb-2" } },
            { "id": "pc-desc", "component": "text", "props": { "bind": "profile.description", "classes": "text-gray-600 text-sm mb-3" } },
            { "id": "pc-actions-label", "component": "text", "props": { "text": "Cleans:", "classes": "text-xs text-gray-500 mb-1" } },
            { "id": "pc-categories", "component": "text", "props": { "bind": "profile.categories", "classes": "text-xs text-gray-500 mb-4" } },
            {
              "id": "pc-buttons",
              "component": "div",
              "props": { "classes": "flex gap-2" },
              "children": [
                { "id": "pc-btn-run", "component": "button", "props": { "label": "Run", "action": "run-profile", "bind": "profile.id", "variant": "primary", "size": "sm", "classes": "flex-1" } },
                { "id": "pc-btn-edit", "component": "button", "props": { "label": "Edit", "action": "edit-profile", "bind": "profile.id", "variant": "ghost", "size": "sm" } }
              ]
            }
          ]
        }
      ]
    }
  ]
}
```

**KAS handlers**: Standard CRUD via `define_crud_routes!(CleaningProfile)`

---

## PAGE 8: Settings (`/settings`)

```json
{
  "id": "settings-page",
  "title": "Settings",
  "route": "/settings",
  "layout": "stacked",
  "elements": [
    {
      "id": "appearance-card",
      "component": "card",
      "props": { "title": "Appearance", "classes": "p-6" },
      "children": [
        {
          "id": "dark-mode-row",
          "component": "div",
          "props": { "classes": "flex items-center justify-between" },
          "children": [
            { "id": "dm-label", "component": "text", "props": { "text": "Dark Mode", "classes": "text-gray-700 dark:text-gray-300" } },
            { "id": "dm-toggle", "component": "toggle", "props": { "bind": "settings.dark_mode", "action": "toggle-dark-mode" } }
          ]
        }
      ]
    },
    {
      "id": "notifications-card",
      "component": "card",
      "props": { "title": "Notifications", "classes": "p-6" },
      "children": [
        {
          "id": "notif-cleanup-row",
          "component": "div",
          "props": { "classes": "flex items-center justify-between" },
          "children": [
            { "id": "nc-label", "component": "text", "props": { "text": "Cleanup Complete", "classes": "text-gray-700 dark:text-gray-300" } },
            { "id": "nc-toggle", "component": "toggle", "props": { "bind": "settings.notify_cleanup", "action": "toggle-notify-cleanup" } }
          ]
        },
        {
          "id": "notif-alerts-row",
          "component": "div",
          "props": { "classes": "flex items-center justify-between" },
          "children": [
            { "id": "na-label", "component": "text", "props": { "text": "Automation Alerts", "classes": "text-gray-700 dark:text-gray-300" } },
            { "id": "na-toggle", "component": "toggle", "props": { "bind": "settings.notify_alerts", "action": "toggle-notify-alerts" } }
          ]
        }
      ]
    },
    {
      "id": "data-card",
      "component": "card",
      "props": { "title": "Data", "classes": "p-6" },
      "children": [
        { "id": "data-export", "component": "button", "props": { "label": "Export Settings", "action": "export-settings", "variant": "secondary", "classes": "w-full justify-center mb-3" } },
        { "id": "data-import", "component": "button", "props": { "label": "Import Settings", "action": "import-settings", "variant": "secondary", "classes": "w-full justify-center mb-3" } },
        { "id": "data-reset", "component": "button", "props": { "label": "Reset to Defaults", "action": "reset-settings", "variant": "outline", "classes": "w-full justify-center text-red-600" } }
      ]
    }
  ]
}
```

---

## PAGE 9: Reports (`/reports`)

**Data source**: CRUD for `CleaningReport`, `ExecutionHistory`

```json
{
  "id": "reports-page",
  "title": "Cleaning Reports",
  "route": "/reports",
  "layout": "stacked",
  "data_binding": "list_reports",
  "elements": [
    {
      "id": "recent-cleanups-card",
      "component": "card",
      "props": { "title": "Recent Cleanups", "classes": "p-6" },
      "children": [
        {
          "id": "cleanup-table",
          "component": "table",
          "props": {
            "columns": [
              { "key": "date", "label": "Date" },
              { "key": "items", "label": "Items" },
              { "key": "reclaimed", "label": "Space Reclaimed" },
              { "key": "duration", "label": "Duration" }
            ],
            "bind": "reports"
          }
        },
        {
          "id": "pagination",
          "component": "pagination",
          "props": { "bind": "pagination", "total_pages": 10, "current_page": 1 }
        }
      ]
    },
    {
      "id": "statistics-card",
      "component": "card",
      "props": { "title": "Statistics", "classes": "p-6" },
      "children": [
        {
          "id": "stats-grid",
          "component": "div",
          "props": { "classes": "grid grid-cols-3 gap-4 text-center" },
          "children": [
            {
              "id": "stat-total-reclaimed",
              "component": "div",
              "children": [
                { "id": "str-value", "component": "text", "props": { "bind": "total_reclaimed_gb", "format": "{value} GB", "classes": "text-3xl font-bold text-primary" } },
                { "id": "str-label", "component": "text", "props": { "text": "Total Reclaimed", "classes": "text-sm text-gray-600" } }
              ]
            },
            {
              "id": "stat-total-cleanups",
              "component": "div",
              "children": [
                { "id": "stc-value", "component": "text", "props": { "bind": "total_cleanups", "classes": "text-3xl font-bold text-primary" } },
                { "id": "stc-label", "component": "text", "props": { "text": "Total Cleanups", "classes": "text-sm text-gray-600" } }
              ]
            },
            {
              "id": "stat-daily-avg",
              "component": "div",
              "children": [
                { "id": "sda-value", "component": "text", "props": { "bind": "daily_average_mb", "format": "{value} MB", "classes": "text-3xl font-bold text-primary" } },
                { "id": "sda-label", "component": "text", "props": { "text": "Daily Average", "classes": "text-sm text-gray-600" } }
              ]
            }
          ]
        }
      ]
    }
  ]
}
```

**KAS handlers**: CRUD for `CleaningReport` and `ExecutionHistory` via KAS

---

## PAGE 10: Logs (`/logs`)

```json
{
  "id": "logs-page",
  "title": "Log Manager",
  "route": "/logs",
  "layout": "stacked",
  "data_binding": "get_log_info",
  "elements": [
    {
      "id": "log-stats-grid",
      "component": "div",
      "props": { "classes": "grid grid-cols-1 md:grid-cols-3 gap-4" },
      "children": [
        {
          "id": "journal-size-card",
          "component": "card",
          "props": { "classes": "p-4" },
          "children": [
            { "id": "js-label", "component": "text", "props": { "text": "Journal Size", "classes": "text-sm text-gray-600" } },
            { "id": "js-value", "component": "text", "props": { "bind": "journal_size_gb", "format": "{value} GB", "classes": "text-2xl font-bold" } }
          ]
        },
        {
          "id": "rotated-logs-card",
          "component": "card",
          "props": { "classes": "p-4" },
          "children": [
            { "id": "rl-label", "component": "text", "props": { "text": "Rotated Logs", "classes": "text-sm text-gray-600" } },
            { "id": "rl-value", "component": "text", "props": { "bind": "rotated_logs_mb", "format": "{value} MB", "classes": "text-2xl font-bold" } }
          ]
        },
        {
          "id": "largest-log-card",
          "component": "card",
          "props": { "classes": "p-4" },
          "children": [
            { "id": "ll-label", "component": "text", "props": { "text": "Largest Log", "classes": "text-sm text-gray-600" } },
            { "id": "ll-value", "component": "text", "props": { "bind": "largest_log", "format": "{name} ({size})", "classes": "text-2xl font-bold" } }
          ]
        }
      ]
    },
    {
      "id": "log-management-card",
      "component": "card",
      "props": { "title": "Log Management", "classes": "p-6" },
      "children": [
        { "id": "lm-vacuum", "component": "button", "props": { "label": "Vacuum Journal (keep last 7 days)", "action": "vacuum-journal", "variant": "secondary", "classes": "w-full justify-center mb-3" } },
        { "id": "lm-clean-rotated", "component": "button", "props": { "label": "Clean Rotated Logs", "action": "clean-rotated-logs", "variant": "secondary", "classes": "w-full justify-center mb-3" } },
        { "id": "lm-analyze", "component": "button", "props": { "label": "Analyze Logrotate Configs", "action": "analyze-logrotate", "variant": "outline", "classes": "w-full justify-center" } }
      ]
    },
    {
      "id": "logrotate-config-card",
      "component": "card",
      "props": { "title": "Logrotate Configuration", "classes": "p-6" },
      "children": [
        {
          "id": "lr-config",
          "component": "div",
          "props": { "classes": "space-y-2 text-sm font-mono bg-gray-100 dark:bg-gray-900 p-4 rounded" },
          "children": [
            { "id": "lr-path-1", "component": "text", "props": { "text": "/etc/logrotate.conf", "classes": "block" } },
            { "id": "lr-path-2", "component": "text", "props": { "text": "/etc/logrotate.d/", "classes": "block" } }
          ]
        }
      ]
    }
  ]
}
```

**KAS handlers needed**:
- `find_empty_directories` → `Vec<String>`
- Journal size estimation via `sysinfo`
- `vacuum-journal`, `clean-rotated-logs` actions

---

## Shared Components Needed (from `dioxus-shared`)

| Component | Semantic Type | Status in shared |
|-----------|--------------|-----------------|
| `card` | `Card` | ✅ Listed |
| `text` | `Text` | ✅ Listed |
| `button` | `Button` | ✅ Listed |
| `badge` | `Badge` | ✅ Listed |
| `progress` | `ProgressBar` | ❌ New semantic type needed |
| `toggle` | `Toggle` | ❌ New semantic type needed |
| `table` | `Table` | ❌ New semantic type needed |
| `pagination` | `Pagination` | ✅ Listed |
| `icon` | `Icon` | ✅ Listed |
| `div` | `Div` | ✅ Listed |
| `action-button` | `ActionButton` | ✅ Listed |
