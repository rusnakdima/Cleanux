//! Cleanux parity test suite (mock-data driven).
//!
//! Implements `docs/parity/test-plan.md` against the old Tauri `master`
//! semantics documented in `docs/parity/parity-report.md`.
//!
//! Rules (from the plan):
//! - All tests run against fake filesystems under tempfile roots — no real
//!   system scanning, no root, no docker.
//! - MISSING functionality is pinned with `#[ignore = "gap: ..."]` tests that
//!   compile against the existing surface and document the master expectation.
//! - Small divergence fixes to master semantics were applied where noted.

use cleanux::application::kas::cleaner_handlers::clean_junk_category;
use cleanux::application::kas::crud_handlers::KasResponse;
use cleanux::application::kas::storage_handlers::{
    create_backup, find_empty_directories, get_directory_size, list_backups, scan_directory,
};
use cleanux::application::routine_service::RoutineService;
use cleanux::domain::entities::automation_recipe::{Action, AutomationRecipe, Trigger};
use cleanux::domain::entities::cleaning_profile::CleaningProfile;
use cleanux::domain::entities::execution_history::{ExecutionHistory, ExecutionStatus};
use cleanux::infrastructure::json_storage::JsonStorage;
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;
use std::time::SystemTime;
use tempfile::TempDir;

// ============================================================================
// Harness — FakePaths + fs_scan fixture tree (plan §1.1)
// ============================================================================

/// Injected roots per plan §3: every module under test takes its paths via
/// constructor/argument — no `std::env::set_var` races.
struct FakePaths {
    _home: TempDir,
    _config: TempDir,
}

impl FakePaths {
    fn new() -> Self {
        Self {
            _home: TempDir::new().expect("temp home"),
            _config: TempDir::new().expect("temp config"),
        }
    }

    fn home(&self) -> std::path::PathBuf {
        self._home.path().to_path_buf()
    }

    /// Master default backup dir layout: `<config>/cleanux/backups` (T4.6).
    fn backup_dir(&self) -> std::path::PathBuf {
        self._config.path().join("cleanux").join("backups")
    }
}

/// Byte-exact file writer for the fixture tree.
fn put_file(path: &std::path::Path, len: usize) {
    std::fs::create_dir_all(path.parent().unwrap()).expect("create parent");
    std::fs::write(path, vec![0u8; len]).expect("write file");
}

/// Build the plan §1.1 `fixtures/fs_scan/` tree under `root` and return the
/// total byte size of all files it contains.
fn build_fs_scan_tree(root: &std::path::Path) -> u64 {
    let p = |rel: &str| root.join(rel);

    // .cache/mozilla: a(10 B) b(2048 B) c(4096 B)
    put_file(&p(".cache/mozilla/a"), 10);
    put_file(&p(".cache/mozilla/b"), 2048);
    put_file(&p(".cache/mozilla/c"), 4096);
    // .cache/google-chrome: d(756_000_000 B) e(1 B)
    put_file(&p(".cache/google-chrome/d"), 756_000_000);
    put_file(&p(".cache/google-chrome/e"), 1);
    // .cache/thumbnails: f(120_000_000 B)
    put_file(&p(".cache/thumbnails/f"), 120_000_000);
    // .cache/flatpak: empty dir
    std::fs::create_dir_all(p(".cache/flatpak")).expect("flatpak dir");
    // Trash
    put_file(&p(".local/share/Trash/files/g.bin"), 256_000_000);
    put_file(&p(".local/share/Trash/files/h.txt"), 12);
    // var-log stand-in
    put_file(&p("var-log/syslog"), 128_000_000);
    put_file(&p("var-log/kern.log"), 64_000_000);
    put_file(&p("var-log/rotated/syslog.1.gz"), 9_000_000);
    // Downloads with the exact large-file boundary pair
    put_file(&p("Downloads/iso/big.iso"), 104_857_601); // > 100 MiB threshold
    put_file(&p("Downloads/doc/small.pdf"), 104_857_599); // threshold - 1

    10 + 2048 + 4096
        + 756_000_000
        + 1
        + 120_000_000
        + 256_000_000
        + 12
        + 128_000_000
        + 64_000_000
        + 9_000_000
        + 104_857_601
        + 104_857_599
}

