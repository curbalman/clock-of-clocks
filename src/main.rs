use iced::time;
use iced::{ widget, Degrees, Element, Fill, Subscription, Vector, Color, Rectangle, Point };
use iced::widget::canvas;
use iced::widget::canvas::{stroke, LineCap, Path, Stroke};


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
        widget::container(widget::row(self.digits
            .iter()
            .map(Digit::view)
            .map(|digit| {
                digit.map(|_| Message::default())
            }))
            .height(Fill)
            .width(Fill)
        )    
        .height(50*6)
        .width(50*24)
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
    clocks: [[MiniClock;4];6],
}

impl Digit {
    const POSITION: [[[Position;4];6];10] = [
        // 0
        [
            [Position::BR, Position::HZ, Position::HZ, Position::BL],
            [Position::VR, Position::BR, Position::BL, Position::VR],
            [Position::VR, Position::VR, Position::VR, Position::VR],
            [Position::VR, Position::VR, Position::VR, Position::VR],
            [Position::VR, Position::UR, Position::UL, Position::VR],
            [Position::UR, Position::HZ, Position::HZ, Position::UL],
        ],
        // 1
        [
            [Position::BR, Position::HZ, Position::BL, Position::NT],
            [Position::UR, Position::BL, Position::VR, Position::NT],
            [Position::NT, Position::VR, Position::VR, Position::NT],
            [Position::NT, Position::VR, Position::VR, Position::NT],
            [Position::BR, Position::UL, Position::UR, Position::BL],
            [Position::UR, Position::HZ, Position::HZ, Position::UL],
        ],
        // 2
        [
            [Position::BR, Position::HZ, Position::HZ, Position::BL],
            [Position::UR, Position::HZ, Position::BL, Position::VR],
            [Position::BR, Position::HZ, Position::UL, Position::VR],
            [Position::VR, Position::BR, Position::HZ, Position::UL],
            [Position::VR, Position::UR, Position::HZ, Position::BL],
            [Position::UR, Position::HZ, Position::HZ, Position::UL],
        ],
        // 3
        [
            [Position::BR, Position::HZ, Position::HZ, Position::BL],
            [Position::UR, Position::HZ, Position::BL, Position::VR],
            [Position::NT, Position::BR, Position::UL, Position::VR],
            [Position::NT, Position::UR, Position::BL, Position::VR],
            [Position::BR, Position::HZ, Position::UL, Position::VR],
            [Position::UR, Position::HZ, Position::HZ, Position::UL],
        ],
        // 4
        [
            [Position::BR, Position::BL, Position::BR, Position::BL],
            [Position::VR, Position::VR, Position::VR, Position::VR],
            [Position::VR, Position::UR, Position::UL, Position::VR],
            [Position::UR, Position::HZ, Position::BL, Position::VR],
            [Position::NT, Position::NT, Position::VR, Position::VR],
            [Position::NT, Position::NT, Position::UR, Position::UL],
        ],
        // 5
        [
            [Position::BR, Position::HZ, Position::HZ, Position::BL],
            [Position::VR, Position::BR, Position::HZ, Position::UL],
            [Position::VR, Position::UR, Position::HZ, Position::BL],
            [Position::UR, Position::HZ, Position::BL, Position::VR],
            [Position::BR, Position::HZ, Position::UL, Position::VR],
            [Position::UR, Position::HZ, Position::HZ, Position::UL],
        ],
        // 6
        [
            [Position::BR, Position::HZ, Position::HZ, Position::BL],
            [Position::VR, Position::BR, Position::HZ, Position::UL],
            [Position::VR, Position::UR, Position::HZ, Position::BL],
            [Position::VR, Position::BR, Position::BL, Position::VR],
            [Position::VR, Position::UR, Position::UL, Position::VR],
            [Position::UR, Position::HZ, Position::HZ, Position::UL],
        ],
        // 7
        [
            [Position::BR, Position::HZ, Position::HZ, Position::BL],
            [Position::UR, Position::HZ, Position::BL, Position::VR],
            [Position::NT, Position::NT, Position::VR, Position::VR],
            [Position::NT, Position::NT, Position::VR, Position::VR],
            [Position::NT, Position::NT, Position::VR, Position::VR],
            [Position::NT, Position::NT, Position::UR, Position::UL],
        ],
        // 8
        [
            [Position::BR, Position::HZ, Position::HZ, Position::BL],
            [Position::VR, Position::BR, Position::BL, Position::VR],
            [Position::VR, Position::UR, Position::UL, Position::VR],
            [Position::VR, Position::BR, Position::BL, Position::VR],
            [Position::VR, Position::UR, Position::UL, Position::VR],
            [Position::UR, Position::HZ, Position::HZ, Position::UL],
        ],
        // 9
        [
            [Position::BR, Position::HZ, Position::HZ, Position::BL],
            [Position::VR, Position::BR, Position::BL, Position::VR],
            [Position::VR, Position::UR, Position::UL, Position::VR],
            [Position::UR, Position::HZ, Position::BL, Position::VR],
            [Position::BR, Position::HZ, Position::UL, Position::VR],
            [Position::UR, Position::HZ, Position::HZ, Position::UL],
        ],
    ];

