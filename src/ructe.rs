use rocket::http::Status;
use rocket::response::content::Html as HtmlContent;
use rocket::response::Responder;
use rocket::{Request, Response};

pub struct Ructe {
    pub content: Vec<u8>,
    pub bad_request: bool,
}

impl<'r> Responder<'r, 'static> for Ructe {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        let mut response = &mut Response::build_from(HtmlContent(self.content).respond_to(req)?);

        if self.bad_request {
            response = response.status(Status::BadRequest);
        }

        response.ok()
    }
}

#[macro_export]
macro_rules! render {
    ($group:tt::$page:tt($($param:expr),*)) => {
        {
            use crate::templates;

            let mut res = Vec::new();
            templates::$group::$page(
                &mut res,
                $(
                    $param
                ),*
            ).unwrap();

            Ructe {
                content: res,
                bad_request: false,
            }
        }
    };
    ($group:tt::$page:tt($page_context:expr; $($param:expr),*)) => {
        {
            use crate::templates;

            let page_context = $page_context;
            let bad_request = page_context.flash_bad_request;

            let mut res = Vec::new();
            templates::$group::$page(
                &mut res,
                page_context,
                $(
                    $param
                ),*
            ).unwrap();

            Ructe {
                content: res,
                bad_request,
            }
        }
    };
}
