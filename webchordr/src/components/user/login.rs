use crate::components::detail_view::DetailView;
use crate::config::Config;
use crate::connection::{ConnectionService, ConnectionStatus};
use crate::helpers::Tri;
use crate::session::{Session, SessionService, SessionUser};
use crate::WebError;
use libchordr::prelude::{Credentials, Error, Password, Username};
use log::info;
use std::convert::TryFrom;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

type LoginStatus = Tri<Session, WebError>;

#[derive(Properties, PartialEq, Clone)]
pub struct LoginProps {
    pub user: SessionUser,
    pub config: Config,
    pub on_success: Callback<Session>,
    pub on_error: Callback<WebError>,
}

pub enum Msg {
    UsernameChange(String),
    PasswordChange(String),
    ChangeLoginStatus(LoginStatus),
    Clicked,
    Submit,
    ChangeConnectionStatus(ConnectionStatus),
}

pub struct Login {
    props: LoginProps,
    link: ComponentLink<Self>,
    username_raw: String,
    password_raw: String,
    username: Tri<Username, Error>,
    password: Tri<Password, Error>,
    login_status: LoginStatus,
    connection_status: ConnectionStatus,
}

impl Component for Login {
    type Message = Msg;
    type Properties = LoginProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            username_raw: "".to_string(),
            password_raw: "".to_string(),
            username: Tri::None,
            password: Tri::None,
            login_status: Tri::None,
            connection_status: ConnectionStatus::OnLine,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UsernameChange(value) => {
                self.username = Username::try_from(&value).into();
                self.username_raw = value;
            }
            Msg::PasswordChange(value) => {
                self.password = Password::try_from(&value).into();
                self.password_raw = value
            }
            Msg::Clicked => info!("Clicked"),
            Msg::Submit => {
                let mut session_service = SessionService::new(self.props.config.clone());
                if self.username.is_some() && self.password.is_some() {
                    let username = match self.username {
                        Tri::Some(ref u) => u.clone(),
                        _ => unreachable!(),
                    };
                    let password = match self.password {
                        Tri::Some(ref u) => u.clone(),
                        _ => unreachable!(),
                    };
                    let change_login_status = self.link.callback(|i| Msg::ChangeLoginStatus(i));

                    spawn_local(async move {
                        let credentials = Credentials::new(username, password);
                        match session_service.try_login(&credentials).await {
                            Ok(u) => {
                                info!("Login successful");
                                let _ = session_service
                                    .set_credentials_in_session_storage(&credentials);
                                change_login_status.emit(LoginStatus::Some(u))
                            }
                            Err(e) => {
                                info!("Login failed: {}", e);
                                change_login_status.emit(LoginStatus::Err(e))
                            }
                        }
                    });
                }
            }

            Msg::ChangeLoginStatus(status) => {
                self.login_status = status.clone();
                match status {
                    Tri::Some(u) => self.props.on_success.emit(u),
                    Tri::None => {}
                    Tri::Err(e) => self.props.on_error.emit(e),
                }
            }

            Msg::ChangeConnectionStatus(status) => {
                self.connection_status = status.clone();
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;

            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        // If the user has just logged in
        if let Tri::Some(session) = &self.login_status {
            let session: &Session = session;
            if let SessionUser::LoggedIn(user) = session.user() {
                return (html! {
                    <DetailView close_uri="#">
                        {format!("Successfully logged in as {}", user.username())}
                    </DetailView>
                }) as Html;
            }
        }

        // If the user is already logged in
        if let SessionUser::LoggedIn(user) = &self.props.user {
            return (html! {
                <DetailView close_uri="#">
                    {format!("Already logged in as {}", user.username())}
                </DetailView>
            }) as Html;
        }

        let username_error = match &self.username {
            Tri::Err(e) => html! { <div class="message error">{format!("{}",e)}</div> },
            _ => html! {},
        };

        let password_error = match &self.password {
            Tri::Err(e) => html! { <div class="message error">{format!("{}",e)}</div> },
            _ => html! {},
        };

        let login_error = match &self.login_status {
            // Tri::Err(e) => html! { <div class="message error">{format!("{}",e)}</div> },
            Tri::Err(_e) => html! { <div class="message error">{"Login incorrect"}</div> },
            _ => html! {},
        };

        let submit = self.link.callback(|e: FocusEvent| {
            e.prevent_default();
            Msg::Submit
        });

        let connection_warning = match self.connection_status {
            ConnectionStatus::OnLine => {
                html! {}
            }
            ConnectionStatus::ServerNotReachable => {
                html! {
                    <div class="message warn">{"The server is not reachable"}</div>
                }
            }
            ConnectionStatus::Offline => {
                html! {
                    <div class="message warn">{"The browser appears to be offline"}</div>
                }
            }
        };

        (html! {
            <DetailView close_uri="#">
                <form onsubmit=submit>
                    <div class="user-login">
                        <div class="form-group user-login-username">
                            <label for="user-login-username">{"Username"}</label>
                            <input type="text"
                                   id="user-login-username"
                                   value=self.username_raw
                                   oninput=self.link.callback(|e: InputData|Msg::UsernameChange(e.value))/>
                            {username_error}
                        </div>
                        <div class="form-group user-login-password">
                            <label for="user-login-password">{"Password"}</label>
                            <input type="password"
                                   id="user-login-password"
                                   value=self.password_raw
                                   oninput=self.link.callback(|e: InputData|Msg::PasswordChange(e.value))
                            />
                            {password_error}
                        </div>
                        {login_error}
                        {connection_warning}
                        <button onclick=self.link.callback(|e|Msg::Clicked)>{"Submit"}</button>
                    </div>
                </form>
            </DetailView>
        }) as Html
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let session_service = SessionService::new(self.props.config.clone());
            let change_login_status = self.link.callback(|i| Msg::ChangeLoginStatus(i));
            let change_connection_status = self.link.callback(|i| Msg::ChangeConnectionStatus(i));
            let connection_service = ConnectionService::new(self.props.config.clone());

            spawn_local(async move {
                if let Ok(u) = session_service.try_from_browser_storage().await {
                    info!("Login successful");
                    change_login_status.emit(LoginStatus::Some(u))
                }
            });
            spawn_local(async move {
                let connection_status = connection_service.get_connection_status().await;
                change_connection_status.emit(connection_status)
            });
        }
    }
}
