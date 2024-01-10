use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: AttrValue,
    pub input_type: AttrValue,
    pub name: AttrValue,
    pub value: AttrValue,
    pub onchange: Callback<Event>

}

#[function_component(Input)]
pub fn input(props: &Props) -> Html {
    let html_id = format!("edit-{}", props.name);
    html! {
        <>
            <label  class="block text-gray-700 text-sm font-bold mb-2" for={html_id.clone()}>{props.label.clone()}</label>
            <input
            class="shadow appearance-none border rounded w-60 py-2 px-3 text-gray-700 leading-tight  focus:shadow-outline"
                id={html_id}
                type={props.input_type.clone()} 
                name={props.name.clone()}
                value={props.value.clone()}
                onchange={props.onchange.clone()}

            />
        </>
    }
}
