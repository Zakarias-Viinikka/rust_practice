# Gotchas

Cost-time issues that aren't obvious from the code.

## Port in URL doesn't survive navigation

If the socket reads the port from `?server_port=N`, the router strips it on the first in-app navigation. The socket connects to the default (3000) silently. Fix: py writes the port through an exposed method that stores it in sessionStorage, and the app reads from there. The app boots to `/custom_port_setting_page` so py can set the port before any registration or setup code navigates away.

## Effects must track every signal they read

An Effect using `.get_untracked()` on a signal won't re-run when that signal changes. "Wait for X then do Y" fires once, on the first read, and never again. Use `.get()` on every signal the effect depends on.

## Connect needs a gate signal, not just conditions

If the connect effect fires on "USER_ID set + init done", it fires as soon as either becomes true — before py has had a chance to set the port. A gate signal (`Blocked` / `AllowedToConnect`) that starts blocked and opens when the port is set makes the ordering explicit. The effect waits for init done + USER_ID + gate allowed.

## pytest hides stdout of passing tests

A JS panic during a passing test is invisible. The panic goes into `js_errors`, but pytest only shows captured output for failing tests — so the message doesn't surface until a later test fails on the accumulated list. Fix: an autouse fixture that fails the current test immediately when new JS errors appeared during it.

## `#[macro_export]` is at the crate root

Not the module path. In `main.rs` of a binary depending on the lib, use `crate_name::macro_name!()`. A `use crate_name::macro_name;` at the top doesn't help — the call site needs the full path.

## Small ones

`__pycache__/` and `*.pyc` in `.gitignore` — they change every run and cause merge noise. Playwright timeouts are milliseconds; `timeout=10000` reads like 10 seconds — a `seconds(n) -> int` helper makes intent clear. `page` fixture is session-scoped (one window per run); any `page.goto(url)` is a full reload, so a `web_app` fixture that navigates once and hands the page to tests avoids accidental reloads.
