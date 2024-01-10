use yew::prelude::*;

use crate::components::input::*;
use crate::components::login_form::LoginForm;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <div class=" flex justify-center items-center h-[100vh]">
            <div class="flex flex-row ">
                <div class="col-md-4">
                   <LoginForm/>
                </div>
            </div>
        </div>
    }
}