fn storage_in(dir: &std::path::Path) -> Arc<JsonStorage> {
    std::fs::create_dir_all(dir).expect("storage dir");
    Arc::new(JsonStorage::new(dir.to_path_buf()))
}

/// Plan §1.3 recipe fixture as literal JSON — pins the wire shape.
const DEEP_CLEAN_JSON: &str = r#"{
  "id": "r-1", "name": "Deep Clean", "enabled": true,
  "trigger": { "type": "manual", "schedule": null },
  "conditions": [],
  "actions": [
    { "action_type": "clean_category", "params": { "category": "cache" } },
    { "action_type": "wait", "params": { "seconds": 0 } }
  ],
  "created_at": "2026-01-01T00:00:00Z", "last_run": null
}"#;

/// Plan §1.2 profile fixture as literal JSON — mirrors master doc shape.
const WEEKLY_PROFILE_JSON: &str = r#"{
  "id": "p-1", "name": "Weekly", "description": "demo",
  "created_at": "2026-01-01T00:00:00Z",
  "paths": [], "exclude_patterns": ["*.iso"],
  "clean_cache": true, "clean_trash": true, "clean_logs": false,
  "min_large_file_size": 104857600
}"#;

fn deep_clean_recipe() -> AutomationRecipe {
    serde_json::from_str(DEEP_CLEAN_JSON).expect("recipe fixture parses")
}

// ============================================================================
// T3 Profiles
// ============================================================================

/// T3.1 — default profile: min_large_file_size == 104857600, all three flags true.
#[test]
fn t3_1_default_profile_matches_master_defaults() {
    let profile = CleaningProfile::default();
    assert_eq!(profile.min_large_file_size, 104_857_600);
    assert!(profile.clean_cache && profile.clean_trash && profile.clean_logs);
    assert!(profile.id.is_none());
    assert!(profile.paths.is_empty());
    assert!(profile.exclude_patterns.is_empty());
}

/// T3.1/T3.4 — weekly.json fixture deserializes into the entity with the
/// master doc shape; `exclude_patterns` ("*.iso") round-trips through serde.
#[test]
fn t3_weekly_profile_fixture_round_trips() {
    let profile: CleaningProfile =
        serde_json::from_str(WEEKLY_PROFILE_JSON).expect("weekly.json parses");
    assert_eq!(profile.id.as_deref(), Some("p-1"));
    assert_eq!(profile.name, "Weekly");
    assert_eq!(profile.min_large_file_size, 104_857_600);
    assert!(profile.clean_cache);
    assert!(profile.clean_trash);
    assert!(!profile.clean_logs, "fixture sets clean_logs=false");
    assert_eq!(profile.exclude_patterns, vec!["*.iso".to_string()]);

    let back: CleaningProfile =
        serde_json::from_str(&serde_json::to_string(&profile).unwrap()).unwrap();
    assert_eq!(
        back.exclude_patterns, profile.exclude_patterns,
        "exclude_patterns must survive a save/load cycle"
    );
}

/// T3.5 — CRUD round-trip through the storage-backed service on JsonStorage:
/// save → list → update → delete; delete of unknown id errors.
#[test]
fn t3_5_crud_round_trip_on_json_storage() {
    let paths = FakePaths::new();
    let storage = storage_in(&paths.home());
    let service = RoutineService::new(storage.clone());

    // save (new)
    let mut recipe = deep_clean_recipe();
    recipe.id = None;
    let saved = service.save_routine(recipe.clone()).expect("save new");
    let id = saved.id.clone().expect("id assigned on create");
    assert_ne!(id, "r-1", "new recipes get a generated id");

    // list
    let all = service.get_routines().expect("list");
    assert_eq!(all.len(), 1);
    assert_eq!(all[0].name, "Deep Clean");

    // update (existing id)
    let mut updated = saved.clone();
    updated.name = "Deep Clean v2".to_string();
    let updated = service.save_routine(updated).expect("update");
    assert_eq!(service.get_routines().unwrap().len(), 1, "update not insert");
    assert_eq!(updated.name, "Deep Clean v2");

    // persistence is real: a fresh service over the same JsonStorage sees it
    let reloaded = RoutineService::new(storage.clone());
    assert_eq!(reloaded.get_routines().unwrap()[0].name, "Deep Clean v2");

    // delete unknown id errors
    let err = reloaded.delete_routine("no-such-id").unwrap_err();
    assert_eq!(err, "Routine not found: no-such-id");

    // delete known id succeeds and empties the collection
    reloaded.delete_routine(&id).expect("delete");
    assert!(reloaded.get_routines().unwrap().is_empty());
}

