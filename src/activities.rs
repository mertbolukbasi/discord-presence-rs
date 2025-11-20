use serde::Serialize;
use serde_repr::Serialize_repr;

/// Represents a user's activity on Discord.
#[derive(Clone, Debug, Serialize)]
pub struct Activity {
    /// The user's activity status.
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
    /// The user's current party status.
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    /// The assets for the activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    assets: Option<Assets>,
    /// The timestamps for the activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamps: Option<Timestamps>,
    /// The party information for the activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    party: Option<Party>,
    /// The secrets for the activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    secrets: Option<Secrets>,
    /// The buttons for the activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    buttons: Option<Vec<Button>>,
    /// The type of activity.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    activity_type: Option<ActivityType>,
    /// The type of status to display.
    #[serde(skip_serializing_if = "Option::is_none")]
    status_display_type: Option<StatusDisplayType>,
}

/// Represents the assets for an activity.
#[derive(Clone, Debug, Serialize)]
pub struct Assets {
    /// The ID of the large image asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    large_image: Option<String>,
    /// The text that appears when hovering over the large image asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    large_text: Option<String>,
    /// The URL of the large image asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    large_url: Option<String>,
    /// The ID of the small image asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    small_image: Option<String>,
    /// The text that appears when hovering over the small image asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    small_text: Option<String>,
    /// The URL of the small image asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    small_url: Option<String>,
}

/// Represents the timestamps for an activity.
#[derive(Clone, Debug, Serialize)]
pub struct Timestamps {
    /// The start time of the activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<u64>,
    /// The end time of the activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<u64>,
}

/// Represents the party information for an activity.
#[derive(Clone, Debug, Serialize)]
pub struct Party {
    /// The ID of the party.
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    /// The size of the party.
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<[u32; 2]>,
}

/// Represents the secrets for an activity.
#[derive(Clone, Debug, Serialize)]
pub struct Secrets {
    /// The secret for joining a party.
    #[serde(skip_serializing_if = "Option::is_none")]
    join: Option<String>,
    /// The secret for spectating a game.
    #[serde(skip_serializing_if = "Option::is_none")]
    spectate: Option<String>,
    /// Whether this is an instance of a game session.
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<bool>,
    /// The secret for a match.
    #[serde(skip_serializing_if = "Option::is_none")]
    r#match: Option<String>,
}

/// Represents a button for an activity.
#[derive(Clone, Debug, Serialize)]
pub struct Button {
    /// The text on the button.
    label: Option<String>,
    /// The URL the button opens.
    url: Option<String>,
}

/// Represents the type of an activity.
#[repr(u8)]
#[derive(Clone, Debug, Serialize_repr)]
pub enum ActivityType {
    /// The user is playing a game.
    Playing = 0,
    /// The user is streaming.
    Streaming = 1,
    /// The user is listening to something.
    Listening = 2,
    /// The user is watching something.
    Watching = 3,
    /// A custom status.
    Custom = 4,
    /// The user is competing in something.
    Competing = 5,
}

/// Represents the type of status to display.
#[repr(u8)]
#[derive(Clone, Debug, Serialize_repr)]
pub enum StatusDisplayType {
    /// Display the name of the activity.
    Name = 0,
    /// Display the state of the activity.
    State = 1,
    /// Display the details of the activity.
    Details = 2,
}

impl Activity {
    /// Creates a new `Activity`.
    pub fn new() -> Self {
        Activity {
            details: None,
            state: None,
            assets: None,
            timestamps: None,
            party: None,
            secrets: None,
            buttons: None,
            activity_type: None,
            status_display_type: None,
        }
    }

    /// Sets the details of the activity.
    pub fn set_details(mut self, details: String) -> Self {
        self.details = Some(details);
        self
    }

    /// Sets the state of the activity.
    pub fn set_state(mut self, state: String) -> Self {
        self.state = Some(state);
        self
    }

    /// Sets the assets of the activity.
    pub fn set_assets(mut self, assets: Assets) -> Self {
        self.assets = Some(assets);
        self
    }

