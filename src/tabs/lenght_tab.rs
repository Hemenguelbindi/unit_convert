use web_sys::{EventTarget, HtmlInputElement, HtmlSelectElement};
use yew::{html, Component, Event, Html, MouseEvent};
use gloo::console::log;
use wasm_bindgen::JsCast;

pub enum LengthMessage {
    UpdateTemperature(String),
    UpdateUnitFrom(String),
    UpdateUnitTo(String),
    Convert,
    Reset
}


pub struct LengthComponet{
    length: String,
    unit_from: String,
    unit_to: String,
    convert: String,
    calculate: bool,
}

impl Component for LengthComponet{
    type Message = LengthMessage;
    type Properties = ();
    

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self{
            length: String::new(),
            unit_from: String::new(),
            unit_to: String::new(),
            convert: String::new(),
            calculate: false,
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LengthMessage::UpdateTemperature(temp) => {
                self.length= temp;
                true
            }
            LengthMessage::UpdateUnitFrom(unit) => {
                self.unit_from = unit;
                log!(&self.unit_from);
                true
            }
            LengthMessage::UpdateUnitTo(unit) => {
                self.unit_to = unit;
                log!(&self.unit_to);
                true
            }
            LengthMessage::Convert => {
                self.calculate = true;
                self.convert = converter_length(self.length.parse::<f64>().unwrap(), &self.unit_from, &self.unit_to);
                true
            }
            LengthMessage::Reset => {
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
                LengthMessage::UpdateTemperature(input.value())
            } else {
                LengthMessage::UpdateTemperature(String::new())
            }
        });

        let on_unit_from = _ctx.link().callback(move |event: Event| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            if let Some(input) = input {
                log!("Value Unit From:",input.value());
                LengthMessage::UpdateUnitFrom(input.value())
            } else {
                LengthMessage::UpdateUnitFrom(String::new())
            }

        });

        let on_unit_to = _ctx.link().callback(move |event: Event| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            if let Some(input) = input {
                log!("Value Unit to:",input.value());
                LengthMessage::UpdateUnitTo(input.value())
            } else {
                LengthMessage::UpdateUnitTo(String::new())
            }

        });

        let convert = _ctx.link().callback(move |event: MouseEvent| {
            event.prevent_default();
            LengthMessage::Convert
        });

        let reset  = _ctx.link().callback(move| event: MouseEvent| {
            event.prevent_default();
            LengthMessage::Reset
        });
        if self.calculate{
            html!{
                <div class="flex flex-col justify-center items-center space-y-4">
                    <h3>{"Result is calculated"}</h3>
                    <p>{self.length.to_string()}{"="} {&self.convert}</p>
                    <button class="bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center"
                        onclick={reset}>
                        {"Reset"}
                    </button>
                </div>
            }
        }else{
            html! {
            <form class="max-w-sm mx-auto">
            <div class="mb-5">
                <label for="large-input" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
                {"Enter the length to convert"}</label>
                <input type="text" id="base-input" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" 
                onchange={on_input}
                />
            </div>
            <div class="mb-5">
                <label for="countries" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Unit to convert from"}</label>
                <select id="countries" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                onchange={on_unit_from}>
                    <option>{"millimeter"}</option>
                    <option>{"centimeter"}</option>
                    <option>{"metr"}</option>
                    <option>{"kilometer"}</option>
                    <option>{"inch"}</option>
                    <option>{"foot"}</option>
                    <option>{"yard"}</option>
                    <option>{"mile"}</option>
                </select>
            </div>
            <div class="mb-5">
                <label for="countries" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Unit to convert from"}</label>
                <select id="countries" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                onchange={on_unit_to}>
                    <option>{"millimeter"}</option>
                    <option>{"centimeter"}</option>
                    <option>{"metr"}</option>
                    <option>{"kilometer"}</option>
                    <option>{"inch"}</option>
                    <option>{"foot"}</option>
                    <option>{"yard"}</option>
                    <option>{"mile"}</option>
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


fn converter_length(length: f64, unit_from: &str, unit_to: &str) -> String {
    match (unit_from, unit_to) {

        ("millimeter", "centimeter") => (length / 10.0).to_string(),
        ("millimeter", "meter") => (length / 1000.0).to_string(),
        ("millimeter", "kilometer") => (length / 1000000.0).to_string(),
        ("millimeter", "inch") => (length / 25.4).to_string(),
        ("millimeter", "foot") => (length / 304.8).to_string(),
        ("millimeter", "yard") => (length / 914.4).to_string(),
        ("millimeter", "mile") => (length / 1609344.0).to_string(),

        ("centimeter", "millimeter") => (length * 10.0).to_string(),
        ("centimeter", "meter") => (length / 100.0).to_string(),
        ("centimeter", "kilometer") => (length / 100000.0).to_string(),
        ("centimeter", "inch") => (length / 2.54).to_string(),
        ("centimeter", "foot") => (length / 30.48).to_string(),
        ("centimeter", "yard") => (length / 91.44).to_string(),
        ("centimeter", "mile") => (length / 160934.4).to_string(),


        ("meter", "millimeter") => (length * 1000.0).to_string(),
        ("meter", "centimeter") => (length * 100.0).to_string(),
        ("meter", "kilometer") => (length / 1000.0).to_string(),
        ("meter", "inch") => (length * 39.3700787).to_string(),
        ("meter", "foot") => (length * 3.2808399).to_string(),
        ("meter", "yard") => (length * 1.0936133).to_string(),
        ("meter", "mile") => (length / 1609.344).to_string(),

        ("kilometer", "millimeter") => (length * 1000000.0).to_string(),
        ("kilometer", "centimeter") => (length * 100000.0).to_string(),
        ("kilometer", "meter") => (length * 1000.0).to_string(),
        ("kilometer", "inch") => (length * 39370.0787).to_string(),
        ("kilometer", "foot") => (length * 3280.8399).to_string(),
        ("kilometer", "yard") => (length * 1093.6133).to_string(),
        ("kilometer", "mile") => (length / 1.609344).to_string(),


        ("inch", "millimeter") => (length * 25.4).to_string(),
        ("inch", "centimeter") => (length * 2.54).to_string(),
        ("inch", "meter") => (length / 39.3700787).to_string(),
        ("inch", "kilometer") => (length / 39370.0787).to_string(),
        ("inch", "foot") => (length / 12.0).to_string(),
        ("inch", "yard") => (length / 36.0).to_string(),
        ("inch", "mile") => (length / 63360.0).to_string(),

        ("foot", "millimeter") => (length * 304.8).to_string(),
        ("foot", "centimeter") => (length * 30.48).to_string(),
        ("foot", "meter") => (length / 3.2808399).to_string(),
        ("foot", "kilometer") => (length / 3280.8399).to_string(),
        ("foot", "inch") => (length * 12.0).to_string(),
        ("foot", "yard") => (length / 3.0).to_string(),
        ("foot", "mile") => (length / 5280.0).to_string(),


        ("yard", "millimeter") => (length * 914.4).to_string(),
        ("yard", "centimeter") => (length * 91.44).to_string(),
        ("yard", "meter") => (length / 1.0936133).to_string(),
        ("yard", "kilometer") => (length / 1093.6133).to_string(),
        ("yard", "inch") => (length * 36.0).to_string(),
        ("yard", "foot") => (length * 3.0).to_string(),
        ("yard", "mile") => (length / 1760.0).to_string(),


        ("mile", "millimeter") => (length * 1609344.0).to_string(),
        ("mile", "centimeter") => (length * 160934.4).to_string(),
        ("mile", "meter") => (length * 1609.344).to_string(),
        ("mile", "kilometer") => (length * 1.609344).to_string(),
        ("mile", "inch") => (length * 63360.0).to_string(),
        ("mile", "foot") => (length * 5280.0).to_string(),
        ("mile", "yard") => (length * 1760.0).to_string(),


        _ => "Unsupported unit conversion".to_string(),
    }
}
