use std::time::Duration;

use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;

use chrono::{DateTime, Utc};
use embedded_multi_page_hmi::{
    page::{BasicPage, ShutdownPage, StartupPage, TextPage},
    Interaction, PageBaseInterface, PageInteractionInterface, PageInterface, PageLifetime,
    PageManager, PageNavigation,
};

use embedded_graphics::{
    fonts::{Font6x8, Text},
    geometry::Point,
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Rectangle, Triangle, Line},
    style::{PrimitiveStyleBuilder, TextStyle},
};

use explorer700::explorer700::{
    build_display, Explorer700, GraphicDisplay128x64Monochrome, JoystickState, OnOff,
};

// ** Display implementation **


pub struct Explorer700Display {
    canvas: GraphicDisplay128x64Monochrome,
}

impl Explorer700Display {
    pub fn new() -> Self {
        Explorer700Display {
            canvas: build_display(),
        }
    }

    fn update(&mut self, title: &str, message: &str) {
        let style = PrimitiveStyleBuilder::new()
            .stroke_width(1)
            .stroke_color(BinaryColor::On)
            .build();
        let text_style = TextStyle::new(Font6x8, BinaryColor::On);

        self.canvas.init().unwrap();  // make sure there is nothing on the display anymore

        // Draw the centered title.
        // TODO cut off too long title
        let width = title.len() as i32 * 6;
        Text::new(title, Point::new(64 - width / 2 , 2)).into_styled(text_style).draw(&mut self.canvas).unwrap();

        // Draw a line below the title
        Line::new(Point::new(0, 10), Point::new(127, 10)).into_styled(style).draw(&mut self.canvas).unwrap();

        // TODO wrap text and cut off to multi line
        // Draw the message, left aligned
        Text::new(message, Point::new(2, 14)).into_styled(text_style).draw(&mut self.canvas).unwrap();

        self.canvas.flush().unwrap();
    }
}

pub struct HomePage(pub TextPage);

impl HomePage {
    pub fn new(home_message: &'static str) -> Self {
        HomePage(TextPage {
            basic: BasicPage::new("Home", None),
            text: home_message,
        })
    }
}

impl PageBaseInterface for HomePage {
    fn title(&self) -> &str {
        self.0.basic.title
    }
}

impl PageInterface<Explorer700Display> for HomePage {
    fn display(&self, display_driver: &mut Explorer700Display) {
        display_driver.update(self.0.basic.title, self.0.text);
    }
}

// overwrite the default interaction model for the home page
impl PageInteractionInterface for HomePage {
    fn dispatch(&mut self, interaction: Interaction) -> PageNavigation {
        match interaction {
            Interaction::Action => PageNavigation::NthSubpage(1),
            Interaction::Back => PageNavigation::SystemStop,
            Interaction::Home => PageNavigation::Home,
            Interaction::Next => PageNavigation::Left,
            Interaction::Previous => PageNavigation::SystemStart,
        }
    }
}

impl PageInterface<Explorer700Display> for StartupPage {
    fn display(&self, display_driver: &mut Explorer700Display) {
        display_driver.update(self.0.basic.title, self.0.text);
    }
}