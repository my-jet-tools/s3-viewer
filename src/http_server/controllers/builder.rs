use std::sync::Arc;

use my_http_server::controllers::ControllersMiddleware;

use crate::app::AppContext;

pub fn build_controllers(app: &Arc<AppContext>) -> ControllersMiddleware {
    let mut result = ControllersMiddleware::new(None, None);

    result.register_get_action(Arc::new(super::ListBucketsAction::new(app.clone())));
    result.register_get_action(Arc::new(super::ListObjectsAction::new(app.clone())));
    result.register_get_action(Arc::new(super::DownloadObjectAction::new(app.clone())));

    result
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::app::AppContext;
    use crate::settings::{S3ConnString, SettingsModel};

    // The `http_route` literals can not name the shared constants, so this pins them together:
    // the UI calls the constants, the server must answer on exactly those routes.
    #[test]
    fn get_routes_match_the_shared_route_constants() {
        let bucket = match S3ConnString::parse(
            "Endpoint=https://e;Region=r;AccessKey=a;SecretKey=s;Bucket=b1",
        ) {
            Ok(bucket) => bucket,
            Err(err) => panic!("the test connection string must be valid: {err}"),
        };

        let settings = SettingsModel {
            http_port: 8000,
            buckets: vec![bucket],
        };

        let controllers = super::build_controllers(&Arc::new(AppContext::new(settings)));

        let mut routes: Vec<&str> = controllers
            .get
            .get_actions()
            .iter()
            .map(|action| action.http_route.route.as_str())
            .collect();
        routes.sort_unstable();

        let mut expected = vec![
            rest_api_shared::LIST_BUCKETS_ROUTE,
            rest_api_shared::LIST_OBJECTS_ROUTE,
            rest_api_shared::DOWNLOAD_OBJECT_ROUTE,
        ];
        expected.sort_unstable();

        assert_eq!(routes, expected);
    }
}
