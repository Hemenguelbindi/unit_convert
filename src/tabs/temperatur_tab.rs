use web_sys::{EventTarget, HtmlInputElement, HtmlSelectElement};
use yew::{html, Component, Event, Html, MouseEvent};
use gloo::console::log;
use wasm_bindgen::JsCast;


pub enum TempMsg {
    UpdateTemperature(String),
    UpdateUnitFrom(String),
    UpdateUnitTo(String),
    Convert,
    Reset,
}


pub struct TemperatureComponent{
    temperature: String,
    unit_from: String,
    unit_to: String,
    convert: String,
    calculate: bool,
}

/// This is the temperature comp

impl Component for TemperatureComponent{
    type Message = TempMsg;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self{
            temperature: String::from("0.0"),
            unit_from: "Celsius".to_string(),
            unit_to: "Fahrenheit".to_string(),
            convert: String::from("0.0"),
            calculate: false,
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TempMsg::UpdateTemperature(temp) => {
                self.temperature = temp;
                true
            }
            TempMsg::UpdateUnitFrom(unit) => {
                self.unit_from = unit;
                log!(&self.unit_from);
                true
            }
            TempMsg::UpdateUnitTo(unit) => {
                self.unit_to = unit;
                log!(&self.unit_to);
                true
            }
            TempMsg::Convert => {
                self.calculate = true;
                self.convert = converter_temperature(self.temperature.parse::<f64>().unwrap(), &self.unit_from, &self.unit_to);
                log!(self.convert.to_string());
                log!("Converting: ", &self.temperature.to_string(), &self.unit_from.to_string(), &self.unit_to.to_string());
                true
            }
            TempMsg::Reset => {
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
                TempMsg::UpdateTemperature(input.value())
            } else {
                TempMsg::UpdateTemperature(String::new())
            }
        });

        let on_unit_from = _ctx.link().callback(move |event: Event| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            if let Some(input) = input {
                log!("Value Unit From:",input.value());
                TempMsg::UpdateUnitFrom(input.value())
            } else {
                TempMsg::UpdateUnitFrom(String::new())
            }

        });

        let on_unit_to = _ctx.link().callback(move |event: Event| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            if let Some(input) = input {
                log!("Value Unit to:",input.value());
                TempMsg::UpdateUnitTo(input.value())
            } else {
                TempMsg::UpdateUnitTo(String::new())
            }

        });

        let convert = _ctx.link().callback(move |event: MouseEvent| {
            event.prevent_default();
            TempMsg::Convert
        });

        let reset  = _ctx.link().callback(move| event: MouseEvent| {
            event.prevent_default();
            TempMsg::Reset
        });

        if self.calculate{
            html!{
                <div class="flex flex-col justify-center items-center space-y-4">
                    <h3>{"Result is calculated"}</h3>
                    <p>{self.temperature.to_string()}{"="} {&self.convert}</p>
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
                    <label for="temperature-input" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
                    {"Enter the temperature to convert"}</label>
                    <input
                        type="text"
                        id="temperature-input"
                        class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                        onchange={on_input}
                    />
                </div>
                <div class="mb-5">
                    <label for="unit-from" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
                    {"Convert from"}</label>
                    <select id="unit-from" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    onchange={on_unit_from}>
                        <option value="Celsius">{"Celsius"}</option>
                        <option value="Fahrenheit">{"Fahrenheit"}</option>
                        <option value="Kelvin">{"Kelvin"}</option>
                    </select>
                </div>
                <div class="mb-5">
                    <label for="unit-to" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
                    {"Convert to"}</label>
                    <select id="unit-to" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    onchange={on_unit_to}>
                        <option value="Celsius">{"Celsius"}</option>
                        <option value="Fahrenheit">{"Fahrenheit"}</option>
                        <option value="Kelvin">{"Kelvin"}</option>
                    </select>
                </div>
                <button type="submit" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                onclick={convert}>{"Convert"}</button>
            </form>
        }
    }
        

}
}

fn converter_temperature(temperature: f64, unit_from: &str, unit_to: &str) -> String {
    match (unit_from, unit_to) {
        ("Celsius", "Fahrenheit") => ((temperature * 9.0/5.0) + 32.0).to_string(),
        ("Fahrenheit", "Celsius") => ((temperature - 32.0) * 5.0/9.0).to_string(),
        ("Fahrenheit", "Kelvin") => (temperature - 273.15).to_string(),
        ("Celsius", "Kelvin") => (temperature + 273.0).to_string(),
        ("Kelvin", "Fahrenheit") => ((temperature - 273.15) * 9.0/5.0 + 32.0).to_string(),
        ("Kelvin", "Celsius") => (temperature - 273.15).to_string(),
        ("Celcius", "Celcius") => temperature.to_string(),
        ("Farenheit", "Fahrenheit") => temperature.to_string(),
        _  => 0.0.to_string(),
    }
}