use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Site {
    value: i64,
}

impl Component for Site {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <body>
                <div class = "the-introduction">
                    <h1 class = "headline">{ "This is madness (COOB should burn)" }</h1>
                    <p class = "movy-boi">{ "Ohjelmointi, eli näppäimistön raivokkaasti paukuttaminen ja hakukoneen käytön opettelu on tapa taivuttaa tietokoneet sun tahtoosi (ei literalistisesti)." }</p>
                </div>
                <div class = "rainbow">
                    <h1 class = "headline">{ "Rainbow" }</h1>
                    <p class = "movy-boi">{ "Rainbow is a programming language that makes it easy to write code that is both readable and efficient." }</p>
                    <p class = "rainbow">{ "Rainbow is a programming language that makes it easy to write code that is both readable and efficient." }</p>
                </div>

            </body>
        }
    }
}

fn main() {
    yew::start_app::<Site>();
}
