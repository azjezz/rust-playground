use actix_web::HttpResponse;
use tera::{Context, Tera};

pub struct Templating {
    tera: Tera,
    debug: bool,
}

impl Templating {
    fn create(dir: &str, debug: bool) -> Templating {
        let  tera = Tera::new(format!("{}/{}/**/*", env!("CARGO_MANIFEST_DIR"), dir).as_str()).unwrap();

        let templating = Templating {
            tera,
            debug,
        };

        return templating;
    }

    pub fn new(dir: &str, debug: bool) -> Templating {
        Self::create(dir, debug)
    }

    pub fn render(&self, template: &str, context: &Context) -> Result<HttpResponse, actix_web::Error>
    {
        let mut tera = self.tera.clone();

        if self.debug {
            tera.full_reload().expect("Failed to fully reload templates.");
        }

        let body = tera.render(template, context);

        return Ok(HttpResponse::Ok().content_type("text/html").body(body.expect("Unable to render template")));
    }
}