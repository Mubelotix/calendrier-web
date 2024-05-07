use yew::prelude::*;
use yew_template::*;

struct App {

}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let date = "27 vendémiaire 232";
        template_html!("src/main.html", ...)
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
