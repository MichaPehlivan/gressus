# Gressus - a planner/agenda webapp in Rust

## What should we be able to do with this app?

Events
- Add/remove an event at a certain date and (optionally) time
- Have a personal account wherein the agenda points are saved
- Get an overview of all events in day view or month view

Tasks
- Add/remove a task at a certain date and (optionally) time.
- View all unfinished tasks.

Categories
- Add/remove custom categories an event or task should fall in.
- View tasks per category and toggle categories

General
- Receive (e-mail) notifications for tasks and events.
- Allow repeating tasks or events.

## With which technology will we implement this?

Frontend
- Leptos

Backend
- Rust
- Actix

Database
- SurrealDB