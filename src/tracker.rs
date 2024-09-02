use url::Url;

/// A source of peers. Can be a [`Tracker`](crate::tracker::Tracker) or a decentralized source.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum PeerSource {
    DHT,
    PEX,
    LSD,
    Tracker(Tracker),
}

/// A centralized variant of a [`Peersource`](crate::tracker::PeerSource).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Tracker {
    scheme: TrackerScheme,
    url: String,
}

impl Tracker {
    pub fn scheme(&self) -> &TrackerScheme {
        &self.scheme
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}

/// A protocol used by a [`Tracker`](crate::tracker::Tracker).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TrackerScheme {
    Websocket,
    Http,
    UDP,
}

/// Error occurred during parsing a [`Tracker`](crate::tracker::Tracker).
#[derive(Clone, Debug, PartialEq)]
pub enum TrackerError {
    InvalidURL { source: url::ParseError },
    InvalidScheme { scheme: String },
}

impl std::fmt::Display for TrackerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrackerError::InvalidURL { source } => write!(f, "Invalid URL: {source}"),
            TrackerError::InvalidScheme { scheme } => write!(f, "Invalid scheme: {scheme}"),
        }
    }
}

impl std::error::Error for TrackerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TrackerError::InvalidURL { source } => Some(source),
            TrackerError::InvalidScheme { scheme: _ } => None,
        }
    }
}

impl From<url::ParseError> for TrackerError {
    fn from(e: url::ParseError) -> TrackerError {
        TrackerError::InvalidURL { source: e }
    }
}

impl PeerSource {
    /// Generate a new PeerSource from a given string URL.
    ///
    /// Only covers the Tracker variant. Other variants should be
    /// instantiated directly.
    pub fn new(url: &str) -> Result<PeerSource, TrackerError> {
        Ok(Tracker::new(url)?.to_peer_source())
    }

    /// Generate a new PeerSource from a given parsed URL.
    ///
    /// Only covers the Tracker variant. Other variants should be
    /// instantiated directly.
    pub fn from_url(url: &Url) -> Result<PeerSource, TrackerError> {
        Ok(Tracker::from_url(url)?.to_peer_source())
    }

    pub fn from_tracker(tracker: &Tracker) -> PeerSource {
        PeerSource::Tracker(tracker.clone())
    }
}

impl Tracker {
    /// Generate a new Tracker from a given string URL.
    pub fn new(url: &str) -> Result<Tracker, TrackerError> {
        let url = Url::parse(url)?;
        Tracker::from_url(&url)
    }

    /// Generate a new Tracker from a parsed URL.
    ///
    /// Will fail if scheme is not "http", "https", "wss" or "udp".
    pub fn from_url(url: &Url) -> Result<Tracker, TrackerError> {
        let scheme = match url.scheme() {
            "http" | "https" => TrackerScheme::Http,
            "wss" => TrackerScheme::Websocket,
            "udp" => TrackerScheme::UDP,
            _ => {
                return Err(TrackerError::InvalidScheme {
                    scheme: url.scheme().to_string(),
                });
            }
        };

        Ok(Tracker {
            scheme,
            url: url.as_str().to_string(),
        })
    }

    /// Turns a centralized Tracker into a wider PeerSource
    pub fn to_peer_source(&self) -> PeerSource {
        PeerSource::from_tracker(self)
    }
}

/// Turn a backend-specific tracker struct into an agnostic [`Tracker`](crate::tracker::Tracker).
pub trait TryIntoTracker {
    fn try_into_tracker(&self) -> Result<Tracker, TrackerError>;
}
