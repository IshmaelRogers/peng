# Next Steps: Building the Web-based LMS

The repository now ships with an `lms_web` service alongside `orchestrator`, `sim_agent` and `grade_push`. `lms_web` exposes REST endpoints for listing courses and launching lab sessions and serves a small HTML page from `static/`. Courses are discovered from the `courses/` directory using simple `course.toml` files.

Launching a session now performs an HTTP call to the `orchestrator` service and returns the SSH connection details directly in the browser.

## Remaining tasks

* **Authentication and LTI** – integrate LTI 1.3 or another auth layer so learners sign in via the LMS.
* **Persistent course data** – render `README.md` and any starter files in the UI. Allow multiple labs per course.
* **UI polish** – replace the basic JavaScript page with a more complete front end.
* **Deployment** – containerize all services and add scripts for running the stack locally or on Kubernetes.

These improvements will turn the proof-of-concept into a production-ready learning platform built entirely in Rust.
