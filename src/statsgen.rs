use crate::sensors::Temperature;
use hostname;
use tera;
use tera_text_filters::snake_case;
use minifier::json::minify;

use log;

pub fn dump(stats: Vec<Temperature>) {
    let mut tera = tera::Tera::default();
    tera.register_filter("snake_case", snake_case);
    let mut ctx = tera::Context::default();
    let hostname = match hostname::get() {
        Ok(h) => h.to_string_lossy().into_owned(),
        Err(_) => "localhost".to_string(),
    };

    ctx.insert("samples", &stats);
    ctx.insert("hostname", &hostname);
    ctx.insert("release", env!("BUILD_TAG"));

    let data = match tera.render_str(include_str!("template/nri-infra.jinja2"), &ctx) {
        Ok(t) => t,
        Err(e) => {
            log::error!("Failed tera render: {}", e);
            ::std::process::exit(-1);
        }
    };

    println!("{}", minify(data.as_str()));
}