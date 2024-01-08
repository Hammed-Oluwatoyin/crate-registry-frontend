use yew::prelude::*;

use crate::components::input::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <div class=" flex justify-center items-center h-[100vh]">
            <div class="flex flex-row ">
                <div class="col-md-4">
                    <form>
                        <div class="mb-3">
                            <Input input_type="text" name="username" label="Username" />
                        </div>
                        <div class="mb-3">
                            <Input input_type="password" name="password" label="Password" />
                        </div>
                        <button  class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"  type="submit">{"Login"}</button>
                    </form>
                </div>
            </div>
        </div>
    }
}
