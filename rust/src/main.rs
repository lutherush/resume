extern crate tui;
extern crate termion;

use std::io;
use std::thread;
use std::time;
use std::sync::mpsc;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::RawBackend;
use tui::widgets::{Widget, Block, SelectableList, Gauge, Paragraph, Borders, Tabs};
use tui::layout::{Group, Direction, Size, Rect};
use tui::style::{Style, Color, Modifier};

pub struct MyTabs<'a> {
    pub titles: Vec<&'a str>,
    pub selection: usize,
}

impl<'a> MyTabs<'a> {
    pub fn next(&mut self) {
        self.selection = (self.selection + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.selection > 0 {
            self.selection -= 1;
        } else {
            self.selection = self.titles.len() - 1;
        }
    }
}

struct App<'a> {
    size: Rect,
    tabs: MyTabs<'a>
}

enum Event {
    Input(event::Key),
    Tick,
}

fn main() {
    let mut app = App {
        size: Rect::default(),
        tabs: MyTabs {
            titles: vec!["Welcome", "Personal", "Skills", "Experience", "Courses", "Looking For"],
            selection: 0,
        }
    };
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();

    thread::spawn(move || {
        let stdin = io::stdin();
        for c in stdin.keys() {
            let evt = c.unwrap();
            input_tx.send(Event::Input(evt)).unwrap();
            if evt == event::Key::Char('q') {
                break;
            }
        }
    });

    thread::spawn(move || {
        let tx = tx.clone();
        loop {
            tx.send(Event::Tick).unwrap();
            thread::sleep(time::Duration::from_millis(200));
        }
    });

    let backend = RawBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();

    loop {
        let size = terminal.size().unwrap();
        if size != app.size {
            terminal.resize(size).unwrap();
            app.size = size;
        }
        draw(&mut terminal, &app).unwrap();
        let evt = rx.recv().unwrap();
        match evt {
            Event::Input(input) => {
                match input {
                    event::Key::Char('q') => {
                        break;
                    }
                    event::Key::Left => {
                        app.tabs.previous();
                    }
                    event::Key::Right => {
                        app.tabs.next();
                    }
                    _ => {}
                }
            }
            Event::Tick => {}
        }
    }
    terminal.show_cursor().unwrap();
    terminal.clear().unwrap();
}

fn draw(t: &mut Terminal<RawBackend>, app: &App) -> Result<(), io::Error> {

    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Fixed(3), Size::Min(0)])
        .render(t, &app.size, |t, chunks| {
            Tabs::default()
                .block(Block::default().borders(Borders::ALL).title("Tabs"))
                .titles(&app.tabs.titles)
                .style(Style::default().fg(Color::Green))
                .highlight_style(Style::default().fg(Color::Yellow))
                .select(app.tabs.selection)
                .render(t, &chunks[0]);
            match app.tabs.selection {
                0 => {
                    draw_welcome(t, &chunks[1]);
                }
                1 => {
                    draw_personal(t, &chunks[1]);
                }
                2 => {
                    draw_skills(t, &chunks[1]);
                }
                3 => {
                    draw_experience(t, &chunks[1]);
                }
                4 => {
                    draw_education(t, &chunks[1]);
                }
                5 => {
                    draw_looking_for(t, &chunks[1]);
                }
                _ => {}
            };
        });
    try!(t.draw());
    Ok(())
}

fn draw_welcome(t: &mut Terminal<RawBackend>, area: &Rect) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(10), Size::Percent(80), Size::Percent(10)])
        .render(t, area, |t, chunks| {
                Group::default()
                .direction(Direction::Horizontal)
                .sizes(&[Size::Percent(10), Size::Percent(80), Size::Percent(10)])
                .render(t, &chunks[1], |t, chunks| {
                    Paragraph::default()
                    .block(Block::default()
                        .borders(Borders::ALL)
                        .title("Welcome to Velimir Baksa's Curriculum Vitae")
                        .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold)))
                    .wrap(true)
                    .text("\nUse {mod=bold;fg=yellow ←}  and {mod=bold;fg=yellow →}  to move between the tabs.\n\n\
                           Use {mod=bold;fg=yellow q} to exit the application.\n\n\
                           I hope you like it!\n\n\
                           {mod=bold;fg=yellow **Note:} Optimized resolution of the command line is 120x40 characters.{mod=bold;fg=yellow **}\
                           ")
                    .render(t, &chunks[1]);
                });
        });
}

