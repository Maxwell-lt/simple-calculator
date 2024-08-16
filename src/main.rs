use calculator::Calculator;
use calculator::Digit;
use calculator::Operator;
use iced::widget::{button, column, row, text};
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
        column![
            text(self.calculator.get_value()).size(50),
            row![
                button("AC")
                    .padding(30)
                    .on_press(CalcMessage::AllClearPressed),
                button("CE")
                    .padding(30)
                    .on_press(CalcMessage::ClearEntryPressed),
                button("=").padding(30).on_press(CalcMessage::EqualsPressed)
            ],
            row![
                button("7").padding(30).on_press(CalcMessage::SevenPressed),
                button("8").padding(30).on_press(CalcMessage::EightPressed),
                button("9").padding(30).on_press(CalcMessage::NinePressed),
                button("+").padding(30).on_press(CalcMessage::AddPressed),
            ],
            row![
                button("4").padding(30).on_press(CalcMessage::FourPressed),
                button("5").padding(30).on_press(CalcMessage::FivePressed),
                button("6").padding(30).on_press(CalcMessage::SixPressed),
                button("-")
                    .padding(30)
                    .on_press(CalcMessage::SubtractPressed),
            ],
            row![
                button("1").padding(30).on_press(CalcMessage::OnePressed),
                button("2").padding(30).on_press(CalcMessage::TwoPressed),
                button("3").padding(30).on_press(CalcMessage::ThreePressed),
                button("*")
                    .padding(30)
                    .on_press(CalcMessage::MultiplyPressed),
            ],
            row![
                button("0").padding(30).on_press(CalcMessage::ZeroPressed),
                button("/").padding(30).on_press(CalcMessage::DividePressed),
            ]
            .align_items(Alignment::End)
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