    /// Sets the timestamps of the activity.
    pub fn set_timestamps(mut self, timestamps: Timestamps) -> Self {
        self.timestamps = Some(timestamps);
        self
    }

    /// Sets the party of the activity.
    pub fn set_party(mut self, party: Party) -> Self {
        self.party = Some(party);
        self
    }

    /// Sets the secrets of the activity.
    pub fn set_secrets(mut self, secrets: Secrets) -> Self {
        self.secrets = Some(secrets);
        self
    }

    /// Sets the buttons of the activity.
    pub fn set_buttons(mut self, buttons: Vec<Button>) -> Self {
        self.buttons = Some(buttons);
        self
    }

    /// Sets the type of the activity.
    pub fn set_activity_type(mut self, activity_type: ActivityType) -> Self {
        self.activity_type = Some(activity_type);
        self
    }

    /// Sets the status display type of the activity.
    pub fn set_status_display_type(mut self, status_display_type: StatusDisplayType) -> Self {
        self.status_display_type = Some(status_display_type);
        self
    }
}

impl Assets {
    /// Creates a new `Assets`.
    pub fn new() -> Self {
        Assets {
            large_image: None,
            large_text: None,
            large_url: None,
            small_image: None,
            small_text: None,
            small_url: None,
        }
    }

    /// Sets the large image of the assets.
    pub fn set_large_image(mut self, large_image: String) -> Self {
        self.large_image = Some(large_image);
        self
    }

    /// Sets the large text of the assets.
    pub fn set_large_text(mut self, large_text: String) -> Self {
        self.large_text = Some(large_text);
        self
    }

    /// Sets the large URL of the assets.
    pub fn set_large_url(mut self, large_url: String) -> Self {
        self.large_url = Some(large_url);
        self
    }

    /// Sets the small image of the assets.
    pub fn set_small_image(mut self, small_image: String) -> Self {
        self.small_image = Some(small_image);
        self
    }

    /// Sets the small text of the assets.
    pub fn set_small_text(mut self, small_text: String) -> Self {
        self.small_text = Some(small_text);
        self
    }

    /// Sets the small URL of the assets.
    pub fn set_small_url(mut self, small_url: String) -> Self {
        self.small_url = Some(small_url);
        self
    }
}

impl Party {
    /// Creates a new `Party`.
    pub fn new() -> Self {
        Party {
            id: None,
            size: None,
        }
    }

    /// Sets the ID of the party.
    pub fn set_id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the size of the party.
    pub fn set_size(mut self, present: u32, size: u32) -> Self {
        self.size = Some([present, size]);
        self
    }
}

impl Timestamps {
    /// Creates a new `Timestamps`.
    pub fn new() -> Self {
        Timestamps {
            start: None,
            end: None,
        }
    }

    /// Sets the start time of the timestamps.
    pub fn set_start(mut self, start: u64) -> Self {
        self.start = Some(start);
        self
    }

    /// Sets the end time of the timestamps.
    pub fn set_end(mut self, end: u64) -> Self {
        self.end = Some(end);
        self
    }
}

impl Secrets {
    /// Creates a new `Secrets`.
    pub fn new() -> Self {
        Secrets {
            join: None,
            spectate: None,
            instance: None,
            r#match: None,
        }
    }

    /// Sets the join secret of the secrets.
    pub fn set_join(mut self, join: String) -> Self {
        self.join = Some(join);
        self
    }

    /// Sets the spectate secret of the secrets.
    pub fn set_spectate(mut self, spectate: String) -> Self {
        self.spectate = Some(spectate);
        self
    }

    /// Sets the instance of the secrets.
    pub fn set_instance(mut self, instance: bool) -> Self {
        self.instance = Some(instance);
        self
    }

    /// Sets the match secret of the secrets.
    pub fn set_match(mut self, r#match: String) -> Self {
        self.r#match = Some(r#match);
        self
    }
}

impl Button {
    /// Creates a new `Button`.
    pub fn new() -> Self {
        Button {
            label: None,
            url: None,
        }
    }

    /// Sets the label of the button.
    pub fn set_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    /// Sets the URL of the button.
    pub fn set_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }
}
