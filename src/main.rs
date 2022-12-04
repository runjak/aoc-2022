use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Error, Request, Response};

fn resp_method_not_allowed() -> Response {
    return Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
        .with_header(header::ALLOW, "GET, HEAD")
        .with_body_text_plain("This method is not allowed\n");
}

fn resp_index() -> Response {
    Response::from_status(StatusCode::OK)
        .with_content_type(mime::TEXT_HTML_UTF_8)
        .with_body(include_str!("index.html"))
}

fn task_01_1() -> String {
    let input = include_str!("../inputs/01/input.txt");
    let elves = input.split("\n\n");

    let mut max_calories: i32 = 0;

    for elve in elves {
        let inventory = elve.split("\n");
        let calories = inventory.filter_map(|item| item.parse::<i32>().ok()).sum();

        if max_calories < calories {
            max_calories = calories;
        }
    }

    return max_calories.to_string();
}

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    match req.get_path() {
        "/" => match req.get_method() {
            &Method::GET => {
                return Ok(resp_index());
            }
            &Method::HEAD => {
                return Ok(resp_index());
            }

            &Method::POST => {
                return Ok(Response::from_status(StatusCode::OK)
                    .with_content_type(mime::TEXT_PLAIN_UTF_8)
                    .with_body("Keeping us posted I see :)\n"));
            }

            _ => {
                return Ok(resp_method_not_allowed());
            }
        },

        "/01-1" => {
            return Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_PLAIN_UTF_8)
                .with_body(task_01_1()));
        }

        _ => Ok(Response::from_status(StatusCode::NOT_FOUND)
            .with_body_text_plain("The page you requested could not be found\n")),
    }
}
