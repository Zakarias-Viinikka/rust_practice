from playwright.sync_api import expect


def test_web_app_shows_connected(page, web_app_url, pause):
    page.goto(web_app_url)
    pause()
    connected = page.locator("[data-testid='connected']")
    connected.wait_for(state="visible", timeout=10000)
    assert connected.inner_text() == "connected"


def test_tester_can_call_rust_methods(page, web_app_url, pause):
    page.goto(web_app_url)
    page.locator("[data-testid='connected']").wait_for(state="visible", timeout=10000)
    pause()

    page.evaluate(
        "window.dispatchEvent(new CustomEvent('e2e_set_message', { detail: 'hi from tester' }))"
    )
    pause()
    expect(page.locator("[data-testid='tester-message']")).to_have_text("hi from tester")

    page.evaluate("window.dispatchEvent(new CustomEvent('e2e_clear_message'))")
    pause()
    expect(page.locator("[data-testid='tester-message']")).to_have_text("")