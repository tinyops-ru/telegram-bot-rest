pub mod filters {
    use warp::Filter;

    use crate::config::config::Config;
    use crate::handlers::handlers;

    const HEADER_AUTH_TOKEN: &str = "token";

    pub fn routes(config: Config) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        return send_message(config)
            .or(get_version())
    }

    pub fn send_message(config: Config) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let rest_auth_token = String::from(config.rest_auth_token.clone());
        let authenticated = warp::header::exact(HEADER_AUTH_TOKEN, string_to_static_str(rest_auth_token));

        return  warp::path!("rest" / "send")
            .and(authenticated)
            .and(warp::post())
            .and(with_config(config.clone()))
            .and(warp::body::form())
            .and_then(handlers::send_message)
    }

    pub fn get_version() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        return warp::path!("version")
            .and(warp::get())
            .and_then(handlers::get_version)
    }

    fn with_config(config: Config) -> impl Filter<Extract = (Config,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || config.clone())
    }

    fn string_to_static_str(s: String) -> &'static str {
        Box::leak(s.into_boxed_str())
    }
}