    fn view(&self) -> Element<SetDigit> {
        // let clock = self.clocks.view().map(|_| SetDigit::default());
        let clock = widget::column(self.clocks
            .iter()
            .map(|row| {
                widget::row(row
                    .iter()
                    .map(|clock| clock.view())
                )
                .into()     // into Element
            })
        );
        let clock =
            Into::<Element<_>>::into(clock)
            .map(|_| SetDigit::default());
        clock
        // let text = widget::text(self.value.to_string()).size(40);
        // widget::column![clock, text].into()
    }

    fn update(&mut self, message: SetDigit) {
        let SetDigit::Value(val) = message;
        if val > 9 { panic!() }
        self.value = val;
        for (i, row) in &mut self.clocks.iter_mut().enumerate() {
            for (j, clock) in &mut row.iter_mut().enumerate() {
                clock.update(Self::POSITION[self.value as usize][i][j]);
            }
        }
    }
}

enum SetDigit {
    Value(u32),
}

impl Default for SetDigit {
    fn default() -> Self {
        Self::Value(0)
    }
}

/****************************************/
/*                                      */
/*              MiniClock               */
/*                                      */
/****************************************/
#[derive(Clone, Copy)]
struct MiniClock {
    position: Position,
}

impl MiniClock {
    fn view(&self) -> Element<Position> {
        widget::canvas(self).height(Fill).width(Fill).into()
    }

    fn update(&mut self, message: Position) {
        self.position = message;
    }
}

impl Default for MiniClock {
    fn default() -> Self {
        Self {
            position: Position::NT,
        }
    }
}

impl<Degrees> canvas::Program<Degrees> for MiniClock {
    type State = ();
    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let center = frame.center();
        frame.translate(Vector::new(center.x, center.y));
        

        let radius = frame.width().min(frame.height()) / 2.0;
        let width = 2_f32;
        let border = Path::circle(Point::ORIGIN, radius);
        let hand = Path::line(Point::ORIGIN, 
                                        Point::new(radius, 0.0));

        let thin_stroke = Stroke {
            width,
            style: stroke::Style::Solid(Color::BLACK),
            line_cap: LineCap::Round,
            ..Stroke::default()
        };

        let wide_stroke = Stroke {
            width: width * 2.0,
            style: stroke::Style::Solid(Color::BLACK),
            line_cap: LineCap::Round,
            ..Stroke::default()
        };
        frame.stroke(&border, thin_stroke);

        frame.push_transform();
        frame.rotate(self.position.direction_0());
        frame.stroke(&hand, wide_stroke);
        frame.pop_transform();

        frame.rotate(self.position.direction_1());
        frame.stroke(&hand, wide_stroke);
        vec![frame.into_geometry()]
    }
}

#[derive(Clone, Copy)]
enum Position {
    HZ      = 0, // Horizontal
    VR      = 1, // Vertical
    UR      = 2, // upper right
    UL      = 3,
    BR      = 4, // bottom right
    BL      = 5,
    NT      = 6, // Neutral
}

impl Position {
    const DIRECTIONS: [[f32; 2]; 7] = [
        [000.0, 180.0],
        [090.0, 270.0],
        [000.0, 270.0],
        [180.0, 270.0],
        [090.0, 000.0],
        [090.0, 180.0],
        [135.0, 135.0],
    ];
    fn direction_0(self) -> Degrees {
        return Degrees(Self::DIRECTIONS[self as usize][0])
    }

    fn direction_1(self) -> Degrees {
        return Degrees(Self::DIRECTIONS[self as usize][1])
    }
}