# Next Steps: Building the Web-based LMS

The repository now includes infrastructure pieces (`orchestrator`, `sim_agent` and
`grade_push`) that are sufficient for launching per-learner containers and pushing
result files. To deliver a full in-browser learning experience, we need a simple
LMS front-end that coordinates courses and integrates with the orchestrator.

## 1. Actix-Web service for course management

Create a new crate (e.g. `lms_web`) in the workspace. This service will:

- expose REST endpoints for listing courses and starting lab sessions
- serve static HTML/JS assets for the learner UI
- forward `POST /launch` requests to the existing `orchestrator` crate

```bash
cargo new lms_web --bin
```
Add it to the `[workspace]` members in the root `Cargo.toml`.

The service can keep a `courses/` directory where each course provides a `course.toml`
file describing its title and available labs. On startup, `lms_web` reads this
folder to populate the course list.

## 2. Minimal HTML front-end

A small page served by `lms_web` can display tabs for each course. When a user
selects a course, the page shows its labs and a "Launch" button. Clicking the
button calls `/api/launch?course=<id>&learner=<id>` which returns the SSH string
and the private key.

For rapid development, plain HTML and a few lines of JavaScript (or a Yew/Leptos
front-end if you prefer Rust to WebAssembly) are enough. Keeping all assets in a
`static/` folder simplifies deployment.

## 3. Modular course definitions

Courses live under `courses/<course_id>` and include:

- `course.toml` – metadata (title, description)
- `README.md` – rendered in the UI
- optional example starter code

The LMS simply iterates over the directories in `courses/` to build the tabs.
Adding a new course is just a matter of creating a new subdirectory with the
expected files. This keeps the system extensible without modifying Rust code.

## 4. Authentication and LTI integration

While a basic version can rely on a simple login page, full LMS integration will
use LTI 1.3 for single sign‑on. The `lms_web` crate can verify LTI launches and
then proxy the learner ID to the orchestrator.

## 5. Putting it all together

1. Implement the `lms_web` service and add static pages for course selection.
2. Configure the orchestrator URL via environment variables so the front-end can
   request new lab sessions.
3. Test by adding a single course for the existing drone simulator and verifying
   the grade push via `sim_agent`.
4. Expand with additional courses over time by adding new directories under
   `courses/`.

These steps provide a thin but functional LMS that runs entirely in Rust and
builds upon the components already present in this repository.
