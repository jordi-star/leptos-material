use std::thread::current;
use std::time::Instant;

use crate::components::button::ButtonStyle;
use crate::components::icon::Icon;
use crate::components::select::SelectOption;
use crate::components::{button::Button, button::ButtonType, iconbutton::IconButton};
use chrono::Date;
use chrono::Datelike;
use chrono::Local;
use chrono::NaiveDate;
use leptos::ev::InputEvent;
use leptos::html::Button;
use leptos::DynAttrs;
use leptos::RwSignal;
use leptos::Signal;
use leptos::SignalGet;
use leptos::SignalGetUntracked;
use leptos::SignalSet;
use leptos::{component, html, logging, view, Children, Class, CollectView, IntoClass, IntoView};
use leptos::{
    create_effect, create_node_ref, create_rw_signal, create_slice, IntoSignal, NodeRef, Show,
    SignalSetter, SignalUpdate, WriteSignal,
};
use leptos::{create_signal, ReadSignal};
use leptos::{event_target_value, IntoAttribute};
use web_sys::{HtmlElement, MouseEvent, ScrollIntoViewOptions, ScrollLogicalPosition};
// use time::{Date};

use crate::components::elevation::Elevation;
use crate::components::select::Select;

pub use chrono::Month;

fn days_in_year_month(year: i32, month: chrono::Month) -> u32 {
    let first_day_of_next_month = NaiveDate::from_ymd_opt(year, month.number_from_month() + 1, 1);
    let overflow_to_next_year = || NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap();
    first_day_of_next_month
        .unwrap_or_else(overflow_to_next_year)
        .pred_opt()
        .unwrap()
        .day()
}

fn get_abbreviated_month_name(month: &chrono::Month) -> &'static str {
    match *month {
        Month::January => "Jan",
        Month::February => "Feb",
        Month::March => "Mar",
        Month::April => "Apr",
        Month::May => "May",
        Month::June => "Jun",
        Month::July => "Jul",
        Month::August => "Aug",
        Month::September => "Sep",
        Month::October => "Oct",
        Month::November => "Nov",
        Month::December => "Dec",
    }
}

#[component]
fn DatePickerHeaderButton(
    state: RwSignal<DatePickerState>,
    text: Signal<String>,
    decrement_action: impl Fn(MouseEvent) + 'static + Clone,
    increment_action: impl Fn(MouseEvent) + 'static + Clone,
    state_to_set: DatePickerState,
) -> impl IntoView {
    let arrows_visible = move || state() == DatePickerState::SelectDay;
    let button_dimmed_style = move || {
        if state() != state_to_set && state() != DatePickerState::SelectDay {
            "opacity: 0.5;"
        } else {
            ""
        }
    };
    let arrows_disabled = move || !arrows_visible();
    view! {
        <IconButton
            button_type=ButtonType::Button
            attr:disabled=arrows_disabled
            on:click=decrement_action.clone()
        >
            {move || arrows_visible().then(|| view! { <Icon name="chevron_left"/> })}
        </IconButton>
        <Button
            button_type=ButtonType::Button
            style=ButtonStyle::Text
            attr:style=button_dimmed_style
            on:click=move |_| {
                if state_to_set == state() {
                    state.set(DatePickerState::SelectDay);
                } else {
                    state.set(state_to_set);
                }
            }
        >

            {text}

            {move || {
                let icon = if state_to_set == state() {
                    "arrow_drop_up"
                } else {
                    "arrow_drop_down"
                };
                let show_dropdown_arrow = state() == DatePickerState::SelectDay
                    || state() == state_to_set;
                show_dropdown_arrow.then(|| view! { <Icon name=icon/> })
            }}

        </Button>
        <IconButton
            button_type=ButtonType::Button
            attr:disabled=arrows_disabled
            on:click=increment_action.clone()
        >
            {move || arrows_visible().then(|| view! { <Icon name="chevron_right"/> })}
        </IconButton>
    }
}

#[component]
fn DatePickerMenuButton(#[prop(into)] value: String, selected: bool) -> impl IntoView {
    let button_node_ref = create_node_ref::<Button>();
    create_effect(move |v| {
        if selected {
            button_node_ref()
                .unwrap()
                .scroll_into_view_with_scroll_into_view_options(
                    ScrollIntoViewOptions::new()
                        .block(ScrollLogicalPosition::Center)
                        .inline(ScrollLogicalPosition::Start),
                );
        }
    });
    view! {
        <button
            type="button"
            class="date-picker-menu-item md-typescale-body-large"
            class:selected=selected
            node_ref=button_node_ref
        >
            <md-ripple></md-ripple>
            <div class="date-picker-menu-icon">
                {if selected { Some(view! { <Icon name="check"/> }) } else { None }}
            </div>
            {value}
        </button>
    }
}

