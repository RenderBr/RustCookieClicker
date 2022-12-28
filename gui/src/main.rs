use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0); 
    let mut multiplier: &UseState<i32> = use_state(&cx, || 1);

    //upgrades
    let mut upgrade1: &UseState<i32> = use_state(&cx, || 0);
    let mut upgrade1_count: &UseState<i32> = use_state(&cx, || 0);

    let mut upgrade2: &UseState<i32> = use_state(&cx, || 0);
    let mut upgrade3: &UseState<i32> = use_state(&cx, || 0);
    let mut upgrade4: &UseState<i32> = use_state(&cx, || 0);

    let mut isWarningOn: &UseState<String> = use_state(&cx, || "none".to_string());
    cx.render(rsx!{
        link {
            rel: "stylesheet",
            href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css",
        },
        style {
            [include_str!("./style.css")]
        }
        div {
            class:"flex justify-center",
            h1 {
                class:"text-size-400 text-3xl",
                "Cookie Clicker in Rust"
            },
            div{
                // text that displays the count of clicks
                class:"text-size-400 text-3xl",
            }
    },
        div {
            class:"flex justify-center",
            img {
                onclick: move |evt| count += (1*multiplier.get()),
                id:"cookie",
                width:"300vw",
                src: "https://assets.stickpng.com/images/580b57fbd9996e24bc43c0fc.png"
            }
        }
        div{
            class:"flex justify-center",
            a{
                class:"text-size-400",
                "You have clicked {count} times!"
            }
        }
        div{
            class:"flex justify-center",
            button{
                class:"bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                onclick: move |evt| {if(count.get() >= &1){ multiplier += 1; count -= 1 *multiplier.get()}else{}},
            }
        }
        div{
            a {
              class:"text-red-100",
              display: "none",
              "You couldn't afford that! You're broke :("
            }
        }
    })
}

#[derive(PartialEq, Props)]
struct UpgradeProps{
    name: String,
    cost: i32,
    multiplier: i32,
    count: i32,
}
//create an element for an upgrade
fn upgrade(cx: Scope<UpgradeProps>) -> Element {

    if(cx.props.count >= 1){


    cx.render(rsx!{

            div{
                class:"flex justify-center",
                button{
                    class:"bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                    onclick: move |evt| {if(cx.props.count >= 1){ cx.props.multiplier += 1; cx.props.count -= 1 *cx.props.multiplier}else{}},
                    //if you don't have enough cookies to buy the upgrade, it will disappear
              
                }
            }       
    })
    }
    else{
        cx.render(rsx!{
            div{
                class:"flex justify-center",
                button{
                    class:"bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                    onclick: move |evt| {if(cx.props.count >= 1){ cx.props.multiplier += 1; cx.props.count -= 1 *cx.props.multiplier}else{}},
                    //if you don't have enough cookies to buy the upgrade, it will disappear
                    display: "none",
                }
            }       
    })


    }
}