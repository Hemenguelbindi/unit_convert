use yew::{Component, html, Html};

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



struct LengthComponet;

impl Component for LengthComponet{
    type Message = ();
    type Properties = ();


    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self{}
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
         html! {
            <form class="max-w-sm mx-auto">
            <div class="mb-5">
                <label for="large-input" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
                {"Enter the length to convert"}</label>
                <input type="text" id="base-input" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" />
            </div>
            <div class="mb-5">
                <label for="large-input" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
                {"Unit to convert from"}</label>
                <input type="text" id="base-input" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" />
            </div>
            <div>
                <label for="large-input" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
                {"Unit to Convert to"}
                </label>
                <input type="text" id="base-input" class="block w-full p-2 text-gray-900 border border-gray-300 rounded-lg bg-gray-50 text-xs focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" />
            </div>
            </form>
        }
    }
}


struct WeightComponet;

impl Component for WeightComponet{
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self{}
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
          html! {
            <form class="max-w-sm mx-auto">
            <div class="mb-5">
                <label for="large-input" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
                {"Enter the weight to convert"}</label>
                <input type="text" id="base-input" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" />
            </div>
            <div class="mb-5">
                <label for="large-input" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
                {"Unit to convert from"}</label>
                <input type="text" id="base-input" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" />
            </div>
            <div>
                <label for="large-input" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
                {"Unit to Convert to"}
                </label>
                <input type="text" id="base-input" class="block w-full p-2 text-gray-900 border border-gray-300 rounded-lg bg-gray-50 text-xs focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" />
            </div>
            </form>
        }
    
    }
}


struct TemperatureComponent;

impl Component for TemperatureComponent{
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self{}
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        html!{
            <form class="max-w-sm mx-auto">
            <div class="mb-5">
                <label for="large-input" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
                {"Enter the temperature to convert"}</label>
                <input type="text" id="base-input" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" />
            </div>
            <div class="mb-5">
                <label for="large-input" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
                {"Unit to convert from"}</label>
                <input type="text" id="base-input" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" />
            </div>
            <div>
                <label for="large-input" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
                {"Unit to Convert to"}
                </label>
                <input type="text" id="base-input" class="block w-full p-2 text-gray-900 border border-gray-300 rounded-lg bg-gray-50 text-xs focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" />
            </div>
            </form>
        }
    }

}
    

