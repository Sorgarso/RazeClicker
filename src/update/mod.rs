mod github;
pub mod state;

use state::UpdateStatus;
use std::sync::{Arc, OnceLock, RwLock};
use tokio::runtime::Handle;

const CHANGELOG_URL: &str =
    "https://raw.githubusercontent.com/Sorgarso/RazeClicker/main/CHANGELOG.md";
pub struct Updater {
    handle: Handle,
    status: Arc<RwLock<UpdateStatus>>,
    changelog: Arc<RwLock<String>>,
    ctx: Arc<OnceLock<egui::Context>>,
}

impl Updater {
    pub fn new(handle: Handle) -> Self {
        Self {
            handle,
            status: Arc::new(RwLock::new(UpdateStatus::Idle)),
            changelog: Arc::new(RwLock::new(String::new())),
            ctx: Arc::new(OnceLock::new()),
        }
    }

    pub fn status(&self) -> UpdateStatus {
        self.status.read().unwrap().clone()
    }

    pub fn set_changelog(&self, new_changelog: String) {
        *self.changelog.write().unwrap() = new_changelog;
    }

    pub fn get_changelog(&self) -> String {
        self.changelog.read().unwrap().clone()
    }

    fn set(status: &RwLock<UpdateStatus>, ctx: &OnceLock<egui::Context>, s: UpdateStatus) {
        *status.write().unwrap() = s;
        if let Some(c) = ctx.get() {
            c.request_repaint();
        }
    }

    pub fn check(&self) {
        if matches!(
            self.status(),
            UpdateStatus::Checking | UpdateStatus::Downloading
        ) {
            return;
        }
        let (st, ctx) = (self.status.clone(), self.ctx.clone());
        Self::set(&st, &ctx, UpdateStatus::Checking);
        self.handle.spawn(async move {
            let next = match github::latest().await {
                Ok(Some(f)) => UpdateStatus::Available {
                    version: f.version,
                    url: f.url,
                },
                Ok(None) => UpdateStatus::UpToDate,
                Err(e) => UpdateStatus::Failed(e),
            };
            Self::set(&st, &ctx, next);
        });
    }

    pub fn install(&self) {
        let UpdateStatus::Available { url, .. } = self.status() else {
            return;
        };
        let (st, ctx) = (self.status.clone(), self.ctx.clone());
        Self::set(&st, &ctx, UpdateStatus::Downloading);
        self.handle.spawn(async move {
            match github::download_and_swap(&url).await {
                Ok(exe) => {
                    let _ = std::process::Command::new(exe).arg("--update").spawn();

                    std::process::exit(0);
                }
                Err(e) => Self::set(&st, &ctx, UpdateStatus::Failed(e)),
            }
        });
    }

    pub fn get_changelogs_github(
        &self,
        version: String,
    ) -> tokio::task::JoinHandle<Result<String, String>> {
        self.handle
            .spawn(async move { github::get_changelogs(CHANGELOG_URL, &version).await })
    }
}