// ============================================================================
// T6 Automation & history
// ============================================================================

/// T6.1 — execute existing routine: last_run set to ~now, ExecutionHistory
/// appended with recipe_id/name, status Completed.
#[test]
fn t6_1_execute_existing_routine_appends_history() {
    let paths = FakePaths::new();
    let storage = storage_in(&paths.home());
    let service = RoutineService::new(storage.clone());

    let mut recipe = deep_clean_recipe();
    recipe.id = None;
    let saved = service.save_routine(recipe).expect("seed");
    let before = SystemTime::now();

    let history = service.execute_routine(saved.id.as_deref().unwrap()).expect("execute");
    let after = SystemTime::now();

    assert_eq!(history.recipe_id, saved.id, "history carries recipe_id");
    assert_eq!(history.recipe_name, "Deep Clean", "history carries recipe name");
    assert!(matches!(history.status, ExecutionStatus::Completed));
    let started = history.started_at;
    assert!(started >= chrono_start(before) && started <= chrono_end(after));

    // last_run persisted on the recipe itself, ~now
    let stored = service
        .get_routine_by_id(saved.id.as_deref().unwrap())
        .expect("lookup")
        .expect("still present");
    let last_run = stored.last_run.expect("last_run set");
    assert!(last_run >= chrono_start(before) && last_run <= chrono_end(after));

    // history row actually appended to storage (master add_to_history)
    let rows = service.get_history().expect("history");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].recipe_name, "Deep Clean");
}

/// T6.2 — execute missing routine: error "Routine not found: <id>" and no
/// history row.
#[test]
fn t6_2_execute_missing_routine_errors_without_history() {
    let paths = FakePaths::new();
    let storage = storage_in(&paths.home());
    let service = RoutineService::new(storage.clone());

    let err = service.execute_routine("ghost").unwrap_err();
    assert_eq!(err, "Routine not found: ghost");
    assert!(service.get_history().unwrap().is_empty(), "no history row");
}

/// T6.3 — history cap: seed 105 entries → after add, len == 100 and the
/// oldest entry is truncated (master add_to_history semantics).
#[test]
fn t6_3_history_capped_at_100_with_oldest_truncated() {
    let paths = FakePaths::new();
    let storage = storage_in(&paths.home());
    let service = RoutineService::new(storage.clone());

    let base = chrono::DateTime::parse_from_rfc3339("2026-01-01T00:00:00Z")
        .unwrap()
        .with_timezone(&chrono::Utc);
    let seed: Vec<ExecutionHistory> = (0..105)
        .map(|i| ExecutionHistory {
            id: Some(format!("h-{:03}", i)),
            recipe_id: None,
            recipe_name: format!("run {}", i),
            status: ExecutionStatus::Completed,
            // h-000 oldest … h-104 newest; storage order is newest-first
            started_at: base + chrono::Duration::seconds(i),
            completed_at: None,
            items_affected: 0,
            space_reclaimed: 0,
            error_message: None,
        })
        .rev()
        .collect();
    storage.save("execution_history", &seed).expect("seed history");

    let entry = ExecutionHistory {
        id: Some("h-new".to_string()),
        recipe_id: None,
        recipe_name: "fresh".to_string(),
        status: ExecutionStatus::Completed,
        started_at: base + chrono::Duration::seconds(999),
        completed_at: None,
        items_affected: 0,
        space_reclaimed: 0,
        error_message: None,
    };
    service.add_to_history(entry).expect("append");

    let rows = service.get_history().unwrap();
    assert_eq!(rows.len(), 100, "cap enforced at 100");
    assert_eq!(rows[0].id.as_deref(), Some("h-new"), "newest first");
    // Oldest five seeded entries truncated away; h-099 survives.
    assert!(
        rows.iter().all(|r| r.id.as_deref() != Some("h-000")),
        "oldest entry truncated"
    );
    assert!(
        rows.iter().any(|r| r.id.as_deref() == Some("h-099")),
        "newest pre-cap entries retained"
    );
}