fn draw_personal(t: &mut Terminal<RawBackend>, area: &Rect) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(50), Size::Percent(50)])
        .render(t, area, |t, chunks| {
            Group::default()
            .direction(Direction::Horizontal)
            .sizes(&[Size::Percent(35), Size::Percent(65)])
            .render(t, &chunks[0], |t, chunks| {
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("Information")
                    .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\n{mod=bold;fg=yellow Name:} Velimir Baksa\n\n\
                       {mod=bold;fg=yellow Date of Birth:} 18/02/1986\n\n\
                       {mod=bold;fg=yellow Nationality:} Croatian\n\n\
                       {mod=bold;fg=yellow Location:} Varazdin Croatia\n\n\
                       {mod=bold;fg=yellow Open to relocation within the E.U.}\n\n\
                      ")
                .render(t, &chunks[0]);
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("About me")
                    .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\nI am a DevOps and SecOps specialist with interested in {mod=bold Artificial intelligence} and {mod=bold Systems Programming}.\n\n\
                       Regular attendee of the {mod=bold C++} and {mod=bold Go} MeetUps in Varazdin and Zagreb.\n\n\
                       I have more then 10 years professional experiances in industry and worked 6 years as red team cyber security expert.\n\n\
                       I know java and i choose not to use java due security issues. Please do not force me to use java on work. Thank you.\n\n\
                       I play {mod=bold guitar} and enjoy heavy metal, bossa nova and jazz.\n\n\
                       I enjoy playing Dungeons and Dragons, videogames, go fishing and explore Asian countries.\n\n\
                      ")
                .render(t, &chunks[1]);
            });
            Group::default()
            .direction(Direction::Horizontal)
            .sizes(&[Size::Percent(20), Size::Percent(45), Size::Percent(35)])
            .render(t, &chunks[1], |t, chunks| {
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("Languages")
                    .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\n{mod=bold;fg=yellow Croatian:} Native\n\n\
                       {mod=bold;fg=yellow German:} Fluent, C1\n\n\
                       {mod=bold;fg=yellow English:} Fluent, C1\n\n\
                      ")
                .render(t, &chunks[0]);
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("Studies")
                    .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\n{mod=bold;fg=yellow 2008 - 2013: Mechanical emgineering, University Nporth, Varazdin Croatia}\n\
                       {mod=bold;fg=yellow 2005 - 2008: Chemical engineering, Faculty of chemical engineering, University Zagreb, Zagreb Croatia}.\n\n\
                      ")
                .render(t, &chunks[1]);
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("Contact")
                    .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\n{mod=bold;fg=yellow Email:} velimir.baksa@gmail.com\n\n\
                       {mod=bold;fg=yellow Phone:} 385977576422\n\n\
                       {mod=bold;fg=yellow Website:} https://lutherush.github.io/\n\n\
                       {mod=bold;fg=yellow Twitter:} https://twitter.com/luterus\n\n\
                       {mod=bold;fg=yellow GitHub:} https://github.com/lutherush\n\n\
                       {mod=bold;fg=yellow LinkedIn:} http://linkedin.com/in/velimirbaksa\n\n\
                      ")
                .render(t, &chunks[2]);
            });
        });
}

