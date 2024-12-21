use iced::widget::{button, row, text_input};
use iced::Element;
use std::ops::{AddAssign, SubAssign};
use std::str::FromStr;

pub trait NumberConsts {
    const ONE: Self;
}
impl NumberConsts for i32 {
    const ONE: Self = 1;
}

pub fn number_input<TNum>(placeholder: &'static str, value: TNum) -> Element<'static, TNum>
where
    TNum: Sized
        + AddAssign
        + SubAssign
        + FromStr
        + Default
        + Copy
        + NumberConsts
        + ToString
        + 'static,
{
    row![
        text_input(placeholder, &value.to_string()).on_input(move |txt_value| {
            if let Ok(val) = TNum::from_str(&txt_value) {
                val
            } else {
                value
            }
        }),
        button("+").on_press({
            let mut v = value;
            v += TNum::ONE;
            v
        }),
        button("-").on_press({
            let mut v = value;
            v -= TNum::ONE;
            v
        }),
    ]
    .into()
}
