mod converter;
use yew;
use converter::Converter;



fn main() {
    yew::Renderer::<Converter>::new().render();
}