#[component]
fn DayNumber(
    date: NaiveDate,
    date_picker_current_selected_date: RwSignal<NaiveDate>,
    set_currently_viewed_month: WriteSignal<chrono::Month>,
    is_outside_month: bool,
) -> impl IntoView {
    let today = Local::now();
    let is_today: bool = today.date_naive() == date;
    let is_selected = move || date_picker_current_selected_date.get() == date;
    view! {
        <button
            type="button"
            class="day-number md-typescale-body-large"
            class:day-outside-month=is_outside_month
            class:day-today=is_today
            class:day-selected=is_selected
            on:mousedown=move |_| {
                if is_outside_month {
                    set_currently_viewed_month.set(Month::try_from(date.month() as u8).unwrap());
                }
                date_picker_current_selected_date.set(date);
            }
        >

            <md-ripple></md-ripple>

            {date.day()}
        </button>
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum DatePickerState {
    SelectDay,
    SelectMonth,
    SelectYear,
}

const YEARS_TO_SHOW_BEFORE_AFTER_CURRENT: u32 = 10;

/// This is a native Leptos component.
/// See [material.io](https://m3.material.io/components/date-pickers/overview).
#[component]
pub fn DatePicker(
    active: RwSignal<bool>,
    #[prop(optional)] starting_date: Option<NaiveDate>,
    on_confirm: impl FnMut(NaiveDate) + Clone + 'static,
) -> impl IntoView {
    let state = create_rw_signal(DatePickerState::SelectDay);
    let selected_date = create_rw_signal(if let Some(date) = starting_date {
        date
    } else {
        Local::now().date_naive()
    });
    let current_month = create_rw_signal(
        chrono::Month::try_from(selected_date.get_untracked().month() as u8).unwrap(),
    );
    let current_year = create_rw_signal(selected_date.get_untracked().year_ce().1);
    let days_in_month =
        Signal::derive(move || days_in_year_month(current_year.get() as i32, current_month()));
    let days_in_last_month = Signal::derive(move || {
        days_in_year_month(current_year.get() as i32, current_month().pred())
    });
    let first_day_of_month = Signal::derive(move || {
        NaiveDate::from_ymd_opt(
            current_year.get() as i32,
            current_month().number_from_month(),
            1,
        )
        .unwrap()
    });
    let last_month_day_buttons = move || {
        let prev_month = current_month().pred();
        (0..(first_day_of_month.get().weekday().num_days_from_sunday()))
            .rev()
            .map(move |number_of_days_since_sunday| {
                let day = days_in_last_month.get() - number_of_days_since_sunday;
                let date = NaiveDate::from_ymd_opt(
                    current_year.get() as i32,
                    prev_month.number_from_month(),
                    day,
                )
                .unwrap();
                view! {
                    <DayNumber
                        date=date
                        date_picker_current_selected_date=selected_date.clone()
                        set_currently_viewed_month=current_month.write_only()
                        is_outside_month=true
                    />
                }
            })
    };
    let day_buttons = move || {
        (1..days_in_month.get() + 1).map(move |day| {
            let date = NaiveDate::from_ymd_opt(
                current_year.get() as i32,
                current_month.get().number_from_month(),
                day,
            )
            .unwrap();
            view! {
                <DayNumber
                    date=date
                    date_picker_current_selected_date=selected_date.clone()
                    set_currently_viewed_month=current_month.write_only()
                    is_outside_month=false
                />
            }
        })
    };
    let next_month_day_buttons = move || {
        let last_day_of_month = NaiveDate::from_ymd_opt(
            current_year.get() as i32,
            current_month().number_from_month(),
            days_in_month.get(),
        )
        .unwrap();
        let next_month = current_month().succ();
        let remaining_weekdays_after_current_month =
            7 - last_day_of_month.weekday().number_from_sunday();
        (1..remaining_weekdays_after_current_month + 1).map(move |day| {
            let date = NaiveDate::from_ymd_opt(
                current_year.get() as i32,
                next_month.number_from_month(),
                day,
            )
            .unwrap();
            view! {
                <DayNumber
                    date=date
                    date_picker_current_selected_date=selected_date.clone()
                    set_currently_viewed_month=current_month.write_only()
                    is_outside_month=true
                />
            }
        })
    };
    let days = move || {
        last_month_day_buttons()
            .chain(day_buttons())
            .chain(next_month_day_buttons())
            .collect_view()
    };
    let month_options = move || {
        let current_month_num = current_month.get().number_from_month();
        (1..(12 + 1)) // Month::try_from expects one-indexed value. Slices are exclusive at the end, add 1.
            .map(|month_num| {
                let selected = current_month_num == month_num;
                let month_value = Month::try_from(month_num as u8).unwrap();
                view! {
                    <DatePickerMenuButton
                        on:click=move |_| {
                            current_month.set(Month::try_from(month_num as u8).unwrap());
                            state.set(DatePickerState::SelectDay);
                        }

                        value=month_value.name()
                        selected=selected
                    />
                }
            })
            .collect_view()
    };
    let year_options = move || {
        let start_year = current_year();
        let min = start_year - YEARS_TO_SHOW_BEFORE_AFTER_CURRENT;
        let max = start_year + YEARS_TO_SHOW_BEFORE_AFTER_CURRENT;
        (min..(max + 1))
            .map(|year| {
                let selected = start_year == year;
                view! {
                    <DatePickerMenuButton
                        on:click=move |_| {
                            current_year.set(year);
                            state.set(DatePickerState::SelectDay);
                        }

                        value=year.to_string()
                        selected=selected
                    />
                }
            })
            .collect_view()
    };
    let abbreviated_current_month_name =
        Signal::derive(move || get_abbreviated_month_name(&current_month()).to_string());
    let year_str = Signal::derive(move || current_year().to_string());
    let decrement_month_on_click = move |_: MouseEvent| {
        if current_month() == Month::January {
            current_year.set(current_year() - 1);
        }
        current_month.set(current_month().pred());
    };
    let increment_month_on_click = move |_: MouseEvent| {
        if current_month() == Month::December {
            current_year.set(current_year() + 1);
        }
        current_month.set(current_month().succ());
    };
    let decrement_year_on_click = move |_: MouseEvent| {
        current_year.set(current_year() - 1);
    };
    let increment_year_on_click = move |_: MouseEvent| {
        current_year.set(current_year() + 1);
    };
    view! {
        <Show when=active>
            <div class="leptos-material-datepicker">
                <Elevation/>
                <div class="datepicker-header">
                    <DatePickerHeaderButton
                        decrement_action=decrement_month_on_click
                        increment_action=increment_month_on_click
                        text=abbreviated_current_month_name
                        state=state
                        state_to_set=DatePickerState::SelectMonth
                    />
                    <DatePickerHeaderButton
                        decrement_action=decrement_year_on_click
                        increment_action=increment_year_on_click
                        text=year_str
                        state=state
                        state_to_set=DatePickerState::SelectYear
                    />
                </div>
                {move || match state.get() {
                    DatePickerState::SelectDay => {
                        view! {
                            <div class="date-grid">

                                <div class="days-of-week">
                                    <p class="md-typescale-body-large">{"S"}</p>
                                    <p class="md-typescale-body-large">{"M"}</p>
                                    <p class="md-typescale-body-large">{"T"}</p>
                                    <p class="md-typescale-body-large">{"W"}</p>
                                    <p class="md-typescale-body-large">{"T"}</p>
                                    <p class="md-typescale-body-large">{"F"}</p>
                                    <p class="md-typescale-body-large">{"S"}</p>
                                </div>
                                {days}
                            </div>
                        }
                            .into_view()
                    }
                    DatePickerState::SelectMonth => {
                        view! { <div class="date-picker-menu">{month_options}</div> }.into_view()
                    }
                    DatePickerState::SelectYear => {
                        view! { <div class="date-picker-menu">{year_options}</div> }.into_view()
                    }
                }}

                <div class="datepicker-footer">
                    <Button
                        button_type=ButtonType::Button
                        on:click=move |_| {
                            active.set(false);
                        }

                        style=ButtonStyle::Text
                    >
                        "Cancel"
                    </Button>
                    <Button
                        button_type=ButtonType::Button
                        on:click={
                            let mut confirm = on_confirm.clone();
                            move |_| {
                                active.set(false);
                                confirm(selected_date.get());
                            }
                        }

                        style=ButtonStyle::Text
                    >
                        "OK"
                    </Button>
                </div>

            </div>
        </Show>
    }
}
