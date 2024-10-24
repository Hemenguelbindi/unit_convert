use web_sys::{EventTarget, HtmlInputElement, HtmlSelectElement};
use yew::{html, Component, Event, Html, MouseEvent};
use gloo::console::log;
use wasm_bindgen::JsCast;


pub enum WeightMessage {
    UpdateTemperature(String),
    UpdateUnitFrom(String),
    UpdateUnitTo(String),
    Convert,
    Reset
}

pub struct WeightComponet{
    weight: String,
    unit_from: String,
    unit_to: String,
    convert: String,
    calculate: bool,
}

impl Component for WeightComponet{
    type Message = WeightMessage;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self{
            weight: String::from("0.0"),
            unit_from: String::from("milligram"),
            unit_to: String::from("gram"),
            convert: String::from(""),
            calculate: false,
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            WeightMessage::UpdateTemperature(weight) => {
                self.weight = weight;
                true
            }
            WeightMessage::UpdateUnitFrom(unit) => {
                self.unit_from = unit;
                true
            }
            WeightMessage::UpdateUnitTo(unit) => {
                self.unit_to = unit;
                true
            }
            WeightMessage::Convert => {
                self.calculate = true;
                self.convert = converter_weight(self.weight.parse::<f64>().unwrap(), &self.unit_from, &self.unit_to);   
                true
            }
            WeightMessage::Reset => {
                self.calculate = false;
                true
            }
        }
    }


    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        let on_input = _ctx.link().callback(move |event: Event| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                log!("Convet value: ",input.value());
                WeightMessage::UpdateTemperature(input.value())
            } else {
                WeightMessage::UpdateTemperature(String::new())
            }
        });

        let on_unit_from = _ctx.link().callback(move |event: Event| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            if let Some(input) = input {
                log!("Value Unit From:",input.value());
                WeightMessage::UpdateUnitFrom(input.value())
            } else {
                WeightMessage::UpdateUnitFrom(String::new())
            }

        });

        let on_unit_to = _ctx.link().callback(move |event: Event| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            if let Some(input) = input {
                log!("Value Unit to:",input.value());
                WeightMessage::UpdateUnitTo(input.value())
            } else {
                WeightMessage::UpdateUnitTo(String::new())
            }

        });
        
        let convert = _ctx.link().callback(move |event: MouseEvent| {
            event.prevent_default();
            WeightMessage::Convert
        });

        let reset  = _ctx.link().callback(move| event: MouseEvent| {
            event.prevent_default();
            WeightMessage::Reset
        });

        if self.calculate{
            html!{
                <div class="flex flex-col justify-center items-center space-y-4">
                    <h3>{"Result is calculated"}</h3>
                    <p>{self.weight.to_string()}{"="} {&self.convert}</p>
                    <button class="bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center"
                        onclick={reset}>
                        {"Reset"}
                    </button>
                </div>
            }
        } else {
          html! {
            <form class="max-w-sm mx-auto">
            <div class="mb-5">
                <label for="large-input" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
                {"Enter the weight to convert"}</label>
                <input type="text" id="base-input" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"

                onchange={on_input}/>
            </div>
            <div class="mb-5">
                <label for="countries" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Unit to convert from"}</label>
                <select id="countries" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                onchange={on_unit_from}>
                    <option>{"milligram"}</option>
                    <option>{"gram"}</option>
                    <option>{"kilogram"}</option>
                    <option>{"ounce"}</option>
                    <option>{"pound"}</option>
                </select>
            </div>
            <div class="mb-5">
                <label for="countries" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Unit to convert to"}</label>
                <select id="countries" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                onchange={on_unit_to}
                >
                    <option>{"milligram"}</option>
                    <option>{"gram"}</option>
                    <option>{"kilogram"}</option>
                    <option>{"ounce"}</option>
                    <option>{"pound"}</option>
                </select>
            </div>
            <button type="submit" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
            onclick={convert}
            >{"Convert"}</button>
            </form>
        }
        }
    }
}

fn converter_weight(weight: f64, unit_from: &str, unit_to: &str) -> String {
    match (unit_from, unit_to) {
        ("milligram", "gram") => (weight / 1000.0).to_string(),
        ("milligram", "kilogram") => (weight / 1000000.0).to_string(),
        ("milligram", "ounce") => (weight / 28349.523125).to_string(),
        ("milligram", "pound") => (weight / 453592.37).to_string(),
        
        ("gram", "milligram") => (weight * 1000.0).to_string(),
        ("gram", "kilogram") => (weight / 1000.0).to_string(),
        ("gram", "ounce") => (weight / 28.349523125).to_string(),
        ("gram", "pound") => (weight / 453.59237).to_string(),    

        ("kilogram", "milligram") => (weight * 1000000.0).to_string(),
        ("kilogram", "gram") => (weight * 1000.0).to_string(),
        ("kilogram", "ounce") => (weight * 35.27396195).to_string(),
        ("kilogram", "pound") => (weight * 2.20462262185).to_string(),


        ("ounce", "milligram") => (weight * 28349.523125).to_string(),
        ("ounce", "gram") => (weight * 28.349523125).to_string(),
        ("ounce", "kilogram") => (weight / 35.27396195).to_string(),
        ("ounce", "pound") => (weight / 16.0).to_string(),


        ("pound", "milligram") => (weight * 453592.37).to_string(),
        ("pound", "gram") => (weight * 453.59237).to_string(),
        ("pound", "kilogram") => (weight / 2.20462262185).to_string(),
        ("pound", "ounce") => (weight * 16.0).to_string(),

        
        _ => "Unsupported unit conversion".to_string(),
    }
}
