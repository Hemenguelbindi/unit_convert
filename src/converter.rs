use yew::{html, Component, Html};
use crate::tabs::temperatur_tab::TemperatureComponent;
use crate::tabs::weight_tab::WeightComponet;
use crate::tabs::lenght_tab::LengthComponet;


pub enum Msg{
    SetTab(Tab),
}

pub enum Tab{
    Length, 
    Weight,
    Temperature,
}

pub struct Converter{
    active_tab: Tab,
}


impl Component for Converter {
    type Message = Msg;
    type Properties = ();


    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self{
             active_tab: Tab::Length,
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg{
            Msg::SetTab(tab) => {
                self.active_tab = tab;
                true
            }
        }
    }


    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        let link = _ctx.link();
        html! {
            <div class="w-screen h-screen flex flex-col items-center justify-center dark:bg-slate-800 bg-gray-100">

                <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-4">{"Unit converter"}</h1>
                <div class="mb-4 border-b border-gray-200 dark:border-gray-700 w-full max-w-xl">
                    <ul class="flex flex-wrap -mb-px text-sm font-medium text-center" id="default-styled-tab" role="tablist">
                        <li class="mr-2" role="presentation">
                            <button class="inline-block p-4 border-b-2 border-transparent hover:text-purple-600 dark:hover:text-purple-500 dark:hover:border-purple-500 text-gray-500 dark:text-gray-400 hover:border-gray-300 dark:border-gray-700 dark:hover:border-gray-500" id="tab-length" role="tab"
                            onclick={link.callback(|_| Msg::SetTab(Tab::Length))}>                           
                            {"Length"}
                            </button>
                        </li>
                        <li class="mr-2" role="presentation">
                            <button class="inline-block p-4 border-b-2 border-transparent hover:text-purple-600 dark:hover:text-purple-500 dark:hover:border-purple-500 text-gray-500 dark:text-gray-400 hover:border-gray-300 dark:border-gray-700 dark:hover:border-gray-500" id="tab-weight" role="tab"
                            onclick={link.callback(|_| Msg::SetTab(Tab::Weight))}>
                                {"Weigth"}
                            </button>
                        </li>
                        <li class="mr-2" role="presentation">
                            <button class="inline-block p-4 border-b-2 border-transparent hover:text-purple-600 dark:hover:text-purple-500 dark:hover:border-purple-500 text-gray-500 dark:text-gray-400 hover:border-gray-300 dark:border-gray-700 dark:hover:border-gray-500" id="tab-weight" role="tab"
                             onclick={link.callback(|_| Msg::SetTab(Tab::Temperature))}>
                                {"Temperature"}
                            </button>
                        </li>
                    </ul>
                    <div class="w-full max-w-xl p-4 bg-white dark:bg-gray-800 rounded-lg shadow">
                        {
                            match self.active_tab {
                                Tab::Length => html! {<LengthComponet/>},
                                Tab::Weight => html! {<WeightComponet/>},
                                Tab::Temperature => html! {<TemperatureComponent/>},
                            }
                        }
                    </div>
                </div>
            </div>
        }
    }
}

