use gtk4::prelude::*;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Button;
use gtk4::Label;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let app = Application::builder()
        .application_id("com.apesoda.test")
        .build();

    app.connect_activate(|app| {
       
        let window = ApplicationWindow::new(app);
        window.set_default_size(500, 500);
        window.set_title(Some("Test App Window"));

        let sm = StateManager::new(State { counter: 0 });
        let sm_clone = sm.clone();

        let label = Label::new(Some("Count: "));
        let label_clone = label.clone();
        
        let button = Button::new();
        button.set_halign(gtk4::Align::Start);
        button.set_label("Click me!");
        button.connect_clicked(move|_button| {
            sm_clone.increment();

            label_clone.set_label(&format!("Count: {}", sm_clone.value()));
        });

        let layout = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        layout.append(&button);
        layout.append(&label);

        window.set_child(Some(&layout));
        window.show();
    });

    app.run();
}
#[derive(Clone)]
struct StateManager {
    state : Rc<RefCell<State>>
}

impl StateManager {
    fn new(state : State) -> Self {
        Self {
            state : Rc::new(RefCell::new(state))
        }
    }

    fn increment(&self) {
        self.state.borrow_mut().counter +=1;
    }

    fn value(&self) -> u32 {
        self.state.borrow().counter
    }
}

struct State {
    counter : u32
}
