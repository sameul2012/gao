use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
// Logger
use log4rs::encode::pattern::PatternEncoder;

// https://docs.rs/log4rs/latest/log4rs/#

pub fn bee(mystr: &str) {
  // get current date
  // let date = chrono::Utc::now(); // 2023-04-01 09:02:13.559984 UTC
  // println!("{}",date);

  let date = chrono::Utc::now().naive_utc(); // Utc::today();
  let formatted_date = date.format("%Y-%m-%d");
  println!("{}", formatted_date);

  // create log file appender
  let logfile = FileAppender::builder()
    .encoder(Box::new(PatternEncoder::default()))
    // set the file name based on the current date
    .build(format!("log/{}.log", formatted_date))
    .unwrap();

  // add the logfile appender to the config
  let config = Config::builder()
    .appender(Appender::builder().build("logfile", Box::new(logfile)))
    .build(Root::builder().appender("logfile").build(LevelFilter::Info))
    .unwrap();

  // init log4rs
  log4rs::init_config(config).unwrap();
  // let con = mystr;
  log::info!("{}", mystr);
}
