from playwright.sync_api import expect


def test_web_app_shows_connected(web_app, pause):
    connected = web_app.locator("[data-testid='connected']")
    connected.wait_for(state="visible", timeout=10000)
    pause()
    assert connected.inner_text() == "connected"


def test_tester_can_call_rust_methods(web_app, pause):
    web_app.locator("[data-testid='connected']").wait_for(state="visible", timeout=10000)
    pause()

    web_app.evaluate(
        "window.dispatchEvent(new CustomEvent('e2e_set_message', { detail: 'hi from tester' }))"
    )
    pause()
    expect(web_app.locator("[data-testid='tester-message']")).to_have_text("hi from tester")

    web_app.evaluate("window.dispatchEvent(new CustomEvent('e2e_clear_message'))")
    pause()
    expect(web_app.locator("[data-testid='tester-message']")).to_have_text("")