#!/usr/bin/env bash
# Instrumentation only, not a production fix or natural-race trigger.
# Native threads required: worker 0 waits for worker 1's legitimate final claim.
# early: final flag store precedes reduction; late: reduction precedes that store.
# No arithmetic, claiming, completion accounting, or existing barrier is changed.
# Use ONE gated step per process. Do not run unrelated workloads with this patch.
set -euo pipefail
usage() {
    echo "Usage: $0 [--check|--dry-run] /path/to/local/rapier-checkout" >&2
    exit 2
}
check=false
case "${1:-}" in
    --check|--dry-run) check=true; shift ;;
    -h|--help) usage ;;
esac
[[ $# == 1 ]] || usage
cd -- "$1"
[[ $(git rev-parse HEAD) == 3406750a38286a0d3c7d0f4be2f9d53153e81d90 ]] || {
    echo 'Refusing: requires exact Rapier v0.36.1 commit 3406750a38286a0d3c7d0f4be2f9d53153e81d90.' >&2
    exit 1
}
cd -- "$(git rev-parse --show-toplevel)"
worker=src/dynamics/solver/staged_island_solver/worker.rs
[[ -f "$worker" && ! -L "$worker" ]] || {
    echo 'Refusing: worker.rs must be a regular file.' >&2
    exit 1
}
# Compare the whole file, so unrelated edits cannot pass a reverse-patch check.
actual=$(git hash-object -- "$worker")
case "$actual" in
    f38c2e5428e09b6792612267d061a5ccd65a26b4) ;;
    bb59c0f83129193356311de157d279132b059081)
        echo 'Scheduling gates already applied; no changes.'
        exit 0 ;;
    *) echo 'Refusing: worker.rs contains conflicting edits.' >&2; exit 1 ;;
esac
patch_text() {
    cat <<'RAPIER_SCHEDULE_PATCH'
--- a/src/dynamics/solver/staged_island_solver/worker.rs
+++ b/src/dynamics/solver/staged_island_solver/worker.rs
@@ -3,6 +3,27 @@
 //! writeback stages.
 
 use core::sync::atomic::Ordering;
+
+// AUDIT ONLY: force two permitted schedules without changing simulation inputs,
+// constraints, flags, or arithmetic. One gated step per process, two native workers.
+mod audit_schedule {
+    use core::sync::atomic::{AtomicBool, Ordering};
+    pub(super) static HELD: AtomicBool = AtomicBool::new(false);
+    pub(super) static RELEASE: AtomicBool = AtomicBool::new(false);
+    pub(super) static WRITTEN: AtomicBool = AtomicBool::new(false);
+    pub(super) fn mode() -> u8 {
+        match std::env::var("RAPIER_AUDIT_SCHEDULE").as_deref() {
+            Ok("early") => 1,
+            Ok("late") => 2,
+            _ => 0,
+        }
+    }
+    pub(super) fn wait(flag: &AtomicBool) {
+        while !flag.load(Ordering::SeqCst) {
+            std::thread::yield_now();
+        }
+    }
+}
 
 #[cfg(feature = "dim3")]
 use crate::dynamics::rigid_body::gyroscopic_corrected_angvel;
@@ -890,6 +911,21 @@
         let num_constraints = soft.num_element_constraints();
         let all_units = 0..num_constraints + soft.awake.len();
         let stage_work = all_units.len() + 1;
+        let audit_mode = audit_schedule::mode();
+        if audit_mode != 0 {
+            assert_eq!(
+                sync.num_workers, 2,
+                "audit requires exactly two solver workers"
+            );
+            if worker_id == 0 {
+                // Model worker 0 being descheduled until worker 1 has claimed the
+                // last element batch. In early mode also let its store finish.
+                audit_schedule::wait(&audit_schedule::HELD);
+                if audit_mode == 1 {
+                    audit_schedule::wait(&audit_schedule::WRITTEN);
+                }
+            }
+        }
         let mut done = 0;
         while let Some(claimed) = sync.claim(
             stage,
@@ -901,10 +937,23 @@
             let constraints = claimed.start.min(num_constraints)..claimed.end.min(num_constraints);
             let awake = claimed.start.max(num_constraints) - num_constraints
                 ..claimed.end.max(num_constraints) - num_constraints;
+            let audit_target = audit_mode != 0 && constraints.contains(&(num_constraints - 1));
+            if audit_target {
+                assert_eq!(worker_id, 1, "worker 1 must own the gated final element");
+                audit_schedule::HELD.store(true, Ordering::SeqCst);
+                if audit_mode == 2 {
+                    // Model preemption after a legitimate claim but before any
+                    // execution of this final constraint batch.
+                    audit_schedule::wait(&audit_schedule::RELEASE);
+                }
+            }
             // SAFETY: each constraint is claimed by exactly one worker and writes its own element; each
             // awake body likewise writes its own particles.
             unsafe {
                 soft.writeback_constraints(constraints, ctx.base_params.dt);
+                if audit_target {
+                    audit_schedule::WRITTEN.store(true, Ordering::SeqCst);
+                }
                 let vs = &*ctx.velocity_solver;
                 for ai in awake {
                     soft.writeback_particles(ai, &vs.solver_bodies, ctx.base_params.dt);
@@ -915,6 +964,11 @@
         sync.complete(stage, done, stage_work);
         if worker_id == 0 {
             soft.writeback_bodies();
+            if audit_mode != 0 {
+                // Release before the existing completion barrier: no production
+                // synchronization, flags, or control flow are otherwise changed.
+                audit_schedule::RELEASE.store(true, Ordering::SeqCst);
+            }
             sync.complete(stage, 1, stage_work);
         }
         stage = sync.sync(stage, stage_work);
RAPIER_SCHEDULE_PATCH
}
patch_text | git apply --check -
if "$check"; then
    echo 'Scheduling patch applies cleanly; no changes (--check).'
else
    patch_text | git apply -
    echo 'Applied scheduling gates to worker.rs only.'
fi
# Deliberately do not edit the reproducer suite manifest. Add this manually there:
# [patch.crates-io]
# rapier2d = { path = "/absolute/path/to/rapier/crates/rapier2d" }
# Then, from that suite (fresh process for each mode):
# RAPIER_AUDIT_SCHEDULE=early cargo run --release --features scheduling-repro --example missed_tear_2d
# RAPIER_AUDIT_SCHEDULE=late cargo run --release --features scheduling-repro --example missed_tear_2d
# Expected stdout: 00000002 (early), 00000001 (late).
# Fix direction: review stage barrier placement AND completion accounting before
# per-body reduction. This script demonstrates the defect; it does not fix it.