/// T6.6 — persistence survives reload: mutate recipes → drop service →
/// rebuild RoutineService from same JsonStorage → mutations present.
#[test]
fn t6_6_persistence_survives_reload() {
    let paths = FakePaths::new();
    let storage = storage_in(&paths.home());

    {
        let service = RoutineService::new(storage.clone());
        let mut recipe = deep_clean_recipe();
        recipe.id = None;
        let saved = service.save_routine(recipe).expect("seed");
        service.delete_routine(saved.id.as_deref().unwrap()).expect("delete");
        let second = deep_clean_recipe(); // keeps explicit id "r-1"
        service.save_routine(second).expect("add other");
    } // service dropped here

    let rebuilt = RoutineService::new(storage.clone());
    let routines = rebuilt.get_routines().expect("list after reload");
    assert_eq!(routines.len(), 1);
    assert_eq!(routines[0].id.as_deref(), Some("r-1"));
    assert_eq!(routines[0].name, "Deep Clean");
    // The deletion was persisted too: nothing else came back.
    assert!(rebuilt.get_routine_by_id("").unwrap().is_none());
}

/// Plan §1.3 — the deep-clean.json recipe fixture deserializes with the exact
/// master wire shape (`trigger.type` rename, action_type/params pairs).
#[test]
fn recipe_fixture_wire_shape_matches_master_doc() {
    let recipe = deep_clean_recipe();
    assert_eq!(recipe.id.as_deref(), Some("r-1"));
    assert_eq!(recipe.trigger.trigger_type, "manual");
    assert!(recipe.trigger.schedule.is_none());
    assert!(recipe.conditions.is_empty());
    assert_eq!(recipe.actions.len(), 2);
    assert_eq!(recipe.actions[0].action_type, "clean_category");
    assert_eq!(recipe.actions[0].params, json!({"category": "cache"}));
    assert_eq!(recipe.actions[1].action_type, "wait");
    assert_eq!(recipe.actions[1].params, json!({"seconds": 0}));
    assert!(recipe.enabled);
    assert!(recipe.last_run.is_none());
}

// ============================================================================
// T8 Regression guards
// ============================================================================

/// T8 — KasResponse envelope shape is stable: fields exactly
/// `status`/`message`/`data`; success/error/not_found statuses.
#[test]
fn t8_kas_response_envelope_shape_stable() {
    let ok: KasResponse<u64> = KasResponse::success(42, "all good");
    assert_eq!(ok.status, "success");
    assert_eq!(ok.message, "all good");
    assert_eq!(ok.data, Some(42));

    let serialized = serde_json::to_value(&ok).unwrap();
    let mut keys: Vec<&str> = serialized
        .as_object()
        .expect("object")
        .keys()
        .map(String::as_str)
        .collect();
    keys.sort_unstable();
    assert_eq!(keys, vec!["data", "message", "status"], "envelope field set changed");

    let err: KasResponse<()> = KasResponse::error("bad");
    assert_eq!(err.status, "error");
    assert!(err.data.is_none());

    let nf: KasResponse<String> = KasResponse::not_found("Profile");
    assert_eq!(nf.status, "not_found");
    assert_eq!(nf.message, "Profile not found");

    // Deserialization round-trip keeps the envelope usable across the bridge.
    let back: KasResponse<u64> = serde_json::from_value(serialized).unwrap();
    assert_eq!(back.data, Some(42));
}

/// T8 — JsonStorage.save writes the whole collection atomically: a failed
/// save never destroys the previously stored content.
#[test]
fn t8_json_storage_failed_save_preserves_previous_content() {
    let dir = TempDir::new().unwrap();
    let storage = JsonStorage::new(dir.path().to_path_buf());

    storage.save("table", &json!({"version": 1})).expect("first save");
    let original =
        std::fs::read_to_string(dir.path().join("table")).expect("stored file exists");

    struct AlwaysFails;
    impl Serialize for AlwaysFails {
        fn serialize<S: serde::Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
            Err(serde::ser::Error::custom("boom"))
        }
    }
    assert!(storage.save("table", &AlwaysFails).is_err(), "serialization failure surfaces");

    let after = std::fs::read_to_string(dir.path().join("table")).expect("file intact");
    assert_eq!(original, after, "previous collection preserved on failed save");
    // No temp litter left behind.
    let leftovers: Vec<_> = std::fs::read_dir(dir.path())
        .unwrap()
        .filter_map(Result::ok)
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.ends_with(".tmp"))
        .collect();
    assert!(leftovers.is_empty(), "temp file renamed away, got {:?}", leftovers);
}

