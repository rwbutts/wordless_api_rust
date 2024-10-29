use std::net::IpAddr;
use std::env;
use warp::Filter;
mod wordless_api;

use wordless_api::HealthCheckResponse;
use wordless_api::GetWordResponse;
use wordless_api::WordExistsResponse;
use wordless_api::QueryMatchCountResponse;
use wordless_api::QueryMatchCountRequest;
use wordless_api::WordlessApi;


#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Check if enough arguments are passed (expecting LISTEN_IP and LISTEN_PORT)
    if args.len() != 3 {
        eprintln!("Usage: {} <LISTEN_IP> <LISTEN_PORT>", args[0]);
        eprintln!("(version {})", WordlessApi::version());
        return;
    }

    // Parse the IP address and port from command line arguments
    let ip: IpAddr = args[1].parse().expect("Invalid IP address");
    let port: u16 = args[2].parse().expect("Invalid port number");


    let static_files = warp::fs::dir("wwwroot");

    let health_check = warp::path!("api" / "healthcheck")
        .and(warp::get())
        .map(|| {
            let response = HealthCheckResponse {
                alive: true,
                version: WordlessApi::version()
            };
            warp::reply::json(&response)
        });

    let get_random_word = warp::path!("api" / "randomword")
        .and(warp::get())
        .map(|| {
            let response = GetWordResponse {
                word: WordlessApi::get_random_word(-1),
                version: WordlessApi::version()
            };
            warp::reply::json(&response)
        });

    let get_word = warp::path!("api" / "getword" / i32)
        .and(warp::get())
        .map(| days_ago: i32 | {
            let response = GetWordResponse {
                word: WordlessApi::get_random_word(days_ago),
                version: WordlessApi::version()
            };
            warp::reply::json(&response)
        });

    let check_word = warp::path!("api" / "checkword" / String)
        .and(warp::get())
        .map(| word : String | {
            let response = WordExistsResponse {
                exists: WordlessApi::word_list().contains( &&*word ),
                version: WordlessApi::version()
            };
            warp::reply::json(&response)
        });

    let query_matches = warp::path!("api" / "querymatchcount")
        .and(warp::post())
        .and(warp::body::json::<QueryMatchCountRequest>())
        .map(|request_params: QueryMatchCountRequest| {
            let response = QueryMatchCountResponse {
                count: WordlessApi::count_compatible_words( WordlessApi::word_list(), &request_params.answer, &request_params.guesses),
                version: WordlessApi::version()
            };
            warp::reply::json(&response)
        });

    // Combine the routes
    let routes = static_files.or(health_check).or(get_random_word).or(get_word).or(check_word).or(query_matches);

    // Start the server
    warp::serve(routes)
        .run((ip, port))
        .await;
}
