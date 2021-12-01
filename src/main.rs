use days::run_day;
use wasm_bindgen::JsCast;
use web_sys::{HtmlSelectElement, HtmlTextAreaElement};
use yew::prelude::*;

mod days;

#[derive(Debug, PartialEq, Eq)]
enum Msg {
    Solve,
    Input(String),
    Day(usize),
}

struct App {
    day: usize,
    input: String,
    part_one: Option<String>,
    part_two: Option<String>,
}

fn format_day(day: usize) -> String {
    format!("Day {}", day)
}

fn maybe_solution(part: Option<&String>) -> String {
    if let Some(thing) = part {
        thing.to_string()
    } else {
        "".to_string()
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            day: 1,
            input: "".to_string(),
            part_one: None,
            part_two: None,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Day(day) => {
                self.day = day;
                true
            }
            Msg::Solve => {
                let (p1, p2) = run_day(self.day, &self.input);
                self.part_one = Some(p1);
                self.part_two = Some(p2);
                true
            }
            Msg::Input(input) => {
                self.input = input;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let selected = ctx.link().callback(|event: Event| {
            let input = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok())
                .unwrap();
            let selected = input.value().parse().unwrap();
            Msg::Day(selected)
        });
        let input = ctx.link().callback(|event: Event| {
            let input = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok())
                .unwrap();
            Msg::Input(input.value())
        });
        html! {
            <main>
                <h1>{ "Advent of Code 2021" }</h1>
                <select onchange={selected}>
                    {for {
                        (1..=25).map(|i| html! { <option value={i.to_string()} selected={self.day == i}>{ format_day(i) }</option> })
                    }}
                </select>
                <textarea onchange={input} rows="20" cols="50" />
                <button onclick={link.callback(|_| Msg::Solve)}>{ "Solve" }</button>
                <section>
                    <p>{ maybe_solution(self.part_one.as_ref()) }</p>
                    <p>{ maybe_solution(self.part_two.as_ref()) }</p>
                </section>
            </main>
        }
    }
}
fn main() {
    yew::start_app::<App>();
}