/// T8 — corrupt-file simulation yields an error from load (never silently
/// partial data), while a subsequent save replaces the corrupt file cleanly.
#[test]
fn t8_json_storage_corrupt_file_simulation() {
    let dir = TempDir::new().unwrap();
    let storage = JsonStorage::new(dir.path().to_path_buf());

    std::fs::write(dir.path().join("table"), "{not valid json").unwrap();
    let load_err = storage.load::<serde_json::Value>("table").unwrap_err();
    assert!(load_err.contains("Failed to parse"), "got: {}", load_err);

    storage.save("table", &json!({"recovered": true})).expect("save over corrupt file");
    let recovered = storage.load::<serde_json::Value>("table").unwrap();
    assert_eq!(recovered, json!({"recovered": true}));

    // Missing file → typed default, no error (RoutineService relies on this).
    let missing: Vec<serde_json::Value> = storage.load("absent").expect("default for absent");
    assert!(missing.is_empty());
}

// ============================================================================
// Directory scan / size / empty dirs (walkdir logic — PARITY target)
// Exercised over the plan §1.1 fs_scan tree.
// ============================================================================

/// scan_directory totals every file's bytes and counts files/subdirs over the
/// fs_scan tree (13 files, exact summed size).
#[tokio::test]
async fn walkdir_scan_directory_totals_fs_scan_tree() {
    let home = TempDir::new().unwrap();
    let expected_total = build_fs_scan_tree(home.path());

    let response = scan_directory(home.path().to_str().unwrap()).await.expect("scan ok");
    assert_eq!(response.status, "success");
    let info = response.data.expect("payload");
    assert_eq!(info.size, expected_total);
    assert_eq!(info.files, 13, "13 files planted by the fixture tree");
    assert!(info.subdirs >= 10, "counts directories (incl. root), got {}", info.subdirs);
}

/// get_directory_size over `.cache`: 10+2048+4096+756000000+1+120000000.
#[tokio::test]
async fn walkdir_get_directory_size_sums_cache_exactly() {
    let home = TempDir::new().unwrap();
    build_fs_scan_tree(home.path());

    let cache = home.path().join(".cache");
    let response =
        get_directory_size(cache.to_str().unwrap()).await.expect("size ok");
    assert_eq!(response.data, Some(876_006_155));

    // Trash summary bytes (plan T1.3): g.bin + h.txt == 256000012.
    let trash = home.path().join(".local/share/Trash/files");
    let response = get_directory_size(trash.to_str().unwrap()).await.unwrap();
    assert_eq!(response.data, Some(256_000_012));
}

/// find_empty_directories reports exactly the one empty dir (`flatpak`) from
/// the fixture tree, as full paths.
#[tokio::test]
async fn walkdir_find_empty_directories_finds_flatpak_only() {
    let home = TempDir::new().unwrap();
    build_fs_scan_tree(home.path());

    let response =
        find_empty_directories(home.path().to_str().unwrap()).await.expect("walk ok");
    let empty = response.data.expect("payload");
    assert_eq!(
        empty,
        vec![home.path().join(".cache/flatpak").to_string_lossy().into_owned()],
        "exactly the empty fixture dir"
    );
}

// ============================================================================
// Gap pins — MISSING functionality, compiled against the existing surface.
// Each test documents the master expectation and fails if ever un-ignored,
// which is exactly what flags the gap.
// ============================================================================

/// T2.4 — master rejects unknown categories with `"Invalid category: bogus"`.
/// Current handler accepts any string and returns a fixed fixture number.
#[tokio::test]
#[ignore = "gap: category cleaning engine missing; clean_junk_category returns a fixed fixture instead of validating categories"]
async fn t2_4_unknown_category_errors_like_master() {
    let response = clean_junk_category("bogus").await.expect("call");
    assert_eq!(response.status, "error");
    assert_eq!(response.message, "Invalid category: bogus");
}