fn draw_skills(t: &mut Terminal<RawBackend>, area: &Rect) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(35), Size::Percent(35), Size::Percent(30)])
        .render(t, area, |t, chunks| {
            Block::default()
                .borders(Borders::ALL)
                .title("Programming Languages")
                .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold))
                .render(t, &chunks[0]);
            Group::default()
                .direction(Direction::Vertical)
                .margin(1)
                .sizes(&[Size::Fixed(2),Size::Fixed(2),Size::Fixed(2),Size::Fixed(2),Size::Fixed(2),Size::Fixed(2),Size::Fixed(2)])
                .render(t, &chunks[0], |t, chunks| {
                Gauge::default()
                    .block(Block::default().title("Python").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("95 / 100"))
                    .percent(95)
                    .render(t, &chunks[0]);
                Gauge::default()
                    .block(Block::default().title("Go:").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("85 / 100"))
                    .percent(85)
                    .render(t, &chunks[1]);
                Gauge::default()
                    .block(Block::default().title("C++(11/14/17):").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("80 / 100"))
                    .percent(80)
                    .render(t, &chunks[2]);
                Gauge::default()
                    .block(Block::default().title("C:").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("70 / 100"))
                    .percent(70)
                    .render(t, &chunks[3]);
            });
            Block::default()
                .borders(Borders::ALL)
                .title("Operating Systems")
                .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold))
                .render(t, &chunks[1]);
            Group::default()
                .direction(Direction::Vertical)
                .margin(1)
                .sizes(&[Size::Fixed(2),Size::Fixed(2),Size::Fixed(2),Size::Fixed(2),Size::Fixed(2)])
                .render(t, &chunks[1], |t, chunks| {
                Gauge::default()
                    .block(Block::default().title("GNU/Linux:").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("95 / 100"))
                    .percent(95)
                    .render(t, &chunks[0]);
                Gauge::default()
                    .block(Block::default().title("FreeBSD:").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("80 / 100"))
                    .percent(80)
                    .render(t, &chunks[1]);
                Gauge::default()
                    .block(Block::default().title("OpenBSD:").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("70 / 100"))
                    .percent(70)
                    .render(t, &chunks[2]);
            });
            Block::default()
                .borders(Borders::ALL)
                .title("Others")
                .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold))
                .render(t, &chunks[2]);
            Group::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .sizes(&[Size::Fixed(2), Size::Fixed(18),Size::Fixed(2),Size::Fixed(15),Size::Fixed(2),Size::Fixed(13),Size::Fixed(2),Size::Fixed(19),Size::Fixed(2),Size::Fixed(15),Size::Fixed(2),Size::Fixed(16),Size::Fixed(2),Size::Fixed(17),Size::Fixed(2)])
                .render(t, &chunks[2], |t, chunks| {
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("Orchestration").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["Kubernetes", "Docker Compose", "Heroku", "GKE"])
                        .render(t, &chunks[1]);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("Build Systems").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["CMake", "Meson", "Make", "Ninja"])
                        .render(t, &chunks[3]);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("CI/CD").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["Gitlab CI", "Jenkins", "Bamboo", "Travis", "circleCI" ])
                        .render(t, &chunks[5]);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("Analysis").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["Golint", "Go vet", "Clang-sanitizer", "Perf", "Valgrind"])
                        .render(t, &chunks[7]);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("Databases").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["MongoDB", "PostgreSQL", "SQLite", "CockroachDB", "MariaDB", "MySQL"])
                        .render(t, &chunks[9]);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("Message Broker").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["RabbitMQ", "Kafka"])
                        .render(t, &chunks[11]);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("Hypervisors").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["KVM", "VirtualBox", "VMWare", "OpenBSD's VMM"])
                        .render(t, &chunks[13]);
            });
    });
}

