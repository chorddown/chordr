use yew::prelude::*;

#[component]
pub fn AppVersion() -> Html {
    let app_version = format!(
        "{}-{}",
        env!("CARGO_PKG_VERSION"),
        env!("CUNDD_BUILD_REVISION")
    );

    html! {
        <div class="app-version-section">
            <div class="app-version-section-version">
                {"App version: "}<span class="no-wrap">{app_version}</span>
            </div>
        </div>
    }
}
