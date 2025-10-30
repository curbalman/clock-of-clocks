use iced::alignment;
use iced::mouse;
use iced::time;
use iced::widget;
use iced::widget::canvas::{stroke, Cache, Geometry, LineCap, Path, Stroke};
use iced::widget::{canvas, container};
use iced::{
    Degrees, Element, Fill, Font, Point, Rectangle, Renderer, Subscription,
    Theme, Vector,
};

pub fn main() -> iced::Result {

    iced::application("A Clock made of Clocks", App::update, App::view)
        .subscription(App::subscription)
        .antialiasing(true)
        .run()
}

struct App {
    now: chrono::DateTime<chrono::Local>,
    digits: [Digit; 6],
}

impl App {
    fn update(&mut self, message: Message) {
        use chrono::Timelike;
        match message {
            Message::Tick(local_time) => {
                self.now = local_time;
            }
        }
        let hour = self.now.hour();
        let minute = self.now.minute();
        let second = self.now.second();
        self.digits[0].update(SetDigit::Value(hour/10));
        self.digits[1].update(SetDigit::Value(hour%10));
        self.digits[2].update(SetDigit::Value(minute/10));
        self.digits[3].update(SetDigit::Value(minute%10));
        self.digits[4].update(SetDigit::Value(second/10));
        self.digits[5].update(SetDigit::Value(second%10));
    }

    fn view(&self) -> Element<Message> {
        widget::row(self.digits
            .iter()
            .map(Digit::view)
            .map(|digit| {
                digit.map(|_| Message::default())
            })
        )
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(time::Duration::from_millis(500))
            .map(|_| Message::Tick(chrono::offset::Local::now()))
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            now: chrono::offset::Local::now(),
            digits: [Digit::default(); 6]
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(chrono::DateTime<chrono::Local>),
}

impl Default for Message {
    fn default() -> Self {
        Self::Tick(chrono::offset::Local::now())
    }
}

/****************************************/
/*                                      */
/*              Digit                   */
/*                                      */
/****************************************/
#[derive(Default, Copy, Clone)]
struct Digit {
    value: u32,
}

enum SetDigit {
    Value(u32),
}

impl Digit {
    fn view(&self) -> Element<SetDigit> {
        widget::text(self.value.to_string()).size(40).into()
    }

    fn update(&mut self, message: SetDigit) {
        let SetDigit::Value(val) = message;
        if val > 9 { panic!() }
        self.value = val;
    }
}



// struct Clock {
//     hand_1: Degrees,
//     hand_2: Degrees,
// }


// impl Clock {
//     fn view<>(&self) -> Element<Degrees> {
//         widget::canvas(Clock{ 
//             hand_1: Degrees(90.0),
//             hand_2: Degrees(135.0)}).into()
//     }

//     fn update(&mut self, _message: Degrees) {}
// }

// impl Default for Clock {
//     fn default() -> Self {
//         Self {
//             hand_1: Degrees(0.0),
//             hand_2: Degrees(0.0),
//         }
//     }
// }

// impl<Degrees> canvas::Program<Degrees> for Clock {
//     type State = ();
//     fn draw(
//         &self,
//         state: &Self::State,
//         renderer: &iced::Renderer,
//         theme: &iced::Theme,
//         bounds: Rectangle,
//         cursor: iced::mouse::Cursor,
//     ) -> Vec<canvas::Geometry> {
//         let mut frame = canvas::Frame::new(renderer, bounds.size());
//         let center = frame.center();
//         frame.translate(Vector::new(center.x, center.y));
        

//         let radius = frame.width().min(frame.height()) / 2.0;
//         let width = 2_f32;
//         let border = Path::circle(Point::ORIGIN, radius);
//         let hand = Path::line(Point::ORIGIN, 
//                                         Point::new(radius, 0.0));

//         let thin_stroke = Stroke {
//             width,
//             style: Style::Solid(Color::BLACK),
//             line_cap: LineCap::Round,
//             ..Stroke::default()
//         };

//         let wide_stroke = Stroke {
//             width: width * 2.0,
//             style: Style::Solid(Color::BLACK),
//             line_cap: LineCap::Round,
//             ..Stroke::default()
//         };
//         frame.stroke(&border, thin_stroke);

//         frame.push_transform();
//         frame.rotate(self.hand_1);
//         frame.stroke(&hand, wide_stroke);
//         frame.pop_transform();

//         frame.rotate(self.hand_2);
//         frame.stroke(&hand, wide_stroke);
//         vec![frame.into_geometry()]
//     }
// }