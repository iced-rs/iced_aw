use iced::{Element, Length};
use iced_aw::style::number_input::Style;
use iced_aw::NumberInput;
use num_traits::{bounds::Bounded, Num, NumAssignOps};
use std::fmt::Display;
use std::marker::PhantomData;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct NumInput<V, M> {
    pub value: V,
    phantomdata: PhantomData<M>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumInputMessage<V> {
    Change(Result<V, String>),
}

impl<V> NumInputMessage<V>
where
    V: Num + NumAssignOps + PartialOrd + Display + FromStr + Copy + Bounded,
{
    pub fn get_data(&self) -> Result<V, String> {
        match self {
            NumInputMessage::Change(Ok(data)) => Ok(*data),
            NumInputMessage::Change(Err(data)) => Err(data.clone()),
        }
    }
}

impl<V> NumInputMessage<V>
where
    V: Eq + Copy,
{
    pub fn get_enum(&self) -> Result<V, String> {
        match self {
            NumInputMessage::Change(Ok(data)) => Ok(*data),
            NumInputMessage::Change(Err(data)) => Err(data.clone()),
        }
    }
}

impl<'a, V, M> NumInput<V, M>
where
    V: Num + NumAssignOps + PartialOrd + Display + FromStr + Copy + Bounded,
    M: 'a + Clone,
{
    pub fn new(value: V) -> NumInput<V, M>
    where
        V: 'static,
    {
        NumInput {
            value,
            phantomdata: PhantomData,
        }
    }

    pub fn view<F>(
        &self,
        id: usize,
        min: V,
        max: V,
        step: V,
        on_change: F,
        style: Option<Style>,
    ) -> Element<M>
    where
        F: 'static + Fn((usize, NumInputMessage<V>)) -> M + Copy,
        V: 'static,
        M: 'static + Clone,
    {
        let mut input = NumberInput::new(self.value, min..max, NumInputMessage::Change)
            .step(step)
            .width(Length::Shrink);

        if let Some(style) = style {
            input = input.style(move |_theme, _status| style);
        }

        Element::new(input).map(move |i| on_change((id, i)))
    }
}
