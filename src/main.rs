
mod api_handlers;
//mod api_response;
//mod api_request;

use std::net::IpAddr;
use std::env;
use warp::Filter;
use api_handlers::version;
use api_handlers::HealthCheckResponse;
use api_handlers::QueryMatchCountRequest;
use api_handlers::Handlers;

const HTTP_VERSION_HEADER: &str = "X-Wordless-Api-Version";

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Check if enough arguments are passed (expecting LISTEN_IP and LISTEN_PORT)
    if args.len() != 3 {
        eprintln!("Usage: {} <LISTEN_IP> <LISTEN_PORT>", args[0]);
        eprintln!("(version {})", version());
        return;
    }

    // Parse the IP address and port from command line arguments
    let ip: IpAddr = args[1].parse().expect("Invalid IP address");
    let port: u16 = args[2].parse().expect("Invalid port number");

    let static_files = warp::fs::dir("wwwroot");

    let health_check = warp::path!("api" / "healthcheck")
        .and(warp::get())
        .map(|| {
            let response = HealthCheckResponse::new();
            warp::reply::json(&response)
        });

    let get_random_word = warp::path!("api" / "randomword")
        .and(warp::get())
        .map(|| {
            let response = Handlers::get_daily_word(-1);
            warp::reply::json(&response)
        });

    let get_word = warp::path!("api" / "getword" / i32)
        .and(warp::get())
        .map(| days_ago: i32 | {
            let response = Handlers::get_daily_word(days_ago);
            warp::reply::json(&response)
        });

    let check_word = warp::path!("api" / "checkword" / String)
        .and(warp::get())
        .map(| word : String | {
            let response = Handlers::check_word_exists( &word );
            warp::reply::json(&response)
        });

    let query_matches = warp::path!("api" / "querymatchcount")
        .and(warp::post())
        .and(warp::body::json::<QueryMatchCountRequest>())
        .map(|request_params: QueryMatchCountRequest| {
            let response = Handlers::count_compatible_words(&request_params);
            warp::reply::json(&response)
        });

    let api_version_header_filter = warp::reply::with::header(HTTP_VERSION_HEADER, version());
    let routes = static_files
        .or(health_check)
        .or(get_random_word)
        .or(get_word)
        .or(check_word)
        .or(query_matches)
        .with(api_version_header_filter);

    warp::serve(routes)
        .run((ip, port))
        .await;
}
