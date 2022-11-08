use near_sdk::env;
use crate::*;

use web4::*;

#[near_bindgen]
impl Contract {
    #[allow(unused_variables)]
    // function to render the frontend using Web4 Protocol
    pub fn web4_get(&self, request: Web4Request) -> Web4Response {
        let path = request.path;

        if path == "/robots.txt" {
            return Web4Response::plain_response("User-agent: *\nDisallow:".to_string());
        }

        let content = if let Some(user_account_id) = request.account_id {
            include_str!("../res/account.html")
                .replace("%ACCOUNT_ID%", user_account_id.as_ref())
        } else {
            include_str!("../res/sign-in.html")
                .replace("%CONTRACT_ID%", env::current_account_id().as_ref())
        };

        return Web4Response::html_response(
            content
                .replace("%FOOTER%", include_str!("../res/footer.inc"))
        );
    }
}