/// T4.2/T4.3 — master lists real archives newest-first with payload fields
/// exactly {name,path,size,modified(modified RFC3339)}; empty backup dir →
/// data [] with message "No backups found". Current handler returns two
/// hardcoded fixtures with {id,name,date,size}.
#[tokio::test]
#[ignore = "gap: backup engine (tar.gz create/list/delete/restore) missing; handlers return hardcoded fixtures"]
async fn t4_list_backups_matches_master_payload_and_ordering() {
    let response = list_backups().await.expect("call");
    let backups = response.data.unwrap();
    assert!(backups.is_empty(), "no archives exist yet");
    assert_eq!(response.message, "No backups found");
}

/// T4.1/T4.4/T4.5/T4.6 — create_backup must write a valid tar.gz of the given
/// paths into `<config>/cleanux/backups` and restore byte-identical trees.
/// Current handler fabricates a BackupInfo without touching the filesystem.
#[tokio::test]
#[ignore = "gap: backup engine missing; create_backup writes no archive and BackupInfo lacks path/modified fields"]
async fn t4_create_backup_writes_real_archive_into_default_dir() {
    let paths = FakePaths::new();
    let response = create_backup("weekly").await.expect("call");
    let info = response.data.unwrap();
    let archive = paths.backup_dir().join("weekly.tar.gz");
    assert!(archive.exists(), "archive written to <config>/cleanux/backups");
    assert!(info.size > 0, "real archived size reported");
}

// ============================================================================
// Gaps WITHOUT any existing surface — no compilable seam exists, so these
// plan cases cannot be authored even as ignored tests. Recorded here as
// documentation (parity-report.md classifies each as MISSING):
//
// T1.1–T1.6  Junk scanners (browser/thumbnail/trash/log/large-file) behind an
//            injectable PathSource — no scanner code exists in this tree.
//            Master constants to pin once implemented:
//            - browser caches end their descriptions with "browser cache"
//            - trash summary counts entries incl. dirs via metadata
//            - log scans use max_depth=2 and take-cap 500 files
//            - large files use strict `metadata.len() > threshold` (so a file
//              at exactly 100 MiB − 1 B is excluded, cf. small.pdf fixture)
//            - non-cache-named files excluded when is_cache_file applies
// T2.1–T2.3  Category cleaning effects (cache dir recreated after
//            remove_dir_all; "Trash clear partial:" abort-on-first-error;
//            unprivileged log cleaning message-only outcome).
// T3.2–T3.4  Profile apply flow ("Profile not found"; apply honors
//            clean_cache/clean_trash/clean_logs flags; exclude_patterns
//            behavior — master stores but does not honor them during apply).
// T5.1–T5.4  Dashboard summary functions (take(2000) cache cap, take(500) log
//            cap depth>2, Downloads/Documents/Videos/Pictures/Desktop scan
//            set, systemctl output parser: header skipped, rows with <4 cols
//            dropped, is_running == (parts[3]=="running")).
// T6.4       Quick-action step ordering (deep-clean cache→trash→logs;
//            morning-refresh logs→wait(2s)→cache) — ActionStep engine absent.
// T6.5       ExecuteCommand allowlist prefix match on
//            [sync, shutdown, reboot, systemctl, service] ("rm -rf /" →
//            "not in the allowed list"; "syncbot" currently passes per master
//            prefix semantics).
// T7.1–T7.5  Security allowlist module (ALLOWED=/home,/tmp,/var/cache,
//            /var/tmp,/snap,/srv,/opt; BLOCKED=/proc,/sys,/dev; exception
//            /proc/self; denies /tmp/runtime-*, /tmp/.X11-unix; trash
//            special-case) — safety-critical prerequisite for any cleaner.
// ============================================================================

// -- chrono helpers -----------------------------------------------------------

fn chrono_start(t: SystemTime) -> chrono::DateTime<chrono::Utc> {
    let d: chrono::DateTime<chrono::Utc> = t.into();
    d - chrono::Duration::seconds(1)
}

fn chrono_end(t: SystemTime) -> chrono::DateTime<chrono::Utc> {
    let d: chrono::DateTime<chrono::Utc> = t.into();
    d + chrono::Duration::seconds(1)
}
