use crate::ice::ice_server::ICEServer;
use crate::policy::bundle_policy::BundlePolicy;
use crate::policy::ice_transport_policy::ICETransportPolicy;
use crate::policy::rtcp_mux_policy::RTCPMuxPolicy;
use crate::policy::sdp_policy::SdpPolicy;

use dtls::crypto::Certificate;

/// A Configuration defines how peer-to-peer communication via PeerConnection
/// is established or re-established.
/// Configurations may be set up once and reused across multiple connections.
/// Configurations are treated as readonly. As long as they are unmodified,
/// they are safe for concurrent use.
#[derive(Default, Clone)]
pub struct Configuration {
    /// iceservers defines a slice describing servers available to be used by
    /// ICE, such as STUN and TURN servers.
    pub ice_servers: Vec<ICEServer>,

    /// icetransport_policy indicates which candidates the ICEAgent is allowed
    /// to use.
    pub ice_transport_policy: ICETransportPolicy,

    /// bundle_policy indicates which media-bundling policy to use when gathering
    /// ICE candidates.
    pub bundle_policy: BundlePolicy,

    /// rtcp_mux_policy indicates which rtcp-mux policy to use when gathering ICE
    /// candidates.
    pub rtcp_mux_policy: RTCPMuxPolicy,

    /// peer_identity sets the target peer identity for the PeerConnection.
    /// The PeerConnection will not establish a connection to a remote peer
    /// unless it can be successfully authenticated with the provided name.
    pub peer_identity: String,

    /// Certificates describes a set of certificates that the PeerConnection
    /// uses to authenticate. Valid values for this parameter are created
    /// through calls to the GenerateCertificate function. Although any given
    /// DTLS connection will use only one certificate, this attribute allows the
    /// caller to provide multiple certificates that support different
    /// algorithms. The final certificate will be selected based on the DTLS
    /// handshake, which establishes which certificates are allowed. The
    /// PeerConnection implementation selects which of the certificates is
    /// used for a given connection; how certificates are selected is outside
    /// the scope of this specification. If this value is absent, then a default
    /// set of certificates is generated for each PeerConnection instance.
    pub certificates: Vec<Certificate>,

    /// icecandidate_pool_size describes the size of the prefetched ICE pool.
    pub ice_candidate_pool_size: u8,

    /// sdp_policy controls the type of SDP offers accepted by and
    /// SDP answers generated by the PeerConnection.
    pub sdp_policy: SdpPolicy,
}

impl Configuration {
    /// get_iceservers side-steps the strict parsing mode of the ice package
    /// (as defined in https://tools.ietf.org/html/rfc7064) by copying and then
    /// stripping any erroneous queries from "stun(s):" URLs before parsing.
    pub(crate) fn get_ice_servers(&self) -> Vec<ICEServer> {
        let mut ice_servers = self.ice_servers.clone();

        for ice_server in &mut ice_servers {
            for raw_url in &mut ice_server.urls {
                if raw_url.starts_with("stun") {
                    // strip the query from "stun(s):" if present
                    let parts: Vec<&str> = raw_url.split('?').collect();
                    *raw_url = parts[0].to_owned();
                }
            }
        }

        ice_servers
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_configuration_get_iceservers() {
        {
            let expected_server_str = "stun:stun.l.google.com:19302";
            let cfg = Configuration {
                ice_servers: vec![ICEServer {
                    urls: vec![expected_server_str.to_owned()],
                    ..Default::default()
                }],
                ..Default::default()
            };

            let parsed_urls = cfg.get_ice_servers();
            assert_eq!(expected_server_str, parsed_urls[0].urls[0]);
        }

        {
            // ignore the fact that stun URLs shouldn't have a query
            let server_str = "stun:global.stun.twilio.com:3478?transport=udp";
            let expected_server_str = "stun:global.stun.twilio.com:3478";
            let cfg = Configuration {
                ice_servers: vec![ICEServer {
                    urls: vec![server_str.to_owned()],
                    ..Default::default()
                }],
                ..Default::default()
            };

            let parsed_urls = cfg.get_ice_servers();
            assert_eq!(expected_server_str, parsed_urls[0].urls[0]);
        }
    }

    #[test]
    fn test_configuration_json() {
        /*TODO: let j = r#"
            {
                "iceServers": [{"URLs": ["turn:turn.example.org"],
                                "username": "jch",
                                "credential": "topsecret"
                              }],
                "iceTransportPolicy": "relay",
                "bundlePolicy": "balanced",
                "rtcpMuxPolicy": "require"
            }"#;

        conf := Configuration{
            ICEServers: []ICEServer{
                {
                    URLs:       []string{"turn:turn.example.org"},
                    Username:   "jch",
                    Credential: "topsecret",
                },
            },
            ICETransportPolicy: ICETransportPolicyRelay,
            BundlePolicy:       BundlePolicyBalanced,
            RTCPMuxPolicy:      RTCPMuxPolicyRequire,
        }

        var conf2 Configuration
        assert.NoError(t, json.Unmarshal([]byte(j), &conf2))
        assert.Equal(t, conf, conf2)

        j2, err := json.Marshal(conf2)
        assert.NoError(t, err)

        var conf3 Configuration
        assert.NoError(t, json.Unmarshal(j2, &conf3))
        assert.Equal(t, conf2, conf3)*/
    }
}