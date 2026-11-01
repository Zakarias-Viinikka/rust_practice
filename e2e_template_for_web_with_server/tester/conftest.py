import os
import time
import pytest
from playwright.sync_api import sync_playwright


@pytest.fixture(scope="session")
def browser():
    headless = os.environ.get("HEADLESS", "0") == "1"
    with sync_playwright() as p:
        b = p.chromium.launch(headless=headless, args=["--start-maximized"])
        yield b
        b.close()


@pytest.fixture(scope="session")
def page(browser):
    context = browser.new_context(no_viewport=True)
    pg = context.new_page()
    pg.js_errors = []
    def on_pageerror(e):
        pg.js_errors.append(str(e))
    pg.on("pageerror", on_pageerror)
    pg.on("console", lambda m: print(f"### CONSOLE {m.type}: {m.text}"))
    if os.environ.get("HEADLESS", "0") != "1":
        time.sleep(1)
    yield pg
    context.close()


@pytest.fixture(autouse=True)
def _no_js_errors_per_test(page, request):
    """pytest hides stdout of passing tests, so a JS panic during a passing
    test is invisible unless we fail that test. This does."""
    start = len(page.js_errors)
    yield
    new = page.js_errors[start:]
    if new:
        raise AssertionError(f"JS errors during {request.node.name}: {new}")


@pytest.fixture(scope="session")
def web_app(page, web_app_url):
    """Lands on /custom_port_setting_page, sets the server port, navigates
    to / as an SPA change, and returns the page ready for tests. No page
    reload from here on."""
    from playwright.sync_api import expect

    page.goto(web_app_url)
    expect(page.locator("[data-testid='custom-port-setting-page']")).to_be_visible(
        timeout=2000
    )
    port = int(os.environ["SERVER_PORT"])
    page.evaluate(f"window.dispatchEvent(new CustomEvent('set_port', {{ detail: {port} }}))")
    page.evaluate("window.dispatchEvent(new CustomEvent('navigate', { detail: '/' }))")
    return page


@pytest.fixture(scope="session")
def web_app_url():
    return os.environ["WEB_APP_URL"]


@pytest.fixture
def pause():
    """Sleep between test actions so a person can watch. No-op when
    HEADLESS=1."""

    def _pause(seconds: float = 1.0):
        if os.environ.get("HEADLESS", "0") != "1":
            time.sleep(seconds)

    return _pause