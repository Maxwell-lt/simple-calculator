use calculator::Calculator;
use calculator::Digit;
use calculator::Operator;
use iced::widget::Column;
use iced::widget::Row;
use iced::widget::{button, text};
use iced::{Alignment, Element, Sandbox, Settings};

mod calculator;

fn main() -> iced::Result {
    CalcApp::run(Settings::default())
}

struct CalcApp {
    calculator: Calculator,
}

#[derive(Debug, Clone, Copy)]
enum CalcMessage {
    ZeroPressed,
    OnePressed,
    TwoPressed,
    ThreePressed,
    FourPressed,
    FivePressed,
    SixPressed,
    SevenPressed,
    EightPressed,
    NinePressed,
    EqualsPressed,
    AddPressed,
    SubtractPressed,
    MultiplyPressed,
    DividePressed,
    ClearEntryPressed,
    AllClearPressed,
}

impl Sandbox for CalcApp {
    type Message = CalcMessage;

    fn new() -> Self {
        Self {
            calculator: Calculator::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Simple Calculator")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            CalcMessage::ZeroPressed => self.calculator.put_digit(Digit::D0),
            CalcMessage::OnePressed => self.calculator.put_digit(Digit::D1),
            CalcMessage::TwoPressed => self.calculator.put_digit(Digit::D2),
            CalcMessage::ThreePressed => self.calculator.put_digit(Digit::D3),
            CalcMessage::FourPressed => self.calculator.put_digit(Digit::D4),
            CalcMessage::FivePressed => self.calculator.put_digit(Digit::D5),
            CalcMessage::SixPressed => self.calculator.put_digit(Digit::D6),
            CalcMessage::SevenPressed => self.calculator.put_digit(Digit::D7),
            CalcMessage::EightPressed => self.calculator.put_digit(Digit::D8),
            CalcMessage::NinePressed => self.calculator.put_digit(Digit::D9),
            CalcMessage::EqualsPressed => self.calculator.evaluate(),
            CalcMessage::AddPressed => self.calculator.put_operator(Operator::Addition),
            CalcMessage::SubtractPressed => self.calculator.put_operator(Operator::Subtraction),
            CalcMessage::MultiplyPressed => self.calculator.put_operator(Operator::Multiplication),
            CalcMessage::DividePressed => self.calculator.put_operator(Operator::Division),
            CalcMessage::ClearEntryPressed => self.calculator.clear_entry(),
            CalcMessage::AllClearPressed => self.calculator.clear_all(),
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        Column::new()
            .spacing(5)
            .align_items(Alignment::End)
            .push(text(self.calculator.get_value()).size(50).horizontal_alignment(iced::alignment::Horizontal::Right))
            .push(Row::new()
                .spacing(5)
                .align_items(Alignment::Start)
                .push(button("AC").height(60).width(60).on_press(CalcMessage::AllClearPressed))
                .push(button("CE").height(60).width(60).on_press(CalcMessage::ClearEntryPressed))
                .push(button("+").height(60).width(60).on_press(CalcMessage::AddPressed))
            )
            .push(Row::new()
                .spacing(5)
                .align_items(Alignment::Start)
                .push(button("7").height(60).width(60).on_press(CalcMessage::SevenPressed))
                .push(button("8").height(60).width(60).on_press(CalcMessage::EightPressed))
                .push(button("9").height(60).width(60).on_press(CalcMessage::NinePressed))
                .push(button("-").height(60).width(60).on_press(CalcMessage::SubtractPressed))
            )
            .push(Row::new()
                .spacing(5)
                .align_items(Alignment::Start)
                .push(button("4").height(60).width(60).on_press(CalcMessage::FourPressed))
                .push(button("5").height(60).width(60).on_press(CalcMessage::FivePressed))
                .push(button("6").height(60).width(60).on_press(CalcMessage::SixPressed))
                .push(button("*").height(60).width(60).on_press(CalcMessage::MultiplyPressed))
            )
            .push(Row::new()
                .spacing(5)
                .align_items(Alignment::Start)
                .push(button("1").height(60).width(60).on_press(CalcMessage::OnePressed))
                .push(button("2").height(60).width(60).on_press(CalcMessage::TwoPressed))
                .push(button("3").height(60).width(60).on_press(CalcMessage::ThreePressed))
                .push(button("/").height(60).width(60).on_press(CalcMessage::DividePressed))
            )
            .push(Row::new()
                .spacing(5)
                .align_items(Alignment::Start)
                .push(button("0").height(60).width(60).on_press(CalcMessage::ZeroPressed))
                .push(button("=").height(60).width(125).on_press(CalcMessage::EqualsPressed))
            )
        .into()
    }
}
