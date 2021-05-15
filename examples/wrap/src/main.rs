use iced::{
    button::{self, Button},
    Sandbox, Settings, Text,
};
use iced_aw::Wrap;

fn main() -> iced::Result {
    RandStrings::run(Settings::default())
}

struct RandStrings {
    buttons: Vec<StrButton>,
}
struct StrButton {
    state: button::State,
    str: String,
}

impl iced::Sandbox for RandStrings {
    type Message = ();

    fn new() -> Self {
        let data:Vec<StrButton> = "sad,dasfasfasfafas,das,d,af,ewg,er,g,dsg,dfg,r,eg,re,h,we,fqw,v,wq,fe,wg,r,fqw,f,q,gfre,hyt,ru,yt,j,htr,nj,yj,uy,k,t,jkt,rsh,ea,h,aet,h,aehg".split(',')
        .into_iter()
        .map(|s| StrButton {
            state: button::State::new(),
            str:s.to_string()
        })
        .collect();
        Self { buttons: data }
    }

    fn title(&self) -> String {
        "wrap".to_owned()
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        self.buttons
            .iter_mut()
            .fold(Wrap::new_vertical(), |wrap, button| {
                let StrButton { state, str } = button;
                wrap.push(Button::new(state, Text::new(str.as_str())))
            })
            .align(iced::Align::Center)
            .into()
    }
}
