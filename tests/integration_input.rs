use rust_engine::core::app::App;
use rust_engine::core::input::{EngineElementState, EngineKey, EngineMouseButton};
use rust_engine::events::{CursorMovedEvent, KeyboardInputEvent, MouseInputEvent};
use rust_engine::InputPlugin;

fn setup_app() -> App {
    let mut app = App::new();
    app.add_plugin(&InputPlugin::new());
    app
}

fn send_keyboard_event(app: &mut App, key: EngineKey, state: EngineElementState) {
    let di = app.get_di_container();
    let evts = di
        .get_mut::<rust_engine::core::events::Events<KeyboardInputEvent>>()
        .expect("Keyboard events should be registered");
    evts.send(KeyboardInputEvent { key, state });
}

fn send_mouse_event(app: &mut App, button: EngineMouseButton, state: EngineElementState) {
    let di = app.get_di_container();
    let evts = di
        .get_mut::<rust_engine::core::events::Events<MouseInputEvent>>()
        .expect("Mouse events should be registered");
    evts.send(MouseInputEvent { button, state });
}

fn send_cursor_event(app: &mut App, x: f32, y: f32) {
    let di = app.get_di_container();
    let evts = di
        .get_mut::<rust_engine::core::events::Events<CursorMovedEvent>>()
        .expect("Cursor events should be registered");
    evts.send(CursorMovedEvent { x, y });
}

#[test]
fn integration_keyboard_press() {
    let mut app = setup_app();

    send_keyboard_event(&mut app, EngineKey::A, EngineElementState::Pressed);
    app.late_update(); // make the sent event readable
    app.process_input();

    let di = app.get_di_container();
    let input = di
        .get_mut::<rust_engine::core::input::Input>()
        .expect("Input should be present");
    assert!(input.is_key_down(EngineKey::A));
    assert!(input.is_key_pressed(EngineKey::A));
}

#[test]
fn integration_keyboard_press_then_release() {
    let mut app = setup_app();

    // Press
    send_keyboard_event(&mut app, EngineKey::A, EngineElementState::Pressed);
    app.late_update();
    app.process_input();

    // Now release
    send_keyboard_event(&mut app, EngineKey::A, EngineElementState::Released);
    app.late_update();
    app.process_input();

    let di = app.get_di_container();
    let input = di
        .get_mut::<rust_engine::core::input::Input>()
        .expect("Input should be present");
    assert!(!input.is_key_down(EngineKey::A));
    assert!(!input.is_key_pressed(EngineKey::A));
}

#[test]
fn integration_mouse_button_press_and_cursor_move() {
    let mut app = setup_app();

    send_mouse_event(
        &mut app,
        EngineMouseButton::Left,
        EngineElementState::Pressed,
    );
    send_cursor_event(&mut app, 123.0, 456.0);
    app.late_update();
    app.process_input();

    let di = app.get_di_container();
    let input = di
        .get_mut::<rust_engine::core::input::Input>()
        .expect("Input should be present");

    assert!(input.is_mouse_button_down(EngineMouseButton::Left));
    assert!(input.is_mouse_button_pressed(EngineMouseButton::Left));
    assert_eq!(input.get_mouse_position(), (123.0, 456.0));
}

#[test]
fn integration_lost_focus_clears_state() {
    let mut app = setup_app();

    // Press some keys and buttons
    send_keyboard_event(&mut app, EngineKey::A, EngineElementState::Pressed);
    send_mouse_event(
        &mut app,
        EngineMouseButton::Right,
        EngineElementState::Pressed,
    );
    app.late_update();
    app.process_input();

    // Ensure they are down
    {
        let di = app.get_di_container();
        let input = di
            .get_mut::<rust_engine::core::input::Input>()
            .expect("Input should be present");
        assert!(input.is_key_down(EngineKey::A));
        assert!(input.is_mouse_button_down(EngineMouseButton::Right));
    }

    // Call lost_focus directly and ensure cleared
    {
        let di = app.get_di_container();
        let input = di
            .get_mut::<rust_engine::core::input::Input>()
            .expect("Input should be present");
        input.lost_focus();
        assert!(!input.is_key_down(EngineKey::A));
        assert!(!input.is_mouse_button_down(EngineMouseButton::Right));
    }
}
