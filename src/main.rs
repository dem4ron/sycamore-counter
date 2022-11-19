use sycamore::prelude::*;
fn main() {
    sycamore::render(|cx| {
        let state = create_signal(cx, 0);
        let counter = view! {cx, div(class="counter-container"){
           button(on:click=|_|{state.set(state.get().as_ref() - 1)}){"-"}
           div{
               span{
                   (state.get())
               }
               button(on:click=|_|{state.set(state.get().as_ref() + 1)}){"+"}
           }
        }};
        view! {
            cx,
            h1(style="text-align:center;margin-bottom:24px;"){"Counter in sycamore"}
            (counter)
        }
    })
}
