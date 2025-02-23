use crate::docs::router_learn::BookRoute;
use crate::*;

pub(crate) fn Hero() -> Element {
    rsx! {
        section { class: "w-full mx-auto dark:text-white flex flex-col justify-between items-center border-gray-300 min-h-[760px] flex-1 dark:border-[#a4a9ac7d] max-h-[960px] px-4",
            div { class: "flex w-full max-w-screen-xl flex-col text-center md:min-h-[520px] min-h-[760px] h-[calc(100vh-4rem)] gap-2 justify-evenly",
                div { class: "flex flex-col-reverse lg:flex-row items-center justify-end lg:justify-between lg:flex-1 flex-none",
                    div { class: "text-center lg:text-left lg:flex-1",
                        div { class: "text-[2.5em] md:text-[3.5em] font-semibold dark:text-white text-ghdarkmetal font-sans leading-snug text-balance",
                            span {
                                class: "bg-gradient-to-r text-8xl from-orange-500 to-amber-300 bg-clip-text text-transparent",
                                "ImplRust" }
                        }
                        h3 { class: "text-3xl font-bold dark:text-white py-5",
                            "Failure is not Option<T>. It's Result<T, E>"
                        }
                        h3 { class: "text-[1.25em] dark:text-white font-light text-ghdarkmetal max-w-screen-sm md:max-w-screen-md md:text-left text-center flex flex-col",
                            span { class: "max-w-screen-md leading-loose",
                                "ImplRust is an "
                                em{"unofficial"}
                                " Rust website dedicated to sharing learning materials and useful resources about Rust."
                            }
                        }
                        div { class: "pt-8 lg:pt-16 text-[1em] flex flex-row space-x-4 mx-auto lg:mx-0 justify-center lg:justify-start",
                            Link {
                                to: Route::Docs06 {
                                    child: BookRoute::Index {
                                        section: Default::default(),
                                    },
                                },
                                class: "bg-ghdarkmetal dark:bg-[#EDEDED] text-white dark:text-black border border-[#a4a9ac7d] m-0 p-2 px-4 rounded md:hover:-translate-y-1 transition-transform duration-300 w-full md:w-auto  dark:shadow-white",
                                "Get started"
                            }
                            Link {
                                to: "https://github.com/ImplFerris/ImplRust",
                                new_tab: true,
                                class: "bg-[#EDEDED] dark:bg-ghdarkmetal  text-black dark:text-white border border-[#a4a9ac7d]  m-0 p-2 px-4 rounded md:hover:-translate-y-1 transition-transform duration-300 w-full md:w-auto gap-2 flex flex-row items-center justify-center",
                                "Star on Github"
                            }
                        }
                    }
                    div { class: "lg:pb-12 h-screen max-h-40 lg:max-h-80 my-8",
                        img {
                            src: asset!("/assets/static/hero-ferris.svg"),
                            class: "dark:hidden w-full h-full",
                            alt: "Animated Icon",
                        }
                        img {
                            src: asset!("/assets/static/hero-ferris.svg"),
                            class: "hidden dark:block w-full h-full",
                            alt: "Animated Icon",
                        }
                    }
                }
            }
        }
    }
}