fn draw_experience(t: &mut Terminal<RawBackend>, area: &Rect) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(30), Size::Percent(30),Size::Percent(20), Size::Percent(20)])
        .render(t, area, |t, chunks| {
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("April 2020 - august 2020: Cloud architect , Appon GmbH, Frankfurt Germany")
                    .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\n{mod=bold;fg=yellow High Performance Platform:} Implemented new functionalities that serve thousands of certificates per second. Took care of the migration of backend storage from MongoDB to PostgreSQL.\n\n\
As a side project I developed an static analysis tool to enforce the code style used in the company.\n\n\
                      ")
                .render(t, &chunks[0]);
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("2011 - 2018: AI developer, Asgard Technology LLC, Pasadena, CA, USA")
                    .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\n{mod=bold;fg=yellow Network library:} Allows the user to interact with Boost ASIO through the std::stream interface.\n\n\
                       {mod=bold;fg=yellow Tools:} Improve the toolchain used internally to allow the usage of clang-tidy for the linting of our code. Another tool allows to switch between several versions of the toolchain without having to change the environment.\n\n\
                       {mod=bold;fg=yellow Backend development:} Several activities regarding the extension and implementation of new features in the backend of the product such as Authentication and GStreamer sinks.\n\n\
                      ")
                .render(t, &chunks[1]);
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("2009 - 2011: Red Team cyber security expert, Modulo FZO, Dubai, UAE")
                    .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\n{mod=bold;fg=yellow Static analysis of code:} Checks if the code has some patterns that have an undefined behaviour, unspecified in the Standard, and/or implementation-defined.\n\n\
                       {mod=bold;fg=yellow Dataflow analysis of code:} Checks the complexity of methods, pointer problems, memory handling, etc.\n\n\
                       Took over two projects to refactor, maintain and add new features.\n\n\
                       Exploring security treats and holes in code and infrastrucitrem explorin physical server security/ \n\n\
                      ")
                .render(t, &chunks[2]);
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("2004 - 2011: Linux kernel developer")
                    .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\n{mod=bold;fg=yellow USB device drivers:} Allows  users to mount various USB devices.\n\n\
                       {mod=bold;fg=yellow File system:} work on EXT4 and BTRFS file systems.\n\n\
                      ")
                .render(t, &chunks[3]);
    });
}

fn draw_education(t: &mut Terminal<RawBackend>, area: &Rect) {
    Group::default()
        .direction(Direction::Vertical)
        .margin(1)
        .sizes(&[Size::Percent(18), Size::Percent(18),Size::Percent(19), Size::Percent(15),Size::Percent(15),Size::Percent(15)])
        .render(t, area, |t, chunks| {
        Paragraph::default()
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Flight Vehicle Aerodynamics (From MIt) - August 2012")
            .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
        .wrap(true)
        .text("\nLearnt how to design and build modern era flight vehicles")
        .render(t, &chunks[0]);
        Paragraph::default()
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Tensor Properties of Materials (from Stanford) - April 2013")
            .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
        .wrap(true)
        .text("\nLearn about tensors calculations and material properties.")
        .render(t, &chunks[1]);
        Paragraph::default()
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Machine learning (from Stanford) - March 2009")
            .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
        .wrap(true)
        .text("\nAdvance machine learning.")
        .render(t, &chunks[2]);
        Paragraph::default()
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Agile for Developers (from Accelebrate) - August 2015")
            .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
        .wrap(true)
        .text("\nTeaches intermediate and advanced object-oriented developers the practices of Agile and Scrum.")
        .render(t, &chunks[3]);
        Paragraph::default()
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Algorithms, Part II (from Coursera) - November 2014")
            .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
        .wrap(true)
        .text("\nFocuses on graph, and string processing algorithms.")
        .render(t, &chunks[4]);
        Paragraph::default()
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Algorithms, Part I (from Coursera) - September 2014")
            .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
        .wrap(true)
        .text("\nCovers elementary data structures, sorting, and searching algorithms.")
        .render(t, &chunks[5]);
    });
}

fn draw_looking_for(t: &mut Terminal<RawBackend>, area: &Rect) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(10), Size::Percent(80), Size::Percent(10)])
        .render(t, area, |t, chunks| {
                Group::default()
                .direction(Direction::Horizontal)
                .sizes(&[Size::Percent(10), Size::Percent(80), Size::Percent(10)])
                .render(t, &chunks[1], |t, chunks| {
                    Paragraph::default()
                    .block(Block::default()
                        .borders(Borders::ALL)
                        .title("What I am looking for?")
                        .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold)))
                    .wrap(true)
                    .text("\n{mod=bold;fg=yellow I am currently looking for new opportunities}\n\n\n\
                           My ideal roles involve a combination of the following:\n\n\
                           \t* Work on automotive and aviation automation.\n\
                           \t* Design, develop and maintain a high performance and reliable systems.\n\
                           \t* Work on next level robotics and automation systems.\n\
                           \t* Create and improve the tools used during the development process.\n\
                           \t* Work in the internals of Operating Systems such as GNU/Linux and FreeBSD.\n\
                           \t* Work on compilers and/or interpreters and designing programming languages.\n\
                           \t* Contribute to Open Source software.\
                           ")
                    .render(t, &chunks[1]);
                });
        });
}
