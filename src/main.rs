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

fn lines_as_numbers(input: &str) -> Vec<i32> {
    return input
        .split("\n")
        .filter_map(|line| line.parse::<i32>().ok())
        .collect();
}

fn elve_calories(input: &str) -> Vec<i32> {
    return input
        .split("\n\n")
        .map(|elve| lines_as_numbers(elve).into_iter().sum())
        .collect();
}

fn task_01_1() -> String {
    let input = include_str!("../inputs/01/input.txt");
    let calories = elve_calories(input);

    let max_calories = calories.into_iter().max().unwrap_or(0);

    return max_calories.to_string();
}

fn task_01_2() -> String {
    let input = include_str!("../inputs/01/input.txt");

    let mut calories = elve_calories(input);
    calories.sort();

    let foo: i32 = calories[calories.len() - 3..calories.len()]
        .into_iter()
        .sum();

    return foo.to_string();
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

        "/01-2" => {
            return Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_PLAIN_UTF_8)
                .with_body(task_01_2()));
        }

        _ => Ok(Response::from_status(StatusCode::NOT_FOUND)
            .with_body_text_plain("The page you requested could not be found\n")),
    }
}
