use app::App;
use window::WindowSpecs;

fn main() {
    let specs = WindowSpecs {
        title: "windowwww",
        width: 800,
        height: 600,
    };
    let mut app = App::new(specs);

    app.run();
}
