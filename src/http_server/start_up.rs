use std::net::SocketAddr;
use std::sync::Arc;

use my_http_server::controllers::swagger::SwaggerMiddleware;
use my_http_server::{MyHttpServer, StaticFilesMiddleware};

use crate::app::{APP_NAME, APP_VERSION, AppContext};

// Swagger, then the API controllers, then MCP (/mcp), then the /api 404, then the UI. The static
// files middleware serves ./wwwroot (relative to the working directory) and answers any other path
// with index.html, so the client-side app can own its routes - which is why MCP must come before
// it. Anything under /api that no controller answered is stopped before that point with a 404.
pub fn start_http_server(app: &Arc<AppContext>) {
    let addr = SocketAddr::from(([0, 0, 0, 0], app.settings.get_http_port()));

    let mut http_server = MyHttpServer::new(addr);

    let controllers = Arc::new(super::build_controllers(app));

    let swagger_middleware = Arc::new(SwaggerMiddleware::new(
        controllers.clone(),
        APP_NAME,
        APP_VERSION,
    ));

    let static_files_middleware = Arc::new(
        StaticFilesMiddleware::new()
            .add_index_file("index.html")
            .set_not_found_file("index.html".to_string()),
    );

    http_server.add_middleware(swagger_middleware);
    http_server.add_middleware(controllers);
    http_server.add_middleware(Arc::new(crate::mcp::build_mcp_middleware(app)));
    http_server.add_middleware(Arc::new(super::ApiRouteNotFoundMiddleware));
    http_server.add_middleware(static_files_middleware);

    http_server.start_auto(app.states.clone(), my_logger::LOGGER.clone());

    println!("{APP_NAME} {APP_VERSION} is listening at http://{addr}");
}
