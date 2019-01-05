#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter};

extern crate horaire;

use horaire::timelines::TimeLine;
use horaire::source::transilien::transilien;
use horaire::source::sncf::sncf;
use horaire::source::ratp::ratp;
// use horaire::errors::*;

fn get_time_lines_html<'a, I>(time_lines: I) -> String
where
    I: Iterator<Item = &'a TimeLine>,
{
    let mut strings = time_lines.fold(String::from("<html><head><meta charset=\"UTF-8\"></head><body>"), |acc, ref mut time_line| {
                        acc + &format!("{}<p>", time_line)
                    });
    strings.pop();
    strings.push_str("</body></html>");
    strings
    // time_lines.map(|time_line| format!("{}", time_line)).collect::<Vec<_>>().join("<p>\n")
}

//#[get("/transilien/<station>", format = "text/html")]
fn rt_transilien(station: &str) -> String {
    get_time_lines_html(transilien(station).unwrap().iter())
}

//#[get("/ratp/<line>/<station>", format = "text/html")]
fn rt_ratp(line:&str, station: &str) -> String {
    get_time_lines_html(ratp(line, station).unwrap().iter())
}

//#[get("/sncf/dest/<station>", format = "text/html")]
fn rt_sncf_dest(station: &str) -> String {
    get_time_lines_html(sncf(station, true).unwrap().iter())
}

//#[get("/sncf/arriv/<station>", format = "text/html")]
fn rt_sncf_arriv(station: &str) -> String {
    get_time_lines_html(sncf(station, false).unwrap().iter())
}

fn main() {
    let mut server = Nickel::new();
    server.get("/transilien/:station", middleware!( |request| rt_transilien(request.param("station").unwrap_or(""))));
    server.get("/ratp/:line/:station", middleware!( |request| rt_ratp(request.param("line").unwrap_or(""), request.param("station").unwrap_or(""))));
    server.get("/sncf/dest/:station", middleware!( |request| rt_sncf_dest(request.param("station").unwrap_or(""))));
    server.get("/sncf/arriv/:station", middleware!( |request| rt_sncf_arriv(request.param("station").unwrap_or(""))));
    server.get("**", middleware!("Hello World"));
    server.listen("127.0.0.1:6767").unwrap();
}